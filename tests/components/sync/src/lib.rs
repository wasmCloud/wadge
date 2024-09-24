mod bindings {
    use crate::Handler;

    wit_bindgen::generate!({
        world: "host",
        path: "../../wit/sync",
        with: {
            "wadge-test:sync/sync": generate,
        }
    });
    export!(Handler);
}
use bindings::exports::wadge_test::sync::sync::{Abc, Foobar, Primitives, Rec, Var};

struct Handler;

struct Res;

impl bindings::exports::wadge_test::sync::sync::GuestRes for Res {
    fn new() -> Res {
        Res
    }

    fn foo(&self) -> String {
        "foo".into()
    }

    fn make_list() -> Vec<bindings::exports::wadge_test::sync::sync::Res> {
        vec![
            bindings::exports::wadge_test::sync::sync::Res::new(Res),
            bindings::exports::wadge_test::sync::sync::Res::new(Res),
            bindings::exports::wadge_test::sync::sync::Res::new(Res),
            bindings::exports::wadge_test::sync::sync::Res::new(Res),
            bindings::exports::wadge_test::sync::sync::Res::new(Res),
        ]
    }
}

impl bindings::exports::wadge_test::sync::sync::Guest for Handler {
    type Res = Res;

    fn identity_bool(arg: bool) -> bool {
        arg
    }

    fn identity_u8(arg: u8) -> u8 {
        arg
    }

    fn identity_u16(arg: u16) -> u16 {
        arg
    }

    fn identity_u32(arg: u32) -> u32 {
        arg
    }

    fn identity_u64(arg: u64) -> u64 {
        arg
    }

    fn identity_s8(arg: i8) -> i8 {
        arg
    }

    fn identity_s16(arg: i16) -> i16 {
        arg
    }

    fn identity_s32(arg: i32) -> i32 {
        arg
    }

    fn identity_s64(arg: i64) -> i64 {
        arg
    }

    fn identity_f32(arg: f32) -> f32 {
        arg
    }

    fn identity_f64(arg: f64) -> f64 {
        arg
    }

    fn identity_char(arg: char) -> char {
        arg
    }

    fn identity_string(arg: String) -> String {
        arg
    }

    fn identity_flags(arg: Abc) -> Abc {
        arg
    }

    fn identity_variant(arg: Var) -> Var {
        arg
    }

    fn identity_enum(arg: Foobar) -> Foobar {
        arg
    }

    fn identity_option_string(arg: Option<String>) -> Option<String> {
        arg
    }

    fn identity_result_string(arg: Result<String, ()>) -> Result<String, ()> {
        arg
    }

    fn identity_tuple(
        arg: (
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            f32,
            f64,
            bool,
            char,
            String,
        ),
    ) -> (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        bool,
        char,
        String,
    ) {
        arg
    }

    fn identity_list_bool(arg: Vec<bool>) -> Vec<bool> {
        arg
    }

    fn identity_list_enum(arg: Vec<Foobar>) -> Vec<Foobar> {
        arg
    }

    fn identity_list_flags(arg: Vec<Abc>) -> Vec<Abc> {
        arg
    }

    fn identity_list_variant(arg: Vec<Var>) -> Vec<Var> {
        arg
    }

    fn identity_list_string(arg: Vec<String>) -> Vec<String> {
        arg
    }

    fn identity_list_list_string(arg: Vec<Vec<String>>) -> Vec<Vec<String>> {
        arg
    }

    fn identity_list_u16(arg: Vec<u16>) -> Vec<u16> {
        arg
    }

    fn identity_record_rec(arg: Rec) -> Rec {
        arg
    }

    fn identity_record_primitives(arg: Primitives) -> Primitives {
        arg
    }

    fn identity_list_record_primitives(arg: Vec<Primitives>) -> Vec<Primitives> {
        arg
    }

    fn identity_list_option_string(arg: Vec<Option<String>>) -> Vec<Option<String>> {
        arg
    }

    fn identity_list_result_string(arg: Vec<Result<String, ()>>) -> Vec<Result<String, ()>> {
        arg
    }

    fn identity_primitives(
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: i8,
        f: i16,
        g: i32,
        h: i64,
        i: f32,
        j: f64,
        l: bool,
        m: char,
        n: String,
    ) -> (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        bool,
        char,
        String,
    ) {
        (a, b, c, d, e, f, g, h, i, j, l, m, n)
    }
}
