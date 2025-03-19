mod another_ns {

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize)]
pub enum Status {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "INACTIVE")]
    Inactive,
}
}


/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(remote = "Self")]
pub enum UnionSomeNsAMetadataSomeNsBMetadata {
    SomeNsAMetadata(some_ns_a::Metadata),
    SomeNsBMetadata(some_ns_b::Metadata),
}

impl From<some_ns_a::Metadata> for UnionSomeNsAMetadataSomeNsBMetadata {
    fn from(v: some_ns_a::Metadata) -> Self {
        Self::SomeNsAMetadata(v)
    }
}

impl TryFrom<UnionSomeNsAMetadataSomeNsBMetadata> for some_ns_a::Metadata {
    type Error = UnionSomeNsAMetadataSomeNsBMetadata;

    fn try_from(v: UnionSomeNsAMetadataSomeNsBMetadata) -> Result<Self, Self::Error> {
        if let UnionSomeNsAMetadataSomeNsBMetadata::SomeNsAMetadata(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<some_ns_b::Metadata> for UnionSomeNsAMetadataSomeNsBMetadata {
    fn from(v: some_ns_b::Metadata) -> Self {
        Self::SomeNsBMetadata(v)
    }
}

impl TryFrom<UnionSomeNsAMetadataSomeNsBMetadata> for some_ns_b::Metadata {
    type Error = UnionSomeNsAMetadataSomeNsBMetadata;

    fn try_from(v: UnionSomeNsAMetadataSomeNsBMetadata) -> Result<Self, Self::Error> {
        if let UnionSomeNsAMetadataSomeNsBMetadata::SomeNsBMetadata(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl serde::Serialize for UnionSomeNsAMetadataSomeNsBMetadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct NewtypeVariantSerializer<S>(S);

        impl<S> serde::Serializer for NewtypeVariantSerializer<S>
        where
            S: serde::Serializer,
        {
            type Ok = S::Ok;
            type Error = S::Error;
            type SerializeSeq = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeTuple = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeTupleStruct = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeTupleVariant = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeMap = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeStruct = serde::ser::Impossible<S::Ok, S::Error>;
            type SerializeStructVariant = serde::ser::Impossible<S::Ok, S::Error>;
            fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_some<T: ?Sized + serde::Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error>{ unimplemented!() }
            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_unit_variant(self ,_name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(self, _name: &'static str, _value: &T,) -> Result<Self::Ok, Self::Error> { unimplemented!() }
            fn serialize_seq(self,_len: Option<usize>,) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
            fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
            fn serialize_tuple_struct(self,_name: &'static str,_len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
            fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
            fn serialize_map(self,_len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
            fn serialize_struct(self,_name: &'static str,_len: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
            fn serialize_struct_variant(self,_name: &'static str,_variant_index: u32,_variant: &'static str,_len: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
            fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
                self,
                _name: &'static str,
                _variant_index: u32,
                _variant: &'static str,
                value: &T,
            ) -> Result<Self::Ok, Self::Error> {
                value.serialize(self.0)
            }
        }

        Self::serialize(self, NewtypeVariantSerializer(serializer))
    }
}

impl<'de> serde::Deserialize<'de> for UnionSomeNsAMetadataSomeNsBMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer)
    }
}
mod some_ns {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct MyRecord {
    pub some_field: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SomeRecord {
    pub label: String,
    #[serde(default = "default_somerecord_parent")]
    pub parent: Option<Box<SomeRecord>>,
    pub children: Vec<SomeRecord>,
    #[serde(default = "default_somerecord_status")]
    pub status: super::another_ns::Status,
    pub metadata_a: super::some_ns_a::Metadata,
    pub metadata_b: super::some_ns_b::Metadata,
    #[serde(default = "default_somerecord_union_field")]
    pub union_field: super::UnionSomeNsAMetadataSomeNsBMetadata,
    pub record_without_ns: MyRecord,
}

#[inline(always)]
fn default_somerecord_parent() -> Option<Box<SomeRecord>> { None }

#[inline(always)]
fn default_somerecord_status() -> super::another_ns::Status { super::another_ns::Status::Unknown }

#[inline(always)]
fn default_somerecord_union_field() -> super::UnionSomeNsAMetadataSomeNsBMetadata { super::UnionSomeNsAMetadataSomeNsBMetadata::SomeNsAMetadata(super::some_ns_a::Metadata { label: "default_label".to_owned(), }) }
}

mod some_ns_a {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Metadata {
    pub label: String,
}
}

mod some_ns_b {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Metadata {
    pub cost: i32,
}
}

