
use bytes::{Buf, BufMut};

use crate::{
    structure::{self, FrcStructure, StructureFieldTypes},
    structure::{FrcStructDesc, FrcStructDescDB},
    FrcValue,
};

#[test]
fn test_value_serde() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Test {
        field: FrcValue,
    }

    let test_inst = Test {
        field: FrcValue::from(1.0),
    };
    let json_str = r#"{"field":1.0}"#;

    let test_de: Test = serde_json::from_str(json_str).unwrap();
    let test_ser = serde_json::to_string(&test_inst).unwrap();

    assert_eq!(json_str, test_ser);
    assert_eq!(test_inst, test_de);
}

#[test]
fn test_table_serde_simple() {
    let value = FrcValue::from(1.0f64);
    assert_eq!(f32::try_from(value.clone()).unwrap(), 1.0);
    fn assert_f64_eq(a: f64, b: f64) {
        assert!((a - b).abs() < 0.0001);
    }
    assert_f64_eq(value.try_into().unwrap(), 1.0);
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct Meter {
    value: f64,
}
impl FrcStructure for Meter {
    const SCHEMA: &'static str = "float64 value;";
    const TYPE: &'static str = "Meter";
    const SIZE: usize = 8;
    const DESCRIPTION: FrcStructDesc = FrcStructDesc {
        schema: Self::SCHEMA,
        type_str: Self::TYPE,
        size: Self::SIZE,
    };

    fn pack(&self, buffer: &mut impl BufMut) {
        buffer.put_f64_le(self.value);
    }

    fn unpack(buffer: &mut impl Buf) -> Self {
        Self {
            value: buffer.get_f64_le(),
        }
    }
}

inventory::submit! { Meter::DESCRIPTION }

#[test]
fn test_structures() {
    use crate as frc_values;

    let test_struct = Meter { value: 1.0 };
    let value = FrcValue::from_struct(test_struct);
    let test_struct2: Meter = value.try_into_struct().unwrap();
    assert_eq!(test_struct, test_struct2);

    #[derive(Debug, PartialEq, Clone, Copy, frc_values_macros::FrcStructure)]
    struct NestedTestStruct {
        boolean: bool,
        test_struct: Meter,
        integer: i32,
    }

    let nested_struct = NestedTestStruct {
        boolean: true,
        test_struct: test_struct.clone(),
        integer: 1,
    };
    let value = FrcValue::from_struct(nested_struct);
    let nested_struct2: NestedTestStruct = value.try_into_struct().unwrap();
    assert_eq!(nested_struct, nested_struct2);

    FrcStructDescDB::add(FrcStructDesc {
        schema: "bool idk;",
        type_str: "proc",
        size: 1,
    });

    //iterate through all inventory values of FrcStructureDescription and print type_str
    for struct_desc in inventory::iter::<FrcStructDesc> {
        println!("{} {{{}}}", struct_desc.type_str, struct_desc.schema)
    }
}

#[test]
fn test_schema() {
    const SCHEMA: &str = "enum {a=1, b=2} int8 val[3]";
    let fields = structure::parse_schema_toplevel(SCHEMA);
    assert_eq!(fields.len(), 1);
    assert_eq!(
        fields[0],
        ("val".to_owned(), 0, StructureFieldTypes::Int8(3))
    )
}

#[test]
fn test_schema_advanced() {
    const SCHEMA: &str = "Rotation2d rot; Translation2d trans;";
    FrcStructDescDB::add(FrcStructDesc {
        schema: "double value",
        type_str: "Rotation2d",
        size: 8,
    });
    FrcStructDescDB::add(FrcStructDesc {
        schema: "double x; double y",
        type_str: "Translation2d",
        size: 16,
    });
    let fields = structure::parse_schema_toplevel(SCHEMA);
    assert_eq!(fields.len(), 3);
    assert_eq!(
        fields,
        vec![
            (
                "rot.value".to_owned(),
                0usize,
                StructureFieldTypes::Float64(1)
            ),
            (
                "trans.x".to_owned(),
                8usize,
                StructureFieldTypes::Float64(1)
            ),
            (
                "trans.y".to_owned(),
                16usize,
                StructureFieldTypes::Float64(1)
            )
        ]
    );
}