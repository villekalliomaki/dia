use async_graphql::*;

#[derive(Default)]
pub struct Add;

#[Object]
impl Add {
    /// For general testing examples.
    async fn add(&self, a: i64, b: i64) -> i64 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gql::build_schema;

    #[tokio::test]
    /// Simple example for testing GQL queries.
    async fn add() {
        let s = build_schema();
        let q = Request::new("query { add(a: 1, b: 1) }");

        let res = s.execute(q).await;

        if let Value::Number(val) = res.data {
            assert_eq!(val.as_i64().unwrap(), 2)
        }
    }
}
