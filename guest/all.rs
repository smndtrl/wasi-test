#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
/// Generated bindings module for this component.
#[allow(dead_code)]
pub(crate) mod bindings {
    pub mod wasi {
        pub mod http {
            #[allow(clippy::all)]
            pub mod types {
                pub type InputStream = super::super::super::wasi::io::streams::InputStream;
                pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
                pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                pub enum Method {
                    Get,
                    Head,
                    Post,
                    Put,
                    Delete,
                    Connect,
                    Options,
                    Trace,
                    Patch,
                    Other(::cargo_component_bindings::rt::string::String),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Method {
                    #[inline]
                    fn clone(&self) -> Method {
                        match self {
                            Method::Get => Method::Get,
                            Method::Head => Method::Head,
                            Method::Post => Method::Post,
                            Method::Put => Method::Put,
                            Method::Delete => Method::Delete,
                            Method::Connect => Method::Connect,
                            Method::Options => Method::Options,
                            Method::Trace => Method::Trace,
                            Method::Patch => Method::Patch,
                            Method::Other(__self_0) => {
                                Method::Other(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for Method {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Method::Get => f.debug_tuple("Method::Get").finish(),
                            Method::Head => f.debug_tuple("Method::Head").finish(),
                            Method::Post => f.debug_tuple("Method::Post").finish(),
                            Method::Put => f.debug_tuple("Method::Put").finish(),
                            Method::Delete => f.debug_tuple("Method::Delete").finish(),
                            Method::Connect => f.debug_tuple("Method::Connect").finish(),
                            Method::Options => f.debug_tuple("Method::Options").finish(),
                            Method::Trace => f.debug_tuple("Method::Trace").finish(),
                            Method::Patch => f.debug_tuple("Method::Patch").finish(),
                            Method::Other(e) => {
                                f.debug_tuple("Method::Other").field(e).finish()
                            }
                        }
                    }
                }
                pub enum Scheme {
                    Http,
                    Https,
                    Other(::cargo_component_bindings::rt::string::String),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Scheme {
                    #[inline]
                    fn clone(&self) -> Scheme {
                        match self {
                            Scheme::Http => Scheme::Http,
                            Scheme::Https => Scheme::Https,
                            Scheme::Other(__self_0) => {
                                Scheme::Other(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for Scheme {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                            Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                            Scheme::Other(e) => {
                                f.debug_tuple("Scheme::Other").field(e).finish()
                            }
                        }
                    }
                }
                pub enum Error {
                    InvalidUrl(::cargo_component_bindings::rt::string::String),
                    TimeoutError(::cargo_component_bindings::rt::string::String),
                    ProtocolError(::cargo_component_bindings::rt::string::String),
                    UnexpectedError(::cargo_component_bindings::rt::string::String),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Error {
                    #[inline]
                    fn clone(&self) -> Error {
                        match self {
                            Error::InvalidUrl(__self_0) => {
                                Error::InvalidUrl(::core::clone::Clone::clone(__self_0))
                            }
                            Error::TimeoutError(__self_0) => {
                                Error::TimeoutError(::core::clone::Clone::clone(__self_0))
                            }
                            Error::ProtocolError(__self_0) => {
                                Error::ProtocolError(::core::clone::Clone::clone(__self_0))
                            }
                            Error::UnexpectedError(__self_0) => {
                                Error::UnexpectedError(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Error::InvalidUrl(e) => {
                                f.debug_tuple("Error::InvalidUrl").field(e).finish()
                            }
                            Error::TimeoutError(e) => {
                                f.debug_tuple("Error::TimeoutError").field(e).finish()
                            }
                            Error::ProtocolError(e) => {
                                f.debug_tuple("Error::ProtocolError").field(e).finish()
                            }
                            Error::UnexpectedError(e) => {
                                f.debug_tuple("Error::UnexpectedError").field(e).finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl std::error::Error for Error {}
                #[repr(transparent)]
                pub struct Fields {
                    handle: ::cargo_component_bindings::rt::Resource<Fields>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Fields {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Fields",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl Fields {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource for Fields {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                pub type Headers = Fields;
                pub type Trailers = Fields;
                #[repr(transparent)]
                pub struct IncomingRequest {
                    handle: ::cargo_component_bindings::rt::Resource<IncomingRequest>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for IncomingRequest {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "IncomingRequest",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl IncomingRequest {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for IncomingRequest {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(transparent)]
                pub struct OutgoingRequest {
                    handle: ::cargo_component_bindings::rt::Resource<OutgoingRequest>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for OutgoingRequest {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OutgoingRequest",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl OutgoingRequest {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for OutgoingRequest {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(C)]
                pub struct RequestOptions {
                    pub connect_timeout_ms: Option<u32>,
                    pub first_byte_timeout_ms: Option<u32>,
                    pub between_bytes_timeout_ms: Option<u32>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for RequestOptions {
                    #[inline]
                    fn clone(&self) -> RequestOptions {
                        let _: ::core::clone::AssertParamIsClone<Option<u32>>;
                        let _: ::core::clone::AssertParamIsClone<Option<u32>>;
                        let _: ::core::clone::AssertParamIsClone<Option<u32>>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for RequestOptions {}
                impl ::core::fmt::Debug for RequestOptions {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("RequestOptions")
                            .field("connect-timeout-ms", &self.connect_timeout_ms)
                            .field("first-byte-timeout-ms", &self.first_byte_timeout_ms)
                            .field(
                                "between-bytes-timeout-ms",
                                &self.between_bytes_timeout_ms,
                            )
                            .finish()
                    }
                }
                #[repr(transparent)]
                pub struct ResponseOutparam {
                    handle: ::cargo_component_bindings::rt::Resource<ResponseOutparam>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ResponseOutparam {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ResponseOutparam",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl ResponseOutparam {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for ResponseOutparam {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                pub type StatusCode = u16;
                #[repr(transparent)]
                pub struct IncomingResponse {
                    handle: ::cargo_component_bindings::rt::Resource<IncomingResponse>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for IncomingResponse {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "IncomingResponse",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl IncomingResponse {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for IncomingResponse {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(transparent)]
                pub struct IncomingBody {
                    handle: ::cargo_component_bindings::rt::Resource<IncomingBody>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for IncomingBody {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "IncomingBody",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl IncomingBody {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for IncomingBody {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(transparent)]
                pub struct FutureTrailers {
                    handle: ::cargo_component_bindings::rt::Resource<FutureTrailers>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for FutureTrailers {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "FutureTrailers",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl FutureTrailers {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for FutureTrailers {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(transparent)]
                pub struct OutgoingResponse {
                    handle: ::cargo_component_bindings::rt::Resource<OutgoingResponse>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for OutgoingResponse {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OutgoingResponse",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl OutgoingResponse {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for OutgoingResponse {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[repr(transparent)]
                pub struct OutgoingBody {
                    handle: ::cargo_component_bindings::rt::Resource<OutgoingBody>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for OutgoingBody {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OutgoingBody",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl OutgoingBody {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for OutgoingBody {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                /// The following block defines a special resource type used by the
                /// `wasi:http/outgoing-handler` interface to emulate
                /// `future<result<response, error>>` in advance of Preview3. Given a
                /// `future-incoming-response`, the client can call the non-blocking `get`
                /// method to get the result if it is available. If the result is not available,
                /// the client can call `listen` to get a `pollable` that can be passed to
                /// `wasi:io/poll.poll-list`.
                #[repr(transparent)]
                pub struct FutureIncomingResponse {
                    handle: ::cargo_component_bindings::rt::Resource<
                        FutureIncomingResponse,
                    >,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for FutureIncomingResponse {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "FutureIncomingResponse",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl FutureIncomingResponse {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for FutureIncomingResponse {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn new(
                        entries: &[(
                            ::cargo_component_bindings::rt::string::String,
                            ::cargo_component_bindings::rt::vec::Vec<u8>,
                        )],
                    ) -> Self {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let vec3 = entries;
                            let len3 = vec3.len() as i32;
                            let layout3 = alloc::Layout::from_size_align_unchecked(
                                vec3.len() * 16,
                                4,
                            );
                            let result3 = if layout3.size() != 0 {
                                let ptr = alloc::alloc(layout3);
                                if ptr.is_null() {
                                    alloc::handle_alloc_error(layout3);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec3.into_iter().enumerate() {
                                let base = result3 as i32 + (i as i32) * 16;
                                {
                                    let (t0_0, t0_1) = e;
                                    let vec1 = t0_0;
                                    let ptr1 = vec1.as_ptr() as i32;
                                    let len1 = vec1.len() as i32;
                                    *((base + 4) as *mut i32) = len1;
                                    *((base + 0) as *mut i32) = ptr1;
                                    let vec2 = t0_1;
                                    let ptr2 = vec2.as_ptr() as i32;
                                    let len2 = vec2.len() as i32;
                                    *((base + 12) as *mut i32) = len2;
                                    *((base + 8) as *mut i32) = ptr2;
                                }
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import(result3 as i32, len3);
                            if layout3.size() != 0 {
                                alloc::dealloc(result3, layout3);
                            }
                            Fields::from_handle(ret as u32)
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn get(
                        &self,
                        name: &str,
                    ) -> ::cargo_component_bindings::rt::vec::Vec<
                        ::cargo_component_bindings::rt::vec::Vec<u8>,
                    > {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let vec0 = name;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            let ptr1 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0, len0, ptr1);
                            let l2 = *((ptr1 + 0) as *const i32);
                            let l3 = *((ptr1 + 4) as *const i32);
                            let base7 = l2;
                            let len7 = l3;
                            let mut result7 = Vec::with_capacity(len7 as usize);
                            for i in 0..len7 {
                                let base = base7 + i * 8;
                                let e7 = {
                                    let l4 = *((base + 0) as *const i32);
                                    let l5 = *((base + 4) as *const i32);
                                    let len6 = l5 as usize;
                                    Vec::from_raw_parts(l4 as *mut _, len6, len6)
                                };
                                result7.push(e7);
                            }
                            ::cargo_component_bindings::rt::dealloc(
                                base7,
                                (len7 as usize) * 8,
                                4,
                            );
                            result7
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn set(
                        &self,
                        name: &str,
                        value: &[::cargo_component_bindings::rt::vec::Vec<u8>],
                    ) {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let vec0 = name;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            let vec2 = value;
                            let len2 = vec2.len() as i32;
                            let layout2 = alloc::Layout::from_size_align_unchecked(
                                vec2.len() * 8,
                                4,
                            );
                            let result2 = if layout2.size() != 0 {
                                let ptr = alloc::alloc(layout2);
                                if ptr.is_null() {
                                    alloc::handle_alloc_error(layout2);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec2.into_iter().enumerate() {
                                let base = result2 as i32 + (i as i32) * 8;
                                {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr() as i32;
                                    let len1 = vec1.len() as i32;
                                    *((base + 4) as *mut i32) = len1;
                                    *((base + 0) as *mut i32) = ptr1;
                                }
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ptr0,
                                len0,
                                result2 as i32,
                                len2,
                            );
                            if layout2.size() != 0 {
                                alloc::dealloc(result2, layout2);
                            }
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn delete(&self, name: &str) {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let vec0 = name;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0, len0);
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn append(&self, name: &str, value: &[u8]) {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let vec0 = name;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            let vec1 = value;
                            let ptr1 = vec1.as_ptr() as i32;
                            let len1 = vec1.len() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0, len0, ptr1, len1);
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn entries(
                        &self,
                    ) -> ::cargo_component_bindings::rt::vec::Vec<
                        (
                            ::cargo_component_bindings::rt::string::String,
                            ::cargo_component_bindings::rt::vec::Vec<u8>,
                        ),
                    > {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = *((ptr0 + 0) as *const i32);
                            let l2 = *((ptr0 + 4) as *const i32);
                            let base9 = l1;
                            let len9 = l2;
                            let mut result9 = Vec::with_capacity(len9 as usize);
                            for i in 0..len9 {
                                let base = base9 + i * 16;
                                let e9 = {
                                    let l3 = *((base + 0) as *const i32);
                                    let l4 = *((base + 4) as *const i32);
                                    let len5 = l4 as usize;
                                    let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
                                    let l6 = *((base + 8) as *const i32);
                                    let l7 = *((base + 12) as *const i32);
                                    let len8 = l7 as usize;
                                    (
                                        ::cargo_component_bindings::rt::string_lift(bytes5),
                                        Vec::from_raw_parts(l6 as *mut _, len8, len8),
                                    )
                                };
                                result9.push(e9);
                            }
                            ::cargo_component_bindings::rt::dealloc(
                                base9,
                                (len9 as usize) * 16,
                                4,
                            );
                            result9
                        }
                    }
                }
                impl Fields {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn clone(&self) -> Fields {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            Fields::from_handle(ret as u32)
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn method(&self) -> Method {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            let v5 = match l1 {
                                0 => Method::Get,
                                1 => Method::Head,
                                2 => Method::Post,
                                3 => Method::Put,
                                4 => Method::Delete,
                                5 => Method::Connect,
                                6 => Method::Options,
                                7 => Method::Trace,
                                8 => Method::Patch,
                                n => {
                                    if true {
                                        match (&n, &9) {
                                            (left_val, right_val) => {
                                                if !(*left_val == *right_val) {
                                                    let kind = ::core::panicking::AssertKind::Eq;
                                                    ::core::panicking::assert_failed(
                                                        kind,
                                                        &*left_val,
                                                        &*right_val,
                                                        ::core::option::Option::Some(
                                                            format_args!("invalid enum discriminant"),
                                                        ),
                                                    );
                                                }
                                            }
                                        };
                                    }
                                    let e5 = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        let l3 = *((ptr0 + 8) as *const i32);
                                        let len4 = l3 as usize;
                                        let bytes4 = Vec::from_raw_parts(l2 as *mut _, len4, len4);
                                        ::cargo_component_bindings::rt::string_lift(bytes4)
                                    };
                                    Method::Other(e5)
                                }
                            };
                            v5
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn path_with_query(
                        &self,
                    ) -> Option<::cargo_component_bindings::rt::string::String> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        let l3 = *((ptr0 + 8) as *const i32);
                                        let len4 = l3 as usize;
                                        let bytes4 = Vec::from_raw_parts(l2 as *mut _, len4, len4);
                                        ::cargo_component_bindings::rt::string_lift(bytes4)
                                    };
                                    Some(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn scheme(&self) -> Option<Scheme> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v6 = match l2 {
                                            0 => Scheme::Http,
                                            1 => Scheme::Https,
                                            n => {
                                                if true {
                                                    match (&n, &2) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                let e6 = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    let len5 = l4 as usize;
                                                    let bytes5 = Vec::from_raw_parts(l3 as *mut _, len5, len5);
                                                    ::cargo_component_bindings::rt::string_lift(bytes5)
                                                };
                                                Scheme::Other(e6)
                                            }
                                        };
                                        v6
                                    };
                                    Some(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn authority(
                        &self,
                    ) -> Option<::cargo_component_bindings::rt::string::String> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        let l3 = *((ptr0 + 8) as *const i32);
                                        let len4 = l3 as usize;
                                        let bytes4 = Vec::from_raw_parts(l2 as *mut _, len4, len4);
                                        ::cargo_component_bindings::rt::string_lift(bytes4)
                                    };
                                    Some(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn headers(&self) -> Headers {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            Fields::from_handle(ret as u32)
                        }
                    }
                }
                impl IncomingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn consume(&self) -> Result<IncomingBody, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        IncomingBody::from_handle(l2 as u32)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutgoingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn new(
                        method: &Method,
                        path_with_query: Option<&str>,
                        scheme: Option<&Scheme>,
                        authority: Option<&str>,
                        headers: &Headers,
                    ) -> Self {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let (result1_0, result1_1, result1_2) = match method {
                                Method::Get => (0i32, 0i32, 0i32),
                                Method::Head => (1i32, 0i32, 0i32),
                                Method::Post => (2i32, 0i32, 0i32),
                                Method::Put => (3i32, 0i32, 0i32),
                                Method::Delete => (4i32, 0i32, 0i32),
                                Method::Connect => (5i32, 0i32, 0i32),
                                Method::Options => (6i32, 0i32, 0i32),
                                Method::Trace => (7i32, 0i32, 0i32),
                                Method::Patch => (8i32, 0i32, 0i32),
                                Method::Other(e) => {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr() as i32;
                                    let len0 = vec0.len() as i32;
                                    (9i32, ptr0, len0)
                                }
                            };
                            let (result3_0, result3_1, result3_2) = match path_with_query {
                                Some(e) => {
                                    let vec2 = e;
                                    let ptr2 = vec2.as_ptr() as i32;
                                    let len2 = vec2.len() as i32;
                                    (1i32, ptr2, len2)
                                }
                                None => (0i32, 0i32, 0i32),
                            };
                            let (result6_0, result6_1, result6_2, result6_3) = match scheme {
                                Some(e) => {
                                    let (result5_0, result5_1, result5_2) = match e {
                                        Scheme::Http => (0i32, 0i32, 0i32),
                                        Scheme::Https => (1i32, 0i32, 0i32),
                                        Scheme::Other(e) => {
                                            let vec4 = e;
                                            let ptr4 = vec4.as_ptr() as i32;
                                            let len4 = vec4.len() as i32;
                                            (2i32, ptr4, len4)
                                        }
                                    };
                                    (1i32, result5_0, result5_1, result5_2)
                                }
                                None => (0i32, 0i32, 0i32, 0i32),
                            };
                            let (result8_0, result8_1, result8_2) = match authority {
                                Some(e) => {
                                    let vec7 = e;
                                    let ptr7 = vec7.as_ptr() as i32;
                                    let len7 = vec7.len() as i32;
                                    (1i32, ptr7, len7)
                                }
                                None => (0i32, 0i32, 0i32),
                            };
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                            ) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import(
                                result1_0,
                                result1_1,
                                result1_2,
                                result3_0,
                                result3_1,
                                result3_2,
                                result6_0,
                                result6_1,
                                result6_2,
                                result6_3,
                                result8_0,
                                result8_1,
                                result8_2,
                                (headers).handle() as i32,
                            );
                            OutgoingRequest::from_handle(ret as u32)
                        }
                    }
                }
                impl OutgoingRequest {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn write(&self) -> Result<OutgoingBody, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        OutgoingBody::from_handle(l2 as u32)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl ResponseOutparam {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn set(
                        param: ResponseOutparam,
                        response: Result<OutgoingResponse, &Error>,
                    ) {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let (result5_0, result5_1, result5_2, result5_3) = match response {
                                Ok(e) => (0i32, (e).into_handle() as i32, 0i32, 0i32),
                                Err(e) => {
                                    let (result4_0, result4_1, result4_2) = match e {
                                        Error::InvalidUrl(e) => {
                                            let vec0 = e;
                                            let ptr0 = vec0.as_ptr() as i32;
                                            let len0 = vec0.len() as i32;
                                            (0i32, ptr0, len0)
                                        }
                                        Error::TimeoutError(e) => {
                                            let vec1 = e;
                                            let ptr1 = vec1.as_ptr() as i32;
                                            let len1 = vec1.len() as i32;
                                            (1i32, ptr1, len1)
                                        }
                                        Error::ProtocolError(e) => {
                                            let vec2 = e;
                                            let ptr2 = vec2.as_ptr() as i32;
                                            let len2 = vec2.len() as i32;
                                            (2i32, ptr2, len2)
                                        }
                                        Error::UnexpectedError(e) => {
                                            let vec3 = e;
                                            let ptr3 = vec3.as_ptr() as i32;
                                            let len3 = vec3.len() as i32;
                                            (3i32, ptr3, len3)
                                        }
                                    };
                                    (1i32, result4_0, result4_1, result4_2)
                                }
                            };
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (param).into_handle() as i32,
                                result5_0,
                                result5_1,
                                result5_2,
                                result5_3,
                            );
                        }
                    }
                }
                impl IncomingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn status(&self) -> StatusCode {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            ret as u16
                        }
                    }
                }
                impl IncomingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn headers(&self) -> Headers {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            Fields::from_handle(ret as u32)
                        }
                    }
                }
                impl IncomingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn consume(&self) -> Result<IncomingBody, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        IncomingBody::from_handle(l2 as u32)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl IncomingBody {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn stream(&self) -> Result<InputStream, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l2 as u32,
                                        )
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl IncomingBody {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn finish(this: IncomingBody) -> FutureTrailers {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((this).into_handle() as i32);
                            FutureTrailers::from_handle(ret as u32)
                        }
                    }
                }
                impl FutureTrailers {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Pollable that resolves when the body has been fully read, and the trailers
                    /// are ready to be consumed.
                    pub fn subscribe(&self) -> Pollable {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                }
                impl FutureTrailers {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Retrieve reference to trailers, if they are ready.
                    pub fn get(&self) -> Option<Result<Trailers, Error>> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 20]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        match l2 {
                                            0 => {
                                                let e = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    Fields::from_handle(l3 as u32)
                                                };
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = {
                                                    let l4 = i32::from(*((ptr0 + 8) as *const u8));
                                                    let v17 = match l4 {
                                                        0 => {
                                                            let e17 = {
                                                                let l5 = *((ptr0 + 12) as *const i32);
                                                                let l6 = *((ptr0 + 16) as *const i32);
                                                                let len7 = l6 as usize;
                                                                let bytes7 = Vec::from_raw_parts(l5 as *mut _, len7, len7);
                                                                ::cargo_component_bindings::rt::string_lift(bytes7)
                                                            };
                                                            Error::InvalidUrl(e17)
                                                        }
                                                        1 => {
                                                            let e17 = {
                                                                let l8 = *((ptr0 + 12) as *const i32);
                                                                let l9 = *((ptr0 + 16) as *const i32);
                                                                let len10 = l9 as usize;
                                                                let bytes10 = Vec::from_raw_parts(
                                                                    l8 as *mut _,
                                                                    len10,
                                                                    len10,
                                                                );
                                                                ::cargo_component_bindings::rt::string_lift(bytes10)
                                                            };
                                                            Error::TimeoutError(e17)
                                                        }
                                                        2 => {
                                                            let e17 = {
                                                                let l11 = *((ptr0 + 12) as *const i32);
                                                                let l12 = *((ptr0 + 16) as *const i32);
                                                                let len13 = l12 as usize;
                                                                let bytes13 = Vec::from_raw_parts(
                                                                    l11 as *mut _,
                                                                    len13,
                                                                    len13,
                                                                );
                                                                ::cargo_component_bindings::rt::string_lift(bytes13)
                                                            };
                                                            Error::ProtocolError(e17)
                                                        }
                                                        n => {
                                                            if true {
                                                                match (&n, &3) {
                                                                    (left_val, right_val) => {
                                                                        if !(*left_val == *right_val) {
                                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                                            ::core::panicking::assert_failed(
                                                                                kind,
                                                                                &*left_val,
                                                                                &*right_val,
                                                                                ::core::option::Option::Some(
                                                                                    format_args!("invalid enum discriminant"),
                                                                                ),
                                                                            );
                                                                        }
                                                                    }
                                                                };
                                                            }
                                                            let e17 = {
                                                                let l14 = *((ptr0 + 12) as *const i32);
                                                                let l15 = *((ptr0 + 16) as *const i32);
                                                                let len16 = l15 as usize;
                                                                let bytes16 = Vec::from_raw_parts(
                                                                    l14 as *mut _,
                                                                    len16,
                                                                    len16,
                                                                );
                                                                ::cargo_component_bindings::rt::string_lift(bytes16)
                                                            };
                                                            Error::UnexpectedError(e17)
                                                        }
                                                    };
                                                    v17
                                                };
                                                Err(e)
                                            }
                                            _ => {
                                                ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                            }
                                        }
                                    };
                                    Some(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutgoingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn new(status_code: StatusCode, headers: &Headers) -> Self {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import(
                                ::cargo_component_bindings::rt::as_i32(status_code),
                                (headers).handle() as i32,
                            );
                            OutgoingResponse::from_handle(ret as u32)
                        }
                    }
                }
                impl OutgoingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Will give the child outgoing-response at most once. subsequent calls will
                    /// return an error.
                    pub fn write(&self) -> Result<OutgoingBody, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        OutgoingBody::from_handle(l2 as u32)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutgoingBody {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Will give the child output-stream at most once. subsequent calls will
                    /// return an error.
                    pub fn write(&self) -> Result<OutputStream, ()> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l2 as u32,
                                        )
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = ();
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutgoingBody {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Finalize an outgoing body, optionally providing trailers. This must be
                    /// called to signal that the response is complete. If the `outgoing-body` is
                    /// dropped without calling `outgoing-body-finalize`, the implementation
                    /// should treat the body as corrupted.
                    pub fn finish(this: OutgoingBody, trailers: Option<Trailers>) {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            let (result0_0, result0_1) = match trailers {
                                Some(e) => (1i32, (e).into_handle() as i32),
                                None => (0i32, 0i32),
                            };
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (this).into_handle() as i32,
                                result0_0,
                                result0_1,
                            );
                        }
                    }
                }
                impl FutureIncomingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    /// option indicates readiness.
                    /// outer result indicates you are allowed to get the
                    /// incoming-response-or-error at most once. subsequent calls after ready
                    /// will return an error here.
                    /// inner result indicates whether the incoming-response was available, or an
                    /// error occured.
                    pub fn get(
                        &self,
                    ) -> Option<Result<Result<IncomingResponse, Error>, ()>> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 24]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        match l2 {
                                            0 => {
                                                let e = {
                                                    let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                                    match l3 {
                                                        0 => {
                                                            let e = {
                                                                let l4 = *((ptr0 + 12) as *const i32);
                                                                IncomingResponse::from_handle(l4 as u32)
                                                            };
                                                            Ok(e)
                                                        }
                                                        1 => {
                                                            let e = {
                                                                let l5 = i32::from(*((ptr0 + 12) as *const u8));
                                                                let v18 = match l5 {
                                                                    0 => {
                                                                        let e18 = {
                                                                            let l6 = *((ptr0 + 16) as *const i32);
                                                                            let l7 = *((ptr0 + 20) as *const i32);
                                                                            let len8 = l7 as usize;
                                                                            let bytes8 = Vec::from_raw_parts(l6 as *mut _, len8, len8);
                                                                            ::cargo_component_bindings::rt::string_lift(bytes8)
                                                                        };
                                                                        Error::InvalidUrl(e18)
                                                                    }
                                                                    1 => {
                                                                        let e18 = {
                                                                            let l9 = *((ptr0 + 16) as *const i32);
                                                                            let l10 = *((ptr0 + 20) as *const i32);
                                                                            let len11 = l10 as usize;
                                                                            let bytes11 = Vec::from_raw_parts(
                                                                                l9 as *mut _,
                                                                                len11,
                                                                                len11,
                                                                            );
                                                                            ::cargo_component_bindings::rt::string_lift(bytes11)
                                                                        };
                                                                        Error::TimeoutError(e18)
                                                                    }
                                                                    2 => {
                                                                        let e18 = {
                                                                            let l12 = *((ptr0 + 16) as *const i32);
                                                                            let l13 = *((ptr0 + 20) as *const i32);
                                                                            let len14 = l13 as usize;
                                                                            let bytes14 = Vec::from_raw_parts(
                                                                                l12 as *mut _,
                                                                                len14,
                                                                                len14,
                                                                            );
                                                                            ::cargo_component_bindings::rt::string_lift(bytes14)
                                                                        };
                                                                        Error::ProtocolError(e18)
                                                                    }
                                                                    n => {
                                                                        if true {
                                                                            match (&n, &3) {
                                                                                (left_val, right_val) => {
                                                                                    if !(*left_val == *right_val) {
                                                                                        let kind = ::core::panicking::AssertKind::Eq;
                                                                                        ::core::panicking::assert_failed(
                                                                                            kind,
                                                                                            &*left_val,
                                                                                            &*right_val,
                                                                                            ::core::option::Option::Some(
                                                                                                format_args!("invalid enum discriminant"),
                                                                                            ),
                                                                                        );
                                                                                    }
                                                                                }
                                                                            };
                                                                        }
                                                                        let e18 = {
                                                                            let l15 = *((ptr0 + 16) as *const i32);
                                                                            let l16 = *((ptr0 + 20) as *const i32);
                                                                            let len17 = l16 as usize;
                                                                            let bytes17 = Vec::from_raw_parts(
                                                                                l15 as *mut _,
                                                                                len17,
                                                                                len17,
                                                                            );
                                                                            ::cargo_component_bindings::rt::string_lift(bytes17)
                                                                        };
                                                                        Error::UnexpectedError(e18)
                                                                    }
                                                                };
                                                                v18
                                                            };
                                                            Err(e)
                                                        }
                                                        _ => {
                                                            ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                                        }
                                                    }
                                                };
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = ();
                                                Err(e)
                                            }
                                            _ => {
                                                ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                            }
                                        }
                                    };
                                    Some(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl FutureIncomingResponse {
                    #[allow(unused_unsafe, clippy::all)]
                    pub fn subscribe(&self) -> Pollable {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod outgoing_handler {
                pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
                pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
                pub type FutureIncomingResponse = super::super::super::wasi::http::types::FutureIncomingResponse;
                pub type Error = super::super::super::wasi::http::types::Error;
                #[allow(unused_unsafe, clippy::all)]
                pub fn handle(
                    request: OutgoingRequest,
                    options: Option<RequestOptions>,
                ) -> Result<FutureIncomingResponse, Error> {
                    #[allow(unused_imports)]
                    use ::cargo_component_bindings::rt::{
                        alloc, vec::Vec, string::String,
                    };
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 16]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let (
                            result4_0,
                            result4_1,
                            result4_2,
                            result4_3,
                            result4_4,
                            result4_5,
                            result4_6,
                        ) = match options {
                            Some(e) => {
                                let super::super::super::wasi::http::types::RequestOptions {
                                    connect_timeout_ms: connect_timeout_ms0,
                                    first_byte_timeout_ms: first_byte_timeout_ms0,
                                    between_bytes_timeout_ms: between_bytes_timeout_ms0,
                                } = e;
                                let (result1_0, result1_1) = match connect_timeout_ms0 {
                                    Some(e) => (1i32, ::cargo_component_bindings::rt::as_i32(e)),
                                    None => (0i32, 0i32),
                                };
                                let (result2_0, result2_1) = match first_byte_timeout_ms0 {
                                    Some(e) => (1i32, ::cargo_component_bindings::rt::as_i32(e)),
                                    None => (0i32, 0i32),
                                };
                                let (result3_0, result3_1) = match between_bytes_timeout_ms0 {
                                    Some(e) => (1i32, ::cargo_component_bindings::rt::as_i32(e)),
                                    None => (0i32, 0i32),
                                };
                                (
                                    1i32,
                                    result1_0,
                                    result1_1,
                                    result2_0,
                                    result2_1,
                                    result3_0,
                                    result3_1,
                                )
                            }
                            None => (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32),
                        };
                        let ptr5 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                        ) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(
                            (request).into_handle() as i32,
                            result4_0,
                            result4_1,
                            result4_2,
                            result4_3,
                            result4_4,
                            result4_5,
                            result4_6,
                            ptr5,
                        );
                        let l6 = i32::from(*((ptr5 + 0) as *const u8));
                        match l6 {
                            0 => {
                                let e = {
                                    let l7 = *((ptr5 + 4) as *const i32);
                                    super::super::super::wasi::http::types::FutureIncomingResponse::from_handle(
                                        l7 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*((ptr5 + 4) as *const u8));
                                    use super::super::super::wasi::http::types::Error as V21;
                                    let v21 = match l8 {
                                        0 => {
                                            let e21 = {
                                                let l9 = *((ptr5 + 8) as *const i32);
                                                let l10 = *((ptr5 + 12) as *const i32);
                                                let len11 = l10 as usize;
                                                let bytes11 = Vec::from_raw_parts(
                                                    l9 as *mut _,
                                                    len11,
                                                    len11,
                                                );
                                                ::cargo_component_bindings::rt::string_lift(bytes11)
                                            };
                                            V21::InvalidUrl(e21)
                                        }
                                        1 => {
                                            let e21 = {
                                                let l12 = *((ptr5 + 8) as *const i32);
                                                let l13 = *((ptr5 + 12) as *const i32);
                                                let len14 = l13 as usize;
                                                let bytes14 = Vec::from_raw_parts(
                                                    l12 as *mut _,
                                                    len14,
                                                    len14,
                                                );
                                                ::cargo_component_bindings::rt::string_lift(bytes14)
                                            };
                                            V21::TimeoutError(e21)
                                        }
                                        2 => {
                                            let e21 = {
                                                let l15 = *((ptr5 + 8) as *const i32);
                                                let l16 = *((ptr5 + 12) as *const i32);
                                                let len17 = l16 as usize;
                                                let bytes17 = Vec::from_raw_parts(
                                                    l15 as *mut _,
                                                    len17,
                                                    len17,
                                                );
                                                ::cargo_component_bindings::rt::string_lift(bytes17)
                                            };
                                            V21::ProtocolError(e21)
                                        }
                                        n => {
                                            if true {
                                                match (&n, &3) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            let e21 = {
                                                let l18 = *((ptr5 + 8) as *const i32);
                                                let l19 = *((ptr5 + 12) as *const i32);
                                                let len20 = l19 as usize;
                                                let bytes20 = Vec::from_raw_parts(
                                                    l18 as *mut _,
                                                    len20,
                                                    len20,
                                                );
                                                ::cargo_component_bindings::rt::string_lift(bytes20)
                                            };
                                            V21::UnexpectedError(e21)
                                        }
                                    };
                                    v21
                                };
                                Err(e)
                            }
                            _ => {
                                ::cargo_component_bindings::rt::invalid_enum_discriminant()
                            }
                        }
                    }
                }
            }
        }
        pub mod io {
            #[allow(clippy::all)]
            pub mod poll {
                /// A "pollable" handle.
                #[repr(transparent)]
                pub struct Pollable {
                    handle: ::cargo_component_bindings::rt::Resource<Pollable>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Pollable {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Pollable",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl Pollable {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource for Pollable {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                #[allow(unused_unsafe, clippy::all)]
                /// Poll for completion on a set of pollables.
                ///
                /// This function takes a list of pollables, which identify I/O sources of
                /// interest, and waits until one or more of the events is ready for I/O.
                ///
                /// The result `list<u32>` contains one or more indices of handles in the
                /// argument list that is ready for I/O.
                ///
                /// If the list contains more elements than can be indexed with a `u32`
                /// value, this function traps.
                ///
                /// A timeout can be implemented by adding a pollable from the
                /// wasi-clocks API to the list.
                ///
                /// This function does not return a `result`; polling in itself does not
                /// do any I/O so it doesn't fail. If any of the I/O sources identified by
                /// the pollables has an error, it is indicated by marking the source as
                /// being reaedy for I/O.
                pub fn poll_list(
                    in_: &[&Pollable],
                ) -> ::cargo_component_bindings::rt::vec::Vec<u32> {
                    #[allow(unused_imports)]
                    use ::cargo_component_bindings::rt::{
                        alloc, vec::Vec, string::String,
                    };
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([u8; 8]);
                        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
                        let vec0 = in_;
                        let len0 = vec0.len() as i32;
                        let layout0 = alloc::Layout::from_size_align_unchecked(
                            vec0.len() * 4,
                            4,
                        );
                        let result0 = if layout0.size() != 0 {
                            let ptr = alloc::alloc(layout0);
                            if ptr.is_null() {
                                alloc::handle_alloc_error(layout0);
                            }
                            ptr
                        } else {
                            { ::core::ptr::null_mut() }
                        };
                        for (i, e) in vec0.into_iter().enumerate() {
                            let base = result0 as i32 + (i as i32) * 4;
                            {
                                *((base + 0) as *mut i32) = (e).handle() as i32;
                            }
                        }
                        let ptr1 = ret_area.as_mut_ptr() as i32;
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import(result0 as i32, len0, ptr1);
                        let l2 = *((ptr1 + 0) as *const i32);
                        let l3 = *((ptr1 + 4) as *const i32);
                        let len4 = l3 as usize;
                        if layout0.size() != 0 {
                            alloc::dealloc(result0, layout0);
                        }
                        Vec::from_raw_parts(l2 as *mut _, len4, len4)
                    }
                }
                #[allow(unused_unsafe, clippy::all)]
                /// Poll for completion on a single pollable.
                ///
                /// This function is similar to `poll-list`, but operates on only a single
                /// pollable. When it returns, the handle is ready for I/O.
                pub fn poll_one(in_: &Pollable) {
                    #[allow(unused_imports)]
                    use ::cargo_component_bindings::rt::{
                        alloc, vec::Vec, string::String,
                    };
                    unsafe {
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                        wit_import((in_).handle() as i32);
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod streams {
                pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                /// Contextual error information about the last failure that happened on
                /// a read, write, or flush from an `input-stream` or `output-stream`.
                ///
                /// This type is returned through the `stream-error` type whenever an
                /// operation on a stream directly fails or an error is discovered
                /// after-the-fact, for example when a write's failure shows up through a
                /// later `flush` or `check-write`.
                ///
                /// Interfaces such as `wasi:filesystem/types` provide functionality to
                /// further "downcast" this error into interface-specific error information.
                #[repr(transparent)]
                pub struct Error {
                    handle: ::cargo_component_bindings::rt::Resource<Error>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Error",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl Error {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource for Error {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                /// An error for input-stream and output-stream operations.
                pub enum StreamError {
                    /// The last operation (a write or flush) failed before completion.
                    ///
                    /// More information is available in the `error` payload.
                    LastOperationFailed(Error),
                    /// The stream is closed: no more input will be accepted by the
                    /// stream. A closed output-stream will return this error on all
                    /// future operations.
                    Closed,
                }
                impl ::core::fmt::Debug for StreamError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            StreamError::LastOperationFailed(e) => {
                                f.debug_tuple("StreamError::LastOperationFailed")
                                    .field(e)
                                    .finish()
                            }
                            StreamError::Closed => {
                                f.debug_tuple("StreamError::Closed").finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for StreamError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl std::error::Error for StreamError {}
                /// An input bytestream.
                ///
                /// `input-stream`s are *non-blocking* to the extent practical on underlying
                /// platforms. I/O operations always return promptly; if fewer bytes are
                /// promptly available than requested, they return the number of bytes promptly
                /// available, which could even be zero. To wait for data to be available,
                /// use the `subscribe` function to obtain a `pollable` which can be polled
                /// for using `wasi:io/poll`.
                #[repr(transparent)]
                pub struct InputStream {
                    handle: ::cargo_component_bindings::rt::Resource<InputStream>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for InputStream {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "InputStream",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl InputStream {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for InputStream {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                /// An output bytestream.
                ///
                /// `output-stream`s are *non-blocking* to the extent practical on
                /// underlying platforms. Except where specified otherwise, I/O operations also
                /// always return promptly, after the number of bytes that can be written
                /// promptly, which could even be zero. To wait for the stream to be ready to
                /// accept data, the `subscribe` function to obtain a `pollable` which can be
                /// polled for using `wasi:io/poll`.
                #[repr(transparent)]
                pub struct OutputStream {
                    handle: ::cargo_component_bindings::rt::Resource<OutputStream>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for OutputStream {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "OutputStream",
                            "handle",
                            &&self.handle,
                        )
                    }
                }
                impl OutputStream {
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: ::cargo_component_bindings::rt::Resource::from_handle(
                                handle,
                            ),
                        }
                    }
                    #[doc(hidden)]
                    pub fn into_handle(self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::into_handle(
                            self.handle,
                        )
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        ::cargo_component_bindings::rt::Resource::handle(&self.handle)
                    }
                }
                unsafe impl ::cargo_component_bindings::rt::WasmResource
                for OutputStream {
                    #[inline]
                    unsafe fn drop(handle: u32) {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        );
                    }
                }
                impl Error {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Returns a string that's suitable to assist humans in debugging this
                    /// error.
                    ///
                    /// The returned string will change across platforms and hosts which
                    /// means that parsing it, for example, would be a
                    /// platform-compatibility hazard.
                    pub fn to_debug_string(
                        &self,
                    ) -> ::cargo_component_bindings::rt::string::String {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 8]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = *((ptr0 + 0) as *const i32);
                            let l2 = *((ptr0 + 4) as *const i32);
                            let len3 = l2 as usize;
                            let bytes3 = Vec::from_raw_parts(l1 as *mut _, len3, len3);
                            ::cargo_component_bindings::rt::string_lift(bytes3)
                        }
                    }
                }
                impl InputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Perform a non-blocking read from the stream.
                    ///
                    /// This function returns a list of bytes containing the data that was
                    /// read, along with a `stream-status` which, indicates whether further
                    /// reads are expected to produce data. The returned list will contain up to
                    /// `len` bytes; it may return fewer than requested, but not more. An
                    /// empty list and `stream-status:open` indicates no more data is
                    /// available at this time, and that the pollable given by `subscribe`
                    /// will be ready when more data is available.
                    ///
                    /// Once a stream has reached the end, subsequent calls to `read` or
                    /// `skip` will always report `stream-status:ended` rather than producing more
                    /// data.
                    ///
                    /// When the caller gives a `len` of 0, it represents a request to read 0
                    /// bytes. This read should  always succeed and return an empty list and
                    /// the current `stream-status`.
                    ///
                    /// The `len` parameter is a `u64`, which could represent a list of u8 which
                    /// is not possible to allocate in wasm32, or not desirable to allocate as
                    /// as a return value by the callee. The callee may return a list of bytes
                    /// less than `len` in size while more bytes are available for reading.
                    pub fn read(
                        &self,
                        len: u64,
                    ) -> Result<
                        ::cargo_component_bindings::rt::vec::Vec<u8>,
                        StreamError,
                    > {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        let l3 = *((ptr0 + 8) as *const i32);
                                        let len4 = l3 as usize;
                                        Vec::from_raw_parts(l2 as *mut _, len4, len4)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v7 = match l5 {
                                            0 => {
                                                let e7 = {
                                                    let l6 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l6 as u32)
                                                };
                                                StreamError::LastOperationFailed(e7)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v7
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl InputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Read bytes from a stream, after blocking until at least one byte can
                    /// be read. Except for blocking, identical to `read`.
                    pub fn blocking_read(
                        &self,
                        len: u64,
                    ) -> Result<
                        ::cargo_component_bindings::rt::vec::Vec<u8>,
                        StreamError,
                    > {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 4) as *const i32);
                                        let l3 = *((ptr0 + 8) as *const i32);
                                        let len4 = l3 as usize;
                                        Vec::from_raw_parts(l2 as *mut _, len4, len4)
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v7 = match l5 {
                                            0 => {
                                                let e7 = {
                                                    let l6 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l6 as u32)
                                                };
                                                StreamError::LastOperationFailed(e7)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v7
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl InputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Skip bytes from a stream.
                    ///
                    /// This is similar to the `read` function, but avoids copying the
                    /// bytes into the instance.
                    ///
                    /// Once a stream has reached the end, subsequent calls to read or
                    /// `skip` will always report end-of-stream rather than producing more
                    /// data.
                    ///
                    /// This function returns the number of bytes skipped, along with a
                    /// `stream-status` indicating whether the end of the stream was
                    /// reached. The returned value will be at most `len`; it may be less.
                    pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl InputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Skip bytes from a stream, after blocking until at least one byte
                    /// can be skipped. Except for blocking behavior, identical to `skip`.
                    pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl InputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Create a `pollable` which will resolve once either the specified stream
                    /// has bytes available to read or the other end of the stream has been
                    /// closed.
                    /// The created `pollable` is a child resource of the `input-stream`.
                    /// Implementations may trap if the `input-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    pub fn subscribe(&self) -> Pollable {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Check readiness for writing. This function never blocks.
                    ///
                    /// Returns the number of bytes permitted for the next call to `write`,
                    /// or an error. Calling `write` with more bytes than this function has
                    /// permitted will trap.
                    ///
                    /// When this function returns 0 bytes, the `subscribe` pollable will
                    /// become ready when this function will report at least 1 byte, or an
                    /// error.
                    pub fn check_write(&self) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Perform a write. This function never blocks.
                    ///
                    /// Precondition: check-write gave permit of Ok(n) and contents has a
                    /// length of less than or equal to n. Otherwise, this function will trap.
                    ///
                    /// returns Err(closed) without writing if the stream has closed since
                    /// the last call to check-write provided a permit.
                    pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let vec0 = contents;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            let ptr1 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0, len0, ptr1);
                            let l2 = i32::from(*((ptr1 + 0) as *const u8));
                            match l2 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr1 + 4) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr1 + 8) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                    /// until all of these operations are complete, or an error occurs.
                    ///
                    /// This is a convenience wrapper around the use of `check-write`,
                    /// `subscribe`, `write`, and `flush`, and is implemented with the
                    /// following pseudo-code:
                    ///
                    /// ```text
                    /// let pollable = this.subscribe();
                    /// while !contents.is_empty() {
                    /// // Wait for the stream to become writable
                    /// poll-one(pollable);
                    /// let Ok(n) = this.check-write(); // eliding error handling
                    /// let len = min(n, contents.len());
                    /// let (chunk, rest) = contents.split_at(len);
                    /// this.write(chunk  );            // eliding error handling
                    /// contents = rest;
                    /// }
                    /// this.flush();
                    /// // Wait for completion of `flush`
                    /// poll-one(pollable);
                    /// // Check for any errors that arose during `flush`
                    /// let _ = this.check-write();         // eliding error handling
                    /// ```
                    pub fn blocking_write_and_flush(
                        &self,
                        contents: &[u8],
                    ) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let vec0 = contents;
                            let ptr0 = vec0.as_ptr() as i32;
                            let len0 = vec0.len() as i32;
                            let ptr1 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0, len0, ptr1);
                            let l2 = i32::from(*((ptr1 + 0) as *const u8));
                            match l2 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr1 + 4) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr1 + 8) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Request to flush buffered output. This function never blocks.
                    ///
                    /// This tells the output-stream that the caller intends any buffered
                    /// output to be flushed. the output which is expected to be flushed
                    /// is all that has been passed to `write` prior to this call.
                    ///
                    /// Upon calling this function, the `output-stream` will not accept any
                    /// writes (`check-write` will return `ok(0)`) until the flush has
                    /// completed. The `subscribe` pollable will become ready when the
                    /// flush has completed and the stream can accept more writes.
                    pub fn flush(&self) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v4 = match l2 {
                                            0 => {
                                                let e4 = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l3 as u32)
                                                };
                                                StreamError::LastOperationFailed(e4)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v4
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Request to flush buffered output, and block until flush completes
                    /// and stream is ready for writing again.
                    pub fn blocking_flush(&self) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import((self).handle() as i32, ptr0);
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v4 = match l2 {
                                            0 => {
                                                let e4 = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l3 as u32)
                                                };
                                                StreamError::LastOperationFailed(e4)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v4
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Create a `pollable` which will resolve once the output-stream
                    /// is ready for more writing, or an error has occured. When this
                    /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                    /// error.
                    ///
                    /// If the stream is closed, this pollable is always ready immediately.
                    ///
                    /// The created `pollable` is a child resource of the `output-stream`.
                    /// Implementations may trap if the `output-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    pub fn subscribe(&self) -> Pollable {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32) -> i32 {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            let ret = wit_import((self).handle() as i32);
                            super::super::super::wasi::io::poll::Pollable::from_handle(
                                ret as u32,
                            )
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Write zeroes to a stream.
                    ///
                    /// this should be used precisely like `write` with the exact same
                    /// preconditions (must use check-write first), but instead of
                    /// passing a list of bytes, you simply pass the number of zero-bytes
                    /// that should be written.
                    pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v4 = match l2 {
                                            0 => {
                                                let e4 = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l3 as u32)
                                                };
                                                StreamError::LastOperationFailed(e4)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v4
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Perform a write of up to 4096 zeroes, and then flush the stream.
                    /// Block until all of these operations are complete, or an error
                    /// occurs.
                    ///
                    /// This is a convenience wrapper around the use of `check-write`,
                    /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                    /// the following pseudo-code:
                    ///
                    /// ```text
                    /// let pollable = this.subscribe();
                    /// while num_zeroes != 0 {
                    /// // Wait for the stream to become writable
                    /// poll-one(pollable);
                    /// let Ok(n) = this.check-write(); // eliding error handling
                    /// let len = min(n, num_zeroes);
                    /// this.write-zeroes(len);         // eliding error handling
                    /// num_zeroes -= len;
                    /// }
                    /// this.flush();
                    /// // Wait for completion of `flush`
                    /// poll-one(pollable);
                    /// // Check for any errors that arose during `flush`
                    /// let _ = this.check-write();         // eliding error handling
                    /// ```
                    pub fn blocking_write_zeroes_and_flush(
                        &self,
                        len: u64,
                    ) -> Result<(), StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(4))]
                            struct RetArea([u8; 12]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l2 = i32::from(*((ptr0 + 4) as *const u8));
                                        let v4 = match l2 {
                                            0 => {
                                                let e4 = {
                                                    let l3 = *((ptr0 + 8) as *const i32);
                                                    Error::from_handle(l3 as u32)
                                                };
                                                StreamError::LastOperationFailed(e4)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v4
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Read from one stream and write to another.
                    ///
                    /// This function returns the number of bytes transferred; it may be less
                    /// than `len`.
                    ///
                    /// Unlike other I/O functions, this function blocks until all the data
                    /// read from the input stream has been written to the output stream.
                    pub fn splice(
                        &self,
                        src: InputStream,
                        len: u64,
                    ) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                (src).into_handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Read from one stream and write to another, with blocking.
                    ///
                    /// This is similar to `splice`, except that it blocks until at least
                    /// one byte can be read.
                    pub fn blocking_splice(
                        &self,
                        src: InputStream,
                        len: u64,
                    ) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i64, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                (src).into_handle() as i32,
                                ::cargo_component_bindings::rt::as_i64(len),
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
                impl OutputStream {
                    #[allow(unused_unsafe, clippy::all)]
                    /// Forward the entire contents of an input stream to an output stream.
                    ///
                    /// This function repeatedly reads from the input stream and writes
                    /// the data to the output stream, until the end of the input stream
                    /// is reached, or an error is encountered.
                    ///
                    /// Unlike other I/O functions, this function blocks until the end
                    /// of the input stream is seen and all the data has been written to
                    /// the output stream.
                    ///
                    /// This function returns the number of bytes transferred, and the status of
                    /// the output stream.
                    pub fn forward(&self, src: InputStream) -> Result<u64, StreamError> {
                        #[allow(unused_imports)]
                        use ::cargo_component_bindings::rt::{
                            alloc, vec::Vec, string::String,
                        };
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea([u8; 16]);
                            let mut ret_area = ::core::mem::MaybeUninit::<
                                RetArea,
                            >::uninit();
                            let ptr0 = ret_area.as_mut_ptr() as i32;
                            #[cfg(not(target_arch = "wasm32"))]
                            fn wit_import(_: i32, _: i32, _: i32) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import(
                                (self).handle() as i32,
                                (src).into_handle() as i32,
                                ptr0,
                            );
                            let l1 = i32::from(*((ptr0 + 0) as *const u8));
                            match l1 {
                                0 => {
                                    let e = {
                                        let l2 = *((ptr0 + 8) as *const i64);
                                        l2 as u64
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l3 = i32::from(*((ptr0 + 8) as *const u8));
                                        let v5 = match l3 {
                                            0 => {
                                                let e5 = {
                                                    let l4 = *((ptr0 + 12) as *const i32);
                                                    Error::from_handle(l4 as u32)
                                                };
                                                StreamError::LastOperationFailed(e5)
                                            }
                                            n => {
                                                if true {
                                                    match (&n, &1) {
                                                        (left_val, right_val) => {
                                                            if !(*left_val == *right_val) {
                                                                let kind = ::core::panicking::AssertKind::Eq;
                                                                ::core::panicking::assert_failed(
                                                                    kind,
                                                                    &*left_val,
                                                                    &*right_val,
                                                                    ::core::option::Option::Some(
                                                                        format_args!("invalid enum discriminant"),
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                    };
                                                }
                                                StreamError::Closed
                                            }
                                        };
                                        v5
                                    };
                                    Err(e)
                                }
                                _ => {
                                    ::cargo_component_bindings::rt::invalid_enum_discriminant()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod exports {
        pub mod smndtrl {
            pub mod simple {
                #[allow(clippy::all)]
                pub mod function {
                    const _: () = {
                        #[doc(hidden)]
                        #[export_name = "smndtrl:simple/function#map"]
                        #[allow(non_snake_case)]
                        unsafe extern "C" fn __export_map(arg0: i32, arg1: i32) -> i32 {
                            #[allow(unused_imports)]
                            use ::cargo_component_bindings::rt::{
                                alloc, vec::Vec, string::String,
                            };
                            let len0 = arg1 as usize;
                            let result1 = <_GuestImpl as Guest>::map(
                                Vec::from_raw_parts(arg0 as *mut _, len0, len0),
                            );
                            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
                            let vec3 = (result1).into_boxed_slice();
                            let ptr3 = vec3.as_ptr() as i32;
                            let len3 = vec3.len() as i32;
                            ::core::mem::forget(vec3);
                            *((ptr2 + 4) as *mut i32) = len3;
                            *((ptr2 + 0) as *mut i32) = ptr3;
                            ptr2
                        }
                        const _: () = {
                            #[doc(hidden)]
                            #[export_name = "cabi_post_smndtrl:simple/function#map"]
                            #[allow(non_snake_case)]
                            unsafe extern "C" fn __post_return_map(arg0: i32) {
                                let l0 = *((arg0 + 0) as *const i32);
                                let l1 = *((arg0 + 4) as *const i32);
                                let base2 = l0;
                                let len2 = l1;
                                ::cargo_component_bindings::rt::dealloc(
                                    base2,
                                    (len2 as usize) * 1,
                                    1,
                                );
                            }
                        };
                    };
                    use super::super::super::super::super::Component as _GuestImpl;
                    pub trait Guest {
                        fn map(
                            input: ::cargo_component_bindings::rt::vec::Vec<u8>,
                        ) -> ::cargo_component_bindings::rt::vec::Vec<u8>;
                    }
                    #[allow(unused_imports)]
                    use ::cargo_component_bindings::rt::{
                        alloc, vec::Vec, string::String,
                    };
                    #[repr(align(4))]
                    struct _RetArea([u8; 8]);
                    static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
                }
            }
        }
    }
}
use crate::bindings::exports::smndtrl::simple::function::Guest;
struct Component;
impl Guest for Component {
    fn map(_: Vec<u8>) -> Vec<u8> {
        ::core::panicking::panic("not yet implemented")
    }
}
