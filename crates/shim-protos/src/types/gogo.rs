// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by pure
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

//! Generated file from `gogoproto/gogo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

/// Extension fields
pub mod exts {

    pub const goproto_enum_prefix: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(62001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_enum_stringer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(62021, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const enum_stringer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(62022, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const enum_customname: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(62023, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const enumdecl: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(62024, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const enumvalue_customname: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(66001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const goproto_getters_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_enum_prefix_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_stringer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const verbose_equal_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63004, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const face_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const gostring_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const populate_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63007, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const stringer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const onlyone_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63009, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const equal_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63013, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const description_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63014, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const testgen_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63015, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const benchgen_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63016, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const marshaler_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63017, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unmarshaler_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63018, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const stable_marshaler_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63019, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const sizer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63020, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_enum_stringer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63021, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const enum_stringer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63022, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unsafe_marshaler_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63023, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unsafe_unmarshaler_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63024, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_extensions_map_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63025, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_unrecognized_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63026, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const gogoproto_import: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63027, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const protosizer_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63028, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const compare_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63029, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const typedecl_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63030, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const enumdecl_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63031, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_registration: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63032, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const messagename_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63033, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_sizecache_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63034, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_unkeyed_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63035, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_getters: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_stringer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const verbose_equal: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64004, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const face: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const gostring: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const populate: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64007, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const stringer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(67008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const onlyone: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64009, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const equal: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64013, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64014, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const testgen: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64015, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const benchgen: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64016, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const marshaler: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64017, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unmarshaler: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64018, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const stable_marshaler: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64019, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const sizer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64020, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unsafe_marshaler: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64023, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const unsafe_unmarshaler: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64024, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_extensions_map: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64025, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_unrecognized: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64026, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const protosizer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64028, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const compare: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64029, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const typedecl: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64030, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const messagename: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64033, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_sizecache: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64034, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const goproto_unkeyed: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64035, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const nullable: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const embed: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const customtype: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const customname: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65004, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const jsontag: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const moretags: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const casttype: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65007, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const castkey: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const castvalue: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65009, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const stdtime: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65010, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const stdduration: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65011, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const wktpointer: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65012, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14gogoproto/gogo.proto\x12\tgogoproto\x1a\x20google/protobuf/descrip\
    tor.proto:N\n\x13goproto_enum_prefix\x18\xb1\xe4\x03\x20\x01(\x08\x12\
    \x1c.google.protobuf.EnumOptionsR\x11goprotoEnumPrefix:R\n\x15goproto_en\
    um_stringer\x18\xc5\xe4\x03\x20\x01(\x08\x12\x1c.google.protobuf.EnumOpt\
    ionsR\x13goprotoEnumStringer:C\n\renum_stringer\x18\xc6\xe4\x03\x20\x01(\
    \x08\x12\x1c.google.protobuf.EnumOptionsR\x0cenumStringer:G\n\x0fenum_cu\
    stomname\x18\xc7\xe4\x03\x20\x01(\t\x12\x1c.google.protobuf.EnumOptionsR\
    \x0eenumCustomname::\n\x08enumdecl\x18\xc8\xe4\x03\x20\x01(\x08\x12\x1c.\
    google.protobuf.EnumOptionsR\x08enumdecl:V\n\x14enumvalue_customname\x18\
    \xd1\x83\x04\x20\x01(\t\x12!.google.protobuf.EnumValueOptionsR\x13enumva\
    lueCustomname:N\n\x13goproto_getters_all\x18\x99\xec\x03\x20\x01(\x08\
    \x12\x1c.google.protobuf.FileOptionsR\x11goprotoGettersAll:U\n\x17goprot\
    o_enum_prefix_all\x18\x9a\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.F\
    ileOptionsR\x14goprotoEnumPrefixAll:P\n\x14goproto_stringer_all\x18\x9b\
    \xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x12goprotoStr\
    ingerAll:J\n\x11verbose_equal_all\x18\x9c\xec\x03\x20\x01(\x08\x12\x1c.g\
    oogle.protobuf.FileOptionsR\x0fverboseEqualAll:9\n\x08face_all\x18\x9d\
    \xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x07faceAll:A\
    \n\x0cgostring_all\x18\x9e\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.\
    FileOptionsR\x0bgostringAll:A\n\x0cpopulate_all\x18\x9f\xec\x03\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\x0bpopulateAll:A\n\x0cstringer\
    _all\x18\xa0\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\
    \x0bstringerAll:?\n\x0bonlyone_all\x18\xa1\xec\x03\x20\x01(\x08\x12\x1c.\
    google.protobuf.FileOptionsR\nonlyoneAll:;\n\tequal_all\x18\xa5\xec\x03\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x08equalAll:G\n\x0fde\
    scription_all\x18\xa6\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileO\
    ptionsR\x0edescriptionAll:?\n\x0btestgen_all\x18\xa7\xec\x03\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\ntestgenAll:A\n\x0cbenchgen_al\
    l\x18\xa8\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0bb\
    enchgenAll:C\n\rmarshaler_all\x18\xa9\xec\x03\x20\x01(\x08\x12\x1c.googl\
    e.protobuf.FileOptionsR\x0cmarshalerAll:G\n\x0funmarshaler_all\x18\xaa\
    \xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eunmarshale\
    rAll:P\n\x14stable_marshaler_all\x18\xab\xec\x03\x20\x01(\x08\x12\x1c.go\
    ogle.protobuf.FileOptionsR\x12stableMarshalerAll:;\n\tsizer_all\x18\xac\
    \xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x08sizerAll:Y\
    \n\x19goproto_enum_stringer_all\x18\xad\xec\x03\x20\x01(\x08\x12\x1c.goo\
    gle.protobuf.FileOptionsR\x16goprotoEnumStringerAll:J\n\x11enum_stringer\
    _all\x18\xae\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\
    \x0fenumStringerAll:P\n\x14unsafe_marshaler_all\x18\xaf\xec\x03\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\x12unsafeMarshalerAll:T\n\x16u\
    nsafe_unmarshaler_all\x18\xb0\xec\x03\x20\x01(\x08\x12\x1c.google.protob\
    uf.FileOptionsR\x14unsafeUnmarshalerAll:[\n\x1agoproto_extensions_map_al\
    l\x18\xb1\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x17g\
    oprotoExtensionsMapAll:X\n\x18goproto_unrecognized_all\x18\xb2\xec\x03\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x16goprotoUnrecognize\
    dAll:I\n\x10gogoproto_import\x18\xb3\xec\x03\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0fgogoprotoImport:E\n\x0eprotosizer_all\x18\xb4\
    \xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\rprotosizerAl\
    l:?\n\x0bcompare_all\x18\xb5\xec\x03\x20\x01(\x08\x12\x1c.google.protobu\
    f.FileOptionsR\ncompareAll:A\n\x0ctypedecl_all\x18\xb6\xec\x03\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\x0btypedeclAll:A\n\x0cenumdecl\
    _all\x18\xb7\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\
    \x0benumdeclAll:Q\n\x14goproto_registration\x18\xb8\xec\x03\x20\x01(\x08\
    \x12\x1c.google.protobuf.FileOptionsR\x13goprotoRegistration:G\n\x0fmess\
    agename_all\x18\xb9\xec\x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOpt\
    ionsR\x0emessagenameAll:R\n\x15goproto_sizecache_all\x18\xba\xec\x03\x20\
    \x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x13goprotoSizecacheAll:N\
    \n\x13goproto_unkeyed_all\x18\xbb\xec\x03\x20\x01(\x08\x12\x1c.google.pr\
    otobuf.FileOptionsR\x11goprotoUnkeyedAll:J\n\x0fgoproto_getters\x18\x81\
    \xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0egoproto\
    Getters:L\n\x10goproto_stringer\x18\x83\xf4\x03\x20\x01(\x08\x12\x1f.goo\
    gle.protobuf.MessageOptionsR\x0fgoprotoStringer:F\n\rverbose_equal\x18\
    \x84\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0cver\
    boseEqual:5\n\x04face\x18\x85\xf4\x03\x20\x01(\x08\x12\x1f.google.protob\
    uf.MessageOptionsR\x04face:=\n\x08gostring\x18\x86\xf4\x03\x20\x01(\x08\
    \x12\x1f.google.protobuf.MessageOptionsR\x08gostring:=\n\x08populate\x18\
    \x87\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x08pop\
    ulate:=\n\x08stringer\x18\xc0\x8b\x04\x20\x01(\x08\x12\x1f.google.protob\
    uf.MessageOptionsR\x08stringer:;\n\x07onlyone\x18\x89\xf4\x03\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x07onlyone:7\n\x05equal\x18\
    \x8d\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x05equ\
    al:C\n\x0bdescription\x18\x8e\xf4\x03\x20\x01(\x08\x12\x1f.google.protob\
    uf.MessageOptionsR\x0bdescription:;\n\x07testgen\x18\x8f\xf4\x03\x20\x01\
    (\x08\x12\x1f.google.protobuf.MessageOptionsR\x07testgen:=\n\x08benchgen\
    \x18\x90\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\
    \x08benchgen:?\n\tmarshaler\x18\x91\xf4\x03\x20\x01(\x08\x12\x1f.google.\
    protobuf.MessageOptionsR\tmarshaler:C\n\x0bunmarshaler\x18\x92\xf4\x03\
    \x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bunmarshaler:L\n\
    \x10stable_marshaler\x18\x93\xf4\x03\x20\x01(\x08\x12\x1f.google.protobu\
    f.MessageOptionsR\x0fstableMarshaler:7\n\x05sizer\x18\x94\xf4\x03\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x05sizer:L\n\x10unsafe\
    _marshaler\x18\x97\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageO\
    ptionsR\x0funsafeMarshaler:P\n\x12unsafe_unmarshaler\x18\x98\xf4\x03\x20\
    \x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x11unsafeUnmarshaler:W\
    \n\x16goproto_extensions_map\x18\x99\xf4\x03\x20\x01(\x08\x12\x1f.google\
    .protobuf.MessageOptionsR\x14goprotoExtensionsMap:T\n\x14goproto_unrecog\
    nized\x18\x9a\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOption\
    sR\x13goprotoUnrecognized:A\n\nprotosizer\x18\x9c\xf4\x03\x20\x01(\x08\
    \x12\x1f.google.protobuf.MessageOptionsR\nprotosizer:;\n\x07compare\x18\
    \x9d\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x07com\
    pare:=\n\x08typedecl\x18\x9e\xf4\x03\x20\x01(\x08\x12\x1f.google.protobu\
    f.MessageOptionsR\x08typedecl:C\n\x0bmessagename\x18\xa1\xf4\x03\x20\x01\
    (\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bmessagename:N\n\x11gopr\
    oto_sizecache\x18\xa2\xf4\x03\x20\x01(\x08\x12\x1f.google.protobuf.Messa\
    geOptionsR\x10goprotoSizecache:J\n\x0fgoproto_unkeyed\x18\xa3\xf4\x03\
    \x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0egoprotoUnkeyed:\
    ;\n\x08nullable\x18\xe9\xfb\x03\x20\x01(\x08\x12\x1d.google.protobuf.Fie\
    ldOptionsR\x08nullable:5\n\x05embed\x18\xea\xfb\x03\x20\x01(\x08\x12\x1d\
    .google.protobuf.FieldOptionsR\x05embed:?\n\ncustomtype\x18\xeb\xfb\x03\
    \x20\x01(\t\x12\x1d.google.protobuf.FieldOptionsR\ncustomtype:?\n\ncusto\
    mname\x18\xec\xfb\x03\x20\x01(\t\x12\x1d.google.protobuf.FieldOptionsR\n\
    customname:9\n\x07jsontag\x18\xed\xfb\x03\x20\x01(\t\x12\x1d.google.prot\
    obuf.FieldOptionsR\x07jsontag:;\n\x08moretags\x18\xee\xfb\x03\x20\x01(\t\
    \x12\x1d.google.protobuf.FieldOptionsR\x08moretags:;\n\x08casttype\x18\
    \xef\xfb\x03\x20\x01(\t\x12\x1d.google.protobuf.FieldOptionsR\x08casttyp\
    e:9\n\x07castkey\x18\xf0\xfb\x03\x20\x01(\t\x12\x1d.google.protobuf.Fiel\
    dOptionsR\x07castkey:=\n\tcastvalue\x18\xf1\xfb\x03\x20\x01(\t\x12\x1d.g\
    oogle.protobuf.FieldOptionsR\tcastvalue:9\n\x07stdtime\x18\xf2\xfb\x03\
    \x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x07stdtime:A\n\x0bst\
    dduration\x18\xf3\xfb\x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOpti\
    onsR\x0bstdduration:?\n\nwktpointer\x18\xf4\xfb\x03\x20\x01(\x08\x12\x1d\
    .google.protobuf.FieldOptionsR\nwktpointerBE\n\x13com.google.protobufB\n\
    GoGoProtosZ\"github.com/gogo/protobuf/gogoprotob\x06proto2\
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
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
