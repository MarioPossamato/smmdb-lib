// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Sound {
    // message fields
    pub x: u32,
    pub y: u32,
    pub sound_type: u32,
    pub variation: bool,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl Sound {
    pub fn new() -> Sound {
        ::std::default::Default::default()
    }

    // uint32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = 0;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: u32) {
        self.x = v;
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    // uint32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = 0;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: u32) {
        self.y = v;
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

    // uint32 sound_type = 3;

    pub fn clear_sound_type(&mut self) {
        self.sound_type = 0;
    }

    // Param is passed by value, moved
    pub fn set_sound_type(&mut self, v: u32) {
        self.sound_type = v;
    }

    pub fn get_sound_type(&self) -> u32 {
        self.sound_type
    }

    // bool variation = 4;

    pub fn clear_variation(&mut self) {
        self.variation = false;
    }

    // Param is passed by value, moved
    pub fn set_variation(&mut self, v: bool) {
        self.variation = v;
    }

    pub fn get_variation(&self) -> bool {
        self.variation
    }
}

impl ::protobuf::Message for Sound {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.y = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sound_type = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.variation = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.x != 0 {
            my_size += ::protobuf::rt::value_size(1, self.x, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.y != 0 {
            my_size += ::protobuf::rt::value_size(2, self.y, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.sound_type != 0 {
            my_size += ::protobuf::rt::value_size(3, self.sound_type, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.variation != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.x != 0 {
            os.write_uint32(1, self.x)?;
        }
        if self.y != 0 {
            os.write_uint32(2, self.y)?;
        }
        if self.sound_type != 0 {
            os.write_uint32(3, self.sound_type)?;
        }
        if self.variation != false {
            os.write_bool(4, self.variation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Sound {
        Sound::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "x",
                    |m: &Sound| { &m.x },
                    |m: &mut Sound| { &mut m.x },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "y",
                    |m: &Sound| { &m.y },
                    |m: &mut Sound| { &mut m.y },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sound_type",
                    |m: &Sound| { &m.sound_type },
                    |m: &mut Sound| { &mut m.sound_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "variation",
                    |m: &Sound| { &m.variation },
                    |m: &mut Sound| { &mut m.variation },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Sound>(
                    "Sound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Sound {
        static mut instance: ::protobuf::lazy::Lazy<Sound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Sound,
        };
        unsafe {
            instance.get(Sound::new)
        }
    }
}

impl ::protobuf::Clear for Sound {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_sound_type();
        self.clear_variation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Sound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Sound {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Sound_SoundType {
    UNKNOWN = 0,
}

impl ::protobuf::ProtobufEnum for Sound_SoundType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Sound_SoundType> {
        match value {
            0 => ::std::option::Option::Some(Sound_SoundType::UNKNOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Sound_SoundType] = &[
            Sound_SoundType::UNKNOWN,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Sound_SoundType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Sound_SoundType {
}

impl ::std::default::Default for Sound_SoundType {
    fn default() -> Self {
        Sound_SoundType::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Sound_SoundType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bSound.proto\x12\0\"j\n\x05Sound\x12\x0b\n\x01x\x18\x01\x20\x01(\rB\
    \0\x12\x0b\n\x01y\x18\x02\x20\x01(\rB\0\x12\x14\n\nsound_type\x18\x03\
    \x20\x01(\rB\0\x12\x13\n\tvariation\x18\x04\x20\x01(\x08B\0\"\x1a\n\tSou\
    ndType\x12\x0b\n\x07UNKNOWN\x10\0\x1a\0:\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
