use apollo_compiler::ApolloCompiler;
use apollo_compiler::HirDatabase;

fn main() {
    let input = "{ hostname { name }}";
    let schema = r#"
    type Hostname {
      name: String
    }
    
    type Query {
      hostname: Hostname
    }
    "#;

    let mut compiler = ApolloCompiler::new();
    compiler.add_type_system(schema, "schema.graphql");
    let query_id = compiler.add_executable(input, "query.graphql");

    let diagnostics = compiler.validate();
    for diagnostic in &diagnostics {
        // this will pretty-print diagnostics using the miette crate.
        println!("{}", diagnostic);
    }
    assert!(diagnostics.is_empty());
    println!("{:?}", compiler.db.schema());
}
