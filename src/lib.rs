mod graphql;

use graphql::Context;

pub fn query() {
    let ctx = Context {};
    let schema = graphql::schema();
    let variables = juniper::Variables::new();

    let (res, _errors) =
        juniper::execute("query { notes {id} }", None, &schema, &variables, &ctx).unwrap();

    println!("{:?}", res);
    println!("{:?}", _errors);
}
