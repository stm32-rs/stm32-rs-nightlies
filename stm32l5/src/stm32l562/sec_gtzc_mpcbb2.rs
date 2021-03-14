#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub mpcbb2_cr: MPCBB2_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - MPCBB control register"]
    pub mpcbb2_lckvtr1: MPCBB2_LCKVTR1,
    #[doc = "0x14 - MPCBB control register"]
    pub mpcbb2_lckvtr2: MPCBB2_LCKVTR2,
    _reserved3: [u8; 232usize],
    #[doc = "0x100 - MPCBBx vector register"]
    pub mpcbb2_vctr0: MPCBB2_VCTR0,
    #[doc = "0x104 - MPCBBx vector register"]
    pub mpcbb2_vctr1: MPCBB2_VCTR1,
    #[doc = "0x108 - MPCBBx vector register"]
    pub mpcbb2_vctr2: MPCBB2_VCTR2,
    #[doc = "0x10c - MPCBBx vector register"]
    pub mpcbb2_vctr3: MPCBB2_VCTR3,
    #[doc = "0x110 - MPCBBx vector register"]
    pub mpcbb2_vctr4: MPCBB2_VCTR4,
    #[doc = "0x114 - MPCBBx vector register"]
    pub mpcbb2_vctr5: MPCBB2_VCTR5,
    #[doc = "0x118 - MPCBBx vector register"]
    pub mpcbb2_vctr6: MPCBB2_VCTR6,
    #[doc = "0x11c - MPCBBx vector register"]
    pub mpcbb2_vctr7: MPCBB2_VCTR7,
    #[doc = "0x120 - MPCBBx vector register"]
    pub mpcbb2_vctr8: MPCBB2_VCTR8,
    #[doc = "0x124 - MPCBBx vector register"]
    pub mpcbb2_vctr9: MPCBB2_VCTR9,
    #[doc = "0x128 - MPCBBx vector register"]
    pub mpcbb2_vctr10: MPCBB2_VCTR10,
    #[doc = "0x12c - MPCBBx vector register"]
    pub mpcbb2_vctr11: MPCBB2_VCTR11,
    #[doc = "0x130 - MPCBBx vector register"]
    pub mpcbb2_vctr12: MPCBB2_VCTR12,
    #[doc = "0x134 - MPCBBx vector register"]
    pub mpcbb2_vctr13: MPCBB2_VCTR13,
    #[doc = "0x138 - MPCBBx vector register"]
    pub mpcbb2_vctr14: MPCBB2_VCTR14,
    #[doc = "0x13c - MPCBBx vector register"]
    pub mpcbb2_vctr15: MPCBB2_VCTR15,
    #[doc = "0x140 - MPCBBx vector register"]
    pub mpcbb2_vctr16: MPCBB2_VCTR16,
    #[doc = "0x144 - MPCBBx vector register"]
    pub mpcbb2_vctr17: MPCBB2_VCTR17,
    #[doc = "0x148 - MPCBBx vector register"]
    pub mpcbb2_vctr18: MPCBB2_VCTR18,
    #[doc = "0x14c - MPCBBx vector register"]
    pub mpcbb2_vctr19: MPCBB2_VCTR19,
    #[doc = "0x150 - MPCBBx vector register"]
    pub mpcbb2_vctr20: MPCBB2_VCTR20,
    #[doc = "0x154 - MPCBBx vector register"]
    pub mpcbb2_vctr21: MPCBB2_VCTR21,
    #[doc = "0x158 - MPCBBx vector register"]
    pub mpcbb2_vctr22: MPCBB2_VCTR22,
    #[doc = "0x15c - MPCBBx vector register"]
    pub mpcbb2_vctr23: MPCBB2_VCTR23,
    #[doc = "0x160 - MPCBBx vector register"]
    pub mpcbb2_vctr24: MPCBB2_VCTR24,
    #[doc = "0x164 - MPCBBx vector register"]
    pub mpcbb2_vctr25: MPCBB2_VCTR25,
    #[doc = "0x168 - MPCBBx vector register"]
    pub mpcbb2_vctr26: MPCBB2_VCTR26,
    #[doc = "0x16c - MPCBBx vector register"]
    pub mpcbb2_vctr27: MPCBB2_VCTR27,
    #[doc = "0x170 - MPCBBx vector register"]
    pub mpcbb2_vctr28: MPCBB2_VCTR28,
    #[doc = "0x174 - MPCBBx vector register"]
    pub mpcbb2_vctr29: MPCBB2_VCTR29,
    #[doc = "0x178 - MPCBBx vector register"]
    pub mpcbb2_vctr30: MPCBB2_VCTR30,
    #[doc = "0x17c - MPCBBx vector register"]
    pub mpcbb2_vctr31: MPCBB2_VCTR31,
    #[doc = "0x180 - MPCBBx vector register"]
    pub mpcbb2_vctr32: MPCBB2_VCTR32,
    #[doc = "0x184 - MPCBBx vector register"]
    pub mpcbb2_vctr33: MPCBB2_VCTR33,
    #[doc = "0x188 - MPCBBx vector register"]
    pub mpcbb2_vctr34: MPCBB2_VCTR34,
    #[doc = "0x18c - MPCBBx vector register"]
    pub mpcbb2_vctr35: MPCBB2_VCTR35,
    #[doc = "0x190 - MPCBBx vector register"]
    pub mpcbb2_vctr36: MPCBB2_VCTR36,
    #[doc = "0x194 - MPCBBx vector register"]
    pub mpcbb2_vctr37: MPCBB2_VCTR37,
    #[doc = "0x198 - MPCBBx vector register"]
    pub mpcbb2_vctr38: MPCBB2_VCTR38,
    #[doc = "0x19c - MPCBBx vector register"]
    pub mpcbb2_vctr39: MPCBB2_VCTR39,
    #[doc = "0x1a0 - MPCBBx vector register"]
    pub mpcbb2_vctr40: MPCBB2_VCTR40,
    #[doc = "0x1a4 - MPCBBx vector register"]
    pub mpcbb2_vctr41: MPCBB2_VCTR41,
    #[doc = "0x1a8 - MPCBBx vector register"]
    pub mpcbb2_vctr42: MPCBB2_VCTR42,
    #[doc = "0x1ac - MPCBBx vector register"]
    pub mpcbb2_vctr43: MPCBB2_VCTR43,
    #[doc = "0x1b0 - MPCBBx vector register"]
    pub mpcbb2_vctr44: MPCBB2_VCTR44,
    #[doc = "0x1b4 - MPCBBx vector register"]
    pub mpcbb2_vctr45: MPCBB2_VCTR45,
    #[doc = "0x1b8 - MPCBBx vector register"]
    pub mpcbb2_vctr46: MPCBB2_VCTR46,
    #[doc = "0x1bc - MPCBBx vector register"]
    pub mpcbb2_vctr47: MPCBB2_VCTR47,
    #[doc = "0x1c0 - MPCBBx vector register"]
    pub mpcbb2_vctr48: MPCBB2_VCTR48,
    #[doc = "0x1c4 - MPCBBx vector register"]
    pub mpcbb2_vctr49: MPCBB2_VCTR49,
    #[doc = "0x1c8 - MPCBBx vector register"]
    pub mpcbb2_vctr50: MPCBB2_VCTR50,
    #[doc = "0x1cc - MPCBBx vector register"]
    pub mpcbb2_vctr51: MPCBB2_VCTR51,
    #[doc = "0x1d0 - MPCBBx vector register"]
    pub mpcbb2_vctr52: MPCBB2_VCTR52,
    #[doc = "0x1d4 - MPCBBx vector register"]
    pub mpcbb2_vctr53: MPCBB2_VCTR53,
    #[doc = "0x1d8 - MPCBBx vector register"]
    pub mpcbb2_vctr54: MPCBB2_VCTR54,
    #[doc = "0x1dc - MPCBBx vector register"]
    pub mpcbb2_vctr55: MPCBB2_VCTR55,
    #[doc = "0x1e0 - MPCBBx vector register"]
    pub mpcbb2_vctr56: MPCBB2_VCTR56,
    #[doc = "0x1e4 - MPCBBx vector register"]
    pub mpcbb2_vctr57: MPCBB2_VCTR57,
    #[doc = "0x1e8 - MPCBBx vector register"]
    pub mpcbb2_vctr58: MPCBB2_VCTR58,
    #[doc = "0x1ec - MPCBBx vector register"]
    pub mpcbb2_vctr59: MPCBB2_VCTR59,
    #[doc = "0x1f0 - MPCBBx vector register"]
    pub mpcbb2_vctr60: MPCBB2_VCTR60,
    #[doc = "0x1f4 - MPCBBx vector register"]
    pub mpcbb2_vctr61: MPCBB2_VCTR61,
    #[doc = "0x1f8 - MPCBBx vector register"]
    pub mpcbb2_vctr62: MPCBB2_VCTR62,
    #[doc = "0x1fc - MPCBBx vector register"]
    pub mpcbb2_vctr63: MPCBB2_VCTR63,
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_cr](mpcbb2_cr) module"]
pub type MPCBB2_CR = crate::Reg<u32, _MPCBB2_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_CR;
#[doc = "`read()` method returns [mpcbb2_cr::R](mpcbb2_cr::R) reader structure"]
impl crate::Readable for MPCBB2_CR {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_cr::W](mpcbb2_cr::W) writer structure"]
impl crate::Writable for MPCBB2_CR {}
#[doc = "MPCBB control register"]
pub mod mpcbb2_cr;
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_lckvtr1](mpcbb2_lckvtr1) module"]
pub type MPCBB2_LCKVTR1 = crate::Reg<u32, _MPCBB2_LCKVTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_LCKVTR1;
#[doc = "`read()` method returns [mpcbb2_lckvtr1::R](mpcbb2_lckvtr1::R) reader structure"]
impl crate::Readable for MPCBB2_LCKVTR1 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_lckvtr1::W](mpcbb2_lckvtr1::W) writer structure"]
impl crate::Writable for MPCBB2_LCKVTR1 {}
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr1;
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_lckvtr2](mpcbb2_lckvtr2) module"]
pub type MPCBB2_LCKVTR2 = crate::Reg<u32, _MPCBB2_LCKVTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_LCKVTR2;
#[doc = "`read()` method returns [mpcbb2_lckvtr2::R](mpcbb2_lckvtr2::R) reader structure"]
impl crate::Readable for MPCBB2_LCKVTR2 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_lckvtr2::W](mpcbb2_lckvtr2::W) writer structure"]
impl crate::Writable for MPCBB2_LCKVTR2 {}
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr2;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr0](mpcbb2_vctr0) module"]
pub type MPCBB2_VCTR0 = crate::Reg<u32, _MPCBB2_VCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR0;
#[doc = "`read()` method returns [mpcbb2_vctr0::R](mpcbb2_vctr0::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR0 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr0::W](mpcbb2_vctr0::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR0 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr0;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr1](mpcbb2_vctr1) module"]
pub type MPCBB2_VCTR1 = crate::Reg<u32, _MPCBB2_VCTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR1;
#[doc = "`read()` method returns [mpcbb2_vctr1::R](mpcbb2_vctr1::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR1 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr1::W](mpcbb2_vctr1::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR1 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr1;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr2](mpcbb2_vctr2) module"]
pub type MPCBB2_VCTR2 = crate::Reg<u32, _MPCBB2_VCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR2;
#[doc = "`read()` method returns [mpcbb2_vctr2::R](mpcbb2_vctr2::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR2 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr2::W](mpcbb2_vctr2::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR2 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr2;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr3](mpcbb2_vctr3) module"]
pub type MPCBB2_VCTR3 = crate::Reg<u32, _MPCBB2_VCTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR3;
#[doc = "`read()` method returns [mpcbb2_vctr3::R](mpcbb2_vctr3::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR3 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr3::W](mpcbb2_vctr3::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR3 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr3;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr4](mpcbb2_vctr4) module"]
pub type MPCBB2_VCTR4 = crate::Reg<u32, _MPCBB2_VCTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR4;
#[doc = "`read()` method returns [mpcbb2_vctr4::R](mpcbb2_vctr4::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR4 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr4::W](mpcbb2_vctr4::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR4 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr4;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr5](mpcbb2_vctr5) module"]
pub type MPCBB2_VCTR5 = crate::Reg<u32, _MPCBB2_VCTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR5;
#[doc = "`read()` method returns [mpcbb2_vctr5::R](mpcbb2_vctr5::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR5 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr5::W](mpcbb2_vctr5::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR5 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr5;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr6](mpcbb2_vctr6) module"]
pub type MPCBB2_VCTR6 = crate::Reg<u32, _MPCBB2_VCTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR6;
#[doc = "`read()` method returns [mpcbb2_vctr6::R](mpcbb2_vctr6::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR6 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr6::W](mpcbb2_vctr6::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR6 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr6;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr7](mpcbb2_vctr7) module"]
pub type MPCBB2_VCTR7 = crate::Reg<u32, _MPCBB2_VCTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR7;
#[doc = "`read()` method returns [mpcbb2_vctr7::R](mpcbb2_vctr7::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR7 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr7::W](mpcbb2_vctr7::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR7 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr7;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr8](mpcbb2_vctr8) module"]
pub type MPCBB2_VCTR8 = crate::Reg<u32, _MPCBB2_VCTR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR8;
#[doc = "`read()` method returns [mpcbb2_vctr8::R](mpcbb2_vctr8::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR8 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr8::W](mpcbb2_vctr8::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR8 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr8;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr9](mpcbb2_vctr9) module"]
pub type MPCBB2_VCTR9 = crate::Reg<u32, _MPCBB2_VCTR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR9;
#[doc = "`read()` method returns [mpcbb2_vctr9::R](mpcbb2_vctr9::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR9 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr9::W](mpcbb2_vctr9::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR9 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr9;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr10](mpcbb2_vctr10) module"]
pub type MPCBB2_VCTR10 = crate::Reg<u32, _MPCBB2_VCTR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR10;
#[doc = "`read()` method returns [mpcbb2_vctr10::R](mpcbb2_vctr10::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR10 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr10::W](mpcbb2_vctr10::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR10 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr10;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr11](mpcbb2_vctr11) module"]
pub type MPCBB2_VCTR11 = crate::Reg<u32, _MPCBB2_VCTR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR11;
#[doc = "`read()` method returns [mpcbb2_vctr11::R](mpcbb2_vctr11::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR11 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr11::W](mpcbb2_vctr11::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR11 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr11;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr12](mpcbb2_vctr12) module"]
pub type MPCBB2_VCTR12 = crate::Reg<u32, _MPCBB2_VCTR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR12;
#[doc = "`read()` method returns [mpcbb2_vctr12::R](mpcbb2_vctr12::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR12 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr12::W](mpcbb2_vctr12::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR12 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr12;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr13](mpcbb2_vctr13) module"]
pub type MPCBB2_VCTR13 = crate::Reg<u32, _MPCBB2_VCTR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR13;
#[doc = "`read()` method returns [mpcbb2_vctr13::R](mpcbb2_vctr13::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR13 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr13::W](mpcbb2_vctr13::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR13 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr13;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr14](mpcbb2_vctr14) module"]
pub type MPCBB2_VCTR14 = crate::Reg<u32, _MPCBB2_VCTR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR14;
#[doc = "`read()` method returns [mpcbb2_vctr14::R](mpcbb2_vctr14::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR14 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr14::W](mpcbb2_vctr14::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR14 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr14;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr15](mpcbb2_vctr15) module"]
pub type MPCBB2_VCTR15 = crate::Reg<u32, _MPCBB2_VCTR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR15;
#[doc = "`read()` method returns [mpcbb2_vctr15::R](mpcbb2_vctr15::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR15 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr15::W](mpcbb2_vctr15::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR15 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr15;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr16](mpcbb2_vctr16) module"]
pub type MPCBB2_VCTR16 = crate::Reg<u32, _MPCBB2_VCTR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR16;
#[doc = "`read()` method returns [mpcbb2_vctr16::R](mpcbb2_vctr16::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR16 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr16::W](mpcbb2_vctr16::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR16 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr16;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr17](mpcbb2_vctr17) module"]
pub type MPCBB2_VCTR17 = crate::Reg<u32, _MPCBB2_VCTR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR17;
#[doc = "`read()` method returns [mpcbb2_vctr17::R](mpcbb2_vctr17::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR17 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr17::W](mpcbb2_vctr17::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR17 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr17;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr18](mpcbb2_vctr18) module"]
pub type MPCBB2_VCTR18 = crate::Reg<u32, _MPCBB2_VCTR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR18;
#[doc = "`read()` method returns [mpcbb2_vctr18::R](mpcbb2_vctr18::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR18 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr18::W](mpcbb2_vctr18::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR18 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr18;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr19](mpcbb2_vctr19) module"]
pub type MPCBB2_VCTR19 = crate::Reg<u32, _MPCBB2_VCTR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR19;
#[doc = "`read()` method returns [mpcbb2_vctr19::R](mpcbb2_vctr19::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR19 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr19::W](mpcbb2_vctr19::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR19 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr19;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr20](mpcbb2_vctr20) module"]
pub type MPCBB2_VCTR20 = crate::Reg<u32, _MPCBB2_VCTR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR20;
#[doc = "`read()` method returns [mpcbb2_vctr20::R](mpcbb2_vctr20::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR20 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr20::W](mpcbb2_vctr20::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR20 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr20;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr21](mpcbb2_vctr21) module"]
pub type MPCBB2_VCTR21 = crate::Reg<u32, _MPCBB2_VCTR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR21;
#[doc = "`read()` method returns [mpcbb2_vctr21::R](mpcbb2_vctr21::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR21 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr21::W](mpcbb2_vctr21::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR21 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr21;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr22](mpcbb2_vctr22) module"]
pub type MPCBB2_VCTR22 = crate::Reg<u32, _MPCBB2_VCTR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR22;
#[doc = "`read()` method returns [mpcbb2_vctr22::R](mpcbb2_vctr22::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR22 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr22::W](mpcbb2_vctr22::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR22 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr22;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr23](mpcbb2_vctr23) module"]
pub type MPCBB2_VCTR23 = crate::Reg<u32, _MPCBB2_VCTR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR23;
#[doc = "`read()` method returns [mpcbb2_vctr23::R](mpcbb2_vctr23::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR23 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr23::W](mpcbb2_vctr23::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR23 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr23;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr24](mpcbb2_vctr24) module"]
pub type MPCBB2_VCTR24 = crate::Reg<u32, _MPCBB2_VCTR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR24;
#[doc = "`read()` method returns [mpcbb2_vctr24::R](mpcbb2_vctr24::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR24 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr24::W](mpcbb2_vctr24::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR24 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr24;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr25](mpcbb2_vctr25) module"]
pub type MPCBB2_VCTR25 = crate::Reg<u32, _MPCBB2_VCTR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR25;
#[doc = "`read()` method returns [mpcbb2_vctr25::R](mpcbb2_vctr25::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR25 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr25::W](mpcbb2_vctr25::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR25 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr25;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr26](mpcbb2_vctr26) module"]
pub type MPCBB2_VCTR26 = crate::Reg<u32, _MPCBB2_VCTR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR26;
#[doc = "`read()` method returns [mpcbb2_vctr26::R](mpcbb2_vctr26::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR26 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr26::W](mpcbb2_vctr26::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR26 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr26;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr27](mpcbb2_vctr27) module"]
pub type MPCBB2_VCTR27 = crate::Reg<u32, _MPCBB2_VCTR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR27;
#[doc = "`read()` method returns [mpcbb2_vctr27::R](mpcbb2_vctr27::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR27 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr27::W](mpcbb2_vctr27::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR27 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr27;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr28](mpcbb2_vctr28) module"]
pub type MPCBB2_VCTR28 = crate::Reg<u32, _MPCBB2_VCTR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR28;
#[doc = "`read()` method returns [mpcbb2_vctr28::R](mpcbb2_vctr28::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR28 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr28::W](mpcbb2_vctr28::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR28 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr28;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr29](mpcbb2_vctr29) module"]
pub type MPCBB2_VCTR29 = crate::Reg<u32, _MPCBB2_VCTR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR29;
#[doc = "`read()` method returns [mpcbb2_vctr29::R](mpcbb2_vctr29::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR29 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr29::W](mpcbb2_vctr29::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR29 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr29;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr30](mpcbb2_vctr30) module"]
pub type MPCBB2_VCTR30 = crate::Reg<u32, _MPCBB2_VCTR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR30;
#[doc = "`read()` method returns [mpcbb2_vctr30::R](mpcbb2_vctr30::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR30 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr30::W](mpcbb2_vctr30::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR30 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr30;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr31](mpcbb2_vctr31) module"]
pub type MPCBB2_VCTR31 = crate::Reg<u32, _MPCBB2_VCTR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR31;
#[doc = "`read()` method returns [mpcbb2_vctr31::R](mpcbb2_vctr31::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR31 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr31::W](mpcbb2_vctr31::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR31 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr31;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr32](mpcbb2_vctr32) module"]
pub type MPCBB2_VCTR32 = crate::Reg<u32, _MPCBB2_VCTR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR32;
#[doc = "`read()` method returns [mpcbb2_vctr32::R](mpcbb2_vctr32::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR32 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr32::W](mpcbb2_vctr32::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR32 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr32;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr33](mpcbb2_vctr33) module"]
pub type MPCBB2_VCTR33 = crate::Reg<u32, _MPCBB2_VCTR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR33;
#[doc = "`read()` method returns [mpcbb2_vctr33::R](mpcbb2_vctr33::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR33 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr33::W](mpcbb2_vctr33::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR33 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr33;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr34](mpcbb2_vctr34) module"]
pub type MPCBB2_VCTR34 = crate::Reg<u32, _MPCBB2_VCTR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR34;
#[doc = "`read()` method returns [mpcbb2_vctr34::R](mpcbb2_vctr34::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR34 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr34::W](mpcbb2_vctr34::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR34 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr34;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr35](mpcbb2_vctr35) module"]
pub type MPCBB2_VCTR35 = crate::Reg<u32, _MPCBB2_VCTR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR35;
#[doc = "`read()` method returns [mpcbb2_vctr35::R](mpcbb2_vctr35::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR35 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr35::W](mpcbb2_vctr35::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR35 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr35;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr36](mpcbb2_vctr36) module"]
pub type MPCBB2_VCTR36 = crate::Reg<u32, _MPCBB2_VCTR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR36;
#[doc = "`read()` method returns [mpcbb2_vctr36::R](mpcbb2_vctr36::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR36 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr36::W](mpcbb2_vctr36::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR36 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr36;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr37](mpcbb2_vctr37) module"]
pub type MPCBB2_VCTR37 = crate::Reg<u32, _MPCBB2_VCTR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR37;
#[doc = "`read()` method returns [mpcbb2_vctr37::R](mpcbb2_vctr37::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR37 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr37::W](mpcbb2_vctr37::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR37 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr37;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr38](mpcbb2_vctr38) module"]
pub type MPCBB2_VCTR38 = crate::Reg<u32, _MPCBB2_VCTR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR38;
#[doc = "`read()` method returns [mpcbb2_vctr38::R](mpcbb2_vctr38::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR38 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr38::W](mpcbb2_vctr38::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR38 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr38;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr39](mpcbb2_vctr39) module"]
pub type MPCBB2_VCTR39 = crate::Reg<u32, _MPCBB2_VCTR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR39;
#[doc = "`read()` method returns [mpcbb2_vctr39::R](mpcbb2_vctr39::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR39 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr39::W](mpcbb2_vctr39::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR39 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr39;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr40](mpcbb2_vctr40) module"]
pub type MPCBB2_VCTR40 = crate::Reg<u32, _MPCBB2_VCTR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR40;
#[doc = "`read()` method returns [mpcbb2_vctr40::R](mpcbb2_vctr40::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR40 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr40::W](mpcbb2_vctr40::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR40 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr40;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr41](mpcbb2_vctr41) module"]
pub type MPCBB2_VCTR41 = crate::Reg<u32, _MPCBB2_VCTR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR41;
#[doc = "`read()` method returns [mpcbb2_vctr41::R](mpcbb2_vctr41::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR41 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr41::W](mpcbb2_vctr41::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR41 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr41;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr42](mpcbb2_vctr42) module"]
pub type MPCBB2_VCTR42 = crate::Reg<u32, _MPCBB2_VCTR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR42;
#[doc = "`read()` method returns [mpcbb2_vctr42::R](mpcbb2_vctr42::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR42 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr42::W](mpcbb2_vctr42::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR42 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr42;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr43](mpcbb2_vctr43) module"]
pub type MPCBB2_VCTR43 = crate::Reg<u32, _MPCBB2_VCTR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR43;
#[doc = "`read()` method returns [mpcbb2_vctr43::R](mpcbb2_vctr43::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR43 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr43::W](mpcbb2_vctr43::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR43 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr43;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr44](mpcbb2_vctr44) module"]
pub type MPCBB2_VCTR44 = crate::Reg<u32, _MPCBB2_VCTR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR44;
#[doc = "`read()` method returns [mpcbb2_vctr44::R](mpcbb2_vctr44::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR44 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr44::W](mpcbb2_vctr44::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR44 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr44;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr45](mpcbb2_vctr45) module"]
pub type MPCBB2_VCTR45 = crate::Reg<u32, _MPCBB2_VCTR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR45;
#[doc = "`read()` method returns [mpcbb2_vctr45::R](mpcbb2_vctr45::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR45 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr45::W](mpcbb2_vctr45::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR45 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr45;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr46](mpcbb2_vctr46) module"]
pub type MPCBB2_VCTR46 = crate::Reg<u32, _MPCBB2_VCTR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR46;
#[doc = "`read()` method returns [mpcbb2_vctr46::R](mpcbb2_vctr46::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR46 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr46::W](mpcbb2_vctr46::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR46 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr46;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr47](mpcbb2_vctr47) module"]
pub type MPCBB2_VCTR47 = crate::Reg<u32, _MPCBB2_VCTR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR47;
#[doc = "`read()` method returns [mpcbb2_vctr47::R](mpcbb2_vctr47::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR47 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr47::W](mpcbb2_vctr47::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR47 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr47;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr48](mpcbb2_vctr48) module"]
pub type MPCBB2_VCTR48 = crate::Reg<u32, _MPCBB2_VCTR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR48;
#[doc = "`read()` method returns [mpcbb2_vctr48::R](mpcbb2_vctr48::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR48 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr48::W](mpcbb2_vctr48::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR48 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr48;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr49](mpcbb2_vctr49) module"]
pub type MPCBB2_VCTR49 = crate::Reg<u32, _MPCBB2_VCTR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR49;
#[doc = "`read()` method returns [mpcbb2_vctr49::R](mpcbb2_vctr49::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR49 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr49::W](mpcbb2_vctr49::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR49 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr49;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr50](mpcbb2_vctr50) module"]
pub type MPCBB2_VCTR50 = crate::Reg<u32, _MPCBB2_VCTR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR50;
#[doc = "`read()` method returns [mpcbb2_vctr50::R](mpcbb2_vctr50::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR50 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr50::W](mpcbb2_vctr50::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR50 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr50;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr51](mpcbb2_vctr51) module"]
pub type MPCBB2_VCTR51 = crate::Reg<u32, _MPCBB2_VCTR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR51;
#[doc = "`read()` method returns [mpcbb2_vctr51::R](mpcbb2_vctr51::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR51 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr51::W](mpcbb2_vctr51::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR51 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr51;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr52](mpcbb2_vctr52) module"]
pub type MPCBB2_VCTR52 = crate::Reg<u32, _MPCBB2_VCTR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR52;
#[doc = "`read()` method returns [mpcbb2_vctr52::R](mpcbb2_vctr52::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR52 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr52::W](mpcbb2_vctr52::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR52 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr52;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr53](mpcbb2_vctr53) module"]
pub type MPCBB2_VCTR53 = crate::Reg<u32, _MPCBB2_VCTR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR53;
#[doc = "`read()` method returns [mpcbb2_vctr53::R](mpcbb2_vctr53::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR53 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr53::W](mpcbb2_vctr53::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR53 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr53;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr54](mpcbb2_vctr54) module"]
pub type MPCBB2_VCTR54 = crate::Reg<u32, _MPCBB2_VCTR54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR54;
#[doc = "`read()` method returns [mpcbb2_vctr54::R](mpcbb2_vctr54::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR54 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr54::W](mpcbb2_vctr54::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR54 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr54;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr55](mpcbb2_vctr55) module"]
pub type MPCBB2_VCTR55 = crate::Reg<u32, _MPCBB2_VCTR55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR55;
#[doc = "`read()` method returns [mpcbb2_vctr55::R](mpcbb2_vctr55::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR55 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr55::W](mpcbb2_vctr55::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR55 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr55;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr56](mpcbb2_vctr56) module"]
pub type MPCBB2_VCTR56 = crate::Reg<u32, _MPCBB2_VCTR56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR56;
#[doc = "`read()` method returns [mpcbb2_vctr56::R](mpcbb2_vctr56::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR56 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr56::W](mpcbb2_vctr56::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR56 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr56;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr57](mpcbb2_vctr57) module"]
pub type MPCBB2_VCTR57 = crate::Reg<u32, _MPCBB2_VCTR57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR57;
#[doc = "`read()` method returns [mpcbb2_vctr57::R](mpcbb2_vctr57::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR57 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr57::W](mpcbb2_vctr57::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR57 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr57;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr58](mpcbb2_vctr58) module"]
pub type MPCBB2_VCTR58 = crate::Reg<u32, _MPCBB2_VCTR58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR58;
#[doc = "`read()` method returns [mpcbb2_vctr58::R](mpcbb2_vctr58::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR58 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr58::W](mpcbb2_vctr58::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR58 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr58;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr59](mpcbb2_vctr59) module"]
pub type MPCBB2_VCTR59 = crate::Reg<u32, _MPCBB2_VCTR59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR59;
#[doc = "`read()` method returns [mpcbb2_vctr59::R](mpcbb2_vctr59::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR59 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr59::W](mpcbb2_vctr59::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR59 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr59;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr60](mpcbb2_vctr60) module"]
pub type MPCBB2_VCTR60 = crate::Reg<u32, _MPCBB2_VCTR60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR60;
#[doc = "`read()` method returns [mpcbb2_vctr60::R](mpcbb2_vctr60::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR60 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr60::W](mpcbb2_vctr60::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR60 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr60;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr61](mpcbb2_vctr61) module"]
pub type MPCBB2_VCTR61 = crate::Reg<u32, _MPCBB2_VCTR61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR61;
#[doc = "`read()` method returns [mpcbb2_vctr61::R](mpcbb2_vctr61::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR61 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr61::W](mpcbb2_vctr61::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR61 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr61;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr62](mpcbb2_vctr62) module"]
pub type MPCBB2_VCTR62 = crate::Reg<u32, _MPCBB2_VCTR62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR62;
#[doc = "`read()` method returns [mpcbb2_vctr62::R](mpcbb2_vctr62::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR62 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr62::W](mpcbb2_vctr62::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR62 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr62;
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr63](mpcbb2_vctr63) module"]
pub type MPCBB2_VCTR63 = crate::Reg<u32, _MPCBB2_VCTR63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPCBB2_VCTR63;
#[doc = "`read()` method returns [mpcbb2_vctr63::R](mpcbb2_vctr63::R) reader structure"]
impl crate::Readable for MPCBB2_VCTR63 {}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr63::W](mpcbb2_vctr63::W) writer structure"]
impl crate::Writable for MPCBB2_VCTR63 {}
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr63;
