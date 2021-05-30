/// Used to write GraphQL tests faster.
/// Builds the schema and the request to mimic a normal HTTP -request based query.
#[allow(unused_macros)]
macro_rules! gql_test {
    ($query:expr) => {{
        {
            use crate::{
                access::{ClientIP, RateLimiter, JWT},
                db::{RedisConn, SqlxConn},
                gql::build_schema,
                Config, CONF_FILE,
            };
            use async_graphql::{Data, Request};
            let conf = Config::from_file(CONF_FILE);

            let mut req = Request::new($query);
            let mut data = Data::default();

            let rd = RedisConn::new(&conf);

            data.insert(RateLimiter::new(rd.clone()));
            data.insert(rd.into_inner());
            data.insert(SqlxConn::new(&conf).await.into_inner());
            data.insert(ClientIP::new("127.0.0.1").unwrap().into_inner());
            data.insert(conf);
            data.insert(JWT::generate());

            req.data = data;

            let res = build_schema().execute(req).await;

            println!("{:#?}", res);

            res
        }
    }};
}
