use surrealdb::Datastore;
use surrealdb::Error;
use surrealdb::Session;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[cfg(test)]
#[tokio::main]
mod surreal {
    use super::*;
    use surrealdb::Datastore;

    // let ds = Datastore::new("tikv://127.0.0.1:2379").await?;
    // let sess = Session::for_kv();
    // let text = String::from("SELECT * FROM type::thing($tb, $id)");
    // let vars = map! {
    //   String::from("tb") => Value::from("my_table"),
    //   String::from("id") => Value::from(1000),
    // };
    // ds.process(&text, &sess, Some(vars), false).await?;
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//   let sess = Session::for_kv();
//   let text = String::from("SELECT * FROM type::thing($tb, $id)");
//   let vars = map! {
//     String::from("tb") => Value::from("my_table"),
//     String::from("id") => Value::from(1000),
//   };
//   ds.process(&text, &sess, Some(vars), false).await?;
// }
