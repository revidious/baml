use std::{
    fs::canonicalize,
    path::{self, PathBuf},
};

use internal_baml_parser_database::ParserDatabase;
use internal_baml_schema_ast::ast::WithName;
use log::info;
use serde_json::json;

use crate::configuration::Generator;

use self::{file::FileCollector, traits::WithWritePythonString};

mod r#class;
mod client;
mod r#enum;
mod field;
mod r#file;
mod function;
mod template;
mod traits;
mod types;
mod variants;

fn generate_py_file<'a>(obj: &impl WithWritePythonString, fc: &'a mut FileCollector) {
    obj.write_py_file(fc);
}

pub(crate) fn generate_py(db: &ParserDatabase, gen: &Generator) -> std::io::Result<()> {
    let mut fc = Default::default();
    db.walk_enums().for_each(|e| generate_py_file(&e, &mut fc));
    db.walk_classes()
        .for_each(|c| generate_py_file(&c, &mut fc));
    db.walk_functions().for_each(|f| {
        generate_py_file(&f, &mut fc);
        f.walk_variants()
            .for_each(|v| generate_py_file(&v, &mut fc));
    });
    db.walk_clients()
        .for_each(|f| generate_py_file(&f, &mut fc));
    generate_py_file(db, &mut fc);
    info!("Writing files to {}", gen.output.to_string_lossy());
    let temp_path = PathBuf::from(format!("{}.tmp", &gen.output.to_string_lossy().to_string()));
    match fc.write(&temp_path) {
        Ok(_) => {
            let _ = std::fs::remove_dir_all(&gen.output);
            std::fs::rename(&temp_path, &gen.output)
        }
        Err(e) => {
            let _ = std::fs::remove_file(&temp_path);
            Err(e)
        }
    }
}

impl WithWritePythonString for ParserDatabase {
    fn write_py_file<'a>(&'a self, fc: &'a mut FileCollector) {
        fc.start_py_file(".", "__init__");
        fc.last_file().add_line("from . import impls");
        fc.complete_file();

        fc.start_export_file(".", "baml_types");
        self.walk_functions().for_each(|f| {
            fc.last_file().add_import(
                &format!(".__do_not_import.functions.{}", f.file_name()),
                &format!("I{}", f.name()),
            );
            fc.last_file().add_import(
                &format!(".__do_not_import.functions.{}", f.file_name()),
                &format!("I{}Output", f.name()),
            )
        });

        self.walk_enums().for_each(|e| {
            fc.last_file().add_import(
                &format!(".__do_not_import.types.enums.{}", e.file_name()),
                e.name(),
            )
        });

        self.walk_classes().for_each(|c| {
            fc.last_file().add_import(
                &format!(".__do_not_import.types.classes.{}", c.file_name()),
                c.name(),
            )
        });
        fc.complete_file();

        fc.start_export_file(".", "__init__.py");
        fc.last_file()
            .add_import(".__do_not_import.generated_baml_client", "baml");
        fc.complete_file();

        fc.start_py_file(".", "generated_baml_client");
        let mut fxs = self
            .walk_functions()
            .map(|f| {
                fc.last_file().add_import(
                    &format!(".functions.{}", f.file_name()),
                    &format!("BAML{}", f.name()),
                );
                f.name()
            })
            .collect::<Vec<_>>();
        fxs.sort();

        let mut clients = self
            .walk_clients()
            .map(|f| {
                fc.last_file().add_import(
                    &format!(".clients.{}", f.file_name()),
                    &format!("{}", f.name()),
                );
                f.name()
            })
            .collect::<Vec<_>>();
        clients.sort();

        template::render_template(
            template::HSTemplate::BAMLClient,
            fc.last_file(),
            json!({ "functions": fxs, "clients": clients }),
        );
        fc.complete_file();
    }

    fn file_name(&self) -> String {
        "generated_baml_client".to_string()
    }
}