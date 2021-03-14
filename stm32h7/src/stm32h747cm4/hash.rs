#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - data input register"]
    pub din: DIN,
    #[doc = "0x08 - start register"]
    pub str: STR,
    #[doc = "0x0c - digest registers"]
    pub hr0: HR0,
    #[doc = "0x10 - digest registers"]
    pub hr1: HR1,
    #[doc = "0x14 - digest registers"]
    pub hr2: HR2,
    #[doc = "0x18 - digest registers"]
    pub hr3: HR3,
    #[doc = "0x1c - digest registers"]
    pub hr4: HR4,
    #[doc = "0x20 - interrupt enable register"]
    pub imr: IMR,
    #[doc = "0x24 - status register"]
    pub sr: SR,
    _reserved10: [u8; 208usize],
    #[doc = "0xf8 - context swap registers"]
    pub csr0: CSR0,
    #[doc = "0xfc - context swap registers"]
    pub csr1: CSR1,
    #[doc = "0x100 - context swap registers"]
    pub csr2: CSR2,
    #[doc = "0x104 - context swap registers"]
    pub csr3: CSR3,
    #[doc = "0x108 - context swap registers"]
    pub csr4: CSR4,
    #[doc = "0x10c - context swap registers"]
    pub csr5: CSR5,
    #[doc = "0x110 - context swap registers"]
    pub csr6: CSR6,
    #[doc = "0x114 - context swap registers"]
    pub csr7: CSR7,
    #[doc = "0x118 - context swap registers"]
    pub csr8: CSR8,
    #[doc = "0x11c - context swap registers"]
    pub csr9: CSR9,
    #[doc = "0x120 - context swap registers"]
    pub csr10: CSR10,
    #[doc = "0x124 - context swap registers"]
    pub csr11: CSR11,
    #[doc = "0x128 - context swap registers"]
    pub csr12: CSR12,
    #[doc = "0x12c - context swap registers"]
    pub csr13: CSR13,
    #[doc = "0x130 - context swap registers"]
    pub csr14: CSR14,
    #[doc = "0x134 - context swap registers"]
    pub csr15: CSR15,
    #[doc = "0x138 - context swap registers"]
    pub csr16: CSR16,
    #[doc = "0x13c - context swap registers"]
    pub csr17: CSR17,
    #[doc = "0x140 - context swap registers"]
    pub csr18: CSR18,
    #[doc = "0x144 - context swap registers"]
    pub csr19: CSR19,
    #[doc = "0x148 - context swap registers"]
    pub csr20: CSR20,
    #[doc = "0x14c - context swap registers"]
    pub csr21: CSR21,
    #[doc = "0x150 - context swap registers"]
    pub csr22: CSR22,
    #[doc = "0x154 - context swap registers"]
    pub csr23: CSR23,
    #[doc = "0x158 - context swap registers"]
    pub csr24: CSR24,
    #[doc = "0x15c - context swap registers"]
    pub csr25: CSR25,
    #[doc = "0x160 - context swap registers"]
    pub csr26: CSR26,
    #[doc = "0x164 - context swap registers"]
    pub csr27: CSR27,
    #[doc = "0x168 - context swap registers"]
    pub csr28: CSR28,
    #[doc = "0x16c - context swap registers"]
    pub csr29: CSR29,
    #[doc = "0x170 - context swap registers"]
    pub csr30: CSR30,
    #[doc = "0x174 - context swap registers"]
    pub csr31: CSR31,
    #[doc = "0x178 - context swap registers"]
    pub csr32: CSR32,
    #[doc = "0x17c - context swap registers"]
    pub csr33: CSR33,
    #[doc = "0x180 - context swap registers"]
    pub csr34: CSR34,
    #[doc = "0x184 - context swap registers"]
    pub csr35: CSR35,
    #[doc = "0x188 - context swap registers"]
    pub csr36: CSR36,
    #[doc = "0x18c - context swap registers"]
    pub csr37: CSR37,
    #[doc = "0x190 - context swap registers"]
    pub csr38: CSR38,
    #[doc = "0x194 - context swap registers"]
    pub csr39: CSR39,
    #[doc = "0x198 - context swap registers"]
    pub csr40: CSR40,
    #[doc = "0x19c - context swap registers"]
    pub csr41: CSR41,
    #[doc = "0x1a0 - context swap registers"]
    pub csr42: CSR42,
    #[doc = "0x1a4 - context swap registers"]
    pub csr43: CSR43,
    #[doc = "0x1a8 - context swap registers"]
    pub csr44: CSR44,
    #[doc = "0x1ac - context swap registers"]
    pub csr45: CSR45,
    #[doc = "0x1b0 - context swap registers"]
    pub csr46: CSR46,
    #[doc = "0x1b4 - context swap registers"]
    pub csr47: CSR47,
    #[doc = "0x1b8 - context swap registers"]
    pub csr48: CSR48,
    #[doc = "0x1bc - context swap registers"]
    pub csr49: CSR49,
    #[doc = "0x1c0 - context swap registers"]
    pub csr50: CSR50,
    #[doc = "0x1c4 - context swap registers"]
    pub csr51: CSR51,
    #[doc = "0x1c8 - context swap registers"]
    pub csr52: CSR52,
    #[doc = "0x1cc - context swap registers"]
    pub csr53: CSR53,
    _reserved64: [u8; 320usize],
    #[doc = "0x310 - HASH digest register"]
    pub hash_hr0: HASH_HR0,
    #[doc = "0x314 - read-only"]
    pub hash_hr1: HASH_HR1,
    #[doc = "0x318 - read-only"]
    pub hash_hr2: HASH_HR2,
    #[doc = "0x31c - read-only"]
    pub hash_hr3: HASH_HR3,
    #[doc = "0x320 - read-only"]
    pub hash_hr4: HASH_HR4,
    #[doc = "0x324 - read-only"]
    pub hash_hr5: HASH_HR5,
    #[doc = "0x328 - read-only"]
    pub hash_hr6: HASH_HR6,
    #[doc = "0x32c - read-only"]
    pub hash_hr7: HASH_HR7,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din](din) module"]
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
#[doc = "`read()` method returns [din::R](din::R) reader structure"]
impl crate::Readable for DIN {}
#[doc = "`write(|w| ..)` method takes [din::W](din::W) writer structure"]
impl crate::Writable for DIN {}
#[doc = "data input register"]
pub mod din;
#[doc = "start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](str) module"]
pub type STR = crate::Reg<u32, _STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STR;
#[doc = "`read()` method returns [str::R](str::R) reader structure"]
impl crate::Readable for STR {}
#[doc = "`write(|w| ..)` method takes [str::W](str::W) writer structure"]
impl crate::Writable for STR {}
#[doc = "start register"]
pub mod str;
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr0](hr0) module"]
pub type HR0 = crate::Reg<u32, _HR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR0;
#[doc = "`read()` method returns [hr0::R](hr0::R) reader structure"]
impl crate::Readable for HR0 {}
#[doc = "digest registers"]
pub mod hr0;
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr1](hr1) module"]
pub type HR1 = crate::Reg<u32, _HR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR1;
#[doc = "`read()` method returns [hr1::R](hr1::R) reader structure"]
impl crate::Readable for HR1 {}
#[doc = "digest registers"]
pub mod hr1;
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr2](hr2) module"]
pub type HR2 = crate::Reg<u32, _HR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR2;
#[doc = "`read()` method returns [hr2::R](hr2::R) reader structure"]
impl crate::Readable for HR2 {}
#[doc = "digest registers"]
pub mod hr2;
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr3](hr3) module"]
pub type HR3 = crate::Reg<u32, _HR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR3;
#[doc = "`read()` method returns [hr3::R](hr3::R) reader structure"]
impl crate::Readable for HR3 {}
#[doc = "digest registers"]
pub mod hr3;
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr4](hr4) module"]
pub type HR4 = crate::Reg<u32, _HR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HR4;
#[doc = "`read()` method returns [hr4::R](hr4::R) reader structure"]
impl crate::Readable for HR4 {}
#[doc = "digest registers"]
pub mod hr4;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr0](csr0) module"]
pub type CSR0 = crate::Reg<u32, _CSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR0;
#[doc = "`read()` method returns [csr0::R](csr0::R) reader structure"]
impl crate::Readable for CSR0 {}
#[doc = "`write(|w| ..)` method takes [csr0::W](csr0::W) writer structure"]
impl crate::Writable for CSR0 {}
#[doc = "context swap registers"]
pub mod csr0;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr1](csr1) module"]
pub type CSR1 = crate::Reg<u32, _CSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR1;
#[doc = "`read()` method returns [csr1::R](csr1::R) reader structure"]
impl crate::Readable for CSR1 {}
#[doc = "`write(|w| ..)` method takes [csr1::W](csr1::W) writer structure"]
impl crate::Writable for CSR1 {}
#[doc = "context swap registers"]
pub mod csr1;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr2](csr2) module"]
pub type CSR2 = crate::Reg<u32, _CSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR2;
#[doc = "`read()` method returns [csr2::R](csr2::R) reader structure"]
impl crate::Readable for CSR2 {}
#[doc = "`write(|w| ..)` method takes [csr2::W](csr2::W) writer structure"]
impl crate::Writable for CSR2 {}
#[doc = "context swap registers"]
pub mod csr2;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr3](csr3) module"]
pub type CSR3 = crate::Reg<u32, _CSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR3;
#[doc = "`read()` method returns [csr3::R](csr3::R) reader structure"]
impl crate::Readable for CSR3 {}
#[doc = "`write(|w| ..)` method takes [csr3::W](csr3::W) writer structure"]
impl crate::Writable for CSR3 {}
#[doc = "context swap registers"]
pub mod csr3;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr4](csr4) module"]
pub type CSR4 = crate::Reg<u32, _CSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR4;
#[doc = "`read()` method returns [csr4::R](csr4::R) reader structure"]
impl crate::Readable for CSR4 {}
#[doc = "`write(|w| ..)` method takes [csr4::W](csr4::W) writer structure"]
impl crate::Writable for CSR4 {}
#[doc = "context swap registers"]
pub mod csr4;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr5](csr5) module"]
pub type CSR5 = crate::Reg<u32, _CSR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR5;
#[doc = "`read()` method returns [csr5::R](csr5::R) reader structure"]
impl crate::Readable for CSR5 {}
#[doc = "`write(|w| ..)` method takes [csr5::W](csr5::W) writer structure"]
impl crate::Writable for CSR5 {}
#[doc = "context swap registers"]
pub mod csr5;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr6](csr6) module"]
pub type CSR6 = crate::Reg<u32, _CSR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR6;
#[doc = "`read()` method returns [csr6::R](csr6::R) reader structure"]
impl crate::Readable for CSR6 {}
#[doc = "`write(|w| ..)` method takes [csr6::W](csr6::W) writer structure"]
impl crate::Writable for CSR6 {}
#[doc = "context swap registers"]
pub mod csr6;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr7](csr7) module"]
pub type CSR7 = crate::Reg<u32, _CSR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR7;
#[doc = "`read()` method returns [csr7::R](csr7::R) reader structure"]
impl crate::Readable for CSR7 {}
#[doc = "`write(|w| ..)` method takes [csr7::W](csr7::W) writer structure"]
impl crate::Writable for CSR7 {}
#[doc = "context swap registers"]
pub mod csr7;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr8](csr8) module"]
pub type CSR8 = crate::Reg<u32, _CSR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR8;
#[doc = "`read()` method returns [csr8::R](csr8::R) reader structure"]
impl crate::Readable for CSR8 {}
#[doc = "`write(|w| ..)` method takes [csr8::W](csr8::W) writer structure"]
impl crate::Writable for CSR8 {}
#[doc = "context swap registers"]
pub mod csr8;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr9](csr9) module"]
pub type CSR9 = crate::Reg<u32, _CSR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR9;
#[doc = "`read()` method returns [csr9::R](csr9::R) reader structure"]
impl crate::Readable for CSR9 {}
#[doc = "`write(|w| ..)` method takes [csr9::W](csr9::W) writer structure"]
impl crate::Writable for CSR9 {}
#[doc = "context swap registers"]
pub mod csr9;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr10](csr10) module"]
pub type CSR10 = crate::Reg<u32, _CSR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR10;
#[doc = "`read()` method returns [csr10::R](csr10::R) reader structure"]
impl crate::Readable for CSR10 {}
#[doc = "`write(|w| ..)` method takes [csr10::W](csr10::W) writer structure"]
impl crate::Writable for CSR10 {}
#[doc = "context swap registers"]
pub mod csr10;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr11](csr11) module"]
pub type CSR11 = crate::Reg<u32, _CSR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR11;
#[doc = "`read()` method returns [csr11::R](csr11::R) reader structure"]
impl crate::Readable for CSR11 {}
#[doc = "`write(|w| ..)` method takes [csr11::W](csr11::W) writer structure"]
impl crate::Writable for CSR11 {}
#[doc = "context swap registers"]
pub mod csr11;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr12](csr12) module"]
pub type CSR12 = crate::Reg<u32, _CSR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR12;
#[doc = "`read()` method returns [csr12::R](csr12::R) reader structure"]
impl crate::Readable for CSR12 {}
#[doc = "`write(|w| ..)` method takes [csr12::W](csr12::W) writer structure"]
impl crate::Writable for CSR12 {}
#[doc = "context swap registers"]
pub mod csr12;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr13](csr13) module"]
pub type CSR13 = crate::Reg<u32, _CSR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR13;
#[doc = "`read()` method returns [csr13::R](csr13::R) reader structure"]
impl crate::Readable for CSR13 {}
#[doc = "`write(|w| ..)` method takes [csr13::W](csr13::W) writer structure"]
impl crate::Writable for CSR13 {}
#[doc = "context swap registers"]
pub mod csr13;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr14](csr14) module"]
pub type CSR14 = crate::Reg<u32, _CSR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR14;
#[doc = "`read()` method returns [csr14::R](csr14::R) reader structure"]
impl crate::Readable for CSR14 {}
#[doc = "`write(|w| ..)` method takes [csr14::W](csr14::W) writer structure"]
impl crate::Writable for CSR14 {}
#[doc = "context swap registers"]
pub mod csr14;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr15](csr15) module"]
pub type CSR15 = crate::Reg<u32, _CSR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR15;
#[doc = "`read()` method returns [csr15::R](csr15::R) reader structure"]
impl crate::Readable for CSR15 {}
#[doc = "`write(|w| ..)` method takes [csr15::W](csr15::W) writer structure"]
impl crate::Writable for CSR15 {}
#[doc = "context swap registers"]
pub mod csr15;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr16](csr16) module"]
pub type CSR16 = crate::Reg<u32, _CSR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR16;
#[doc = "`read()` method returns [csr16::R](csr16::R) reader structure"]
impl crate::Readable for CSR16 {}
#[doc = "`write(|w| ..)` method takes [csr16::W](csr16::W) writer structure"]
impl crate::Writable for CSR16 {}
#[doc = "context swap registers"]
pub mod csr16;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr17](csr17) module"]
pub type CSR17 = crate::Reg<u32, _CSR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR17;
#[doc = "`read()` method returns [csr17::R](csr17::R) reader structure"]
impl crate::Readable for CSR17 {}
#[doc = "`write(|w| ..)` method takes [csr17::W](csr17::W) writer structure"]
impl crate::Writable for CSR17 {}
#[doc = "context swap registers"]
pub mod csr17;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr18](csr18) module"]
pub type CSR18 = crate::Reg<u32, _CSR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR18;
#[doc = "`read()` method returns [csr18::R](csr18::R) reader structure"]
impl crate::Readable for CSR18 {}
#[doc = "`write(|w| ..)` method takes [csr18::W](csr18::W) writer structure"]
impl crate::Writable for CSR18 {}
#[doc = "context swap registers"]
pub mod csr18;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr19](csr19) module"]
pub type CSR19 = crate::Reg<u32, _CSR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR19;
#[doc = "`read()` method returns [csr19::R](csr19::R) reader structure"]
impl crate::Readable for CSR19 {}
#[doc = "`write(|w| ..)` method takes [csr19::W](csr19::W) writer structure"]
impl crate::Writable for CSR19 {}
#[doc = "context swap registers"]
pub mod csr19;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr20](csr20) module"]
pub type CSR20 = crate::Reg<u32, _CSR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR20;
#[doc = "`read()` method returns [csr20::R](csr20::R) reader structure"]
impl crate::Readable for CSR20 {}
#[doc = "`write(|w| ..)` method takes [csr20::W](csr20::W) writer structure"]
impl crate::Writable for CSR20 {}
#[doc = "context swap registers"]
pub mod csr20;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr21](csr21) module"]
pub type CSR21 = crate::Reg<u32, _CSR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR21;
#[doc = "`read()` method returns [csr21::R](csr21::R) reader structure"]
impl crate::Readable for CSR21 {}
#[doc = "`write(|w| ..)` method takes [csr21::W](csr21::W) writer structure"]
impl crate::Writable for CSR21 {}
#[doc = "context swap registers"]
pub mod csr21;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr22](csr22) module"]
pub type CSR22 = crate::Reg<u32, _CSR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR22;
#[doc = "`read()` method returns [csr22::R](csr22::R) reader structure"]
impl crate::Readable for CSR22 {}
#[doc = "`write(|w| ..)` method takes [csr22::W](csr22::W) writer structure"]
impl crate::Writable for CSR22 {}
#[doc = "context swap registers"]
pub mod csr22;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr23](csr23) module"]
pub type CSR23 = crate::Reg<u32, _CSR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR23;
#[doc = "`read()` method returns [csr23::R](csr23::R) reader structure"]
impl crate::Readable for CSR23 {}
#[doc = "`write(|w| ..)` method takes [csr23::W](csr23::W) writer structure"]
impl crate::Writable for CSR23 {}
#[doc = "context swap registers"]
pub mod csr23;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr24](csr24) module"]
pub type CSR24 = crate::Reg<u32, _CSR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR24;
#[doc = "`read()` method returns [csr24::R](csr24::R) reader structure"]
impl crate::Readable for CSR24 {}
#[doc = "`write(|w| ..)` method takes [csr24::W](csr24::W) writer structure"]
impl crate::Writable for CSR24 {}
#[doc = "context swap registers"]
pub mod csr24;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr25](csr25) module"]
pub type CSR25 = crate::Reg<u32, _CSR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR25;
#[doc = "`read()` method returns [csr25::R](csr25::R) reader structure"]
impl crate::Readable for CSR25 {}
#[doc = "`write(|w| ..)` method takes [csr25::W](csr25::W) writer structure"]
impl crate::Writable for CSR25 {}
#[doc = "context swap registers"]
pub mod csr25;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr26](csr26) module"]
pub type CSR26 = crate::Reg<u32, _CSR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR26;
#[doc = "`read()` method returns [csr26::R](csr26::R) reader structure"]
impl crate::Readable for CSR26 {}
#[doc = "`write(|w| ..)` method takes [csr26::W](csr26::W) writer structure"]
impl crate::Writable for CSR26 {}
#[doc = "context swap registers"]
pub mod csr26;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr27](csr27) module"]
pub type CSR27 = crate::Reg<u32, _CSR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR27;
#[doc = "`read()` method returns [csr27::R](csr27::R) reader structure"]
impl crate::Readable for CSR27 {}
#[doc = "`write(|w| ..)` method takes [csr27::W](csr27::W) writer structure"]
impl crate::Writable for CSR27 {}
#[doc = "context swap registers"]
pub mod csr27;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr28](csr28) module"]
pub type CSR28 = crate::Reg<u32, _CSR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR28;
#[doc = "`read()` method returns [csr28::R](csr28::R) reader structure"]
impl crate::Readable for CSR28 {}
#[doc = "`write(|w| ..)` method takes [csr28::W](csr28::W) writer structure"]
impl crate::Writable for CSR28 {}
#[doc = "context swap registers"]
pub mod csr28;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr29](csr29) module"]
pub type CSR29 = crate::Reg<u32, _CSR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR29;
#[doc = "`read()` method returns [csr29::R](csr29::R) reader structure"]
impl crate::Readable for CSR29 {}
#[doc = "`write(|w| ..)` method takes [csr29::W](csr29::W) writer structure"]
impl crate::Writable for CSR29 {}
#[doc = "context swap registers"]
pub mod csr29;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr30](csr30) module"]
pub type CSR30 = crate::Reg<u32, _CSR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR30;
#[doc = "`read()` method returns [csr30::R](csr30::R) reader structure"]
impl crate::Readable for CSR30 {}
#[doc = "`write(|w| ..)` method takes [csr30::W](csr30::W) writer structure"]
impl crate::Writable for CSR30 {}
#[doc = "context swap registers"]
pub mod csr30;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr31](csr31) module"]
pub type CSR31 = crate::Reg<u32, _CSR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR31;
#[doc = "`read()` method returns [csr31::R](csr31::R) reader structure"]
impl crate::Readable for CSR31 {}
#[doc = "`write(|w| ..)` method takes [csr31::W](csr31::W) writer structure"]
impl crate::Writable for CSR31 {}
#[doc = "context swap registers"]
pub mod csr31;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr32](csr32) module"]
pub type CSR32 = crate::Reg<u32, _CSR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR32;
#[doc = "`read()` method returns [csr32::R](csr32::R) reader structure"]
impl crate::Readable for CSR32 {}
#[doc = "`write(|w| ..)` method takes [csr32::W](csr32::W) writer structure"]
impl crate::Writable for CSR32 {}
#[doc = "context swap registers"]
pub mod csr32;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr33](csr33) module"]
pub type CSR33 = crate::Reg<u32, _CSR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR33;
#[doc = "`read()` method returns [csr33::R](csr33::R) reader structure"]
impl crate::Readable for CSR33 {}
#[doc = "`write(|w| ..)` method takes [csr33::W](csr33::W) writer structure"]
impl crate::Writable for CSR33 {}
#[doc = "context swap registers"]
pub mod csr33;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr34](csr34) module"]
pub type CSR34 = crate::Reg<u32, _CSR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR34;
#[doc = "`read()` method returns [csr34::R](csr34::R) reader structure"]
impl crate::Readable for CSR34 {}
#[doc = "`write(|w| ..)` method takes [csr34::W](csr34::W) writer structure"]
impl crate::Writable for CSR34 {}
#[doc = "context swap registers"]
pub mod csr34;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr35](csr35) module"]
pub type CSR35 = crate::Reg<u32, _CSR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR35;
#[doc = "`read()` method returns [csr35::R](csr35::R) reader structure"]
impl crate::Readable for CSR35 {}
#[doc = "`write(|w| ..)` method takes [csr35::W](csr35::W) writer structure"]
impl crate::Writable for CSR35 {}
#[doc = "context swap registers"]
pub mod csr35;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr36](csr36) module"]
pub type CSR36 = crate::Reg<u32, _CSR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR36;
#[doc = "`read()` method returns [csr36::R](csr36::R) reader structure"]
impl crate::Readable for CSR36 {}
#[doc = "`write(|w| ..)` method takes [csr36::W](csr36::W) writer structure"]
impl crate::Writable for CSR36 {}
#[doc = "context swap registers"]
pub mod csr36;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr37](csr37) module"]
pub type CSR37 = crate::Reg<u32, _CSR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR37;
#[doc = "`read()` method returns [csr37::R](csr37::R) reader structure"]
impl crate::Readable for CSR37 {}
#[doc = "`write(|w| ..)` method takes [csr37::W](csr37::W) writer structure"]
impl crate::Writable for CSR37 {}
#[doc = "context swap registers"]
pub mod csr37;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr38](csr38) module"]
pub type CSR38 = crate::Reg<u32, _CSR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR38;
#[doc = "`read()` method returns [csr38::R](csr38::R) reader structure"]
impl crate::Readable for CSR38 {}
#[doc = "`write(|w| ..)` method takes [csr38::W](csr38::W) writer structure"]
impl crate::Writable for CSR38 {}
#[doc = "context swap registers"]
pub mod csr38;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr39](csr39) module"]
pub type CSR39 = crate::Reg<u32, _CSR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR39;
#[doc = "`read()` method returns [csr39::R](csr39::R) reader structure"]
impl crate::Readable for CSR39 {}
#[doc = "`write(|w| ..)` method takes [csr39::W](csr39::W) writer structure"]
impl crate::Writable for CSR39 {}
#[doc = "context swap registers"]
pub mod csr39;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr40](csr40) module"]
pub type CSR40 = crate::Reg<u32, _CSR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR40;
#[doc = "`read()` method returns [csr40::R](csr40::R) reader structure"]
impl crate::Readable for CSR40 {}
#[doc = "`write(|w| ..)` method takes [csr40::W](csr40::W) writer structure"]
impl crate::Writable for CSR40 {}
#[doc = "context swap registers"]
pub mod csr40;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr41](csr41) module"]
pub type CSR41 = crate::Reg<u32, _CSR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR41;
#[doc = "`read()` method returns [csr41::R](csr41::R) reader structure"]
impl crate::Readable for CSR41 {}
#[doc = "`write(|w| ..)` method takes [csr41::W](csr41::W) writer structure"]
impl crate::Writable for CSR41 {}
#[doc = "context swap registers"]
pub mod csr41;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr42](csr42) module"]
pub type CSR42 = crate::Reg<u32, _CSR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR42;
#[doc = "`read()` method returns [csr42::R](csr42::R) reader structure"]
impl crate::Readable for CSR42 {}
#[doc = "`write(|w| ..)` method takes [csr42::W](csr42::W) writer structure"]
impl crate::Writable for CSR42 {}
#[doc = "context swap registers"]
pub mod csr42;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr43](csr43) module"]
pub type CSR43 = crate::Reg<u32, _CSR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR43;
#[doc = "`read()` method returns [csr43::R](csr43::R) reader structure"]
impl crate::Readable for CSR43 {}
#[doc = "`write(|w| ..)` method takes [csr43::W](csr43::W) writer structure"]
impl crate::Writable for CSR43 {}
#[doc = "context swap registers"]
pub mod csr43;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr44](csr44) module"]
pub type CSR44 = crate::Reg<u32, _CSR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR44;
#[doc = "`read()` method returns [csr44::R](csr44::R) reader structure"]
impl crate::Readable for CSR44 {}
#[doc = "`write(|w| ..)` method takes [csr44::W](csr44::W) writer structure"]
impl crate::Writable for CSR44 {}
#[doc = "context swap registers"]
pub mod csr44;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr45](csr45) module"]
pub type CSR45 = crate::Reg<u32, _CSR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR45;
#[doc = "`read()` method returns [csr45::R](csr45::R) reader structure"]
impl crate::Readable for CSR45 {}
#[doc = "`write(|w| ..)` method takes [csr45::W](csr45::W) writer structure"]
impl crate::Writable for CSR45 {}
#[doc = "context swap registers"]
pub mod csr45;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr46](csr46) module"]
pub type CSR46 = crate::Reg<u32, _CSR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR46;
#[doc = "`read()` method returns [csr46::R](csr46::R) reader structure"]
impl crate::Readable for CSR46 {}
#[doc = "`write(|w| ..)` method takes [csr46::W](csr46::W) writer structure"]
impl crate::Writable for CSR46 {}
#[doc = "context swap registers"]
pub mod csr46;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr47](csr47) module"]
pub type CSR47 = crate::Reg<u32, _CSR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR47;
#[doc = "`read()` method returns [csr47::R](csr47::R) reader structure"]
impl crate::Readable for CSR47 {}
#[doc = "`write(|w| ..)` method takes [csr47::W](csr47::W) writer structure"]
impl crate::Writable for CSR47 {}
#[doc = "context swap registers"]
pub mod csr47;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr48](csr48) module"]
pub type CSR48 = crate::Reg<u32, _CSR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR48;
#[doc = "`read()` method returns [csr48::R](csr48::R) reader structure"]
impl crate::Readable for CSR48 {}
#[doc = "`write(|w| ..)` method takes [csr48::W](csr48::W) writer structure"]
impl crate::Writable for CSR48 {}
#[doc = "context swap registers"]
pub mod csr48;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr49](csr49) module"]
pub type CSR49 = crate::Reg<u32, _CSR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR49;
#[doc = "`read()` method returns [csr49::R](csr49::R) reader structure"]
impl crate::Readable for CSR49 {}
#[doc = "`write(|w| ..)` method takes [csr49::W](csr49::W) writer structure"]
impl crate::Writable for CSR49 {}
#[doc = "context swap registers"]
pub mod csr49;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr50](csr50) module"]
pub type CSR50 = crate::Reg<u32, _CSR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR50;
#[doc = "`read()` method returns [csr50::R](csr50::R) reader structure"]
impl crate::Readable for CSR50 {}
#[doc = "`write(|w| ..)` method takes [csr50::W](csr50::W) writer structure"]
impl crate::Writable for CSR50 {}
#[doc = "context swap registers"]
pub mod csr50;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr51](csr51) module"]
pub type CSR51 = crate::Reg<u32, _CSR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR51;
#[doc = "`read()` method returns [csr51::R](csr51::R) reader structure"]
impl crate::Readable for CSR51 {}
#[doc = "`write(|w| ..)` method takes [csr51::W](csr51::W) writer structure"]
impl crate::Writable for CSR51 {}
#[doc = "context swap registers"]
pub mod csr51;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr52](csr52) module"]
pub type CSR52 = crate::Reg<u32, _CSR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR52;
#[doc = "`read()` method returns [csr52::R](csr52::R) reader structure"]
impl crate::Readable for CSR52 {}
#[doc = "`write(|w| ..)` method takes [csr52::W](csr52::W) writer structure"]
impl crate::Writable for CSR52 {}
#[doc = "context swap registers"]
pub mod csr52;
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr53](csr53) module"]
pub type CSR53 = crate::Reg<u32, _CSR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR53;
#[doc = "`read()` method returns [csr53::R](csr53::R) reader structure"]
impl crate::Readable for CSR53 {}
#[doc = "`write(|w| ..)` method takes [csr53::W](csr53::W) writer structure"]
impl crate::Writable for CSR53 {}
#[doc = "context swap registers"]
pub mod csr53;
#[doc = "HASH digest register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr0](hash_hr0) module"]
pub type HASH_HR0 = crate::Reg<u32, _HASH_HR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR0;
#[doc = "`read()` method returns [hash_hr0::R](hash_hr0::R) reader structure"]
impl crate::Readable for HASH_HR0 {}
#[doc = "HASH digest register"]
pub mod hash_hr0;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr1](hash_hr1) module"]
pub type HASH_HR1 = crate::Reg<u32, _HASH_HR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR1;
#[doc = "`read()` method returns [hash_hr1::R](hash_hr1::R) reader structure"]
impl crate::Readable for HASH_HR1 {}
#[doc = "read-only"]
pub mod hash_hr1;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr2](hash_hr2) module"]
pub type HASH_HR2 = crate::Reg<u32, _HASH_HR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR2;
#[doc = "`read()` method returns [hash_hr2::R](hash_hr2::R) reader structure"]
impl crate::Readable for HASH_HR2 {}
#[doc = "read-only"]
pub mod hash_hr2;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr3](hash_hr3) module"]
pub type HASH_HR3 = crate::Reg<u32, _HASH_HR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR3;
#[doc = "`read()` method returns [hash_hr3::R](hash_hr3::R) reader structure"]
impl crate::Readable for HASH_HR3 {}
#[doc = "read-only"]
pub mod hash_hr3;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr4](hash_hr4) module"]
pub type HASH_HR4 = crate::Reg<u32, _HASH_HR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR4;
#[doc = "`read()` method returns [hash_hr4::R](hash_hr4::R) reader structure"]
impl crate::Readable for HASH_HR4 {}
#[doc = "read-only"]
pub mod hash_hr4;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr5](hash_hr5) module"]
pub type HASH_HR5 = crate::Reg<u32, _HASH_HR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR5;
#[doc = "`read()` method returns [hash_hr5::R](hash_hr5::R) reader structure"]
impl crate::Readable for HASH_HR5 {}
#[doc = "read-only"]
pub mod hash_hr5;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr6](hash_hr6) module"]
pub type HASH_HR6 = crate::Reg<u32, _HASH_HR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR6;
#[doc = "`read()` method returns [hash_hr6::R](hash_hr6::R) reader structure"]
impl crate::Readable for HASH_HR6 {}
#[doc = "read-only"]
pub mod hash_hr6;
#[doc = "read-only\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr7](hash_hr7) module"]
pub type HASH_HR7 = crate::Reg<u32, _HASH_HR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR7;
#[doc = "`read()` method returns [hash_hr7::R](hash_hr7::R) reader structure"]
impl crate::Readable for HASH_HR7 {}
#[doc = "read-only"]
pub mod hash_hr7;
