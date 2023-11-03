#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate dotenv;
use chatgpt::prelude::{gpt_function, ChatGPT};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
mod common {
    pub mod client_to_server {
        use std::time::SystemTime;
        use serde::{Deserialize, Serialize};
        use super::metadata::MessageOrigin;
        pub struct ClientToServerMessageBundle {
            pub connection_id: u32,
            pub socket_address: std::net::SocketAddr,
            pub send_time: SystemTime,
            pub received_time: SystemTime,
            pub origin: MessageOrigin,
            pub message: ClientToServerMessageData,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ClientToServerMessageBundle {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "connection_id",
                    "socket_address",
                    "send_time",
                    "received_time",
                    "origin",
                    "message",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.connection_id,
                    &self.socket_address,
                    &self.send_time,
                    &self.received_time,
                    &self.origin,
                    &&self.message,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ClientToServerMessageBundle",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ClientToServerMessageBundle {
            #[inline]
            fn clone(&self) -> ClientToServerMessageBundle {
                ClientToServerMessageBundle {
                    connection_id: ::core::clone::Clone::clone(&self.connection_id),
                    socket_address: ::core::clone::Clone::clone(&self.socket_address),
                    send_time: ::core::clone::Clone::clone(&self.send_time),
                    received_time: ::core::clone::Clone::clone(&self.received_time),
                    origin: ::core::clone::Clone::clone(&self.origin),
                    message: ::core::clone::Clone::clone(&self.message),
                }
            }
        }
        impl ClientToServerMessageBundle {
            pub fn new(
                connection_id: u32,
                socket_address: std::net::SocketAddr,
                received_time: SystemTime,
                message: ClientToServerMessage,
            ) -> Self {
                Self {
                    connection_id,
                    socket_address,
                    send_time: message.send_time,
                    received_time,
                    origin: message.origin,
                    message: message.data,
                }
            }
        }
        pub struct ClientToServerMessage {
            pub send_time: SystemTime,
            pub origin: MessageOrigin,
            pub data: ClientToServerMessageData,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ClientToServerMessage {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ClientToServerMessage",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "send_time",
                        &self.send_time,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "origin",
                        &self.origin,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "data",
                        &self.data,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ClientToServerMessage {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "send_time" => _serde::__private::Ok(__Field::__field0),
                                "origin" => _serde::__private::Ok(__Field::__field1),
                                "data" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"send_time" => _serde::__private::Ok(__Field::__field0),
                                b"origin" => _serde::__private::Ok(__Field::__field1),
                                b"data" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ClientToServerMessage>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ClientToServerMessage;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ClientToServerMessage",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                SystemTime,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ClientToServerMessage with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                MessageOrigin,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ClientToServerMessage with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                ClientToServerMessageData,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ClientToServerMessage with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ClientToServerMessage {
                                send_time: __field0,
                                origin: __field1,
                                data: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<SystemTime> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<MessageOrigin> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                ClientToServerMessageData,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "send_time",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<SystemTime>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("origin"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                MessageOrigin,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ClientToServerMessageData,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("send_time")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("origin")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("data")?
                                }
                            };
                            _serde::__private::Ok(ClientToServerMessage {
                                send_time: __field0,
                                origin: __field1,
                                data: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "send_time",
                        "origin",
                        "data",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ClientToServerMessage",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ClientToServerMessage,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ClientToServerMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ClientToServerMessage",
                    "send_time",
                    &self.send_time,
                    "origin",
                    &self.origin,
                    "data",
                    &&self.data,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ClientToServerMessage {
            #[inline]
            fn clone(&self) -> ClientToServerMessage {
                ClientToServerMessage {
                    send_time: ::core::clone::Clone::clone(&self.send_time),
                    origin: ::core::clone::Clone::clone(&self.origin),
                    data: ::core::clone::Clone::clone(&self.data),
                }
            }
        }
        impl ClientToServerMessage {
            pub fn new(origin: MessageOrigin, data: ClientToServerMessageData) -> Self {
                Self {
                    send_time: SystemTime::now(),
                    origin,
                    data,
                }
            }
        }
        pub enum ClientToServerMessageData {
            Message { message: String },
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ClientToServerMessageData {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ClientToServerMessageData::Message { ref message } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                                __serializer,
                                "ClientToServerMessageData",
                                0u32,
                                "Message",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStructVariant::serialize_field(
                                &mut __serde_state,
                                "message",
                                message,
                            )?;
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ClientToServerMessageData {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 1",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Message" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Message" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            ClientToServerMessageData,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ClientToServerMessageData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum ClientToServerMessageData",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field {
                                        __field0,
                                        __ignore,
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                0u64 => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "message" => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"message" => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __Field {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: _serde::__private::PhantomData<
                                            ClientToServerMessageData,
                                        >,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = ClientToServerMessageData;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "struct variant ClientToServerMessageData::Message",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                                String,
                                            >(&mut __seq)? {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde::__private::Err(
                                                        _serde::de::Error::invalid_length(
                                                            0usize,
                                                            &"struct variant ClientToServerMessageData::Message with 1 element",
                                                        ),
                                                    );
                                                }
                                            };
                                            _serde::__private::Ok(ClientToServerMessageData::Message {
                                                message: __field0,
                                            })
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private::Option::is_some(&__field0) {
                                                            return _serde::__private::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                                    "message",
                                                                ),
                                                            );
                                                        }
                                                        __field0 = _serde::__private::Some(
                                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                        );
                                                    }
                                                    _ => {
                                                        let _ = _serde::de::MapAccess::next_value::<
                                                            _serde::de::IgnoredAny,
                                                        >(&mut __map)?;
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private::Some(__field0) => __field0,
                                                _serde::__private::None => {
                                                    _serde::__private::de::missing_field("message")?
                                                }
                                            };
                                            _serde::__private::Ok(ClientToServerMessageData::Message {
                                                message: __field0,
                                            })
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &'static [&'static str] = &["message"];
                                    _serde::de::VariantAccess::struct_variant(
                                        __variant,
                                        FIELDS,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<
                                                ClientToServerMessageData,
                                            >,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Message"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ClientToServerMessageData",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ClientToServerMessageData,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ClientToServerMessageData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ClientToServerMessageData::Message { message: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Message",
                            "message",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ClientToServerMessageData {
            #[inline]
            fn clone(&self) -> ClientToServerMessageData {
                match self {
                    ClientToServerMessageData::Message { message: __self_0 } => {
                        ClientToServerMessageData::Message {
                            message: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
    }
    pub mod commands {
        pub enum PublicCommands {
            GiveHistory,
            RegisterSource { user_id: u32 },
        }
        pub enum UserCommands {
            GetHistory { client_id: u32 },
        }
        pub enum DebugCommands {}
        pub enum OverrideCommands {
            Terminate,
        }
    }
    pub mod metadata {
        use serde::{Deserialize, Serialize};
        pub enum MessageOrigin {
            Discord,
            Telegram,
            Cli,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MessageOrigin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        MessageOrigin::Discord => "Discord",
                        MessageOrigin::Telegram => "Telegram",
                        MessageOrigin::Cli => "Cli",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MessageOrigin {
            #[inline]
            fn clone(&self) -> MessageOrigin {
                match self {
                    MessageOrigin::Discord => MessageOrigin::Discord,
                    MessageOrigin::Telegram => MessageOrigin::Telegram,
                    MessageOrigin::Cli => MessageOrigin::Cli,
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for MessageOrigin {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        MessageOrigin::Discord => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "MessageOrigin",
                                0u32,
                                "Discord",
                            )
                        }
                        MessageOrigin::Telegram => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "MessageOrigin",
                                1u32,
                                "Telegram",
                            )
                        }
                        MessageOrigin::Cli => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "MessageOrigin",
                                2u32,
                                "Cli",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for MessageOrigin {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 3",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Discord" => _serde::__private::Ok(__Field::__field0),
                                "Telegram" => _serde::__private::Ok(__Field::__field1),
                                "Cli" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Discord" => _serde::__private::Ok(__Field::__field0),
                                b"Telegram" => _serde::__private::Ok(__Field::__field1),
                                b"Cli" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<MessageOrigin>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = MessageOrigin;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum MessageOrigin",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(MessageOrigin::Discord)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(MessageOrigin::Telegram)
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(MessageOrigin::Cli)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Discord",
                        "Telegram",
                        "Cli",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "MessageOrigin",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<MessageOrigin>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod server_to_client {
        use serde::{Deserialize, Serialize};
        pub enum ServerToClientMessage {
            Message { message: String },
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ServerToClientMessage {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ServerToClientMessage::Message { ref message } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                                __serializer,
                                "ServerToClientMessage",
                                0u32,
                                "Message",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStructVariant::serialize_field(
                                &mut __serde_state,
                                "message",
                                message,
                            )?;
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ServerToClientMessage {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 1",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Message" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Message" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ServerToClientMessage>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ServerToClientMessage;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum ServerToClientMessage",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field {
                                        __field0,
                                        __ignore,
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                0u64 => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "message" => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"message" => _serde::__private::Ok(__Field::__field0),
                                                _ => _serde::__private::Ok(__Field::__ignore),
                                            }
                                        }
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __Field {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: _serde::__private::PhantomData<
                                            ServerToClientMessage,
                                        >,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = ServerToClientMessage;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "struct variant ServerToClientMessage::Message",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                                String,
                                            >(&mut __seq)? {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde::__private::Err(
                                                        _serde::de::Error::invalid_length(
                                                            0usize,
                                                            &"struct variant ServerToClientMessage::Message with 1 element",
                                                        ),
                                                    );
                                                }
                                            };
                                            _serde::__private::Ok(ServerToClientMessage::Message {
                                                message: __field0,
                                            })
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                                __Field,
                                            >(&mut __map)? {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private::Option::is_some(&__field0) {
                                                            return _serde::__private::Err(
                                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                                    "message",
                                                                ),
                                                            );
                                                        }
                                                        __field0 = _serde::__private::Some(
                                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                        );
                                                    }
                                                    _ => {
                                                        let _ = _serde::de::MapAccess::next_value::<
                                                            _serde::de::IgnoredAny,
                                                        >(&mut __map)?;
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private::Some(__field0) => __field0,
                                                _serde::__private::None => {
                                                    _serde::__private::de::missing_field("message")?
                                                }
                                            };
                                            _serde::__private::Ok(ServerToClientMessage::Message {
                                                message: __field0,
                                            })
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &'static [&'static str] = &["message"];
                                    _serde::de::VariantAccess::struct_variant(
                                        __variant,
                                        FIELDS,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<
                                                ServerToClientMessage,
                                            >,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Message"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ServerToClientMessage",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ServerToClientMessage,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ServerToClientMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ServerToClientMessage::Message { message: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Message",
                            "message",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ServerToClientMessage {
            #[inline]
            fn clone(&self) -> ServerToClientMessage {
                match self {
                    ServerToClientMessage::Message { message: __self_0 } => {
                        ServerToClientMessage::Message {
                            message: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
    }
}
mod server {
    pub mod client_bookkeeping {
        use std::{collections::HashMap, net::SocketAddr, sync::{atomic::AtomicU32, Arc}};
        use crossbeam::queue::ArrayQueue;
        use lazy_static::lazy_static;
        use tokio::sync::RwLock;
        use crate::common::server_to_client::ServerToClientMessage;
        pub type ClientMessageQueue = Arc<ArrayQueue<ServerToClientMessage>>;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct NEXT_CONNECTION_ID {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static NEXT_CONNECTION_ID: NEXT_CONNECTION_ID = NEXT_CONNECTION_ID {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for NEXT_CONNECTION_ID {
            type Target = Arc<AtomicU32>;
            fn deref(&self) -> &Arc<AtomicU32> {
                #[inline(always)]
                fn __static_ref_initialize() -> Arc<AtomicU32> {
                    Arc::new(AtomicU32::new(0))
                }
                #[inline(always)]
                fn __stability() -> &'static Arc<AtomicU32> {
                    static LAZY: ::lazy_static::lazy::Lazy<Arc<AtomicU32>> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for NEXT_CONNECTION_ID {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct CONNECTION_OUTBOUND_MAILBOXES {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static CONNECTION_OUTBOUND_MAILBOXES: CONNECTION_OUTBOUND_MAILBOXES = CONNECTION_OUTBOUND_MAILBOXES {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for CONNECTION_OUTBOUND_MAILBOXES {
            type Target = RwLock<HashMap<u32, ClientMessageQueue>>;
            fn deref(&self) -> &RwLock<HashMap<u32, ClientMessageQueue>> {
                #[inline(always)]
                fn __static_ref_initialize() -> RwLock<
                    HashMap<u32, ClientMessageQueue>,
                > {
                    RwLock::new(HashMap::new())
                }
                #[inline(always)]
                fn __stability() -> &'static RwLock<HashMap<u32, ClientMessageQueue>> {
                    static LAZY: ::lazy_static::lazy::Lazy<
                        RwLock<HashMap<u32, ClientMessageQueue>>,
                    > = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for CONNECTION_OUTBOUND_MAILBOXES {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub fn get_next_connection_id() -> u32 {
            NEXT_CONNECTION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        }
        pub async fn do_bookkeeping_for_new_connection(
            socket_address: SocketAddr,
        ) -> u32 {
            let connection_id = get_next_connection_id();
            {
                let mut outbound_mailboxes_write = CONNECTION_OUTBOUND_MAILBOXES
                    .write()
                    .await;
                let mailbox = Arc::new(ArrayQueue::new(100));
                outbound_mailboxes_write.insert(connection_id, mailbox);
            }
            {
                ::std::io::_print(
                    format_args!(
                        "New Connected {0}. Assigned ID: {1}\n",
                        socket_address,
                        connection_id,
                    ),
                );
            };
            connection_id
        }
        ///  Removes client allocated bookkeeping resources.
        pub async fn do_bookkeeping_for_remove_connection(id: u32) {
            {
                let mut clients_write = CONNECTION_OUTBOUND_MAILBOXES.write().await;
                clients_write.remove(&id);
            }
            {
                ::std::io::_print(
                    format_args!("Client {0} network resources cleaned up.\n", id),
                );
            };
        }
    }
    pub mod enque_outbound_messages {
        use crate::common::server_to_client::ServerToClientMessage;
        use super::client_bookkeeping::CONNECTION_OUTBOUND_MAILBOXES;
        pub async fn send_to_one_client(client_id: u32, message: ServerToClientMessage) {
            let clients_read = CONNECTION_OUTBOUND_MAILBOXES.read().await;
            if let Some(queue) = clients_read.get(&client_id) {
                if queue.push(message).is_err() {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Failed to enqueue message for client {0}\n",
                                client_id,
                            ),
                        );
                    };
                }
            } else {
                {
                    ::std::io::_eprint(
                        format_args!("Failed to find client {0}\n", client_id),
                    );
                };
            }
        }
        pub async fn broadcast_to_all_except(
            sender_id: u32,
            message: ServerToClientMessage,
        ) {
            let clients_read = CONNECTION_OUTBOUND_MAILBOXES.read().await;
            for (&client_id, queue) in clients_read.iter() {
                if client_id == sender_id {
                    continue;
                }
                if queue.push(message.clone()).is_err() {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Failed to enqueue message for client {0}\n",
                                client_id,
                            ),
                        );
                    };
                }
            }
        }
        pub async fn broadcast_to_all(message: ServerToClientMessage) {
            let clients_read = CONNECTION_OUTBOUND_MAILBOXES.read().await;
            for (_, queue) in clients_read.iter() {
                if queue.push(message.clone()).is_err() {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to enqueue message for client\n"),
                        );
                    };
                }
            }
        }
    }
    pub mod logging {
        use std::sync::Arc;
        use crossbeam::queue::ArrayQueue;
        use lazy_static::lazy_static;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct LOG_EVENT_QUEUE {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static LOG_EVENT_QUEUE: LOG_EVENT_QUEUE = LOG_EVENT_QUEUE {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for LOG_EVENT_QUEUE {
            type Target = Arc<ArrayQueue<LogEvent>>;
            fn deref(&self) -> &Arc<ArrayQueue<LogEvent>> {
                #[inline(always)]
                fn __static_ref_initialize() -> Arc<ArrayQueue<LogEvent>> {
                    Arc::new(ArrayQueue::new(1000))
                }
                #[inline(always)]
                fn __stability() -> &'static Arc<ArrayQueue<LogEvent>> {
                    static LAZY: ::lazy_static::lazy::Lazy<Arc<ArrayQueue<LogEvent>>> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for LOG_EVENT_QUEUE {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub enum LogEvent {
            Disconnect,
            Connect,
            ClientMessage,
            ServerMessage,
        }
    }
    pub mod message_processing {
        use crate::{
            common::{
                client_to_server::ClientToServerMessageData,
                server_to_client::ServerToClientMessage,
            },
            server::enque_outbound_messages::send_to_one_client,
        };
        use super::tcp_networking::INCOMING_MESSAGE_QUEUE;
        pub async fn process_message_queue() {
            loop {
                while let Some(message_bundle) = INCOMING_MESSAGE_QUEUE.pop() {
                    let connection_id = message_bundle.connection_id;
                    match message_bundle.message {
                        ClientToServerMessageData::Message { message } => {
                            {
                                ::std::io::_print(
                                    format_args!("{0} says: {1}\n", connection_id, message),
                                );
                            };
                            let msg = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "rcvd ur message at {0:?}",
                                        message_bundle.received_time,
                                    ),
                                );
                                res
                            };
                            let outbound_message = ServerToClientMessage::Message {
                                message: msg,
                            };
                            send_to_one_client(connection_id, outbound_message).await;
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
    pub mod settings {
        pub const SERVER_ADDR: &str = "127.0.0.1:8080";
    }
    pub mod tasks {
        use super::message_processing::process_message_queue;
        pub async fn launch_tasks() {
            let message_queue_task = tokio::spawn(process_message_queue());
            let _ = {
                use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                use ::tokio::macros::support::Poll::{Ready, Pending};
                let mut futures = (maybe_done(message_queue_task),);
                let mut futures = &mut futures;
                let mut skip_next_time: u32 = 0;
                poll_fn(move |cx| {
                        const COUNT: u32 = 0 + 1;
                        let mut is_pending = false;
                        let mut to_run = COUNT;
                        let mut skip = skip_next_time;
                        skip_next_time = if skip + 1 == COUNT { 0 } else { skip + 1 };
                        loop {
                            if skip == 0 {
                                if to_run == 0 {
                                    break;
                                }
                                to_run -= 1;
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                if fut.poll(cx).is_pending() {
                                    is_pending = true;
                                }
                            } else {
                                skip -= 1;
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready((
                                {
                                    let (fut, ..) = &mut futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    fut.take_output().expect("expected completed future")
                                },
                            ))
                        }
                    })
                    .await
            };
        }
    }
    pub mod tcp_networking {
        use std::{
            net::SocketAddr,
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
        };
        use crossbeam::queue::ArrayQueue;
        use lazy_static::lazy_static;
        use tokio::{
            io::{self, AsyncReadExt, AsyncWriteExt},
            net::{
                tcp::{OwnedReadHalf, OwnedWriteHalf},
                TcpListener, TcpStream,
            },
        };
        use super::{
            client_bookkeeping::{
                do_bookkeeping_for_remove_connection, CONNECTION_OUTBOUND_MAILBOXES,
            },
            settings::SERVER_ADDR,
        };
        use crate::{
            common::client_to_server::{
                ClientToServerMessage, ClientToServerMessageBundle,
            },
            server::client_bookkeeping::do_bookkeeping_for_new_connection,
        };
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct INCOMING_MESSAGE_QUEUE {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static INCOMING_MESSAGE_QUEUE: INCOMING_MESSAGE_QUEUE = INCOMING_MESSAGE_QUEUE {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for INCOMING_MESSAGE_QUEUE {
            type Target = Arc<ArrayQueue<ClientToServerMessageBundle>>;
            fn deref(&self) -> &Arc<ArrayQueue<ClientToServerMessageBundle>> {
                #[inline(always)]
                fn __static_ref_initialize() -> Arc<
                    ArrayQueue<ClientToServerMessageBundle>,
                > {
                    Arc::new(ArrayQueue::new(32))
                }
                #[inline(always)]
                fn __stability() -> &'static Arc<
                    ArrayQueue<ClientToServerMessageBundle>,
                > {
                    static LAZY: ::lazy_static::lazy::Lazy<
                        Arc<ArrayQueue<ClientToServerMessageBundle>>,
                    > = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for INCOMING_MESSAGE_QUEUE {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub async fn init() -> tokio::io::Result<()> {
            {
                ::std::io::_print(format_args!("Initializing socket...\n"));
            };
            let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
            {
                ::std::io::_print(format_args!("Socket Initialized!\n"));
            };
            {
                ::std::io::_print(format_args!("Awaiting connections...\n"));
            };
            loop {
                let (socket, socket_address) = listener.accept().await?;
                {
                    ::std::io::_print(format_args!("Connection received!\n"));
                };
                tokio::spawn(handle_new_connection(socket, socket_address));
            }
        }
        pub async fn handle_new_connection(
            socket: TcpStream,
            socket_address: SocketAddr,
        ) -> tokio::io::Result<()> {
            let connection_id = do_bookkeeping_for_new_connection(socket_address).await;
            {
                ::std::io::_print(format_args!("Spawning rx/tx tasks...\n"));
            };
            let (read_half, write_half) = socket.into_split();
            let disconnected = Arc::new(AtomicBool::new(false));
            let rx_task_handle = tokio::spawn(
                continuously_read_inbound_messages(
                    read_half,
                    socket_address,
                    connection_id,
                    disconnected.clone(),
                ),
            );
            let tx_task_handle = tokio::spawn(
                continuously_send_outbound_messages(
                    write_half,
                    socket_address,
                    connection_id,
                    disconnected.clone(),
                ),
            );
            let _ = {
                use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                use ::tokio::macros::support::Poll::{Ready, Pending};
                let mut futures = (
                    maybe_done(rx_task_handle),
                    maybe_done(tx_task_handle),
                );
                let mut futures = &mut futures;
                let mut skip_next_time: u32 = 0;
                poll_fn(move |cx| {
                        const COUNT: u32 = 0 + 1 + 1;
                        let mut is_pending = false;
                        let mut to_run = COUNT;
                        let mut skip = skip_next_time;
                        skip_next_time = if skip + 1 == COUNT { 0 } else { skip + 1 };
                        loop {
                            if skip == 0 {
                                if to_run == 0 {
                                    break;
                                }
                                to_run -= 1;
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                if fut.as_mut().poll(cx).is_pending() {
                                    is_pending = true;
                                } else if fut
                                    .as_mut()
                                    .output_mut()
                                    .expect("expected completed future")
                                    .is_err()
                                {
                                    return Ready(
                                        Err(
                                            fut
                                                .take_output()
                                                .expect("expected completed future")
                                                .err()
                                                .unwrap(),
                                        ),
                                    )
                                }
                            } else {
                                skip -= 1;
                            }
                            if skip == 0 {
                                if to_run == 0 {
                                    break;
                                }
                                to_run -= 1;
                                let (_, fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                if fut.as_mut().poll(cx).is_pending() {
                                    is_pending = true;
                                } else if fut
                                    .as_mut()
                                    .output_mut()
                                    .expect("expected completed future")
                                    .is_err()
                                {
                                    return Ready(
                                        Err(
                                            fut
                                                .take_output()
                                                .expect("expected completed future")
                                                .err()
                                                .unwrap(),
                                        ),
                                    )
                                }
                            } else {
                                skip -= 1;
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(
                                Ok((
                                    {
                                        let (fut, ..) = &mut futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        fut.take_output()
                                            .expect("expected completed future")
                                            .ok()
                                            .expect("expected Ok(_)")
                                    },
                                    {
                                        let (_, fut, ..) = &mut futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        fut.take_output()
                                            .expect("expected completed future")
                                            .ok()
                                            .expect("expected Ok(_)")
                                    },
                                )),
                            )
                        }
                    })
                    .await
            }?;
            do_bookkeeping_for_remove_connection(connection_id).await;
            Ok(())
        }
        pub async fn continuously_read_inbound_messages(
            mut socket: OwnedReadHalf,
            socket_address: SocketAddr,
            connection_id: u32,
            disconnected: Arc<AtomicBool>,
        ) -> io::Result<()> {
            {
                ::std::io::_print(format_args!("Listening for incoming messages...\n"));
            };
            let mut buffer = [0; 1024];
            loop {
                let nbytes = socket.read(&mut buffer).await?;
                if nbytes == 0 {
                    disconnected.store(true, Ordering::SeqCst);
                    return Ok(());
                }
                let result: Result<ClientToServerMessage, _> = bincode::deserialize(
                    &buffer[..nbytes],
                );
                match result {
                    Ok(message) => {
                        {
                            ::std::io::_print(format_args!("rcvd a message\n"));
                        };
                        let received_time = std::time::SystemTime::now();
                        let message_bundle = ClientToServerMessageBundle::new(
                            connection_id,
                            socket_address,
                            received_time,
                            message,
                        );
                        if INCOMING_MESSAGE_QUEUE.push(message_bundle).is_err() {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "Inbound message queue full: dropping message from {0}\n",
                                        connection_id,
                                    ),
                                );
                            };
                        }
                    }
                    Err(e) => {
                        {
                            ::std::io::_eprint(
                                format_args!("Error parsing client data: {0:?}\n", e),
                            );
                        };
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
        pub async fn continuously_send_outbound_messages(
            mut socket: OwnedWriteHalf,
            _socket_address: SocketAddr,
            id: u32,
            disconnected: Arc<AtomicBool>,
        ) -> io::Result<()> {
            loop {
                {
                    if disconnected.load(Ordering::SeqCst) {
                        return Ok(());
                    }
                }
                {
                    let clients_read = CONNECTION_OUTBOUND_MAILBOXES.read().await;
                    if let Some(outgoing_messages) = clients_read.get(&id) {
                        if let Some(message) = outgoing_messages.pop() {
                            match bincode::serialize(&message) {
                                Ok(binary_message) => {
                                    socket.write_all(&binary_message).await?;
                                }
                                Err(e) => {
                                    {
                                        ::std::io::_eprint(
                                            format_args!("Error serializing message: {0:?}\n", e),
                                        );
                                    };
                                }
                            }
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}
fn main() {
    let body = async {
        dotenv().ok();
        let prompt = "read me the contents of `dummyfile.txt` and write them to a new `dummyfile1.txt`";
        if let Ok(response) = go_get_remote_response(prompt).await {
            {
                ::std::io::_print(format_args!("response: {0}\n", response));
            };
        }
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
pub async fn go_get_remote_response(prompt: &str) -> Result<String, Box<dyn Error>> {
    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env file");
    let client = ChatGPT::new(openai_api_key)?;
    let mut conversation = client.new_conversation();
    conversation.add_function(read_file());
    conversation.add_function(write_file());
    if let Ok(response) = conversation.send_message_functions(prompt).await {
        let message_choices = response.message_choices;
        loop {
            if let Some(message_choice) = message_choices.first() {
                let message = message_choice.message.clone();
                {
                    ::std::io::_print(
                        format_args!("{0:?}: {1}\n", message.role, message.content),
                    );
                };
                if message_choice.finish_reason == "stop" {
                    return Ok(message.content);
                }
            }
        }
    }
    Ok("test".to_string())
}
#[doc(hidden)]
#[allow(missing_docs)]
mod __read_file_data {
    use super::*;
    use chatgpt::functions::schemars;
    #[allow(non_camel_case_types, missing_docs)]
    #[doc(hidden)]
    pub struct __read_file_FunctionArguments {
        ///The name of the file to read
        filename: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for __read_file_FunctionArguments {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "filename" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"filename" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<
                        __read_file_FunctionArguments,
                    >,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = __read_file_FunctionArguments;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct __read_file_FunctionArguments",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct __read_file_FunctionArguments with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(__read_file_FunctionArguments {
                            filename: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "filename",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("filename")?
                            }
                        };
                        _serde::__private::Ok(__read_file_FunctionArguments {
                            filename: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["filename"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "__read_file_FunctionArguments",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            __read_file_FunctionArguments,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for __read_file_FunctionArguments {
            fn schema_name() -> std::string::String {
                "__read_file_FunctionArguments".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(
                    "test::__read_file_data::__read_file_FunctionArguments",
                )
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        object_validation
                            .properties
                            .insert(
                                "filename".to_owned(),
                                schemars::_private::apply_metadata(
                                    gen.subschema_for::<String>(),
                                    schemars::schema::Metadata {
                                        description: Some(
                                            "The name of the file to read".to_owned(),
                                        ),
                                        ..Default::default()
                                    },
                                ),
                            );
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("filename".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::fmt::Debug for __read_file_FunctionArguments {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "__read_file_FunctionArguments",
                "filename",
                &&self.filename,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::clone::Clone for __read_file_FunctionArguments {
        #[inline]
        fn clone(&self) -> __read_file_FunctionArguments {
            __read_file_FunctionArguments {
                filename: ::core::clone::Clone::clone(&self.filename),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types, missing_docs)]
    pub struct __read_file_Function;
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::fmt::Debug for __read_file_Function {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "__read_file_Function")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::marker::Copy for __read_file_Function {}
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::clone::Clone for __read_file_Function {
        #[inline]
        fn clone(&self) -> __read_file_Function {
            *self
        }
    }
    impl chatgpt::functions::CallableAsyncFunction<__read_file_FunctionArguments>
    for __read_file_Function {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn invoke<'async_trait>(
            arguments: __read_file_FunctionArguments,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = chatgpt::Result<chatgpt::functions::serde_json::Value>,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    chatgpt::Result<chatgpt::functions::serde_json::Value>,
                > {
                    return __ret;
                }
                let arguments = arguments;
                let __ret: chatgpt::Result<chatgpt::functions::serde_json::Value> = {
                    let filename = arguments.filename;
                    let result = {
                        let mut file = File::open(filename).await?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).await?;
                        Ok(contents)
                    };
                    chatgpt::functions::serde_json::to_value(&result)
                        .map_err(chatgpt::err::Error::from)
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
use __read_file_data::*;
fn read_file() -> chatgpt::functions::GptFunction<
    __read_file_FunctionArguments,
    __read_file_Function,
> {
    use chatgpt::functions::*;
    GptFunction {
        descriptor: FunctionDescriptor {
            name: "read_file",
            description: "Read the contents of a file and return them as a string",
            parameters: core::marker::PhantomData::<
                __read_file_FunctionArguments,
            >::default(),
        },
        callable: core::marker::PhantomData::<__read_file_Function>::default(),
    }
}
#[doc(hidden)]
#[allow(missing_docs)]
mod __write_file_data {
    use super::*;
    use chatgpt::functions::schemars;
    #[allow(non_camel_case_types, missing_docs)]
    #[doc(hidden)]
    pub struct __write_file_FunctionArguments {
        ///The name of the file to be written
        filename: String,
        ///String to be written to file
        contents: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for __write_file_FunctionArguments {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "filename" => _serde::__private::Ok(__Field::__field0),
                            "contents" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"filename" => _serde::__private::Ok(__Field::__field0),
                            b"contents" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<
                        __write_file_FunctionArguments,
                    >,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = __write_file_FunctionArguments;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct __write_file_FunctionArguments",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct __write_file_FunctionArguments with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct __write_file_FunctionArguments with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(__write_file_FunctionArguments {
                            filename: __field0,
                            contents: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "filename",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "contents",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("filename")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("contents")?
                            }
                        };
                        _serde::__private::Ok(__write_file_FunctionArguments {
                            filename: __field0,
                            contents: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["filename", "contents"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "__write_file_FunctionArguments",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            __write_file_FunctionArguments,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        #[automatically_derived]
        #[allow(unused_braces)]
        impl schemars::JsonSchema for __write_file_FunctionArguments {
            fn schema_name() -> std::string::String {
                "__write_file_FunctionArguments".to_owned()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(
                    "test::__write_file_data::__write_file_FunctionArguments",
                )
            }
            fn json_schema(
                gen: &mut schemars::gen::SchemaGenerator,
            ) -> schemars::schema::Schema {
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        object_validation
                            .properties
                            .insert(
                                "filename".to_owned(),
                                schemars::_private::apply_metadata(
                                    gen.subschema_for::<String>(),
                                    schemars::schema::Metadata {
                                        description: Some(
                                            "The name of the file to be written".to_owned(),
                                        ),
                                        ..Default::default()
                                    },
                                ),
                            );
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("filename".to_owned());
                        }
                    }
                    {
                        object_validation
                            .properties
                            .insert(
                                "contents".to_owned(),
                                schemars::_private::apply_metadata(
                                    gen.subschema_for::<String>(),
                                    schemars::schema::Metadata {
                                        description: Some(
                                            "String to be written to file".to_owned(),
                                        ),
                                        ..Default::default()
                                    },
                                ),
                            );
                        if !<String as schemars::JsonSchema>::_schemars_private_is_option() {
                            object_validation.required.insert("contents".to_owned());
                        }
                    }
                    schemars::schema::Schema::Object(schema_object)
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::fmt::Debug for __write_file_FunctionArguments {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "__write_file_FunctionArguments",
                "filename",
                &self.filename,
                "contents",
                &&self.contents,
            )
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::clone::Clone for __write_file_FunctionArguments {
        #[inline]
        fn clone(&self) -> __write_file_FunctionArguments {
            __write_file_FunctionArguments {
                filename: ::core::clone::Clone::clone(&self.filename),
                contents: ::core::clone::Clone::clone(&self.contents),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_camel_case_types, missing_docs)]
    pub struct __write_file_Function;
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::fmt::Debug for __write_file_Function {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "__write_file_Function")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::marker::Copy for __write_file_Function {}
    #[automatically_derived]
    #[allow(non_camel_case_types, missing_docs)]
    impl ::core::clone::Clone for __write_file_Function {
        #[inline]
        fn clone(&self) -> __write_file_Function {
            *self
        }
    }
    impl chatgpt::functions::CallableAsyncFunction<__write_file_FunctionArguments>
    for __write_file_Function {
        #[allow(
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn invoke<'async_trait>(
            arguments: __write_file_FunctionArguments,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = chatgpt::Result<chatgpt::functions::serde_json::Value>,
                > + ::core::marker::Send + 'async_trait,
            >,
        > {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    chatgpt::Result<chatgpt::functions::serde_json::Value>,
                > {
                    return __ret;
                }
                let arguments = arguments;
                let __ret: chatgpt::Result<chatgpt::functions::serde_json::Value> = {
                    let filename = arguments.filename;
                    let contents = arguments.contents;
                    let result = {
                        let mut file = File::create(filename).await?;
                        file.write_all(contents.as_bytes()).await?;
                    };
                    chatgpt::functions::serde_json::to_value(&result)
                        .map_err(chatgpt::err::Error::from)
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
use __write_file_data::*;
fn write_file() -> chatgpt::functions::GptFunction<
    __write_file_FunctionArguments,
    __write_file_Function,
> {
    use chatgpt::functions::*;
    GptFunction {
        descriptor: FunctionDescriptor {
            name: "write_file",
            description: "Write the contents of a string to a file",
            parameters: core::marker::PhantomData::<
                __write_file_FunctionArguments,
            >::default(),
        },
        callable: core::marker::PhantomData::<__write_file_Function>::default(),
    }
}
