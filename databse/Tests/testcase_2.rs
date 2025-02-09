
use sqlx::*;

use syn::export::str;
use databse::TABLE;
use utils::Initializatons;
#[derive(TABLE)]
struct Student{
    name:String,
    age:i32,
    class:i32
}
// async fn insert(conn:&mut  sqlx::PgConnection,arg:Student)-> std::result::Result<sqlx::postgres::PgQueryResult, Box<dyn std::error::Error>> {
//     let querry = format!("Insert into {} values({})",#name_format,#values_qutation);
//     let res = sqlx::query(&querry)
//         .bind(arg.name)
//         .bind(arg.age)
//         .bind(arg.class)
//         .execute(conn)
//         .await?;
//     Ok(res)
// }
#[tokio::main]
async fn main() {
    let mut data = Initializatons::initializer().await.unwrap();
    let result = Student::CreateTable(&mut data).await
        .expect("unable to create a table");

    // let vec = Student::find_all(&mut data).await.expect("TODO: panic message");
}