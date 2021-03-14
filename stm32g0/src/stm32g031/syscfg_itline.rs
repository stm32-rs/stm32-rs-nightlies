#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - interrupt line 0 status register"]
    pub itline0: ITLINE0,
    #[doc = "0x84 - interrupt line 1 status register"]
    pub itline1: ITLINE1,
    #[doc = "0x88 - interrupt line 2 status register"]
    pub itline2: ITLINE2,
    #[doc = "0x8c - interrupt line 3 status register"]
    pub itline3: ITLINE3,
    #[doc = "0x90 - interrupt line 4 status register"]
    pub itline4: ITLINE4,
    #[doc = "0x94 - interrupt line 5 status register"]
    pub itline5: ITLINE5,
    #[doc = "0x98 - interrupt line 6 status register"]
    pub itline6: ITLINE6,
    #[doc = "0x9c - interrupt line 7 status register"]
    pub itline7: ITLINE7,
    _reserved8: [u8; 4usize],
    #[doc = "0xa4 - interrupt line 9 status register"]
    pub itline9: ITLINE9,
    #[doc = "0xa8 - interrupt line 10 status register"]
    pub itline10: ITLINE10,
    #[doc = "0xac - interrupt line 11 status register"]
    pub itline11: ITLINE11,
    #[doc = "0xb0 - interrupt line 12 status register"]
    pub itline12: ITLINE12,
    #[doc = "0xb4 - interrupt line 13 status register"]
    pub itline13: ITLINE13,
    #[doc = "0xb8 - interrupt line 14 status register"]
    pub itline14: ITLINE14,
    #[doc = "0xbc - interrupt line 15 status register"]
    pub itline15: ITLINE15,
    #[doc = "0xc0 - interrupt line 16 status register"]
    pub itline16: ITLINE16,
    #[doc = "0xc4 - interrupt line 17 status register"]
    pub itline17: ITLINE17,
    #[doc = "0xc8 - interrupt line 18 status register"]
    pub itline18: ITLINE18,
    #[doc = "0xcc - interrupt line 19 status register"]
    pub itline19: ITLINE19,
    _reserved19: [u8; 4usize],
    #[doc = "0xd4 - interrupt line 21 status register"]
    pub itline21: ITLINE21,
    #[doc = "0xd8 - interrupt line 22 status register"]
    pub itline22: ITLINE22,
    #[doc = "0xdc - interrupt line 23 status register"]
    pub itline23: ITLINE23,
    #[doc = "0xe0 - interrupt line 24 status register"]
    pub itline24: ITLINE24,
    #[doc = "0xe4 - interrupt line 25 status register"]
    pub itline25: ITLINE25,
    #[doc = "0xe8 - interrupt line 26 status register"]
    pub itline26: ITLINE26,
    #[doc = "0xec - interrupt line 27 status register"]
    pub itline27: ITLINE27,
    #[doc = "0xf0 - interrupt line 28 status register"]
    pub itline28: ITLINE28,
    #[doc = "0xf4 - interrupt line 29 status register"]
    pub itline29: ITLINE29,
}
#[doc = "interrupt line 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline0](itline0) module"]
pub type ITLINE0 = crate::Reg<u32, _ITLINE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE0;
#[doc = "`read()` method returns [itline0::R](itline0::R) reader structure"]
impl crate::Readable for ITLINE0 {}
#[doc = "interrupt line 0 status register"]
pub mod itline0;
#[doc = "interrupt line 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline1](itline1) module"]
pub type ITLINE1 = crate::Reg<u32, _ITLINE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE1;
#[doc = "`read()` method returns [itline1::R](itline1::R) reader structure"]
impl crate::Readable for ITLINE1 {}
#[doc = "interrupt line 1 status register"]
pub mod itline1;
#[doc = "interrupt line 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline2](itline2) module"]
pub type ITLINE2 = crate::Reg<u32, _ITLINE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE2;
#[doc = "`read()` method returns [itline2::R](itline2::R) reader structure"]
impl crate::Readable for ITLINE2 {}
#[doc = "interrupt line 2 status register"]
pub mod itline2;
#[doc = "interrupt line 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline3](itline3) module"]
pub type ITLINE3 = crate::Reg<u32, _ITLINE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE3;
#[doc = "`read()` method returns [itline3::R](itline3::R) reader structure"]
impl crate::Readable for ITLINE3 {}
#[doc = "interrupt line 3 status register"]
pub mod itline3;
#[doc = "interrupt line 4 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline4](itline4) module"]
pub type ITLINE4 = crate::Reg<u32, _ITLINE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE4;
#[doc = "`read()` method returns [itline4::R](itline4::R) reader structure"]
impl crate::Readable for ITLINE4 {}
#[doc = "interrupt line 4 status register"]
pub mod itline4;
#[doc = "interrupt line 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline5](itline5) module"]
pub type ITLINE5 = crate::Reg<u32, _ITLINE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE5;
#[doc = "`read()` method returns [itline5::R](itline5::R) reader structure"]
impl crate::Readable for ITLINE5 {}
#[doc = "interrupt line 5 status register"]
pub mod itline5;
#[doc = "interrupt line 6 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline6](itline6) module"]
pub type ITLINE6 = crate::Reg<u32, _ITLINE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE6;
#[doc = "`read()` method returns [itline6::R](itline6::R) reader structure"]
impl crate::Readable for ITLINE6 {}
#[doc = "interrupt line 6 status register"]
pub mod itline6;
#[doc = "interrupt line 7 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline7](itline7) module"]
pub type ITLINE7 = crate::Reg<u32, _ITLINE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE7;
#[doc = "`read()` method returns [itline7::R](itline7::R) reader structure"]
impl crate::Readable for ITLINE7 {}
#[doc = "interrupt line 7 status register"]
pub mod itline7;
#[doc = "interrupt line 9 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline9](itline9) module"]
pub type ITLINE9 = crate::Reg<u32, _ITLINE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE9;
#[doc = "`read()` method returns [itline9::R](itline9::R) reader structure"]
impl crate::Readable for ITLINE9 {}
#[doc = "interrupt line 9 status register"]
pub mod itline9;
#[doc = "interrupt line 10 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline10](itline10) module"]
pub type ITLINE10 = crate::Reg<u32, _ITLINE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE10;
#[doc = "`read()` method returns [itline10::R](itline10::R) reader structure"]
impl crate::Readable for ITLINE10 {}
#[doc = "interrupt line 10 status register"]
pub mod itline10;
#[doc = "interrupt line 11 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline11](itline11) module"]
pub type ITLINE11 = crate::Reg<u32, _ITLINE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE11;
#[doc = "`read()` method returns [itline11::R](itline11::R) reader structure"]
impl crate::Readable for ITLINE11 {}
#[doc = "interrupt line 11 status register"]
pub mod itline11;
#[doc = "interrupt line 12 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline12](itline12) module"]
pub type ITLINE12 = crate::Reg<u32, _ITLINE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE12;
#[doc = "`read()` method returns [itline12::R](itline12::R) reader structure"]
impl crate::Readable for ITLINE12 {}
#[doc = "interrupt line 12 status register"]
pub mod itline12;
#[doc = "interrupt line 13 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline13](itline13) module"]
pub type ITLINE13 = crate::Reg<u32, _ITLINE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE13;
#[doc = "`read()` method returns [itline13::R](itline13::R) reader structure"]
impl crate::Readable for ITLINE13 {}
#[doc = "interrupt line 13 status register"]
pub mod itline13;
#[doc = "interrupt line 14 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline14](itline14) module"]
pub type ITLINE14 = crate::Reg<u32, _ITLINE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE14;
#[doc = "`read()` method returns [itline14::R](itline14::R) reader structure"]
impl crate::Readable for ITLINE14 {}
#[doc = "interrupt line 14 status register"]
pub mod itline14;
#[doc = "interrupt line 15 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline15](itline15) module"]
pub type ITLINE15 = crate::Reg<u32, _ITLINE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE15;
#[doc = "`read()` method returns [itline15::R](itline15::R) reader structure"]
impl crate::Readable for ITLINE15 {}
#[doc = "interrupt line 15 status register"]
pub mod itline15;
#[doc = "interrupt line 16 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline16](itline16) module"]
pub type ITLINE16 = crate::Reg<u32, _ITLINE16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE16;
#[doc = "`read()` method returns [itline16::R](itline16::R) reader structure"]
impl crate::Readable for ITLINE16 {}
#[doc = "interrupt line 16 status register"]
pub mod itline16;
#[doc = "interrupt line 17 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline17](itline17) module"]
pub type ITLINE17 = crate::Reg<u32, _ITLINE17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE17;
#[doc = "`read()` method returns [itline17::R](itline17::R) reader structure"]
impl crate::Readable for ITLINE17 {}
#[doc = "interrupt line 17 status register"]
pub mod itline17;
#[doc = "interrupt line 18 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline18](itline18) module"]
pub type ITLINE18 = crate::Reg<u32, _ITLINE18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE18;
#[doc = "`read()` method returns [itline18::R](itline18::R) reader structure"]
impl crate::Readable for ITLINE18 {}
#[doc = "interrupt line 18 status register"]
pub mod itline18;
#[doc = "interrupt line 19 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline19](itline19) module"]
pub type ITLINE19 = crate::Reg<u32, _ITLINE19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE19;
#[doc = "`read()` method returns [itline19::R](itline19::R) reader structure"]
impl crate::Readable for ITLINE19 {}
#[doc = "interrupt line 19 status register"]
pub mod itline19;
#[doc = "interrupt line 21 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline21](itline21) module"]
pub type ITLINE21 = crate::Reg<u32, _ITLINE21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE21;
#[doc = "`read()` method returns [itline21::R](itline21::R) reader structure"]
impl crate::Readable for ITLINE21 {}
#[doc = "interrupt line 21 status register"]
pub mod itline21;
#[doc = "interrupt line 22 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline22](itline22) module"]
pub type ITLINE22 = crate::Reg<u32, _ITLINE22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE22;
#[doc = "`read()` method returns [itline22::R](itline22::R) reader structure"]
impl crate::Readable for ITLINE22 {}
#[doc = "interrupt line 22 status register"]
pub mod itline22;
#[doc = "interrupt line 23 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline23](itline23) module"]
pub type ITLINE23 = crate::Reg<u32, _ITLINE23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE23;
#[doc = "`read()` method returns [itline23::R](itline23::R) reader structure"]
impl crate::Readable for ITLINE23 {}
#[doc = "interrupt line 23 status register"]
pub mod itline23;
#[doc = "interrupt line 24 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline24](itline24) module"]
pub type ITLINE24 = crate::Reg<u32, _ITLINE24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE24;
#[doc = "`read()` method returns [itline24::R](itline24::R) reader structure"]
impl crate::Readable for ITLINE24 {}
#[doc = "interrupt line 24 status register"]
pub mod itline24;
#[doc = "interrupt line 25 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline25](itline25) module"]
pub type ITLINE25 = crate::Reg<u32, _ITLINE25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE25;
#[doc = "`read()` method returns [itline25::R](itline25::R) reader structure"]
impl crate::Readable for ITLINE25 {}
#[doc = "interrupt line 25 status register"]
pub mod itline25;
#[doc = "interrupt line 26 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline26](itline26) module"]
pub type ITLINE26 = crate::Reg<u32, _ITLINE26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE26;
#[doc = "`read()` method returns [itline26::R](itline26::R) reader structure"]
impl crate::Readable for ITLINE26 {}
#[doc = "interrupt line 26 status register"]
pub mod itline26;
#[doc = "interrupt line 27 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline27](itline27) module"]
pub type ITLINE27 = crate::Reg<u32, _ITLINE27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE27;
#[doc = "`read()` method returns [itline27::R](itline27::R) reader structure"]
impl crate::Readable for ITLINE27 {}
#[doc = "interrupt line 27 status register"]
pub mod itline27;
#[doc = "interrupt line 28 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline28](itline28) module"]
pub type ITLINE28 = crate::Reg<u32, _ITLINE28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE28;
#[doc = "`read()` method returns [itline28::R](itline28::R) reader structure"]
impl crate::Readable for ITLINE28 {}
#[doc = "interrupt line 28 status register"]
pub mod itline28;
#[doc = "interrupt line 29 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline29](itline29) module"]
pub type ITLINE29 = crate::Reg<u32, _ITLINE29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITLINE29;
#[doc = "`read()` method returns [itline29::R](itline29::R) reader structure"]
impl crate::Readable for ITLINE29 {}
#[doc = "interrupt line 29 status register"]
pub mod itline29;
