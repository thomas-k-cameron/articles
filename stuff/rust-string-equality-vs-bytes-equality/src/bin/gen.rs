fn main() {
    let mut stack = vec!["_ => -1,".to_string()];
    let mut stack2 = vec!["_ => -1,".to_string()];
    for i in 0..50_000 {
        let val = uuid::Uuid::new_v4();
        let s = format!("\"{val}\" => {i},");
        let s2 = format!("b\"{val}\" => {i},");
        stack.push(s);
        stack2.push(s2);
    }

    stack.reverse();
    stack2.reverse();
    let got = stack.join("\n");
    let got2 = stack2.join("\n");

    let s = format!(
        r#"

        pub fn uuid_v4_cmp_string(s: &str) -> i64 {{
            match s {{{got}}}
        }}


        pub fn uuid_v4_cmp_bytes(s: &str) -> i64 {{
            match s.as_bytes() {{{got2}}}
        }}
    "#
    );
    std::fs::write("./src/lib.rs", s.as_bytes()).unwrap();
}
