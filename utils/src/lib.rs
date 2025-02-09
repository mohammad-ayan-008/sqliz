pub mod Initializatons{


    pub async fn initializer() -> Result<sqlx::PgConnection, Box<dyn std::error::Error>> {
        let options = load_env();
        let connection = sqlx::Connection::connect_with(&options).await?;
        Ok(connection)
    }

    pub fn load_env() -> sqlx::postgres::PgConnectOptions {
        dotenv::dotenv().ok();
        sqlx::postgres::PgConnectOptions::new()
            .database(&std::env::var("DATABASE_NAME").expect("DATABASE_NAME is missing in env"))
            .host(&std::env::var("HOST").expect("HOST is missing in env"))
            .password(&std::env::var("D_PASSWORD").expect("PASSWORD is missing in env"))
            .username(&std::env::var("D_USERNAME").expect("USERNAME is missing in env"))
    }
}

pub mod sql_gen{
    #[macro_export]
    macro_rules! sql {
        ([INSERT] $table_name:ident [$($column_name:ident => $column_value:expr),*])=>(
            {
                // INSERT INTO TABLE {TABLE_NAME}(fields,fields,fields) VALUES(value1,value2,value3)
                let mut querry = format!("INSERT INTO  {} (",stringify!($table_name).to_lowercase());
                let mut fields = Vec::new();
                let mut values = Vec::new();
                $(
                  let fields_q = format!("{}",stringify!($column_name));
                  if let Some(_) = $column_value.to_string().parse::<i32>().ok(){
                     values.push(format!("{}",$column_value));
                  }else{
                   values.push(format!("\'{}\'",$column_value));
                  }
                  fields.push(fields_q);
                )*
                querry.push_str(&format!("{}) VALUES(",&fields.join(",")));
                querry.push_str(&format!("{});",&values.join(",")));
                querry
            }
        );
        ([CREATE]  $table_name:ident { $($column_name:ident -> $column_type:ident($size:expr)),*}) => (
           {
               let mut coln = Vec::new();
               let mut querry = format!("CREATE TABLE {} (",stringify!($table_name).to_lowercase());
               $(
                  let coln_def = format!("{} {}({})",stringify!($column_name),stringify!($column_type),stringify!($size));
                  coln.push(coln_def);
               )*
               querry.push_str(&coln.join(","));
               querry.push_str(");");
               querry
          }
        )
    }

    use std::fmt::format;
    pub use sql;
}