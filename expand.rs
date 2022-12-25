mod models {
    use chrono::NaiveDate;
    use serde::Deserialize;
    pub struct File {
        pub fid: u32,
        pub aid: u32,
        pub eid: u32,
        pub gid: u32,
        pub state: i16,
        pub size: i64,
        pub ed2k: String,
        pub colour_depth: String,
        pub quality: String,
        pub source: String,
        pub audio_codec_list: Vec<String>,
        pub audio_bitrate_list: Vec<i32>,
        pub video_codec: Vec<String>,
        pub video_bitrate: Vec<String>,
        pub video_resolution: Vec<String>,
        pub dub_language: String,
        pub sub_language: String,
        pub length_in_seconds: i32,
        pub description: String,
        pub aired_date: NaiveDate,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for File {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __ignore,
                }
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
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
                            "fid" => _serde::__private::Ok(__Field::__field0),
                            "aid" => _serde::__private::Ok(__Field::__field1),
                            "eid" => _serde::__private::Ok(__Field::__field2),
                            "gid" => _serde::__private::Ok(__Field::__field3),
                            "state" => _serde::__private::Ok(__Field::__field4),
                            "size" => _serde::__private::Ok(__Field::__field5),
                            "ed2k" => _serde::__private::Ok(__Field::__field6),
                            "colour_depth" => _serde::__private::Ok(__Field::__field7),
                            "quality" => _serde::__private::Ok(__Field::__field8),
                            "source" => _serde::__private::Ok(__Field::__field9),
                            "audio_codec_list" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            "audio_bitrate_list" => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            "video_codec" => _serde::__private::Ok(__Field::__field12),
                            "video_bitrate" => _serde::__private::Ok(__Field::__field13),
                            "video_resolution" => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            "dub_language" => _serde::__private::Ok(__Field::__field15),
                            "sub_language" => _serde::__private::Ok(__Field::__field16),
                            "length_in_seconds" => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            "description" => _serde::__private::Ok(__Field::__field18),
                            "aired_date" => _serde::__private::Ok(__Field::__field19),
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
                            b"fid" => _serde::__private::Ok(__Field::__field0),
                            b"aid" => _serde::__private::Ok(__Field::__field1),
                            b"eid" => _serde::__private::Ok(__Field::__field2),
                            b"gid" => _serde::__private::Ok(__Field::__field3),
                            b"state" => _serde::__private::Ok(__Field::__field4),
                            b"size" => _serde::__private::Ok(__Field::__field5),
                            b"ed2k" => _serde::__private::Ok(__Field::__field6),
                            b"colour_depth" => _serde::__private::Ok(__Field::__field7),
                            b"quality" => _serde::__private::Ok(__Field::__field8),
                            b"source" => _serde::__private::Ok(__Field::__field9),
                            b"audio_codec_list" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            b"audio_bitrate_list" => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            b"video_codec" => _serde::__private::Ok(__Field::__field12),
                            b"video_bitrate" => _serde::__private::Ok(__Field::__field13),
                            b"video_resolution" => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            b"dub_language" => _serde::__private::Ok(__Field::__field15),
                            b"sub_language" => _serde::__private::Ok(__Field::__field16),
                            b"length_in_seconds" => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            b"description" => _serde::__private::Ok(__Field::__field18),
                            b"aired_date" => _serde::__private::Ok(__Field::__field19),
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
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<File>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = File;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct File",
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
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            i16,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            i64,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field9 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        9usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field10 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field11 = match match _serde::de::SeqAccess::next_element::<
                            Vec<i32>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field12 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        12usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field13 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        13usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field14 = match match _serde::de::SeqAccess::next_element::<
                            Vec<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        14usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field15 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        15usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field16 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        16usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field17 = match match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        17usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field18 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        18usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        let __field19 = match match _serde::de::SeqAccess::next_element::<
                            NaiveDate,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        19usize,
                                        &"struct File with 20 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(File {
                            fid: __field0,
                            aid: __field1,
                            eid: __field2,
                            gid: __field3,
                            state: __field4,
                            size: __field5,
                            ed2k: __field6,
                            colour_depth: __field7,
                            quality: __field8,
                            source: __field9,
                            audio_codec_list: __field10,
                            audio_bitrate_list: __field11,
                            video_codec: __field12,
                            video_bitrate: __field13,
                            video_resolution: __field14,
                            dub_language: __field15,
                            sub_language: __field16,
                            length_in_seconds: __field17,
                            description: __field18,
                            aired_date: __field19,
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
                        let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<i16> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<i64> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field10: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field11: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                        let mut __field12: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field13: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field14: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                        let mut __field15: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field16: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field17: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field18: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field19: _serde::__private::Option<NaiveDate> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("fid"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("aid"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("eid"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("gid"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("state"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i16>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("size"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ed2k"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "colour_depth",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "quality",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("source"),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "audio_codec_list",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private::Option::is_some(&__field11) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "audio_bitrate_list",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<i32>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field12 => {
                                    if _serde::__private::Option::is_some(&__field12) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "video_codec",
                                            ),
                                        );
                                    }
                                    __field12 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field13 => {
                                    if _serde::__private::Option::is_some(&__field13) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "video_bitrate",
                                            ),
                                        );
                                    }
                                    __field13 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field14 => {
                                    if _serde::__private::Option::is_some(&__field14) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "video_resolution",
                                            ),
                                        );
                                    }
                                    __field14 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field15 => {
                                    if _serde::__private::Option::is_some(&__field15) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "dub_language",
                                            ),
                                        );
                                    }
                                    __field15 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field16 => {
                                    if _serde::__private::Option::is_some(&__field16) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sub_language",
                                            ),
                                        );
                                    }
                                    __field16 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field17 => {
                                    if _serde::__private::Option::is_some(&__field17) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "length_in_seconds",
                                            ),
                                        );
                                    }
                                    __field17 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field18 => {
                                    if _serde::__private::Option::is_some(&__field18) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field18 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field19 => {
                                    if _serde::__private::Option::is_some(&__field19) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aired_date",
                                            ),
                                        );
                                    }
                                    __field19 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            NaiveDate,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("fid") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("aid") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("eid") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("gid") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("state") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("size") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("ed2k") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("colour_depth") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("quality") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("source") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "audio_codec_list",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::__private::Some(__field11) => __field11,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "audio_bitrate_list",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field12 = match __field12 {
                            _serde::__private::Some(__field12) => __field12,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("video_codec") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field13 = match __field13 {
                            _serde::__private::Some(__field13) => __field13,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "video_bitrate",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field14 = match __field14 {
                            _serde::__private::Some(__field14) => __field14,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "video_resolution",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field15 = match __field15 {
                            _serde::__private::Some(__field15) => __field15,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("dub_language") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field16 = match __field16 {
                            _serde::__private::Some(__field16) => __field16,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("sub_language") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field17 = match __field17 {
                            _serde::__private::Some(__field17) => __field17,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "length_in_seconds",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field18 = match __field18 {
                            _serde::__private::Some(__field18) => __field18,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("description") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field19 = match __field19 {
                            _serde::__private::Some(__field19) => __field19,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("aired_date") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(File {
                            fid: __field0,
                            aid: __field1,
                            eid: __field2,
                            gid: __field3,
                            state: __field4,
                            size: __field5,
                            ed2k: __field6,
                            colour_depth: __field7,
                            quality: __field8,
                            source: __field9,
                            audio_codec_list: __field10,
                            audio_bitrate_list: __field11,
                            video_codec: __field12,
                            video_bitrate: __field13,
                            video_resolution: __field14,
                            dub_language: __field15,
                            sub_language: __field16,
                            length_in_seconds: __field17,
                            description: __field18,
                            aired_date: __field19,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "fid",
                    "aid",
                    "eid",
                    "gid",
                    "state",
                    "size",
                    "ed2k",
                    "colour_depth",
                    "quality",
                    "source",
                    "audio_codec_list",
                    "audio_bitrate_list",
                    "video_codec",
                    "video_bitrate",
                    "video_resolution",
                    "dub_language",
                    "sub_language",
                    "length_in_seconds",
                    "description",
                    "aired_date",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "File",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<File>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
}
