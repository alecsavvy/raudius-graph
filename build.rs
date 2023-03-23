use convert_case::{Case, Casing};
use std::fs::{read_dir, File};
use std::io::prelude::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut f = File::create("src/query_2.rs").expect("could not create file");
    let header = "
    use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
    use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
    use crate::entities::{prelude::*, *};

    pub type AudiusSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

    pub struct QueryRoot;

    #[Object]
    impl QueryRoot {
        async fn howdy(&self) -> &'static str {
            \"partner 🤠\"
        }
    ";

    f.write_all(header.as_bytes())
        .expect("could not write to file");

    let get_all_template = |mod_name: String| -> String {
        let entity_name = mod_name.clone().to_case(Case::Pascal);
        let return_name = format!("{}::Model", mod_name.clone().to_lowercase());
        format!(
            "
        async fn {}(&self, ctx: &Context<'_>) -> Result<Vec<{}>, DbErr>
        {{    let db = ctx
                .data::<DatabaseConnection>()
                .map_err(|e| DbErr::Custom(e.message))?;
            {}::find().all(db).await
        }}

    ",
            mod_name, return_name, entity_name
        )
    };

    let paths = read_dir("./src/entities").expect("could not get entity files");
    for path in paths {
        let path = path.unwrap();
        let path = path.path();
        let path = path.to_str().unwrap();
        let path = path.replace("./src/entities/", "");
        let path = path.replace(".rs", "");
        if path.eq("mod") || path.eq("prelude") || path.contains("enums") {
            continue;
        }
        let func = get_all_template(path.to_string());
        f.write_all(func.as_bytes())
            .expect("could not write to file");
    }

    let footer = "
        }
    ";

    f.write_all(footer.as_bytes())
        .expect("could not write to file");
}
