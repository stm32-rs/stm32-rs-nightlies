#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub mpcbb1_cr: MPCBB1_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - MPCBB control register"]
    pub mpcbb1_lckvtr1: MPCBB1_LCKVTR1,
    #[doc = "0x14 - MPCBB control register"]
    pub mpcbb1_lckvtr2: MPCBB1_LCKVTR2,
    _reserved3: [u8; 232usize],
    #[doc = "0x100 - MPCBBx vector register"]
    pub mpcbb1_vctr0: MPCBB1_VCTR0,
    #[doc = "0x104 - MPCBBx vector register"]
    pub mpcbb1_vctr1: MPCBB1_VCTR1,
    #[doc = "0x108 - MPCBBx vector register"]
    pub mpcbb1_vctr2: MPCBB1_VCTR2,
    #[doc = "0x10c - MPCBBx vector register"]
    pub mpcbb1_vctr3: MPCBB1_VCTR3,
    #[doc = "0x110 - MPCBBx vector register"]
    pub mpcbb1_vctr4: MPCBB1_VCTR4,
    #[doc = "0x114 - MPCBBx vector register"]
    pub mpcbb1_vctr5: MPCBB1_VCTR5,
    #[doc = "0x118 - MPCBBx vector register"]
    pub mpcbb1_vctr6: MPCBB1_VCTR6,
    #[doc = "0x11c - MPCBBx vector register"]
    pub mpcbb1_vctr7: MPCBB1_VCTR7,
    #[doc = "0x120 - MPCBBx vector register"]
    pub mpcbb1_vctr8: MPCBB1_VCTR8,
    #[doc = "0x124 - MPCBBx vector register"]
    pub mpcbb1_vctr9: MPCBB1_VCTR9,
    #[doc = "0x128 - MPCBBx vector register"]
    pub mpcbb1_vctr10: MPCBB1_VCTR10,
    #[doc = "0x12c - MPCBBx vector register"]
    pub mpcbb1_vctr11: MPCBB1_VCTR11,
    #[doc = "0x130 - MPCBBx vector register"]
    pub mpcbb1_vctr12: MPCBB1_VCTR12,
    #[doc = "0x134 - MPCBBx vector register"]
    pub mpcbb1_vctr13: MPCBB1_VCTR13,
    #[doc = "0x138 - MPCBBx vector register"]
    pub mpcbb1_vctr14: MPCBB1_VCTR14,
    #[doc = "0x13c - MPCBBx vector register"]
    pub mpcbb1_vctr15: MPCBB1_VCTR15,
    #[doc = "0x140 - MPCBBx vector register"]
    pub mpcbb1_vctr16: MPCBB1_VCTR16,
    #[doc = "0x144 - MPCBBx vector register"]
    pub mpcbb1_vctr17: MPCBB1_VCTR17,
    #[doc = "0x148 - MPCBBx vector register"]
    pub mpcbb1_vctr18: MPCBB1_VCTR18,
    #[doc = "0x14c - MPCBBx vector register"]
    pub mpcbb1_vctr19: MPCBB1_VCTR19,
    #[doc = "0x150 - MPCBBx vector register"]
    pub mpcbb1_vctr20: MPCBB1_VCTR20,
    #[doc = "0x154 - MPCBBx vector register"]
    pub mpcbb1_vctr21: MPCBB1_VCTR21,
    #[doc = "0x158 - MPCBBx vector register"]
    pub mpcbb1_vctr22: MPCBB1_VCTR22,
    #[doc = "0x15c - MPCBBx vector register"]
    pub mpcbb1_vctr23: MPCBB1_VCTR23,
    #[doc = "0x160 - MPCBBx vector register"]
    pub mpcbb1_vctr24: MPCBB1_VCTR24,
    #[doc = "0x164 - MPCBBx vector register"]
    pub mpcbb1_vctr25: MPCBB1_VCTR25,
    #[doc = "0x168 - MPCBBx vector register"]
    pub mpcbb1_vctr26: MPCBB1_VCTR26,
    #[doc = "0x16c - MPCBBx vector register"]
    pub mpcbb1_vctr27: MPCBB1_VCTR27,
    #[doc = "0x170 - MPCBBx vector register"]
    pub mpcbb1_vctr28: MPCBB1_VCTR28,
    #[doc = "0x174 - MPCBBx vector register"]
    pub mpcbb1_vctr29: MPCBB1_VCTR29,
    #[doc = "0x178 - MPCBBx vector register"]
    pub mpcbb1_vctr30: MPCBB1_VCTR30,
    #[doc = "0x17c - MPCBBx vector register"]
    pub mpcbb1_vctr31: MPCBB1_VCTR31,
    #[doc = "0x180 - MPCBBx vector register"]
    pub mpcbb1_vctr32: MPCBB1_VCTR32,
    #[doc = "0x184 - MPCBBx vector register"]
    pub mpcbb1_vctr33: MPCBB1_VCTR33,
    #[doc = "0x188 - MPCBBx vector register"]
    pub mpcbb1_vctr34: MPCBB1_VCTR34,
    #[doc = "0x18c - MPCBBx vector register"]
    pub mpcbb1_vctr35: MPCBB1_VCTR35,
    #[doc = "0x190 - MPCBBx vector register"]
    pub mpcbb1_vctr36: MPCBB1_VCTR36,
    #[doc = "0x194 - MPCBBx vector register"]
    pub mpcbb1_vctr37: MPCBB1_VCTR37,
    #[doc = "0x198 - MPCBBx vector register"]
    pub mpcbb1_vctr38: MPCBB1_VCTR38,
    #[doc = "0x19c - MPCBBx vector register"]
    pub mpcbb1_vctr39: MPCBB1_VCTR39,
    #[doc = "0x1a0 - MPCBBx vector register"]
    pub mpcbb1_vctr40: MPCBB1_VCTR40,
    #[doc = "0x1a4 - MPCBBx vector register"]
    pub mpcbb1_vctr41: MPCBB1_VCTR41,
    #[doc = "0x1a8 - MPCBBx vector register"]
    pub mpcbb1_vctr42: MPCBB1_VCTR42,
    #[doc = "0x1ac - MPCBBx vector register"]
    pub mpcbb1_vctr43: MPCBB1_VCTR43,
    #[doc = "0x1b0 - MPCBBx vector register"]
    pub mpcbb1_vctr44: MPCBB1_VCTR44,
    #[doc = "0x1b4 - MPCBBx vector register"]
    pub mpcbb1_vctr45: MPCBB1_VCTR45,
    #[doc = "0x1b8 - MPCBBx vector register"]
    pub mpcbb1_vctr46: MPCBB1_VCTR46,
    #[doc = "0x1bc - MPCBBx vector register"]
    pub mpcbb1_vctr47: MPCBB1_VCTR47,
    #[doc = "0x1c0 - MPCBBx vector register"]
    pub mpcbb1_vctr48: MPCBB1_VCTR48,
    #[doc = "0x1c4 - MPCBBx vector register"]
    pub mpcbb1_vctr49: MPCBB1_VCTR49,
    #[doc = "0x1c8 - MPCBBx vector register"]
    pub mpcbb1_vctr50: MPCBB1_VCTR50,
    #[doc = "0x1cc - MPCBBx vector register"]
    pub mpcbb1_vctr51: MPCBB1_VCTR51,
    #[doc = "0x1d0 - MPCBBx vector register"]
    pub mpcbb1_vctr52: MPCBB1_VCTR52,
    #[doc = "0x1d4 - MPCBBx vector register"]
    pub mpcbb1_vctr53: MPCBB1_VCTR53,
    #[doc = "0x1d8 - MPCBBx vector register"]
    pub mpcbb1_vctr54: MPCBB1_VCTR54,
    #[doc = "0x1dc - MPCBBx vector register"]
    pub mpcbb1_vctr55: MPCBB1_VCTR55,
    #[doc = "0x1e0 - MPCBBx vector register"]
    pub mpcbb1_vctr56: MPCBB1_VCTR56,
    #[doc = "0x1e4 - MPCBBx vector register"]
    pub mpcbb1_vctr57: MPCBB1_VCTR57,
    #[doc = "0x1e8 - MPCBBx vector register"]
    pub mpcbb1_vctr58: MPCBB1_VCTR58,
    #[doc = "0x1ec - MPCBBx vector register"]
    pub mpcbb1_vctr59: MPCBB1_VCTR59,
    #[doc = "0x1f0 - MPCBBx vector register"]
    pub mpcbb1_vctr60: MPCBB1_VCTR60,
    #[doc = "0x1f4 - MPCBBx vector register"]
    pub mpcbb1_vctr61: MPCBB1_VCTR61,
    #[doc = "0x1f8 - MPCBBx vector register"]
    pub mpcbb1_vctr62: MPCBB1_VCTR62,
    #[doc = "0x1fc - MPCBBx vector register"]
    pub mpcbb1_vctr63: MPCBB1_VCTR63,
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_cr](mpcbb1_cr) module"]
pub type MPCBB1_CR = crate::Reg<u32, _MPCBB1_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_CR;
#[doc = "`read()` method returns [mpcbb1_cr::R](mpcbb1_cr::R) reader structure"]
impl crate::Readable for MPCBB1_CR {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_cr::W](mpcbb1_cr::W) writer structure"]
impl crate::Writable for MPCBB1_CR {}
#[doc = "MPCBB control register"]
pub mod mpcbb1_cr;
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_lckvtr1](mpcbb1_lckvtr1) module"]
pub type MPCBB1_LCKVTR1 = crate::Reg<u32, _MPCBB1_LCKVTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_LCKVTR1;
#[doc = "`read()` method returns [mpcbb1_lckvtr1::R](mpcbb1_lckvtr1::R) reader structure"]
impl crate::Readable for MPCBB1_LCKVTR1 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_lckvtr1::W](mpcbb1_lckvtr1::W) writer structure"]
impl crate::Writable for MPCBB1_LCKVTR1 {}
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr1;
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_lckvtr2](mpcbb1_lckvtr2) module"]
pub type MPCBB1_LCKVTR2 = crate::Reg<u32, _MPCBB1_LCKVTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_LCKVTR2;
#[doc = "`read()` method returns [mpcbb1_lckvtr2::R](mpcbb1_lckvtr2::R) reader structure"]
impl crate::Readable for MPCBB1_LCKVTR2 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_lckvtr2::W](mpcbb1_lckvtr2::W) writer structure"]
impl crate::Writable for MPCBB1_LCKVTR2 {}
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr2;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr0](mpcbb1_vctr0) module"]
pub type MPCBB1_VCTR0 = crate::Reg<u32, _MPCBB1_VCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR0;
#[doc = "`read()` method returns [mpcbb1_vctr0::R](mpcbb1_vctr0::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR0 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr0::W](mpcbb1_vctr0::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR0 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr0;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr1](mpcbb1_vctr1) module"]
pub type MPCBB1_VCTR1 = crate::Reg<u32, _MPCBB1_VCTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR1;
#[doc = "`read()` method returns [mpcbb1_vctr1::R](mpcbb1_vctr1::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR1 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr1::W](mpcbb1_vctr1::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR1 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr1;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr2](mpcbb1_vctr2) module"]
pub type MPCBB1_VCTR2 = crate::Reg<u32, _MPCBB1_VCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR2;
#[doc = "`read()` method returns [mpcbb1_vctr2::R](mpcbb1_vctr2::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR2 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr2::W](mpcbb1_vctr2::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR2 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr2;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr3](mpcbb1_vctr3) module"]
pub type MPCBB1_VCTR3 = crate::Reg<u32, _MPCBB1_VCTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR3;
#[doc = "`read()` method returns [mpcbb1_vctr3::R](mpcbb1_vctr3::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR3 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr3::W](mpcbb1_vctr3::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR3 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr3;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr4](mpcbb1_vctr4) module"]
pub type MPCBB1_VCTR4 = crate::Reg<u32, _MPCBB1_VCTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR4;
#[doc = "`read()` method returns [mpcbb1_vctr4::R](mpcbb1_vctr4::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR4 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr4::W](mpcbb1_vctr4::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR4 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr4;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr5](mpcbb1_vctr5) module"]
pub type MPCBB1_VCTR5 = crate::Reg<u32, _MPCBB1_VCTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR5;
#[doc = "`read()` method returns [mpcbb1_vctr5::R](mpcbb1_vctr5::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR5 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr5::W](mpcbb1_vctr5::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR5 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr5;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr6](mpcbb1_vctr6) module"]
pub type MPCBB1_VCTR6 = crate::Reg<u32, _MPCBB1_VCTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR6;
#[doc = "`read()` method returns [mpcbb1_vctr6::R](mpcbb1_vctr6::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR6 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr6::W](mpcbb1_vctr6::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR6 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr6;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr7](mpcbb1_vctr7) module"]
pub type MPCBB1_VCTR7 = crate::Reg<u32, _MPCBB1_VCTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR7;
#[doc = "`read()` method returns [mpcbb1_vctr7::R](mpcbb1_vctr7::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR7 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr7::W](mpcbb1_vctr7::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR7 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr7;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr8](mpcbb1_vctr8) module"]
pub type MPCBB1_VCTR8 = crate::Reg<u32, _MPCBB1_VCTR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR8;
#[doc = "`read()` method returns [mpcbb1_vctr8::R](mpcbb1_vctr8::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR8 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr8::W](mpcbb1_vctr8::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR8 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr8;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr9](mpcbb1_vctr9) module"]
pub type MPCBB1_VCTR9 = crate::Reg<u32, _MPCBB1_VCTR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR9;
#[doc = "`read()` method returns [mpcbb1_vctr9::R](mpcbb1_vctr9::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR9 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr9::W](mpcbb1_vctr9::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR9 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr9;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr10](mpcbb1_vctr10) module"]
pub type MPCBB1_VCTR10 = crate::Reg<u32, _MPCBB1_VCTR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR10;
#[doc = "`read()` method returns [mpcbb1_vctr10::R](mpcbb1_vctr10::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR10 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr10::W](mpcbb1_vctr10::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR10 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr10;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr11](mpcbb1_vctr11) module"]
pub type MPCBB1_VCTR11 = crate::Reg<u32, _MPCBB1_VCTR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR11;
#[doc = "`read()` method returns [mpcbb1_vctr11::R](mpcbb1_vctr11::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR11 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr11::W](mpcbb1_vctr11::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR11 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr11;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr12](mpcbb1_vctr12) module"]
pub type MPCBB1_VCTR12 = crate::Reg<u32, _MPCBB1_VCTR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR12;
#[doc = "`read()` method returns [mpcbb1_vctr12::R](mpcbb1_vctr12::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR12 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr12::W](mpcbb1_vctr12::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR12 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr12;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr13](mpcbb1_vctr13) module"]
pub type MPCBB1_VCTR13 = crate::Reg<u32, _MPCBB1_VCTR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR13;
#[doc = "`read()` method returns [mpcbb1_vctr13::R](mpcbb1_vctr13::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR13 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr13::W](mpcbb1_vctr13::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR13 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr13;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr14](mpcbb1_vctr14) module"]
pub type MPCBB1_VCTR14 = crate::Reg<u32, _MPCBB1_VCTR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR14;
#[doc = "`read()` method returns [mpcbb1_vctr14::R](mpcbb1_vctr14::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR14 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr14::W](mpcbb1_vctr14::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR14 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr14;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr15](mpcbb1_vctr15) module"]
pub type MPCBB1_VCTR15 = crate::Reg<u32, _MPCBB1_VCTR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR15;
#[doc = "`read()` method returns [mpcbb1_vctr15::R](mpcbb1_vctr15::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR15 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr15::W](mpcbb1_vctr15::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR15 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr15;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr16](mpcbb1_vctr16) module"]
pub type MPCBB1_VCTR16 = crate::Reg<u32, _MPCBB1_VCTR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR16;
#[doc = "`read()` method returns [mpcbb1_vctr16::R](mpcbb1_vctr16::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR16 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr16::W](mpcbb1_vctr16::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR16 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr16;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr17](mpcbb1_vctr17) module"]
pub type MPCBB1_VCTR17 = crate::Reg<u32, _MPCBB1_VCTR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR17;
#[doc = "`read()` method returns [mpcbb1_vctr17::R](mpcbb1_vctr17::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR17 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr17::W](mpcbb1_vctr17::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR17 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr17;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr18](mpcbb1_vctr18) module"]
pub type MPCBB1_VCTR18 = crate::Reg<u32, _MPCBB1_VCTR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR18;
#[doc = "`read()` method returns [mpcbb1_vctr18::R](mpcbb1_vctr18::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR18 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr18::W](mpcbb1_vctr18::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR18 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr18;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr19](mpcbb1_vctr19) module"]
pub type MPCBB1_VCTR19 = crate::Reg<u32, _MPCBB1_VCTR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR19;
#[doc = "`read()` method returns [mpcbb1_vctr19::R](mpcbb1_vctr19::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR19 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr19::W](mpcbb1_vctr19::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR19 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr19;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr20](mpcbb1_vctr20) module"]
pub type MPCBB1_VCTR20 = crate::Reg<u32, _MPCBB1_VCTR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR20;
#[doc = "`read()` method returns [mpcbb1_vctr20::R](mpcbb1_vctr20::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR20 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr20::W](mpcbb1_vctr20::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR20 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr20;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr21](mpcbb1_vctr21) module"]
pub type MPCBB1_VCTR21 = crate::Reg<u32, _MPCBB1_VCTR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR21;
#[doc = "`read()` method returns [mpcbb1_vctr21::R](mpcbb1_vctr21::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR21 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr21::W](mpcbb1_vctr21::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR21 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr21;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr22](mpcbb1_vctr22) module"]
pub type MPCBB1_VCTR22 = crate::Reg<u32, _MPCBB1_VCTR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR22;
#[doc = "`read()` method returns [mpcbb1_vctr22::R](mpcbb1_vctr22::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR22 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr22::W](mpcbb1_vctr22::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR22 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr22;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr23](mpcbb1_vctr23) module"]
pub type MPCBB1_VCTR23 = crate::Reg<u32, _MPCBB1_VCTR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR23;
#[doc = "`read()` method returns [mpcbb1_vctr23::R](mpcbb1_vctr23::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR23 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr23::W](mpcbb1_vctr23::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR23 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr23;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr24](mpcbb1_vctr24) module"]
pub type MPCBB1_VCTR24 = crate::Reg<u32, _MPCBB1_VCTR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR24;
#[doc = "`read()` method returns [mpcbb1_vctr24::R](mpcbb1_vctr24::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR24 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr24::W](mpcbb1_vctr24::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR24 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr24;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr25](mpcbb1_vctr25) module"]
pub type MPCBB1_VCTR25 = crate::Reg<u32, _MPCBB1_VCTR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR25;
#[doc = "`read()` method returns [mpcbb1_vctr25::R](mpcbb1_vctr25::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR25 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr25::W](mpcbb1_vctr25::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR25 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr25;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr26](mpcbb1_vctr26) module"]
pub type MPCBB1_VCTR26 = crate::Reg<u32, _MPCBB1_VCTR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR26;
#[doc = "`read()` method returns [mpcbb1_vctr26::R](mpcbb1_vctr26::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR26 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr26::W](mpcbb1_vctr26::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR26 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr26;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr27](mpcbb1_vctr27) module"]
pub type MPCBB1_VCTR27 = crate::Reg<u32, _MPCBB1_VCTR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR27;
#[doc = "`read()` method returns [mpcbb1_vctr27::R](mpcbb1_vctr27::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR27 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr27::W](mpcbb1_vctr27::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR27 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr27;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr28](mpcbb1_vctr28) module"]
pub type MPCBB1_VCTR28 = crate::Reg<u32, _MPCBB1_VCTR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR28;
#[doc = "`read()` method returns [mpcbb1_vctr28::R](mpcbb1_vctr28::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR28 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr28::W](mpcbb1_vctr28::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR28 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr28;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr29](mpcbb1_vctr29) module"]
pub type MPCBB1_VCTR29 = crate::Reg<u32, _MPCBB1_VCTR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR29;
#[doc = "`read()` method returns [mpcbb1_vctr29::R](mpcbb1_vctr29::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR29 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr29::W](mpcbb1_vctr29::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR29 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr29;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr30](mpcbb1_vctr30) module"]
pub type MPCBB1_VCTR30 = crate::Reg<u32, _MPCBB1_VCTR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR30;
#[doc = "`read()` method returns [mpcbb1_vctr30::R](mpcbb1_vctr30::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR30 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr30::W](mpcbb1_vctr30::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR30 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr30;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr31](mpcbb1_vctr31) module"]
pub type MPCBB1_VCTR31 = crate::Reg<u32, _MPCBB1_VCTR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR31;
#[doc = "`read()` method returns [mpcbb1_vctr31::R](mpcbb1_vctr31::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR31 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr31::W](mpcbb1_vctr31::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR31 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr31;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr32](mpcbb1_vctr32) module"]
pub type MPCBB1_VCTR32 = crate::Reg<u32, _MPCBB1_VCTR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR32;
#[doc = "`read()` method returns [mpcbb1_vctr32::R](mpcbb1_vctr32::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR32 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr32::W](mpcbb1_vctr32::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR32 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr32;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr33](mpcbb1_vctr33) module"]
pub type MPCBB1_VCTR33 = crate::Reg<u32, _MPCBB1_VCTR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR33;
#[doc = "`read()` method returns [mpcbb1_vctr33::R](mpcbb1_vctr33::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR33 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr33::W](mpcbb1_vctr33::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR33 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr33;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr34](mpcbb1_vctr34) module"]
pub type MPCBB1_VCTR34 = crate::Reg<u32, _MPCBB1_VCTR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR34;
#[doc = "`read()` method returns [mpcbb1_vctr34::R](mpcbb1_vctr34::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR34 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr34::W](mpcbb1_vctr34::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR34 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr34;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr35](mpcbb1_vctr35) module"]
pub type MPCBB1_VCTR35 = crate::Reg<u32, _MPCBB1_VCTR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR35;
#[doc = "`read()` method returns [mpcbb1_vctr35::R](mpcbb1_vctr35::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR35 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr35::W](mpcbb1_vctr35::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR35 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr35;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr36](mpcbb1_vctr36) module"]
pub type MPCBB1_VCTR36 = crate::Reg<u32, _MPCBB1_VCTR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR36;
#[doc = "`read()` method returns [mpcbb1_vctr36::R](mpcbb1_vctr36::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR36 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr36::W](mpcbb1_vctr36::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR36 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr36;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr37](mpcbb1_vctr37) module"]
pub type MPCBB1_VCTR37 = crate::Reg<u32, _MPCBB1_VCTR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR37;
#[doc = "`read()` method returns [mpcbb1_vctr37::R](mpcbb1_vctr37::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR37 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr37::W](mpcbb1_vctr37::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR37 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr37;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr38](mpcbb1_vctr38) module"]
pub type MPCBB1_VCTR38 = crate::Reg<u32, _MPCBB1_VCTR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR38;
#[doc = "`read()` method returns [mpcbb1_vctr38::R](mpcbb1_vctr38::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR38 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr38::W](mpcbb1_vctr38::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR38 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr38;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr39](mpcbb1_vctr39) module"]
pub type MPCBB1_VCTR39 = crate::Reg<u32, _MPCBB1_VCTR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR39;
#[doc = "`read()` method returns [mpcbb1_vctr39::R](mpcbb1_vctr39::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR39 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr39::W](mpcbb1_vctr39::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR39 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr39;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr40](mpcbb1_vctr40) module"]
pub type MPCBB1_VCTR40 = crate::Reg<u32, _MPCBB1_VCTR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR40;
#[doc = "`read()` method returns [mpcbb1_vctr40::R](mpcbb1_vctr40::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR40 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr40::W](mpcbb1_vctr40::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR40 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr40;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr41](mpcbb1_vctr41) module"]
pub type MPCBB1_VCTR41 = crate::Reg<u32, _MPCBB1_VCTR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR41;
#[doc = "`read()` method returns [mpcbb1_vctr41::R](mpcbb1_vctr41::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR41 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr41::W](mpcbb1_vctr41::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR41 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr41;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr42](mpcbb1_vctr42) module"]
pub type MPCBB1_VCTR42 = crate::Reg<u32, _MPCBB1_VCTR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR42;
#[doc = "`read()` method returns [mpcbb1_vctr42::R](mpcbb1_vctr42::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR42 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr42::W](mpcbb1_vctr42::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR42 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr42;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr43](mpcbb1_vctr43) module"]
pub type MPCBB1_VCTR43 = crate::Reg<u32, _MPCBB1_VCTR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR43;
#[doc = "`read()` method returns [mpcbb1_vctr43::R](mpcbb1_vctr43::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR43 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr43::W](mpcbb1_vctr43::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR43 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr43;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr44](mpcbb1_vctr44) module"]
pub type MPCBB1_VCTR44 = crate::Reg<u32, _MPCBB1_VCTR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR44;
#[doc = "`read()` method returns [mpcbb1_vctr44::R](mpcbb1_vctr44::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR44 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr44::W](mpcbb1_vctr44::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR44 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr44;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr45](mpcbb1_vctr45) module"]
pub type MPCBB1_VCTR45 = crate::Reg<u32, _MPCBB1_VCTR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR45;
#[doc = "`read()` method returns [mpcbb1_vctr45::R](mpcbb1_vctr45::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR45 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr45::W](mpcbb1_vctr45::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR45 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr45;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr46](mpcbb1_vctr46) module"]
pub type MPCBB1_VCTR46 = crate::Reg<u32, _MPCBB1_VCTR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR46;
#[doc = "`read()` method returns [mpcbb1_vctr46::R](mpcbb1_vctr46::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR46 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr46::W](mpcbb1_vctr46::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR46 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr46;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr47](mpcbb1_vctr47) module"]
pub type MPCBB1_VCTR47 = crate::Reg<u32, _MPCBB1_VCTR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR47;
#[doc = "`read()` method returns [mpcbb1_vctr47::R](mpcbb1_vctr47::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR47 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr47::W](mpcbb1_vctr47::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR47 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr47;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr48](mpcbb1_vctr48) module"]
pub type MPCBB1_VCTR48 = crate::Reg<u32, _MPCBB1_VCTR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR48;
#[doc = "`read()` method returns [mpcbb1_vctr48::R](mpcbb1_vctr48::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR48 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr48::W](mpcbb1_vctr48::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR48 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr48;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr49](mpcbb1_vctr49) module"]
pub type MPCBB1_VCTR49 = crate::Reg<u32, _MPCBB1_VCTR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR49;
#[doc = "`read()` method returns [mpcbb1_vctr49::R](mpcbb1_vctr49::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR49 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr49::W](mpcbb1_vctr49::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR49 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr49;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr50](mpcbb1_vctr50) module"]
pub type MPCBB1_VCTR50 = crate::Reg<u32, _MPCBB1_VCTR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR50;
#[doc = "`read()` method returns [mpcbb1_vctr50::R](mpcbb1_vctr50::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR50 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr50::W](mpcbb1_vctr50::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR50 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr50;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr51](mpcbb1_vctr51) module"]
pub type MPCBB1_VCTR51 = crate::Reg<u32, _MPCBB1_VCTR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR51;
#[doc = "`read()` method returns [mpcbb1_vctr51::R](mpcbb1_vctr51::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR51 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr51::W](mpcbb1_vctr51::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR51 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr51;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr52](mpcbb1_vctr52) module"]
pub type MPCBB1_VCTR52 = crate::Reg<u32, _MPCBB1_VCTR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR52;
#[doc = "`read()` method returns [mpcbb1_vctr52::R](mpcbb1_vctr52::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR52 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr52::W](mpcbb1_vctr52::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR52 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr52;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr53](mpcbb1_vctr53) module"]
pub type MPCBB1_VCTR53 = crate::Reg<u32, _MPCBB1_VCTR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR53;
#[doc = "`read()` method returns [mpcbb1_vctr53::R](mpcbb1_vctr53::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR53 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr53::W](mpcbb1_vctr53::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR53 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr53;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr54](mpcbb1_vctr54) module"]
pub type MPCBB1_VCTR54 = crate::Reg<u32, _MPCBB1_VCTR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR54;
#[doc = "`read()` method returns [mpcbb1_vctr54::R](mpcbb1_vctr54::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR54 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr54::W](mpcbb1_vctr54::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR54 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr54;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr55](mpcbb1_vctr55) module"]
pub type MPCBB1_VCTR55 = crate::Reg<u32, _MPCBB1_VCTR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR55;
#[doc = "`read()` method returns [mpcbb1_vctr55::R](mpcbb1_vctr55::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR55 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr55::W](mpcbb1_vctr55::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR55 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr55;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr56](mpcbb1_vctr56) module"]
pub type MPCBB1_VCTR56 = crate::Reg<u32, _MPCBB1_VCTR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR56;
#[doc = "`read()` method returns [mpcbb1_vctr56::R](mpcbb1_vctr56::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR56 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr56::W](mpcbb1_vctr56::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR56 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr56;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr57](mpcbb1_vctr57) module"]
pub type MPCBB1_VCTR57 = crate::Reg<u32, _MPCBB1_VCTR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR57;
#[doc = "`read()` method returns [mpcbb1_vctr57::R](mpcbb1_vctr57::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR57 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr57::W](mpcbb1_vctr57::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR57 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr57;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr58](mpcbb1_vctr58) module"]
pub type MPCBB1_VCTR58 = crate::Reg<u32, _MPCBB1_VCTR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR58;
#[doc = "`read()` method returns [mpcbb1_vctr58::R](mpcbb1_vctr58::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR58 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr58::W](mpcbb1_vctr58::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR58 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr58;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr59](mpcbb1_vctr59) module"]
pub type MPCBB1_VCTR59 = crate::Reg<u32, _MPCBB1_VCTR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR59;
#[doc = "`read()` method returns [mpcbb1_vctr59::R](mpcbb1_vctr59::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR59 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr59::W](mpcbb1_vctr59::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR59 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr59;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr60](mpcbb1_vctr60) module"]
pub type MPCBB1_VCTR60 = crate::Reg<u32, _MPCBB1_VCTR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR60;
#[doc = "`read()` method returns [mpcbb1_vctr60::R](mpcbb1_vctr60::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR60 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr60::W](mpcbb1_vctr60::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR60 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr60;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr61](mpcbb1_vctr61) module"]
pub type MPCBB1_VCTR61 = crate::Reg<u32, _MPCBB1_VCTR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR61;
#[doc = "`read()` method returns [mpcbb1_vctr61::R](mpcbb1_vctr61::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR61 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr61::W](mpcbb1_vctr61::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR61 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr61;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr62](mpcbb1_vctr62) module"]
pub type MPCBB1_VCTR62 = crate::Reg<u32, _MPCBB1_VCTR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR62;
#[doc = "`read()` method returns [mpcbb1_vctr62::R](mpcbb1_vctr62::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR62 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr62::W](mpcbb1_vctr62::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR62 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr62;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr63](mpcbb1_vctr63) module"]
pub type MPCBB1_VCTR63 = crate::Reg<u32, _MPCBB1_VCTR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB1_VCTR63;
#[doc = "`read()` method returns [mpcbb1_vctr63::R](mpcbb1_vctr63::R) reader structure"]
impl crate::Readable for MPCBB1_VCTR63 {}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr63::W](mpcbb1_vctr63::W) writer structure"]
impl crate::Writable for MPCBB1_VCTR63 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr63;
