/// Used to write GraphQL tests faster.
/// Builds the schema and the request to mimic a normal HTTP -request based query.
#[allow(unused_macros)]
macro_rules! gql_test {
    ($query:expr) => {{
        {
            use crate::{
                access::{ClientIP, RateLimiter},
                db::{RedisConn, SqlxConn},
                gql::build_schema,
                Config, CONF_FILE,
            };
            use async_graphql::Request;
            let conf = Config::from_file(CONF_FILE);

            let mut req = Request::new($query);
            let rd = RedisConn::new(&conf);

            req = req.data(RateLimiter::new(rd.clone()));
            req = req.data(rd.into_inner());
            req = req.data(SqlxConn::new(&conf).await.into_inner());
            req = req.data(ClientIP::new("127.0.0.1").unwrap().into_inner());
            req = req.data(conf);

            let res = build_schema().execute(req).await;

            println!("{:#?}", res);

            res
        }
    }};
}
