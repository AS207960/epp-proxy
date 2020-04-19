//! Custom serde XML serializer
//!
//! The special serde tag name `$value` equates to the inner value of an XML element.
//! Tags starting with `$attr:` will be encoded as attributes rather than new elements.
//! Namespaces and prefixes can be set using the tag name format `{namespace}prefix:tag-name`.

use serde::{ser, Serialize};

pub struct Serializer {
    _cur_tag: Vec<String>,
}

/// Serialise serde item to XML
///
/// # Arguments
/// * `value` - The value to be serialised
/// * `root` - The root XML element name
/// * `ns` - The default XML namespace
pub fn to_string<T>(value: &T, root: &str, ns: &str) -> Result<String, serde::de::value::Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { _cur_tag: vec![] };
    let val = value.serialize(&mut serializer)?;
    let out = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><{} xmlns=\"{}\">{}</{}>",
        root,
        ns,
        val.as_str(),
        root
    );
    Ok(out)
}

#[derive(Debug)]
pub enum _SerializerData {
    CData(String),
    String(String),
    Seq(Vec<_SerializerData>),
    Struct { attrs: String, contents: String },
}

impl _SerializerData {
    fn as_str(&self) -> String {
        match self {
            _SerializerData::CData(s) => s.clone(),
            _SerializerData::String(s) => s.clone(),
            _SerializerData::Seq(s) => s.iter().map(|d| d.as_str()).collect::<Vec<_>>().join(""),
            _SerializerData::Struct { contents, .. } => contents.clone(),
        }
    }
}

fn format_tag(key: &str, val: &_SerializerData) -> String {
    let mut output = String::new();
    let re = regex::Regex::new(r"^(?:\{(?P<n>.+)\})?(?:(?P<p>.+):)?(?P<e>.+)$").unwrap();
    let caps = re.captures(key).unwrap();
    let open_tag = |attrs, empty: bool| {
        if key != "$value" {
            if let Some(ns) = caps.name("n") {
                if let Some(p) = caps.name("p") {
                    format!(
                        "<{}:{} xmlns:{}=\"{}\"{}{}>",
                        p.as_str(),
                        caps.name("e").unwrap().as_str(),
                        p.as_str(),
                        ns.as_str(),
                        attrs,
                        if empty { "/" } else { "" }
                    )
                } else {
                    format!(
                        "<{} xmlns=\"{}\"{}{}>",
                        caps.name("e").unwrap().as_str(),
                        ns.as_str(),
                        attrs,
                        if empty { "/" } else { "" }
                    )
                }
            } else {
                format!("<{}{}{}>", key, attrs, if empty { "/" } else { "" })
            }
        } else {
            "".to_string()
        }
    };
    let close_tag = if key != "$value" {
        if let Some(p) = caps.name("p") {
            format!("</{}:{}>", p.as_str(), caps.name("e").unwrap().as_str())
        } else {
            format!("</{}>", key)
        }
    } else {
        "".to_string()
    };
    match val {
        _SerializerData::CData(s) => {
            if s.is_empty() {
                let open_tag_str = open_tag("", true);
                output += &open_tag_str;
            } else {
                let open_tag_str = open_tag("", false);
                output += &format!("{}<![CDATA[{}]]>{}", open_tag_str, s, close_tag);
            }
        }
        _SerializerData::String(s) => {
            if s.is_empty() {
                let open_tag_str = open_tag("", true);
                output += &open_tag_str;
            } else {
                let open_tag_str = open_tag("", false);
                output += &format!("{}{}{}", open_tag_str, s, close_tag);
            }
        }
        _SerializerData::Seq(s) => {
            for i in s {
                output += &format_tag(key, i);
            }
        }
        _SerializerData::Struct { attrs, contents } => {
            if contents.is_empty() {
                let open_tag_str = if !attrs.is_empty() {
                    open_tag(&format!(" {}", attrs), true)
                } else {
                    open_tag("", true)
                };
                output += &open_tag_str;
            } else {
                let open_tag_str = if !attrs.is_empty() {
                    open_tag(&format!(" {}", attrs), false)
                } else {
                    open_tag("", false)
                };
                output += &format!("{}{}{}", open_tag_str, contents, close_tag);
            }
        }
    }
    output
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;
    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = StructSerializer<'a>;
    type SerializeStructVariant = StructVariantSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<_SerializerData, Self::Error> {
        let val = if v { "true" } else { "false" };
        Ok(_SerializerData::String(val.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<_SerializerData, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<_SerializerData, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<_SerializerData, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<_SerializerData, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<_SerializerData, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<_SerializerData, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<_SerializerData, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<_SerializerData, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::CData(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(hex::encode(v)))
    }

    fn serialize_none(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String("".to_string()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<_SerializerData, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<_SerializerData, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<_SerializerData, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<_SerializerData, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<_SerializerData, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<_SerializerData, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self)?;
        Ok(_SerializerData::String(format_tag(variant, &value)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer {
            parent: self,
            output: vec![],
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            parent: self,
            output: String::new(),
            cur_key: String::new(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            parent: self,
            attrs: vec![],
            keys: vec![],
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer {
            parent: self,
            attrs: vec![],
            keys: vec![],
            tag: variant.to_string(),
        })
    }
}

pub struct SeqSerializer<'a> {
    parent: &'a mut Serializer,
    output: Vec<_SerializerData>,
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        self.output.push(val);
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::Seq(self.output))
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        self.output.push(val);
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::Seq(self.output))
    }
}

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        self.output.push(val);
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::Seq(self.output))
    }
}

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        self.output.push(val);
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::Seq(self.output))
    }
}

pub struct MapSerializer<'a> {
    parent: &'a mut Serializer,
    output: String,
    cur_key: String,
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = key.serialize(&mut *self.parent)?;
        self.cur_key = val.as_str();
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        self.output += &format_tag(&self.cur_key, &val);
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(self.output))
    }
}

pub struct StructSerializer<'a> {
    parent: &'a mut Serializer,
    attrs: Vec<String>,
    keys: Vec<String>,
}

impl<'a> ser::SerializeStruct for StructSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        if key.starts_with("$attr:") {
            self.attrs
                .push(format!("{}=\"{}\"", &key[6..], val.as_str()))
        } else {
            self.keys.push(format_tag(key, &val));
        }
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::Struct {
            attrs: self.attrs.join(" "),
            contents: self.keys.join(""),
        })
    }
}

pub struct StructVariantSerializer<'a> {
    parent: &'a mut Serializer,
    keys: Vec<String>,
    attrs: Vec<String>,
    tag: String,
}

impl<'a> ser::SerializeStructVariant for StructVariantSerializer<'a> {
    type Ok = _SerializerData;
    type Error = serde::de::value::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let val = value.serialize(&mut *self.parent)?;
        if key.starts_with("$attr:") {
            self.attrs
                .push(format!("{}=\"{}\"", &key[6..], val.as_str()));
        } else {
            self.keys.push(format_tag(key, &val));
        }
        Ok(())
    }

    fn end(self) -> Result<_SerializerData, Self::Error> {
        Ok(_SerializerData::String(format_tag(
            &self.tag,
            &_SerializerData::Struct {
                attrs: self.attrs.join(" "),
                contents: self.keys.join(""),
            },
        )))
    }
}
