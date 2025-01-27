// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.1
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steammessages_unified_base.steamworkssdk.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EProtoExecutionSite)
pub enum EProtoExecutionSite {
    // @@protoc_insertion_point(enum_value:EProtoExecutionSite.k_EProtoExecutionSiteUnknown)
    k_EProtoExecutionSiteUnknown = 0,
    // @@protoc_insertion_point(enum_value:EProtoExecutionSite.k_EProtoExecutionSiteSteamClient)
    k_EProtoExecutionSiteSteamClient = 3,
}

impl ::protobuf::Enum for EProtoExecutionSite {
    const NAME: &'static str = "EProtoExecutionSite";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EProtoExecutionSite> {
        match value {
            0 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            3 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EProtoExecutionSite> {
        match str {
            "k_EProtoExecutionSiteUnknown" => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            "k_EProtoExecutionSiteSteamClient" => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EProtoExecutionSite] = &[
        EProtoExecutionSite::k_EProtoExecutionSiteUnknown,
        EProtoExecutionSite::k_EProtoExecutionSiteSteamClient,
    ];
}

impl ::protobuf::EnumFull for EProtoExecutionSite {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EProtoExecutionSite").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            EProtoExecutionSite::k_EProtoExecutionSiteUnknown => 0,
            EProtoExecutionSite::k_EProtoExecutionSiteSteamClient => 1,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EProtoExecutionSite {
    fn default() -> Self {
        EProtoExecutionSite::k_EProtoExecutionSiteUnknown
    }
}

impl EProtoExecutionSite {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EProtoExecutionSite>("EProtoExecutionSite")
    }
}

/// Extension fields
pub mod exts {

    pub const description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const service_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const service_execution_site: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::EnumOrUnknown<super::EProtoExecutionSite>> = ::protobuf::ext::ExtFieldOptional::new(50008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM);

    pub const method_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const enum_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const enum_value_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.steammessages_unified_base.steamworkssdk.proto\x1a\x20google/protobuf\
    /descriptor.proto*]\n\x13EProtoExecutionSite\x12\x20\n\x1ck_EProtoExecut\
    ionSiteUnknown\x10\0\x12$\n\x20k_EProtoExecutionSiteSteamClient\x10\x03:\
    A\n\x0bdescription\x18\xd0\x86\x03\x20\x01(\t\x12\x1d.google.protobuf.Fi\
    eldOptionsR\x0bdescription:R\n\x13service_description\x18\xd0\x86\x03\
    \x20\x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\x12serviceDescriptio\
    n:\x8b\x01\n\x16service_execution_site\x18\xd8\x86\x03\x20\x01(\x0e2\x14\
    .EProtoExecutionSite\x12\x1f.google.protobuf.ServiceOptions:\x1ck_EProto\
    ExecutionSiteUnknownR\x14serviceExecutionSite:O\n\x12method_description\
    \x18\xd0\x86\x03\x20\x01(\t\x12\x1e.google.protobuf.MethodOptionsR\x11me\
    thodDescription:I\n\x10enum_description\x18\xd0\x86\x03\x20\x01(\t\x12\
    \x1c.google.protobuf.EnumOptionsR\x0fenumDescription:Y\n\x16enum_value_d\
    escription\x18\xd0\x86\x03\x20\x01(\t\x12!.google.protobuf.EnumValueOpti\
    onsR\x14enumValueDescriptionB\x05H\x01\x80\x01\0\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(EProtoExecutionSite::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
