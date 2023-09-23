use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;

pub fn schema() -> Result<Schema, SchemaError> {
    let mut query = Object::new("Qurey").field(Field::new(
        "hostname",
        TypeRef::named(TypeRef::STRING),
        |_| FieldFuture::new(async { Ok(Some(Value::from("example.com"))) }),
    ));
    let plugins: Vec<&str> = vec!["foo", "bar"];
    let mut fields: Vec<Field> = vec![];
    for plugin in plugins {
        fields.push(Field::new(
            plugin,
            TypeRef::named(TypeRef::STRING),
            move |_| FieldFuture::new(async move { Ok(Some(Value::from(plugin))) }),
        ));
    }
    for f in fields {
        query = query.field(f);
    }
    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
}
