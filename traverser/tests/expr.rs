mod lib;

#[test]
fn traverse_expr() {
    assert_traverse_trace! {
        source: r#"
            return n*m;
        "#,
        assert_trace: [
            "enter_program",
            "enter_stmt_list",
            "enter_stmt",
            "enter_return_stmt",
            "enter_expr",
            "enter_binary_expr",
            "enter_expr",
            "enter_ident", // n
            "exit_ident",
            "exit_expr",
            "enter_expr",
            "enter_ident", // m
            "exit_ident",
            "exit_expr",
            "exit_binary_expr",
            "exit_expr",
            "exit_return_stmt",
            "exit_stmt",
            "exit_stmt_list",
            "exit_program",
        ]
    }
}
