// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    request_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(|| {
                Response {
                    request_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string request_id = 1;

    pub fn clear_request_id(&mut self) {
        self.request_id.clear();
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: ::std::string::String) {
        self.request_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_id(&mut self) -> &mut ::std::string::String {
        if self.request_id.is_none() {
            self.request_id.set_default();
        };
        self.request_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_id(&mut self) -> ::std::string::String {
        self.request_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_request_id(&self) -> &str {
        match self.request_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.request_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.request_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.request_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "request_id",
                    Response::has_request_id,
                    Response::get_request_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_request_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.request_id == other.request_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ErrorResponse {
    // message fields
    field_type: ::std::option::Option<ErrorResponse_Type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ErrorResponse {}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorResponse {
        static mut instance: ::protobuf::lazy::Lazy<ErrorResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorResponse,
        };
        unsafe {
            instance.get(|| {
                ErrorResponse {
                    field_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .limitd.ErrorResponse.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ErrorResponse_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ErrorResponse_Type {
        self.field_type.unwrap_or(ErrorResponse_Type::UNKNOWN_BUCKET_TYPE)
    }
}

impl ::protobuf::Message for ErrorResponse {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ErrorResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ErrorResponse {
    fn new() -> ErrorResponse {
        ErrorResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErrorResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    ErrorResponse::has_field_type,
                    ErrorResponse::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorResponse>(
                    "ErrorResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErrorResponse {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ErrorResponse {
    fn eq(&self, other: &ErrorResponse) -> bool {
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ErrorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorResponse_Type {
    UNKNOWN_BUCKET_TYPE = 1,
}

impl ::protobuf::ProtobufEnum for ErrorResponse_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorResponse_Type> {
        match value {
            1 => ::std::option::Option::Some(ErrorResponse_Type::UNKNOWN_BUCKET_TYPE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorResponse_Type] = &[
            ErrorResponse_Type::UNKNOWN_BUCKET_TYPE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ErrorResponse_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorResponse_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorResponse_Type {
}

#[derive(Clone,Default)]
pub struct TakeResponse {
    // message fields
    conformant: ::std::option::Option<bool>,
    delayed: ::std::option::Option<bool>,
    remaining: ::std::option::Option<i32>,
    reset: ::std::option::Option<i32>,
    limit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TakeResponse {}

impl TakeResponse {
    pub fn new() -> TakeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TakeResponse {
        static mut instance: ::protobuf::lazy::Lazy<TakeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TakeResponse,
        };
        unsafe {
            instance.get(|| {
                TakeResponse {
                    conformant: ::std::option::Option::None,
                    delayed: ::std::option::Option::None,
                    remaining: ::std::option::Option::None,
                    reset: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool conformant = 1;

    pub fn clear_conformant(&mut self) {
        self.conformant = ::std::option::Option::None;
    }

    pub fn has_conformant(&self) -> bool {
        self.conformant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conformant(&mut self, v: bool) {
        self.conformant = ::std::option::Option::Some(v);
    }

    pub fn get_conformant(&self) -> bool {
        self.conformant.unwrap_or(false)
    }

    // optional bool delayed = 2;

    pub fn clear_delayed(&mut self) {
        self.delayed = ::std::option::Option::None;
    }

    pub fn has_delayed(&self) -> bool {
        self.delayed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delayed(&mut self, v: bool) {
        self.delayed = ::std::option::Option::Some(v);
    }

    pub fn get_delayed(&self) -> bool {
        self.delayed.unwrap_or(false)
    }

    // required int32 remaining = 3;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: i32) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> i32 {
        self.remaining.unwrap_or(0)
    }

    // required int32 reset = 4;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: i32) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> i32 {
        self.reset.unwrap_or(0)
    }

    // required int32 limit = 5;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i32) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> i32 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for TakeResponse {
    fn is_initialized(&self) -> bool {
        if self.conformant.is_none() {
            return false;
        };
        if self.remaining.is_none() {
            return false;
        };
        if self.reset.is_none() {
            return false;
        };
        if self.limit.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.conformant = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.delayed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.reset = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.limit = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.conformant.is_some() {
            my_size += 2;
        };
        if self.delayed.is_some() {
            my_size += 2;
        };
        for value in &self.remaining {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.reset {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.limit {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conformant {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.delayed {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.remaining {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.reset {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.limit {
            try!(os.write_int32(5, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TakeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TakeResponse {
    fn new() -> TakeResponse {
        TakeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TakeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "conformant",
                    TakeResponse::has_conformant,
                    TakeResponse::get_conformant,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "delayed",
                    TakeResponse::has_delayed,
                    TakeResponse::get_delayed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "remaining",
                    TakeResponse::has_remaining,
                    TakeResponse::get_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "reset",
                    TakeResponse::has_reset,
                    TakeResponse::get_reset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "limit",
                    TakeResponse::has_limit,
                    TakeResponse::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TakeResponse>(
                    "TakeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TakeResponse {
    fn clear(&mut self) {
        self.clear_conformant();
        self.clear_delayed();
        self.clear_remaining();
        self.clear_reset();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TakeResponse {
    fn eq(&self, other: &TakeResponse) -> bool {
        self.conformant == other.conformant &&
        self.delayed == other.delayed &&
        self.remaining == other.remaining &&
        self.reset == other.reset &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TakeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutResponse {
    // message fields
    remaining: ::std::option::Option<i32>,
    reset: ::std::option::Option<i32>,
    limit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutResponse {}

impl PutResponse {
    pub fn new() -> PutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutResponse,
        };
        unsafe {
            instance.get(|| {
                PutResponse {
                    remaining: ::std::option::Option::None,
                    reset: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 remaining = 1;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: i32) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> i32 {
        self.remaining.unwrap_or(0)
    }

    // required int32 reset = 2;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: i32) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> i32 {
        self.reset.unwrap_or(0)
    }

    // required int32 limit = 3;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i32) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> i32 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for PutResponse {
    fn is_initialized(&self) -> bool {
        if self.remaining.is_none() {
            return false;
        };
        if self.reset.is_none() {
            return false;
        };
        if self.limit.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.reset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.limit = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.remaining {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.reset {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.limit {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.remaining {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.reset {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.limit {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PutResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutResponse {
    fn new() -> PutResponse {
        PutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "remaining",
                    PutResponse::has_remaining,
                    PutResponse::get_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "reset",
                    PutResponse::has_reset,
                    PutResponse::get_reset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "limit",
                    PutResponse::has_limit,
                    PutResponse::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutResponse>(
                    "PutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutResponse {
    fn clear(&mut self) {
        self.clear_remaining();
        self.clear_reset();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutResponse {
    fn eq(&self, other: &PutResponse) -> bool {
        self.remaining == other.remaining &&
        self.reset == other.reset &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusResponseItem {
    // message fields
    instance: ::protobuf::SingularField<::std::string::String>,
    remaining: ::std::option::Option<i32>,
    reset: ::std::option::Option<i32>,
    limit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusResponseItem {}

impl StatusResponseItem {
    pub fn new() -> StatusResponseItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusResponseItem {
        static mut instance: ::protobuf::lazy::Lazy<StatusResponseItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusResponseItem,
        };
        unsafe {
            instance.get(|| {
                StatusResponseItem {
                    instance: ::protobuf::SingularField::none(),
                    remaining: ::std::option::Option::None,
                    reset: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string instance = 1;

    pub fn clear_instance(&mut self) {
        self.instance.clear();
    }

    pub fn has_instance(&self) -> bool {
        self.instance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance(&mut self, v: ::std::string::String) {
        self.instance = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance(&mut self) -> &mut ::std::string::String {
        if self.instance.is_none() {
            self.instance.set_default();
        };
        self.instance.as_mut().unwrap()
    }

    // Take field
    pub fn take_instance(&mut self) -> ::std::string::String {
        self.instance.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_instance(&self) -> &str {
        match self.instance.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 remaining = 2;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: i32) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> i32 {
        self.remaining.unwrap_or(0)
    }

    // required int32 reset = 3;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: i32) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> i32 {
        self.reset.unwrap_or(0)
    }

    // required int32 limit = 4;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: i32) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> i32 {
        self.limit.unwrap_or(0)
    }
}

impl ::protobuf::Message for StatusResponseItem {
    fn is_initialized(&self) -> bool {
        if self.instance.is_none() {
            return false;
        };
        if self.remaining.is_none() {
            return false;
        };
        if self.reset.is_none() {
            return false;
        };
        if self.limit.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.instance));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.reset = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.limit = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.instance {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.remaining {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.reset {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.limit {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.instance.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.remaining {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.reset {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.limit {
            try!(os.write_int32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusResponseItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusResponseItem {
    fn new() -> StatusResponseItem {
        StatusResponseItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusResponseItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "instance",
                    StatusResponseItem::has_instance,
                    StatusResponseItem::get_instance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "remaining",
                    StatusResponseItem::has_remaining,
                    StatusResponseItem::get_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "reset",
                    StatusResponseItem::has_reset,
                    StatusResponseItem::get_reset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "limit",
                    StatusResponseItem::has_limit,
                    StatusResponseItem::get_limit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusResponseItem>(
                    "StatusResponseItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusResponseItem {
    fn clear(&mut self) {
        self.clear_instance();
        self.clear_remaining();
        self.clear_reset();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusResponseItem {
    fn eq(&self, other: &StatusResponseItem) -> bool {
        self.instance == other.instance &&
        self.remaining == other.remaining &&
        self.reset == other.reset &&
        self.limit == other.limit &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusResponseItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusResponse {
    // message fields
    items: ::protobuf::RepeatedField<StatusResponseItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusResponse {}

impl StatusResponse {
    pub fn new() -> StatusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusResponse {
        static mut instance: ::protobuf::lazy::Lazy<StatusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusResponse,
        };
        unsafe {
            instance.get(|| {
                StatusResponse {
                    items: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .limitd.StatusResponseItem items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<StatusResponseItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<StatusResponseItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<StatusResponseItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[StatusResponseItem] {
        &self.items
    }
}

impl ::protobuf::Message for StatusResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusResponse {
    fn new() -> StatusResponse {
        StatusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    StatusResponse::get_items,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusResponse>(
                    "StatusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusResponse {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusResponse {
    fn eq(&self, other: &StatusResponse) -> bool {
        self.items == other.items &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PongResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PongResponse {}

impl PongResponse {
    pub fn new() -> PongResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PongResponse {
        static mut instance: ::protobuf::lazy::Lazy<PongResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PongResponse,
        };
        unsafe {
            instance.get(|| {
                PongResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PongResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PongResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PongResponse {
    fn new() -> PongResponse {
        PongResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PongResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PongResponse>(
                    "PongResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PongResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PongResponse {
    fn eq(&self, other: &PongResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PongResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x06, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x22, 0x28, 0x0a, 0x08, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x2a, 0x08, 0x08, 0x64, 0x10, 0x80,
    0x80, 0x80, 0x80, 0x02, 0x22, 0x95, 0x01, 0x0a, 0x0d, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x28, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65,
    0x32, 0x39, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x64, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x2e, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x64, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1f, 0x0a, 0x04, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x42,
    0x55, 0x43, 0x4b, 0x45, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x10, 0x01, 0x22, 0xa5, 0x01, 0x0a,
    0x0c, 0x54, 0x61, 0x6b, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a,
    0x0a, 0x63, 0x6f, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x08, 0x12, 0x16, 0x0a, 0x07, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x6d,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05,
    0x72, 0x65, 0x73, 0x65, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x18, 0x05, 0x20, 0x02, 0x28, 0x05, 0x32, 0x38, 0x0a, 0x08, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x65, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x54, 0x61, 0x6b, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x10, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x77, 0x0a, 0x0b, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x72, 0x65, 0x73, 0x65, 0x74, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x05, 0x32, 0x37, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x18, 0x66, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e,
    0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x2e, 0x6c, 0x69,
    0x6d, 0x69, 0x74, 0x64, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x57, 0x0a,
    0x12, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x49,
    0x74, 0x65, 0x6d, 0x12, 0x10, 0x0a, 0x08, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69,
    0x6e, 0x67, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x72, 0x65, 0x73, 0x65,
    0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x22, 0x77, 0x0a, 0x0e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x49,
    0x74, 0x65, 0x6d, 0x32, 0x3a, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18,
    0x67, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x2e,
    0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x48, 0x0a, 0x0c, 0x50, 0x6f, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32,
    0x38, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x68, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64, 0x2e, 0x50, 0x6f, 0x6e, 0x67, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x2e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x64,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4a, 0xc1, 0x0e, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x3e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x02, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x05, 0x12, 0x03, 0x03,
    0x02, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x05, 0x00, 0x12, 0x03, 0x03, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x05, 0x00, 0x01, 0x12, 0x03, 0x03, 0x0d, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x05, 0x00, 0x02, 0x12, 0x03, 0x03, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x01, 0x06, 0x12,
    0x04, 0x09, 0x02, 0x0b, 0x03, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x06, 0x00, 0x12, 0x03, 0x0a,
    0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x06, 0x00, 0x02, 0x12, 0x03, 0x09, 0x09, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x06, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x06, 0x00, 0x06, 0x12, 0x03, 0x0a, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x1b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x06, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x26, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00,
    0x12, 0x04, 0x0d, 0x02, 0x0f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0e, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0e, 0x04, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x0e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x14, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x14,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x02, 0x06, 0x12, 0x04, 0x15, 0x02, 0x17, 0x03, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x06, 0x00, 0x12, 0x03, 0x16, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x06, 0x00, 0x02, 0x12, 0x03, 0x15, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x06, 0x00,
    0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x06, 0x00, 0x06, 0x12,
    0x03, 0x16, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x06, 0x00, 0x01, 0x12, 0x03, 0x16,
    0x1a, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x06, 0x00, 0x03, 0x12, 0x03, 0x16, 0x25, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x19, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x19, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a,
    0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x08, 0x12, 0x03, 0x1a, 0x1c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x07, 0x12, 0x03, 0x1a, 0x27, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x1c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1d,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1d, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x1e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x1e, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x21, 0x00, 0x29,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x21, 0x08, 0x13, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x03, 0x06, 0x12, 0x04, 0x22, 0x02, 0x24, 0x03, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x06, 0x00, 0x12, 0x03, 0x23, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x06, 0x00, 0x02,
    0x12, 0x03, 0x22, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x06, 0x00, 0x04, 0x12, 0x03,
    0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x06, 0x00, 0x06, 0x12, 0x03, 0x23, 0x0d,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x23, 0x19, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x06, 0x00, 0x03, 0x12, 0x03, 0x23, 0x24, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x26, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x26, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x27, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x28, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x28,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x2b, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x2b, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x2d, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2d,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2d, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x2e, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x2e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x32, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x32, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x06, 0x12, 0x04, 0x33, 0x02, 0x35, 0x03, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x06, 0x00, 0x12, 0x03, 0x34, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x06, 0x00, 0x02, 0x12, 0x03, 0x33, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x06, 0x00,
    0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x06, 0x00, 0x06, 0x12,
    0x03, 0x34, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x06, 0x00, 0x01, 0x12, 0x03, 0x34,
    0x1c, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x06, 0x00, 0x03, 0x12, 0x03, 0x34, 0x27, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x37, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x37, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x3a, 0x00, 0x3e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x06, 0x06, 0x12, 0x04, 0x3b, 0x02, 0x3d, 0x03, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x06, 0x00, 0x12, 0x03, 0x3c, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x06, 0x00, 0x02,
    0x12, 0x03, 0x3b, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x06, 0x00, 0x04, 0x12, 0x03,
    0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x06, 0x00, 0x06, 0x12, 0x03, 0x3c, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x06, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x1a, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x06, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x25, 0x28,
];

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
