use graphql_client::{reqwest::post_graphql, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/puppy_smiles.graphql",
    response_derives = "Debug"
)]
struct Users;

