use std::fs;
use std::path::PathBuf;
use std::process::Command;

use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;

pub fn schema() -> Result<Schema, SchemaError> {
    let mut query = Object::new("Query");
    let plugins = plugins();
    let mut fields: Vec<Field> = vec![];
    for plug in plugins {
        let path: String = plug.path.file_name().unwrap().to_str().unwrap().to_string();
        fields.push(Field::new(
            path.clone(),
            TypeRef::named(TypeRef::STRING),
            FieldFuture::new(run_plugin(plug))
        ));
    }
    for f in fields {
        query = query.field(f);
    }
    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
}

#[derive(Clone)]
struct Plugin {
    path: PathBuf,
}

async fn run_plugin(plug: Plugin) -> Result<Option<Value>, async_graphql::Error> {
        match Command::new("echo").arg("Hello world").output() {
            Ok(s) => Ok(Some(Value::String(String::from_utf8(s.stdout).unwrap()))),
            Err(e) => Err(async_graphql::Error::new_with_source(e)),
        }
}

fn plugins() -> Vec<Plugin> {
    fs::read_dir("../plugins/")
        .unwrap()
        .map(|p| Plugin {
            path: p.unwrap().path(),
        })
        .collect::<Vec<Plugin>>()
}
