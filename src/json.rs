use crate::ast::*;

pub fn convert_to_json(document: Vec<Block>) -> String {
    fn convert_expr(expr: Expression) -> String {
        match expr {
            Expression::String(content) => format!("\"{}\"", content),
            Expression::Array(a) => format!(
                "[{}]",
                a.into_iter()
                    .map(convert_expr)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::Block(block) => {
                format!("{{ \"{}\": {} }}", block.key, convert_expr(block.value))
            }
            Expression::Boolean(b) => {
                format!("{}", b)
            }
        }
    }

    let unprocessed_json = format!(
        "{{ {} }}",
        document
            .into_iter()
            .map(|b| format!("\"{}\": {}", b.key, convert_expr(b.value)))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    jsonxf::pretty_print(&unprocessed_json).unwrap_or(unprocessed_json)
}
