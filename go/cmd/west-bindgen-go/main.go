package main

import (
	"bytes"
	"flag"
	"fmt"
	"go/ast"
	"go/format"
	"go/printer"
	"go/token"
	"go/types"
	"log"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"golang.org/x/tools/go/packages"
)

var (
	output  = flag.String("output", "west_bindings_test.go", "output file path from the root of the package directory")
	gofmt   = flag.Bool("gofmt", true, "whether to format the generated code")
	pkgName = flag.String("package", "", "package name, defaults to test package at path specified")

	unsafePointerTy = &ast.SelectorExpr{
		X:   &ast.Ident{Name: "unsafe"},
		Sel: &ast.Ident{Name: "Pointer"},
	}
	instanceTy = &ast.SelectorExpr{
		X:   &ast.Ident{Name: "west"},
		Sel: &ast.Ident{Name: "Instance"},
	}
	instancePtrTy = &ast.StarExpr{X: instanceTy}

	pinnerVar   = &ast.Ident{Name: "__p"}
	errVar      = &ast.Ident{Name: "__err"}
	instanceVar = &ast.Ident{Name: "__instance"}

	ptrVar = &ast.Ident{Name: "ptr"}

	pinMethod = &ast.SelectorExpr{
		X:   pinnerVar,
		Sel: &ast.Ident{Name: "Pin"},
	}

	normalizer = strings.NewReplacer("-", "___", "/", "__", ".", "_")
)

func init() {
	log.SetFlags(0)
	flag.Parse()
}

type wasmImport struct {
	instance string
	name     string
	decl     *ast.FuncDecl
}

func importType(fs *token.FileSet, imports map[string]*ast.Ident, expr *ast.Expr, ty types.Type) error {
	switch ty := ty.(type) {
	case *types.Basic:
		return nil
	case *types.Pointer:
		switch e := (*expr).(type) {
		case *ast.StarExpr:
			return importType(fs, imports, &e.X, ty.Elem())
		default:
			pos := fs.Position(e.Pos())
			return fmt.Errorf("%s:%d: unexpected pointer type expression AST type %T for type %s", pos.Filename, pos.Line, e, ty)
		}
	case *types.Array:
		switch e := (*expr).(type) {
		case *ast.ArrayType:
			return importType(fs, imports, &e.Elt, ty.Elem())
		default:
			pos := fs.Position(e.Pos())
			return fmt.Errorf("%s:%d: unexpected array type expression AST type %T for type %s", pos.Filename, pos.Line, e, ty)
		}
	case *types.Struct:
		n := ty.NumFields()
		if n == 0 {
			return nil
		}
		pos := fs.Position((*expr).Pos())
		return fmt.Errorf("%s:%d: anonymous structs are not currently supported", pos.Filename, pos.Line)

	case *types.Named:
		pkg := ty.Obj().Pkg()
		if pkg != nil {
			path := pkg.Path()
			imports[path] = &ast.Ident{Name: normalizer.Replace(path)}
		}
		tys := ty.TypeArgs()
		switch e := (*expr).(type) {
		case *ast.Ident:
			if pkg != nil {
				*expr = &ast.SelectorExpr{
					X:   imports[pkg.Path()],
					Sel: e,
				}
			}
			return nil
		case *ast.SelectorExpr:
			if pkg != nil {
				e.X = imports[pkg.Path()]
			}
			return nil
		case *ast.IndexExpr:
			switch e := e.X.(type) {
			case *ast.Ident:
				if pkg != nil {
					*expr = &ast.SelectorExpr{
						X:   imports[pkg.Path()],
						Sel: e,
					}
				}
			case *ast.SelectorExpr:
				if pkg != nil {
					e.X = imports[pkg.Path()]
				}
			default:
				pos := fs.Position(e.Pos())
				return fmt.Errorf("%s:%d: unexpected named type expression AST index type %T for type %s", pos.Filename, pos.Line, e, ty)
			}
			n := tys.Len()
			if n != 1 {
				pos := fs.Position(e.Pos())
				return fmt.Errorf("%s:%d: mismatched type argument count %d in index AST expression for type %s", pos.Filename, pos.Line, n, ty)
			}
			return importType(fs, imports, &e.Index, tys.At(0))

		case *ast.IndexListExpr:
			switch e := e.X.(type) {
			case *ast.Ident:
				if pkg != nil {
					*expr = &ast.SelectorExpr{
						X:   imports[pkg.Path()],
						Sel: e,
					}
				}
			case *ast.SelectorExpr:
				if pkg != nil {
					e.X = imports[pkg.Path()]
				}
			default:
				pos := fs.Position(e.Pos())
				return fmt.Errorf("%s:%d: unexpected named type expression AST index type %T for type %s", pos.Filename, pos.Line, e, ty)
			}
			n := tys.Len()
			if n != len(e.Indices) {
				pos := fs.Position(e.Pos())
				return fmt.Errorf("%s:%d: mismatched type argument count %d in index list AST expression for type %s", pos.Filename, pos.Line, len(e.Indices), ty)
			}
			for i := 0; i < n; i++ {
				if err := importType(fs, imports, &e.Indices[i], tys.At(i)); err != nil {
					return err
				}
			}
			return nil
		default:
			pos := fs.Position(e.Pos())
			return fmt.Errorf("%s:%d: unexpected named type expression AST type %T for type %s", pos.Filename, pos.Line, e, ty)
		}
	default:
		log.Printf("unhandled type: %T", ty)
		return nil
	}
}

func generate(path string) error {
	fpath := filepath.Join(path, *output)
	if err := os.RemoveAll(fpath); err != nil {
		return fmt.Errorf("failed to remove file at `%s`: %w", fpath, err)
	}
	pkgs, err := packages.Load(&packages.Config{
		Mode: packages.NeedName |
			packages.NeedFiles |
			packages.NeedImports |
			packages.NeedDeps |
			packages.NeedTypes |
			packages.NeedTypesInfo |
			packages.NeedSyntax,
	}, path)
	if err != nil {
		return fmt.Errorf("failed to import `%s`: %w", path, err)
	}
	if len(pkgs) != 1 {
		return fmt.Errorf("loaded unexpected amount of packages from `%s`, expected 1, got %d", path, len(pkgs))
	}
	pkg := pkgs[0]

	file, err := os.Create(fpath)
	if err != nil {
		return fmt.Errorf("failed to create file at `%s`: %w", fpath, err)
	}
	defer file.Close()

	allPkgs := make(map[string]*packages.Package, len(pkg.Imports))
	imports := pkg.Imports
	for len(imports) > 0 {
		next := make(map[string]*packages.Package, len(imports))
		for p, pkg := range imports {
			if _, ok := allPkgs[p]; ok {
				continue
			}
			allPkgs[p] = pkg
			for p, pkg := range pkg.Imports {
				if _, ok := allPkgs[p]; ok {
					continue
				}
				next[p] = pkg
			}
		}
		imports = next
	}

	goImports := map[string]*ast.Ident{}
	var wasmImports []wasmImport
	for _, pkg := range allPkgs {
		for _, f := range pkg.Syntax {
			for _, decl := range f.Decls {
				decl, ok := decl.(*ast.FuncDecl)
				if !ok || decl.Body != nil || decl.Doc == nil {
					continue
				}

				var dir *ast.Comment
				for _, doc := range decl.Doc.List {
					if !strings.HasPrefix(doc.Text, "//go:wasmimport") {
						continue
					}
					dir = doc
					break
				}
				if dir == nil {
					continue
				}

				instance, name, ok := strings.Cut(strings.TrimPrefix(dir.Text, "//go:wasmimport "), " ")
				if !ok {
					pos := pkg.Fset.Position(dir.Pos())
					return fmt.Errorf("%s:%d: unexpected `go:wasmimport` directive format: %s", pos.Filename, pos.Line, dir.Text)
				}

				var callArgs []ast.Expr
				appendArg := func(arg *ast.Field) error {
					ty, ok := pkg.TypesInfo.Types[arg.Type]
					if !ok {
						pos := pkg.Fset.Position(arg.Type.Pos())
						return fmt.Errorf("%s:%d: unknown type: %s", pos.Filename, pos.Line, arg.Type)
					}
					_, isPtr := ty.Type.(*types.Pointer)
					if err := importType(pkg.Fset, goImports, &arg.Type, ty.Type); err != nil {
						return err
					}
					var args []ast.Expr
					for _, name := range arg.Names {
						args = append(args, name)

						callExprs := []ast.Expr{name}
						if !isPtr {
							callExprs = []ast.Expr{
								&ast.UnaryExpr{
									Op: token.AND,
									X:  name,
								},
							}
						}
						callArgs = append(callArgs, &ast.CallExpr{
							Fun: &ast.FuncLit{
								Type: &ast.FuncType{
									Results: &ast.FieldList{List: []*ast.Field{{
										Type: unsafePointerTy,
									}}},
								},
								Body: &ast.BlockStmt{
									List: []ast.Stmt{
										&ast.AssignStmt{
											Lhs: []ast.Expr{ptrVar},
											Tok: token.DEFINE,
											Rhs: []ast.Expr{
												&ast.CallExpr{
													Fun:  unsafePointerTy,
													Args: callExprs,
												},
											},
										},
										&ast.ExprStmt{
											X: &ast.CallExpr{
												Fun:  pinMethod,
												Args: []ast.Expr{ptrVar},
											},
										},
										&ast.ReturnStmt{Results: []ast.Expr{ptrVar}},
									},
								},
							},
						})
					}
					return nil
				}

				if decl.Type.Params != nil {
					for _, p := range decl.Type.Params.List {
						if err := appendArg(p); err != nil {
							return err
						}
					}
				}
				if decl.Type.Results != nil {
					for _, r := range decl.Type.Results.List {
						if err := appendArg(r); err != nil {
							return err
						}
					}
				}
				decl.Doc.List = []*ast.Comment{{
					Text: fmt.Sprintf(`//go:linkname %s %s.%s`, decl.Name.Name, pkg.PkgPath, decl.Name.Name),
				}}
				decl.Body = &ast.BlockStmt{
					List: []ast.Stmt{
						&ast.DeclStmt{
							Decl: &ast.GenDecl{
								Tok: token.VAR,
								Specs: []ast.Spec{
									&ast.ValueSpec{
										Names: []*ast.Ident{pinnerVar},
										Type: &ast.SelectorExpr{
											X:   &ast.Ident{Name: "runtime"},
											Sel: &ast.Ident{Name: "Pinner"},
										},
									},
								},
							},
						},
						&ast.DeferStmt{
							Call: &ast.CallExpr{
								Fun: &ast.SelectorExpr{
									X:   pinnerVar,
									Sel: &ast.Ident{Name: "Unpin"},
								},
							},
						},
						&ast.IfStmt{
							Init: &ast.AssignStmt{
								Lhs: []ast.Expr{
									errVar,
								},
								Tok: token.DEFINE,
								Rhs: []ast.Expr{
									&ast.CallExpr{
										Fun: &ast.SelectorExpr{
											X:   &ast.Ident{Name: "west"},
											Sel: &ast.Ident{Name: "WithCurrentInstance"},
										},
										Args: []ast.Expr{
											&ast.FuncLit{
												Type: &ast.FuncType{
													Params: &ast.FieldList{
														List: []*ast.Field{
															{
																Names: []*ast.Ident{instanceVar},
																Type:  instancePtrTy,
															},
														},
													},
													Results: &ast.FieldList{
														List: []*ast.Field{
															{
																Type: &ast.Ident{Name: "error"},
															},
														},
													},
												},
												Body: &ast.BlockStmt{
													List: []ast.Stmt{
														&ast.ReturnStmt{
															Results: []ast.Expr{
																&ast.CallExpr{
																	Fun: &ast.SelectorExpr{
																		X:   instanceVar,
																		Sel: &ast.Ident{Name: "Call"},
																	},
																	Args: append(
																		[]ast.Expr{
																			&ast.BasicLit{Kind: token.STRING, Value: fmt.Sprintf(`"%s"`, instance)},
																			&ast.BasicLit{Kind: token.STRING, Value: fmt.Sprintf(`"%s"`, name)},
																		},
																		callArgs...,
																	),
																},
															},
														},
													},
												},
											},
										},
									},
								},
							},
							Cond: &ast.BinaryExpr{
								X:  errVar,
								Op: token.NEQ,
								Y:  &ast.Ident{Name: "nil"},
							},
							Body: &ast.BlockStmt{
								List: []ast.Stmt{
									&ast.ExprStmt{
										X: &ast.CallExpr{
											Fun: &ast.CallExpr{
												Fun: &ast.SelectorExpr{
													X:   &ast.Ident{Name: "west"},
													Sel: &ast.Ident{Name: "CurrentErrorHandler"},
												},
											},
											Args: []ast.Expr{
												errVar,
											},
										},
									},
								},
							},
						},
						&ast.ReturnStmt{},
					},
				}

				i := sort.Search(len(wasmImports), func(i int) bool {
					if instance == wasmImports[i].instance {
						return name <= wasmImports[i].name
					}
					return instance <= wasmImports[i].instance
				})
				wi := wasmImport{
					instance: instance,
					name:     name,
					decl:     decl,
				}
				if i < len(wasmImports) {
					wasmImports = append(wasmImports[:i], append([]wasmImport{wi}, wasmImports[i:]...)...)
				} else if i == len(wasmImports) {
					wasmImports = append(wasmImports, wi)
				}
			}
		}
	}
	if len(wasmImports) == 0 {
		log.Println("no `wasmimport` directives found, skip generation")
		return nil
	}
	name := *pkgName
	if name == "" {
		name = pkg.Name
		if pkg.Name != "main" {
			name = fmt.Sprintf("%s_test", pkg.Name)
		}
	}
	importSpecs := []*ast.ImportSpec{
		{
			Path: &ast.BasicLit{Kind: token.STRING, Value: `"runtime"`},
		},
		{
			Path: &ast.BasicLit{Kind: token.STRING, Value: `"unsafe"`},
		},
		{
			Name: &ast.Ident{Name: "west"},
			Path: &ast.BasicLit{Kind: token.STRING, Value: `"github.com/rvolosatovs/west/go"`},
		},
	}
	for path, name := range goImports {
		importSpecs = append(importSpecs, &ast.ImportSpec{
			Name: name,
			Path: &ast.BasicLit{Kind: token.STRING, Value: fmt.Sprintf(`"%s"`, path)},
		})
	}
	importGenSpecs := make([]ast.Spec, len(importSpecs))
	for i := range importSpecs {
		importGenSpecs[i] = importSpecs[i]
	}
	decls := []ast.Decl{
		&ast.GenDecl{
			Tok:   token.IMPORT,
			Specs: importGenSpecs,
		},
		&ast.GenDecl{
			Tok: token.CONST,
			Specs: []ast.Spec{
				&ast.ValueSpec{
					Names: []*ast.Ident{{Name: "_"}},
					Type:  &ast.Ident{Name: "string"},
					Values: []ast.Expr{
						&ast.SelectorExpr{
							X:   &ast.Ident{Name: "runtime"},
							Sel: &ast.Ident{Name: "Compiler"},
						},
					},
				},
			},
		},
		&ast.GenDecl{
			Tok: token.VAR,
			Specs: []ast.Spec{
				&ast.ValueSpec{
					Names: []*ast.Ident{{Name: "_"}},
					Type:  unsafePointerTy,
				},
			},
		},
	}
	for _, wi := range wasmImports {
		decls = append(decls, wi.decl)
	}
	var buf bytes.Buffer
	buf.WriteString("// Code generated by west-bindgen-go DO NOT EDIT\n\n")
	f := &ast.File{
		Name:    &ast.Ident{Name: name},
		Imports: importSpecs,
		Decls:   decls,
	}
	ast.SortImports(pkg.Fset, f)
	write := printer.Fprint
	if *gofmt {
		write = format.Node
	}
	if err := write(&buf, pkg.Fset, f); err != nil {
		return fmt.Errorf("failed to write AST: %w", err)
	}
	b := buf.Bytes()
	if *gofmt {
		b, err = format.Source(b)
		if err != nil {
			return fmt.Errorf("failed to format generated code: %w", err)
		}
	}

	_, err = file.Write(b)
	if err != nil {
		return fmt.Errorf("failed to write file: %w", err)
	}
	return nil
}

func run() error {
	args := flag.Args()
	if len(args) == 0 {
		args = []string{"."}
	}
	for _, path := range args {
		if err := generate(path); err != nil {
			return fmt.Errorf("failed to generate bindings for `%s`:\n%w", path, err)
		}
	}
	return nil
}

func main() {
	if err := run(); err != nil {
		log.Fatalf("failed to run: %s", err)
	}
}
