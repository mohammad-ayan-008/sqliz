use std::error::Error;

use sqlx::*;
use databse::TABLE;

use utils::{sql, Initializatons};

#[derive(TABLE)]
struct Student{
    name:String,
    age:i64,
    class:i64
}
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let mut data = Initializatons::initializer().await.unwrap();
    let result = Student::CreateTable(&mut data).await
        .expect("unable to create a table");


    let struct_name = Student{
        name:String::from("data 123"),
        age: 21,
        class: 9,
    };

    let querrry = sql!(
      [INSERT]
        student[
            name => struct_name.name ,
            age =>  struct_name.age ,
            class => struct_name.class
        ]
    );
    println!("{}",querrry);

    //let result = Student::insert(&mut data,struct_name).await?;
    let result = Student::exec(&mut data,querrry).await?;
    let vec = Student::find_all(&mut data).await?;
    let field = vec.iter().map(|f|{
         Student{
             name: f.get("name"),
             age: f.get("age"),
             class: f.get("class")
         }
    });
    println!("{:?}",vec);

    Ok(())
}
