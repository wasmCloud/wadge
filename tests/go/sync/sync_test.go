//go:generate go run go.wasmcloud.dev/wadge/cmd/wadge-bindgen-go -test
//go:generate cargo build -p sync-test-component --target wasm32-unknown-unknown
//go:generate cp ../../../target/wasm32-unknown-unknown/debug/sync_test_component.wasm component.wasm

package sync_test

import (
	_ "embed"
	"log"
	"log/slog"
	"os"
	"testing"
	"unsafe"

	"github.com/bytecodealliance/wasm-tools-go/cm"
	"github.com/stretchr/testify/assert"
	"go.wasmcloud.dev/wadge"
	"go.wasmcloud.dev/wadge/tests/go/sync/bindings/wadge-test/sync/sync"
)

//go:embed component.wasm
var component []byte

func init() {
	log.SetFlags(0)
	slog.SetDefault(slog.New(slog.NewTextHandler(os.Stderr, &slog.HandlerOptions{
		Level: slog.LevelDebug, ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey {
				return slog.Attr{}
			}
			return a
		},
	})))

	instance, err := wadge.NewInstance(&wadge.Config{
		Wasm: component,
	})
	if err != nil {
		log.Fatalf("failed to construct new instance: %s", err)
	}
	wadge.SetInstance(instance)
}

func TestIdentityBool(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, true,
			sync.IdentityBool(true),
		)
	})
}

func TestIdentityU8(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, uint8(42),
			sync.IdentityU8(42),
		)
	})
}

func TestIdentityU16(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, uint16(42),
			sync.IdentityU16(42),
		)
	})
}

func TestIdentityU32(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, uint32(42),
			sync.IdentityU32(42),
		)
	})
}

func TestIdentityU64(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, uint64(42),
			sync.IdentityU64(42),
		)
	})
}

func TestIdentityS8(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, int8(-42),
			sync.IdentityS8(-42),
		)
	})
}

func TestIdentityS16(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, int16(-42),
			sync.IdentityS16(-42),
		)
	})
}

func TestIdentityS32(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, int32(-42),
			sync.IdentityS32(-42),
		)
	})
}

func TestIdentityS64(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, int64(-42),
			sync.IdentityS64(-42),
		)
	})
}

func TestIdentityF32(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, float32(-42.42),
			sync.IdentityF32(-42.42),
		)
	})
}

func TestIdentityF64(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, float64(-42.42),
			sync.IdentityF64(-42.42),
		)
	})
}

func TestIdentityChar(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t, '🤡',
			sync.IdentityChar('🤡'),
		)
	})
}

func TestIdentityString(t *testing.T) {
	t.Run("empty", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, "",
				sync.IdentityString(""),
			)
		})
	})
	t.Run("foo", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, "foo",
				sync.IdentityString("foo"),
			)
		})
	})
}

func TestIdentityFlags(t *testing.T) {
	t.Run("a", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.AbcA,
				sync.IdentityFlags(sync.AbcA),
			)
		})
	})
	t.Run("c", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.AbcC,
				sync.IdentityFlags(sync.AbcC),
			)
		})
	})
	t.Run("a|c", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.AbcA|sync.AbcC,
				sync.IdentityFlags(sync.AbcA|sync.AbcC),
			)
		})
	})
	t.Run("a|b|c", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.AbcA|sync.AbcB|sync.AbcC,
				sync.IdentityFlags(sync.AbcA|sync.AbcB|sync.AbcC),
			)
		})
	})
}

func TestIdentityEnum(t *testing.T) {
	t.Run("foo", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.FoobarFoo,
				sync.IdentityEnum(sync.FoobarFoo),
			)
		})
	})
	t.Run("bar", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.FoobarBar,
				sync.IdentityEnum(sync.FoobarBar),
			)
		})
	})
}

func TestIdentityVariant(t *testing.T) {
	t.Run("empty", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, sync.VarEmpty(),
				sync.IdentityVariant(sync.VarEmpty()),
			)
		})
	})
	t.Run("var", func(t *testing.T) {
		t.Run("empty", func(t *testing.T) {
			wadge.RunTest(t, func() {
				expected := sync.VarSome(sync.Rec{})
				assert.Equal(t, expected,
					sync.IdentityVariant(expected),
				)
			})
		})
		t.Run("foo", func(t *testing.T) {
			wadge.RunTest(t, func() {
				expected := sync.VarSome(sync.Rec{
					Nested: sync.RecNested{
						Foo: "foo",
					},
				})
				assert.Equal(t, expected,
					sync.IdentityVariant(expected),
				)
			})
		})
	})
}

func TestIdentityOptionString(t *testing.T) {
	t.Run("none", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.None[string]()
			assert.Equal(t, expected,
				sync.IdentityOptionString(expected),
			)
		})
	})
	t.Run("some('')", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.Some("")
			assert.Equal(t, expected,
				sync.IdentityOptionString(expected),
			)
		})
	})
	t.Run("some(foo)", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.Some("foo")
			assert.Equal(t, expected,
				sync.IdentityOptionString(expected),
			)
		})
	})
	t.Run("some(foobar)", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.Some("foobar")
			assert.Equal(t, expected,
				sync.IdentityOptionString(expected),
			)
		})
	})
}

func TestIdentityResultString(t *testing.T) {
	t.Run("ok('')", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.OK[cm.Result[string, string, struct{}]]("")
			assert.Equal(t, expected,
				sync.IdentityResultString(expected),
			)
		})
	})
	t.Run("ok(foo)", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.OK[cm.Result[string, string, struct{}]]("foo")
			assert.Equal(t, expected,
				sync.IdentityResultString(expected),
			)
		})
	})
	t.Run("err", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := cm.Err[cm.Result[string, string, struct{}]](struct{}{})
			assert.Equal(t, expected,
				sync.IdentityResultString(expected),
			)
		})
	})
}

func TestIdentityRecordPrimitives(t *testing.T) {
	wadge.RunTest(t, func() {
		expected := sync.Primitives{
			A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8, I: 9, J: 10, K: true, L: '🤡', M: "test",
		}
		assert.Equal(t, expected,
			sync.IdentityRecordPrimitives(expected),
		)
	})
}

func TestIdentityRecordRec(t *testing.T) {
	t.Run("empty", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := sync.Rec{}
			assert.Equal(t, expected,
				sync.IdentityRecordRec(expected),
			)
		})
	})
	t.Run("foo", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := sync.Rec{
				Nested: sync.RecNested{
					Foo: "foo",
				},
			}
			assert.Equal(t, expected,
				sync.IdentityRecordRec(expected),
			)
		})
	})
}

func TestIdentityTuple(t *testing.T) {
	wadge.RunTest(t, func() {
		expected := cm.Tuple13[uint8, uint16, uint32, uint64, int8, int16, int32, int64, float32, float64, bool, rune, string]{
			F0: 1, F1: 2, F2: 3, F3: 4, F4: 5, F5: 6, F6: 7, F7: 8, F8: 9, F9: 10, F10: true, F11: '🤡', F12: "test",
		}
		assert.Equal(t,
			sync.IdentityTuple(expected),
			expected,
		)
	})
}

func TestIdentityListBool(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []bool(nil),
				sync.IdentityListBool(cm.NewList[bool](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[true false true]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []bool{true, false, true}
			assert.Equal(t, expected,
				sync.IdentityListBool(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityListU16(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []uint16(nil),
				sync.IdentityListU16(cm.NewList[uint16](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[1 2 3 4]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []uint16{1, 2, 3, 4}
			assert.Equal(t, expected,
				sync.IdentityListU16(cm.NewList(unsafe.SliceData(expected), 4)).Slice(),
			)
		})
	})
}

func TestIdentityListString(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []string(nil),
				sync.IdentityListString(cm.NewList[string](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[foo bar baz]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []string{"foo", "bar", "baz"}
			assert.Equal(t, expected,
				sync.IdentityListString(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityListEnum(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []sync.Foobar(nil),
				sync.IdentityListEnum(cm.NewList[sync.Foobar](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[foo]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Foobar{sync.FoobarFoo}
			assert.Equal(t, expected,
				sync.IdentityListEnum(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[foo bar foo]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Foobar{sync.FoobarFoo, sync.FoobarBar, sync.FoobarFoo}
			assert.Equal(t, expected,
				sync.IdentityListEnum(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityListFlags(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []sync.Abc(nil),
				sync.IdentityListFlags(cm.NewList[sync.Abc](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[a]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Abc{sync.AbcA}
			assert.Equal(t, expected,
				sync.IdentityListFlags(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[b]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Abc{sync.AbcB}
			assert.Equal(t, expected,
				sync.IdentityListFlags(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[a|b]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Abc{sync.AbcA | sync.AbcB}
			assert.Equal(t, expected,
				sync.IdentityListFlags(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[a a]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Abc{sync.AbcA, sync.AbcA}
			assert.Equal(t, expected,
				sync.IdentityListFlags(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[a a|c a|b|c]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Abc{sync.AbcA, sync.AbcA | sync.AbcC, sync.AbcA | sync.AbcB | sync.AbcC}
			assert.Equal(t, expected,
				sync.IdentityListFlags(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityListRecordPrimitives(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []sync.Primitives(nil),
				sync.IdentityListRecordPrimitives(cm.NewList[sync.Primitives](nil, 0)).Slice(),
			)
		})
	})
	t.Run("1", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Primitives{
				{
					A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8, I: 9, J: 10, K: true, L: '🤡', M: "test",
				},
			}
			assert.Equal(t, expected,
				sync.IdentityListRecordPrimitives(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("3", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Primitives{
				{
					A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8, I: 9, J: 10, K: true, L: '🤡', M: "test",
				},
				{},
				{
					A: 1, B: 2, C: 3, D: 4, E: 5, F: 6, G: 7, H: 8, I: 9, J: 10, K: false, L: 'a', M: "foobar",
				},
			}
			assert.Equal(t, expected,
				sync.IdentityListRecordPrimitives(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityListVariant(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []sync.Var(nil),
				sync.IdentityListVariant(cm.NewList[sync.Var](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[empty]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Var{sync.VarEmpty()}
			assert.Equal(t, expected,
				sync.IdentityListVariant(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[var(empty)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Var{sync.VarSome(sync.Rec{})}
			assert.Equal(t, expected,
				sync.IdentityListVariant(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[var(foo)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Var{sync.VarSome(sync.Rec{
				Nested: sync.RecNested{
					Foo: "foo",
				},
			})}
			assert.Equal(t, expected,
				sync.IdentityListVariant(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[var(foo), empty]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Var{
				sync.VarSome(sync.Rec{
					Nested: sync.RecNested{
						Foo: "foo",
					},
				}),
				sync.VarEmpty(),
			}
			assert.Equal(t, expected,
				sync.IdentityListVariant(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[var(foo), empty, var(bar), var(empty), var(baz)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []sync.Var{
				sync.VarSome(sync.Rec{
					Nested: sync.RecNested{
						Foo: "foo",
					},
				}),
				sync.VarEmpty(),
				sync.VarSome(sync.Rec{
					Nested: sync.RecNested{
						Foo: "bar",
					},
				}),
				sync.VarSome(sync.Rec{}),
				sync.VarSome(sync.Rec{
					Nested: sync.RecNested{
						Foo: "baz",
					},
				}),
			}
			assert.Equal(t, expected,
				sync.IdentityListVariant(cm.NewList(unsafe.SliceData(expected), 5)).Slice(),
			)
		})
	})
}

func TestIdentityListOptionString(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []cm.Option[string](nil),
				sync.IdentityListOptionString(cm.NewList[cm.Option[string]](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[none]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{cm.None[string]()}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[some(foo)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{cm.Some("foo")}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[some(foo) some(bar)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.Some("foo"),
				cm.Some("bar"),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[none none]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.None[string](),
				cm.None[string](),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[some(foobar) none]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.Some("foo"),
				cm.None[string](),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[none some(foo)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.None[string](),
				cm.Some("foo"),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[none none none]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.None[string](),
				cm.None[string](),
				cm.None[string](),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
	t.Run("[none some(foo) some('') none]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.None[string](),
				cm.Some("foo"),
				cm.Some(""),
				cm.None[string](),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 4)).Slice(),
			)
		})
	})
	t.Run("[none some(foo) some('') none some(bar) some(baz) none some('')]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Option[string]{
				cm.None[string](),
				cm.Some("foo"),
				cm.Some(""),
				cm.None[string](),
				cm.Some("bar"),
				cm.Some("baz"),
				cm.None[string](),
				cm.Some(""),
			}
			assert.Equal(t, expected,
				sync.IdentityListOptionString(cm.NewList(unsafe.SliceData(expected), 8)).Slice(),
			)
		})
	})
}

func TestIdentityListResultString(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []cm.Result[string, string, struct{}](nil),
				sync.IdentityListResultString(cm.NewList[cm.Result[string, string, struct{}]](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[err]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Result[string, string, struct{}]{cm.Err[cm.Result[string, string, struct{}]](struct{}{})}
			assert.Equal(t, expected,
				sync.IdentityListResultString(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[ok(foo)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Result[string, string, struct{}]{cm.OK[cm.Result[string, string, struct{}]]("foo")}
			assert.Equal(t, expected,
				sync.IdentityListResultString(cm.NewList(unsafe.SliceData(expected), 1)).Slice(),
			)
		})
	})
	t.Run("[ok(foo) ok(bar)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Result[string, string, struct{}]{
				cm.OK[cm.Result[string, string, struct{}]]("foo"),
				cm.OK[cm.Result[string, string, struct{}]]("bar"),
			}
			assert.Equal(t, expected,
				sync.IdentityListResultString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[ok(foo) ok(bar) err]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Result[string, string, struct{}]{
				cm.OK[cm.Result[string, string, struct{}]]("foo"),
				cm.OK[cm.Result[string, string, struct{}]]("bar"),
				cm.Err[cm.Result[string, string, struct{}]](struct{}{}),
			}
			assert.Equal(t, expected,
				sync.IdentityListResultString(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
	t.Run("[ok(foo) ok(bar) err ok() ok() err ok(baz)]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.Result[string, string, struct{}]{
				cm.OK[cm.Result[string, string, struct{}]]("foo"),
				cm.OK[cm.Result[string, string, struct{}]]("bar"),
				cm.Err[cm.Result[string, string, struct{}]](struct{}{}),
				cm.OK[cm.Result[string, string, struct{}]](""),
				cm.OK[cm.Result[string, string, struct{}]](""),
				cm.Err[cm.Result[string, string, struct{}]](struct{}{}),
				cm.OK[cm.Result[string, string, struct{}]]("baz"),
			}
			assert.Equal(t, expected,
				sync.IdentityListResultString(cm.NewList(unsafe.SliceData(expected), 7)).Slice(),
			)
		})
	})
}

func TestIdentityListListString(t *testing.T) {
	t.Run("[]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			assert.Equal(t, []cm.List[string](nil),
				sync.IdentityListListString(cm.NewList[cm.List[string]](nil, 0)).Slice(),
			)
		})
	})
	t.Run("[[foo bar][baz]]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.List[string]{
				cm.NewList(
					unsafe.SliceData([]string{"foo", "bar"}),
					2,
				),
				cm.NewList(
					unsafe.SliceData([]string{"baz"}),
					1,
				),
			}
			assert.Equal(t, expected,
				sync.IdentityListListString(cm.NewList(unsafe.SliceData(expected), 2)).Slice(),
			)
		})
	})
	t.Run("[[foo '' bar][]['']]", func(t *testing.T) {
		wadge.RunTest(t, func() {
			expected := []cm.List[string]{
				cm.NewList(
					unsafe.SliceData([]string{"foo", "", "bar"}),
					3,
				),
				cm.NewList[string](
					nil,
					0,
				),
				cm.NewList(
					unsafe.SliceData([]string{""}),
					1,
				),
			}
			assert.Equal(t, expected,
				sync.IdentityListListString(cm.NewList(unsafe.SliceData(expected), 3)).Slice(),
			)
		})
	})
}

func TestIdentityPrimitives(t *testing.T) {
	wadge.RunTest(t, func() {
		assert.Equal(t,
			sync.IdentityPrimitives(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, true, '🤡', "test"),
			cm.Tuple13[uint8, uint16, uint32, uint64, int8, int16, int32, int64, float32, float64, bool, rune, string]{
				F0: 1, F1: 2, F2: 3, F3: 4, F4: 5, F5: 6, F6: 7, F7: 8, F8: 9, F9: 10, F10: true, F11: '🤡', F12: "test",
			})
	})
}

func TestRes(t *testing.T) {
	wadge.RunTest(t, func() {
		res := sync.NewRes()
		assert.Equal(t, res.Foo(), "foo")
		assert.Equal(t, res.Foo(), "foo")
		assert.Equal(t, res.Foo(), "foo")
		res.ResourceDrop()

		for _, res := range sync.ResMakeList().Slice() {
			assert.Equal(t, res.Foo(), "foo")
			assert.Equal(t, res.Foo(), "foo")
			assert.Equal(t, res.Foo(), "foo")
			res.ResourceDrop()
		}
	})
}
