
use proc_macro::{TokenStream};
use std::error::Error;
use std::fmt::{format, Debug};
use quote::{quote, ToTokens};
use sqlx::query;
use sqlx::types::JsonValue::String;
use syn::{Ident, parse_macro_input, DeriveInput, Field};




#[proc_macro_derive(TABLE)]
pub fn table(datastream:TokenStream)->TokenStream {
    let ast = parse_macro_input!(datastream as DeriveInput);
    let name = &ast.ident;
    let name_format = format!("{}",name.to_string().to_lowercase());
    let name_table = Ident::new(&name_format, name.span());
    let name_fn = format!("Create{}Table", name);
    let fields = if let syn::Data::Struct(
        syn::DataStruct {
            fields: syn::Fields::Named(
                syn::FieldsNamed { ref named, .. }
            ), ..
        }
    ) = ast.data {
        named
    } else {
        unimplemented!()
    };

    let named = fields.iter().map(|f| {
        let name = &f.ident;
        let removed = name.as_ref();
        let data = &removed.unwrap().to_string();
        let types_field = &f.ty;
        let type_d = types_field.into_token_stream().to_string();
        if type_d == "String" {
            format!("{} text,", data)
        } else {
            format!("{} integer,", data)
        }
    });


    let genrated_fields = fields.iter().map(|f| {
        let name = &f.ident;
            quote! {
                .bind(arg.#name)
            }
    });


    let mut names = "".to_string();
    let mut values_qutation = "".to_string();
    let mut values = 1;
    let len = fields.len();

    for (index,i) in fields.iter().enumerate(){
    // let als = fields.iter().enumerate().map(|(index,i)|{
        if len - 1 == index {
            names.push_str(&format!("{} {} {}", &i.ident.as_ref().unwrap().to_string(), getType(&i), ""));
            values_qutation.push_str(&format!("${} ", values));
            values += 1;
        } else {
            names.push_str(&format!("{} {} {}", &i.ident.as_ref().unwrap().to_string(), getType(&i), ","));
            values_qutation.push_str(&format!("${}, ", values));
            values += 1;
        }
    };


    let code = quote! {
        impl #name{
           async fn CreateTable (conn:&mut sqlx::PgConnection)->Result<sqlx::postgres::PgQueryResult,Box<dyn std::error::Error>> {

              let query = format!(
                "CREATE TABLE IF NOT EXISTS {}({})",
                     #name_format,
                     #names
               );

              let data = sqlx::query(&query)
                .execute(conn)
                .await?;

                Ok(data)
             }

            async fn find_all(conn:&mut sqlx::PgConnection )->Result<Vec<sqlx::postgres::PgRow>,Box<dyn Error>>{
                 let name = "";

                 let querry = format!(
                   "select * from {}",
                      #name_format
                  );

                 let data = sqlx::query(&querry)
                   .fetch_all(conn)
                   .await?;
                    Ok(data)
            }

            #[deprecated(since= "v 1",note = " use exec(psqlConnection,String)")]
            async fn insert(conn:&mut  sqlx::PgConnection,arg:Student)-> std::result::Result<sqlx::postgres::PgQueryResult, Box<dyn std::error::Error>> {
                     let querry = format!("Insert into {} values({})",#name_format,#values_qutation);
                     println!("{}",querry);
                           let res = sqlx::query(&querry)
                              #(#genrated_fields)*
                             .execute(conn)
                             .await?;
                    Ok(res)
            }
            async fn exec(conn:&mut  sqlx::PgConnection,arg:String)-> std::result::Result<sqlx::postgres::PgQueryResult, Box<dyn std::error::Error>> {
                           let res = sqlx::query(&arg)
                             .execute(conn)
                             .await?;
                    Ok(res)
            }

        }



    };
    TokenStream::from(code)

}


fn getType(types_field:&Field,)->std::string::String{
    let type_d = types_field.into_token_stream().to_string();
    if type_d.contains("String") {
        "text".to_string()
    } else {
        "int".to_string()
    }
}


