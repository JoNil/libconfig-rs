use super::error::Error;
use serde::{ser, Serialize};

#[derive(Clone)]
pub struct Serializer {
    output: String,
    indent: usize,
    /// Stack tracking whether each struct/map level emitted braces
    braces_stack: Vec<bool>,
}

impl Serializer {
    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push(' ');
        }
    }
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        indent: 0,
        braces_stack: Vec::new(),
    };
    serializer.output += "config : ";
    value.serialize(&mut serializer)?;
    serializer.output += ";\n";
    Ok(serializer.output)
}

#[allow(clippy::multiple_bound_locations)]
impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(i32::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(i32::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        self.output += "L";
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(u32::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(u32::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        self.output += "L";
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:?}");
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("{v:?}");
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.output += "[ ]";
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.output += "[ ";
        value.serialize(&mut *self)?;
        self.output += " ]";
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.output += "[ ]";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.output += "[ ]";
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.output += &format!("\"{variant}\"");
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.output += "{\n";
        self.indent += 4;
        self.write_indent();
        self.output += variant;
        self.output += " : ( ";
        value.serialize(&mut *self)?;
        self.output += " );\n";
        self.indent -= 4;
        self.write_indent();
        self.output += "}";
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output += "( ";
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.output += "[ ";
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.output += "{\n";
        self.indent += 4;
        self.braces_stack.push(true);
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.output += "{\n";
        self.indent += 4;
        self.braces_stack.push(true);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output += "{\n";
        self.indent += 4;
        self.write_indent();
        self.output += variant;
        self.output += " : {\n";
        self.indent += 4;
        self.braces_stack.push(true);
        self.braces_stack.push(true);
        Ok(self)
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.output.ends_with("( ") {
            self.output += ", ";
        }
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += " )";
        Ok(())
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.output.ends_with("[ ") {
            self.output += ", ";
        }
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += " ]";
        Ok(())
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.write_indent();
        let start = self.output.len();
        key.serialize(&mut **self)?;
        // Strip quotes from string keys for libconfig group format
        let key_part = self.output[start..].to_string();
        if key_part.starts_with('"') && key_part.ends_with('"') && key_part.len() > 2 {
            self.output.truncate(start);
            self.output += &key_part[1..key_part.len() - 1];
        }
        self.output += " : ";
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        self.output += ";\n";
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some(true) = self.braces_stack.pop() {
            self.indent -= 4;
            self.write_indent();
            self.output += "}";
        }
        Ok(())
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.write_indent();
        self.output += key;
        self.output += " : ";
        value.serialize(&mut **self)?;
        self.output += ";\n";
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some(true) = self.braces_stack.pop() {
            self.indent -= 4;
            self.write_indent();
            self.output += "}";
        }
        Ok(())
    }
}

#[allow(clippy::multiple_bound_locations)]
impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.write_indent();
        self.output += key;
        self.output += " : ";
        value.serialize(&mut **self)?;
        self.output += ";\n";
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // Close inner struct brace
        if let Some(true) = self.braces_stack.pop() {
            self.indent -= 4;
            self.write_indent();
            self.output += "}";
        }
        // Close outer variant wrapper brace
        if let Some(true) = self.braces_stack.pop() {
            self.output += ";\n";
            self.indent -= 4;
            self.write_indent();
            self.output += "}";
        }
        Ok(())
    }
}
