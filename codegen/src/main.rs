use codegen::{Enum, Field, Scope, Struct, Type, Variant, Impl, Function};
use grammers_tl_parser::tl::{Category, Definition, Flag, ParameterType, Type as TlType};
use heck::{CamelCase, MixedCase, SnakeCase};
use std::collections::HashMap;
use std::fs::{File, DirBuilder};
use std::process::exit;
use std::{io, io::Read};
use std::path::Path;
use std::io::Write;

fn main() -> io::Result<()> {
    let initial_defs = parse_tl_file()?;
    let mut types = HashMap::new();

    for def in initial_defs.iter() {
        let ty_defs = types.entry(&def.ty.name).or_insert(vec![]);
        ty_defs.push(def);
    }

    let path = "./types";
    DirBuilder::new().recursive(true).create(path)?;


    for (type_name, type_defs) in types {
        let mut scope = Scope::new();
        let type_defs = type_defs
            .into_iter()
            .filter(|v| v.category == Category::Types)
            .collect::<Vec<&Definition>>();
        match type_defs.len() {
            0 => {
                eprintln!("no type defs for {}", type_name);
                exit(1);
            }
            1 => {
                let (strct, simpl) = make_struct(type_defs.first().unwrap());
                    scope.push_struct(strct);
                    scope.push_impl(simpl);
            }
            _ => {
                let mut type_enum = Enum::new(type_name);
                for def in type_defs {
                    let def_name = to_camel_case(def.name.as_str());
                    let mut var = Variant::new(def_name.as_str());
                    var.tuple(def_name.as_str());
                    type_enum.push_variant(var);

                    let (strct, simpl) = make_struct(def);
                    scope.push_struct(strct);
                    scope.push_impl(simpl);
                }
                scope.push_enum(type_enum);
            }
        }

        let mut file = File::create(format!("{}/{}.rs", path, to_snake_case(type_name)))?;
        file.write_all(scope.to_string().as_bytes())?;
    }

    Ok(())
}

fn to_camel_case(str_from: &str) -> String {
    str_from.to_camel_case()
}

fn to_snake_case(str_from: &str) -> String {
    str_from.to_snake_case()
}

fn make_struct(def: &Definition) -> (Struct, Impl) {
    let type_name = to_camel_case(def.name.as_str());
    let mut strct = Struct::new(type_name.as_str());
    strct.derive("Deserialize");
    let mut optional_string_type = Type::new("Option");
    optional_string_type.generic(Type::new("String"));

    let mut extra_field = Field::new("extra", optional_string_type.clone());
    extra_field.annotation(vec![
        r#"#[serde(rename(serialize = "@extra", deserialize = "@extra"))"#
    ]);
    strct.push_field(extra_field);

    let mut client_field = Field::new("client_id", optional_string_type.clone());
    client_field.annotation(vec![
        r#"#[serde(rename(serialize = "@client_id", deserialize = "@client_id"))"#
    ]);
    strct.push_field(client_field);

    let mut strct_impl = Impl::new(type_name);
    for param in &def.params {
        let (field_type, ref_field_type) = match &param.ty {
            ParameterType::Flags => {
                panic!("{:?}", param)
            }
            ParameterType::Normal { ty, flag } => match flag {
                None => {
                    let tn = to_camel_case(ty.name.as_str());
                    (Type::new(tn.as_str()), Type::new(format!("&{}", tn).as_str()))
                },
                Some(flag) => {
                    panic!("{:?}", param)
                }
            },
        };
        let field_name = param.name.as_str().to_snake_case();
        let field = Field::new(field_name.as_str(), field_type.clone());
        strct.push_field(field);

        // getter for a field
        let mut func = Function::new(field_name.as_str());
        func.arg_self().ret(ref_field_type).line(format!("&self.{}", field_name));
        strct_impl.push_fn(func);

    }

    strct_impl
    (strct, strct_impl)
}

fn parse_tl_file() -> io::Result<Vec<Definition>> {
    let file_path = match std::env::args().skip(1).next() {
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "file_path not specified",
            ))
        }
        Some(value) => value,
    };
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(grammers_tl_parser::parse_tl_file(&contents)
        .filter_map(|v| v.ok())
        .collect::<Vec<Definition>>())
}
