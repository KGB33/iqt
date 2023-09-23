use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_graphql::dynamic::TypeRef;
use async_graphql::Value;

pub fn schema() -> Result<Schema, SchemaError> {
    let query = Object::new("Qurey").field(Field::new(
        "hostname",
        TypeRef::named(TypeRef::STRING),
        |_| FieldFuture::new(async { Ok(Some(Value::from("example.com"))) }),
    ));
    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
}
