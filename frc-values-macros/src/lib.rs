use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

/// Derive macro for implementing `FrcStructure` on a struct.
#[proc_macro_derive(FrcStructure)]
pub fn frc_structure(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_frc_struct(&ast).into()
}

fn schema_type_name(name: &syn::Ident) -> String {
    match name.to_string().as_str() {
        "f64" => "float64",
        "f32" => "float32",
        "i64" => "int64",
        "i32" => "int32",
        "i16" => "int16",
        "i8" => "int8",
        "u64" => "uint64",
        "u32" => "uint32",
        "u16" => "uint16",
        "u8" => "uint8",
        "bool" => "bool",
        "char" => "char",
        any => any,
    }
    .to_owned()
}

fn type_size(path: &syn::Path) -> String {
    let segment = path.segments.last().unwrap();
    let segment_name = &segment.ident;
    match segment_name.to_string().as_str() {
        "f64" => "8",
        "f32" => "4",
        "i64" => "8",
        "i32" => "4",
        "i16" => "2",
        "i8" => "1",
        "u64" => "8",
        "u32" => "4",
        "u16" => "2",
        "u8" => "1",
        "bool" => "1",
        "char" => "1",
        _ => {
            return format!(
                "<{:} as frc_values::structure::FrcStructure>::SIZE",
                path.into_token_stream()
            )
        }
    }
    .to_owned()
}

fn type_pack(path: &syn::Path, field: &syn::Ident) -> String {
    let segment = path.segments.last().unwrap();
    let segment_name = &segment.ident;
    match segment_name.to_string().as_str() {
        "f64" => format!("buffer.put_f64_le(self.{});", field),
        "f32" => format!("buffer.put_f32_le(self.{});", field),
        "i64" => format!("buffer.put_i64_le(self.{});", field),
        "i32" => format!("buffer.put_i32_le(self.{});", field),
        "i16" => format!("buffer.put_i16_le(self.{});", field),
        "i8" => format!("buffer.put_i8(self.{});", field),
        "u64" => format!("buffer.put_u64_le(self.{});", field),
        "u32" => format!("buffer.put_u32_le(self.{});", field),
        "u16" => format!("buffer.put_u16_le(self.{});", field),
        "u8" => format!("buffer.put_u8(self.{});", field),
        "bool" => format!("buffer.put_u8(if self.{} {{ 1 }} else {{ 0 }});", field),
        "char" => format!("buffer.put_u8(self.{} as u8);", field),
        _ => {
            return format!(
                "frc_values::structure::FrcStructure::pack(&self.{:}, buffer);",
                field
            )
        }
    }
}

fn array_type_pack(path: &syn::Path, field: &syn::Ident, len: usize) -> String {
    let segment = path.segments.last().unwrap();
    let segment_name = &segment.ident;
    let elem_pack = match segment_name.to_string().as_str() {
        "f64" => format!("buffer.put_f64_le(self.{}[i]);", field),
        "f32" => format!("buffer.put_f32_le(self.{}[i]);", field),
        "i64" => format!("buffer.put_i64_le(self.{}[i]);", field),
        "i32" => format!("buffer.put_i32_le(self.{}[i]);", field),
        "i16" => format!("buffer.put_i16_le(self.{}[i]);", field),
        "i8" => format!("buffer.put_i8(self.{}[i]);", field),
        "u64" => format!("buffer.put_u64_le(self.{}[i]);", field),
        "u32" => format!("buffer.put_u32_le(self.{}[i]);", field),
        "u16" => format!("buffer.put_u16_le(self.{}[i]);", field),
        "u8" => format!("buffer.put_u8(self.{}[i]);", field),
        "bool" => format!("buffer.put_u8(if self.{}[i] {{ 1 }} else {{ 0 }});", field),
        "char" => format!("buffer.put_u8(self.{}[i] as u8);", field),
        _ => format!(
            "frc_values::structure::FrcStructure::pack(&self.{:}[i], buffer);",
            field
        ),
    };
    format!("for i in 0..{} {{ {} }}", len, elem_pack)
}

fn type_unpack(path: &syn::Path, field: &syn::Ident) -> String {
    let segment = path.segments.last().unwrap();
    let segment_name = &segment.ident;
    match segment_name.to_string().as_str() {
        "f64" => format!("{}: buffer.get_f64_le()", field),
        "f32" => format!("{}: buffer.get_f32_le()", field),
        "i64" => format!("{}: buffer.get_i64_le()", field),
        "i32" => format!("{}: buffer.get_i32_le()", field),
        "i16" => format!("{}: buffer.get_i16_le()", field),
        "i8" => format!("{}: buffer.get_i8()", field),
        "u64" => format!("{}: buffer.get_u64_le()", field),
        "u32" => format!("{}: buffer.get_u32_le()", field),
        "u16" => format!("{}: buffer.get_u16_le()", field),
        "u8" => format!("{}: buffer.get_u8()", field),
        "bool" => format!("{}: buffer.get_u8() != 0", field),
        "char" => format!("{}: buffer.get_u8() as char", field),
        _ => {
            return format!(
                "{}: <{:} as frc_values::structure::FrcStructure>::unpack(buffer)",
                field,
                path.into_token_stream()
            )
        }
    }
}

fn array_type_unpack(path: &syn::Path, field: &syn::Ident, len: usize) -> String {
    let segment = path.segments.last().unwrap();
    let segment_name = &segment.ident;
    let elem_unpack = match segment_name.to_string().as_str() {
        "f64" => format!("buffer.get_f64_le()"),
        "f32" => format!("buffer.get_f32_le()"),
        "i64" => format!("buffer.get_i64_le()"),
        "i32" => format!("buffer.get_i32_le()"),
        "i16" => format!("buffer.get_i16_le()"),
        "i8" => format!("buffer.get_i8()"),
        "u64" => format!("buffer.get_u64_le()"),
        "u32" => format!("buffer.get_u32_le()"),
        "u16" => format!("buffer.get_u16_le()"),
        "u8" => format!("buffer.get_u8()"),
        "bool" => format!("buffer.get_u8() != 0"),
        "char" => format!("buffer.get_u8() as char"),
        _ => return format!("{}: [{}]",field, format!(
            "<{:} as frc_values::structure::FrcStructure>::unpack(buffer), ",
            path.into_token_stream()
        ).repeat(len))
    };
    let zero_val = match segment_name.to_string().as_str() {
        "f64" => "0.0f64",
        "f32" => "0.0f32",
        "i64" => "0i64",
        "i32" => "0i32",
        "i16" => "0i16",
        "i8" => "0i8",
        "u64" => "0u64",
        "u32" => "0u32",
        "u16" => "0u16",
        "u8" => "0u8",
        "bool" => "false",
        "char" => "'\\0'",
        _ => unreachable!(),
    };
    format!(
        "{}: {{ let mut {} = [{}; {}]; for i in 0..{} {{ {}[i] = {} }}; {} }}",
        field, field, zero_val, len, len, field, elem_unpack, field
    )
}

fn impl_frc_struct(ast: &syn::DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let mut schema = String::new();
    let mut size_expr = String::new();
    let mut packing = Vec::new();
    let mut unpacking = Vec::new();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }),
        ..
    }) = &ast.data
    {
        for field in fields {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            match field_type {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.last().unwrap();
                    let segment_name = &segment.ident;
                    let type_name = schema_type_name(segment_name);
                    schema.push_str(format!(" {} {};", type_name, field_name).as_str());
                    size_expr.push_str(format!("{} + ", type_size(path)).as_str());
                    packing.push(type_pack(path, field_name));
                    unpacking.push(type_unpack(path, field_name));
                }
                syn::Type::Array(syn::TypeArray { elem, len, .. }) => match elem.as_ref() {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let segment = path.segments.last().unwrap();
                        let segment_name = &segment.ident;
                        let type_name = schema_type_name(segment_name);
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Int(len_lit),
                            ..
                        }) = len
                        {
                            let len = len_lit.base10_parse::<usize>().unwrap();
                            schema.push_str(
                                format!(" {} {}[{}];", type_name, field_name, len).as_str(),
                            );
                            size_expr
                                .push_str(format!("({} * {}) + ", type_size(path), len).as_str());
                            packing.push(array_type_pack(path, field_name, len));
                            unpacking.push(array_type_unpack(path, field_name, len));
                        } else {
                            panic!("Only arrays with literal lengths are supported");
                        }
                    }
                    _ => panic!("Invalid array type"),
                },
                _ => panic!("Only structs/prims and arrays of structs/prims are supported"),
            }
        }
    } else {
        panic!("Only structs with named fields are supported");
    }
    size_expr.push_str(" 0");
    let size_expr = syn::parse_str::<syn::Expr>(size_expr.as_str()).unwrap();
    let schema = syn::parse_str::<syn::LitStr>(format!("\"{}\"", schema).as_str()).unwrap();
    let packing = packing
        .iter()
        .map(|s| syn::parse_str::<syn::Stmt>(s.as_str()).unwrap());
    let unpacking = unpacking
        .iter()
        .map(|s| syn::parse_str::<syn::FieldValue>(s.as_str()).unwrap());
    let expanded = quote! {
        impl frc_values::structure::FrcStructure for #name {
            const SIZE: usize = #size_expr;
            const SCHEMA: &'static str = #schema;
            const TYPE: &'static str = stringify!(#name);
            const DESCRIPTION: frc_values::structure::FrcStructDesc = frc_values::structure::FrcStructDesc {
                schema: Self::SCHEMA,
                type_str: Self::TYPE,
                size: Self::SIZE,
            };

            fn pack(&self, buffer: &mut impl frc_values::bytes::BufMut) {
                #(#packing)*
            }

            fn unpack(buffer: &mut impl frc_values::bytes::Buf) -> Self {
                Self {
                    #(#unpacking),*
                }
            }
        }
        frc_values::inventory::submit! { <#name as frc_values::structure::FrcStructure>::DESCRIPTION }
    };

    expanded
}
