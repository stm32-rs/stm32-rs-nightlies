#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HASH control register"]
    pub hash_cr: HASH_CR,
    #[doc = "0x04 - HASH_DIN is the data input register."]
    pub hash_din: HASH_DIN,
    #[doc = "0x08 - The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
    pub hash_str: HASH_STR,
    #[doc = "0x0c - HASH digest register 0"]
    pub hash_hr0: HASH_HR0,
    #[doc = "0x10 - HASH digest register 1"]
    pub hash_hr1: HASH_HR1,
    #[doc = "0x14 - HASH digest register 2"]
    pub hash_hr2: HASH_HR2,
    #[doc = "0x18 - HASH digest register 3"]
    pub hash_hr3: HASH_HR3,
    #[doc = "0x1c - HASH digest register 4"]
    pub hash_hr4: HASH_HR4,
    #[doc = "0x20 - HASH interrupt enable register"]
    pub hash_imr: HASH_IMR,
    #[doc = "0x24 - HASH status register"]
    pub hash_sr: HASH_SR,
    _reserved10: [u8; 208usize],
    #[doc = "0xf8 - These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
    pub hash_csr0: HASH_CSR0,
    #[doc = "0xfc - HASH context swap registers"]
    pub hash_csr1: HASH_CSR1,
    #[doc = "0x100 - HASH context swap registers"]
    pub hash_csr2: HASH_CSR2,
    #[doc = "0x104 - HASH context swap registers"]
    pub hash_csr3: HASH_CSR3,
    #[doc = "0x108 - HASH context swap registers"]
    pub hash_csr4: HASH_CSR4,
    #[doc = "0x10c - HASH context swap registers"]
    pub hash_csr5: HASH_CSR5,
    #[doc = "0x110 - HASH context swap registers"]
    pub hash_csr6: HASH_CSR6,
    #[doc = "0x114 - HASH context swap registers"]
    pub hash_csr7: HASH_CSR7,
    #[doc = "0x118 - HASH context swap registers"]
    pub hash_csr8: HASH_CSR8,
    #[doc = "0x11c - HASH context swap registers"]
    pub hash_csr9: HASH_CSR9,
    #[doc = "0x120 - HASH context swap registers"]
    pub hash_csr10: HASH_CSR10,
    #[doc = "0x124 - HASH context swap registers"]
    pub hash_csr11: HASH_CSR11,
    #[doc = "0x128 - HASH context swap registers"]
    pub hash_csr12: HASH_CSR12,
    #[doc = "0x12c - HASH context swap registers"]
    pub hash_csr13: HASH_CSR13,
    #[doc = "0x130 - HASH context swap registers"]
    pub hash_csr14: HASH_CSR14,
    #[doc = "0x134 - HASH context swap registers"]
    pub hash_csr15: HASH_CSR15,
    #[doc = "0x138 - HASH context swap registers"]
    pub hash_csr16: HASH_CSR16,
    #[doc = "0x13c - HASH context swap registers"]
    pub hash_csr17: HASH_CSR17,
    #[doc = "0x140 - HASH context swap registers"]
    pub hash_csr18: HASH_CSR18,
    #[doc = "0x144 - HASH context swap registers"]
    pub hash_csr19: HASH_CSR19,
    #[doc = "0x148 - HASH context swap registers"]
    pub hash_csr20: HASH_CSR20,
    #[doc = "0x14c - HASH context swap registers"]
    pub hash_csr21: HASH_CSR21,
    #[doc = "0x150 - HASH context swap registers"]
    pub hash_csr22: HASH_CSR22,
    #[doc = "0x154 - HASH context swap registers"]
    pub hash_csr23: HASH_CSR23,
    #[doc = "0x158 - HASH context swap registers"]
    pub hash_csr24: HASH_CSR24,
    #[doc = "0x15c - HASH context swap registers"]
    pub hash_csr25: HASH_CSR25,
    #[doc = "0x160 - HASH context swap registers"]
    pub hash_csr26: HASH_CSR26,
    #[doc = "0x164 - HASH context swap registers"]
    pub hash_csr27: HASH_CSR27,
    #[doc = "0x168 - HASH context swap registers"]
    pub hash_csr28: HASH_CSR28,
    #[doc = "0x16c - HASH context swap registers"]
    pub hash_csr29: HASH_CSR29,
    #[doc = "0x170 - HASH context swap registers"]
    pub hash_csr30: HASH_CSR30,
    #[doc = "0x174 - HASH context swap registers"]
    pub hash_csr31: HASH_CSR31,
    #[doc = "0x178 - HASH context swap registers"]
    pub hash_csr32: HASH_CSR32,
    #[doc = "0x17c - HASH context swap registers"]
    pub hash_csr33: HASH_CSR33,
    #[doc = "0x180 - HASH context swap registers"]
    pub hash_csr34: HASH_CSR34,
    #[doc = "0x184 - HASH context swap registers"]
    pub hash_csr35: HASH_CSR35,
    #[doc = "0x188 - HASH context swap registers"]
    pub hash_csr36: HASH_CSR36,
    #[doc = "0x18c - HASH context swap registers"]
    pub hash_csr37: HASH_CSR37,
    #[doc = "0x190 - HASH context swap registers"]
    pub hash_csr38: HASH_CSR38,
    #[doc = "0x194 - HASH context swap registers"]
    pub hash_csr39: HASH_CSR39,
    #[doc = "0x198 - HASH context swap registers"]
    pub hash_csr40: HASH_CSR40,
    #[doc = "0x19c - HASH context swap registers"]
    pub hash_csr41: HASH_CSR41,
    #[doc = "0x1a0 - HASH context swap registers"]
    pub hash_csr42: HASH_CSR42,
    #[doc = "0x1a4 - HASH context swap registers"]
    pub hash_csr43: HASH_CSR43,
    #[doc = "0x1a8 - HASH context swap registers"]
    pub hash_csr44: HASH_CSR44,
    #[doc = "0x1ac - HASH context swap registers"]
    pub hash_csr45: HASH_CSR45,
    #[doc = "0x1b0 - HASH context swap registers"]
    pub hash_csr46: HASH_CSR46,
    #[doc = "0x1b4 - HASH context swap registers"]
    pub hash_csr47: HASH_CSR47,
    #[doc = "0x1b8 - HASH context swap registers"]
    pub hash_csr48: HASH_CSR48,
    #[doc = "0x1bc - HASH context swap registers"]
    pub hash_csr49: HASH_CSR49,
    #[doc = "0x1c0 - HASH context swap registers"]
    pub hash_csr50: HASH_CSR50,
    #[doc = "0x1c4 - HASH context swap registers"]
    pub hash_csr51: HASH_CSR51,
    #[doc = "0x1c8 - HASH context swap registers"]
    pub hash_csr52: HASH_CSR52,
    #[doc = "0x1cc - HASH context swap registers"]
    pub hash_csr53: HASH_CSR53,
    _reserved64: [u8; 340usize],
    #[doc = "0x324 - HASH digest register 5"]
    pub hash_hr5: HASH_HR5,
    #[doc = "0x328 - HASH digest register 6"]
    pub hash_hr6: HASH_HR6,
    #[doc = "0x32c - HASH digest register 7"]
    pub hash_hr7: HASH_HR7,
    _reserved67: [u8; 192usize],
    #[doc = "0x3f0 - HASH Hardware Configuration Register"]
    pub hash_hwcfgr: HASH_HWCFGR,
    #[doc = "0x3f4 - HASH Version Register"]
    pub hash_verr: HASH_VERR,
    #[doc = "0x3f8 - HASH Identification"]
    pub hash_ipidr: HASH_IPIDR,
    #[doc = "0x3fc - HASH Hardware Magic ID"]
    pub hash_mid: HASH_MID,
}
#[doc = "HASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_cr](hash_cr) module"]
pub type HASH_CR = crate::Reg<u32, _HASH_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CR;
#[doc = "`read()` method returns [hash_cr::R](hash_cr::R) reader structure"]
impl crate::Readable for HASH_CR {}
#[doc = "`write(|w| ..)` method takes [hash_cr::W](hash_cr::W) writer structure"]
impl crate::Writable for HASH_CR {}
#[doc = "HASH control register"]
pub mod hash_cr;
#[doc = "HASH_DIN is the data input register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_din](hash_din) module"]
pub type HASH_DIN = crate::Reg<u32, _HASH_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_DIN;
#[doc = "`read()` method returns [hash_din::R](hash_din::R) reader structure"]
impl crate::Readable for HASH_DIN {}
#[doc = "`write(|w| ..)` method takes [hash_din::W](hash_din::W) writer structure"]
impl crate::Writable for HASH_DIN {}
#[doc = "HASH_DIN is the data input register."]
pub mod hash_din;
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_str](hash_str) module"]
pub type HASH_STR = crate::Reg<u32, _HASH_STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_STR;
#[doc = "`read()` method returns [hash_str::R](hash_str::R) reader structure"]
impl crate::Readable for HASH_STR {}
#[doc = "`write(|w| ..)` method takes [hash_str::W](hash_str::W) writer structure"]
impl crate::Writable for HASH_STR {}
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1"]
pub mod hash_str;
#[doc = "HASH digest register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr0](hash_hr0) module"]
pub type HASH_HR0 = crate::Reg<u32, _HASH_HR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR0;
#[doc = "`read()` method returns [hash_hr0::R](hash_hr0::R) reader structure"]
impl crate::Readable for HASH_HR0 {}
#[doc = "HASH digest register 0"]
pub mod hash_hr0;
#[doc = "HASH digest register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr1](hash_hr1) module"]
pub type HASH_HR1 = crate::Reg<u32, _HASH_HR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR1;
#[doc = "`read()` method returns [hash_hr1::R](hash_hr1::R) reader structure"]
impl crate::Readable for HASH_HR1 {}
#[doc = "HASH digest register 1"]
pub mod hash_hr1;
#[doc = "HASH digest register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr2](hash_hr2) module"]
pub type HASH_HR2 = crate::Reg<u32, _HASH_HR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR2;
#[doc = "`read()` method returns [hash_hr2::R](hash_hr2::R) reader structure"]
impl crate::Readable for HASH_HR2 {}
#[doc = "HASH digest register 2"]
pub mod hash_hr2;
#[doc = "HASH digest register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr3](hash_hr3) module"]
pub type HASH_HR3 = crate::Reg<u32, _HASH_HR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR3;
#[doc = "`read()` method returns [hash_hr3::R](hash_hr3::R) reader structure"]
impl crate::Readable for HASH_HR3 {}
#[doc = "HASH digest register 3"]
pub mod hash_hr3;
#[doc = "HASH digest register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr4](hash_hr4) module"]
pub type HASH_HR4 = crate::Reg<u32, _HASH_HR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR4;
#[doc = "`read()` method returns [hash_hr4::R](hash_hr4::R) reader structure"]
impl crate::Readable for HASH_HR4 {}
#[doc = "HASH digest register 4"]
pub mod hash_hr4;
#[doc = "HASH interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_imr](hash_imr) module"]
pub type HASH_IMR = crate::Reg<u32, _HASH_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_IMR;
#[doc = "`read()` method returns [hash_imr::R](hash_imr::R) reader structure"]
impl crate::Readable for HASH_IMR {}
#[doc = "`write(|w| ..)` method takes [hash_imr::W](hash_imr::W) writer structure"]
impl crate::Writable for HASH_IMR {}
#[doc = "HASH interrupt enable register"]
pub mod hash_imr;
#[doc = "HASH status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_sr](hash_sr) module"]
pub type HASH_SR = crate::Reg<u32, _HASH_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_SR;
#[doc = "`read()` method returns [hash_sr::R](hash_sr::R) reader structure"]
impl crate::Readable for HASH_SR {}
#[doc = "`write(|w| ..)` method takes [hash_sr::W](hash_sr::W) writer structure"]
impl crate::Writable for HASH_SR {}
#[doc = "HASH status register"]
pub mod hash_sr;
#[doc = "These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr0](hash_csr0) module"]
pub type HASH_CSR0 = crate::Reg<u32, _HASH_CSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR0;
#[doc = "`read()` method returns [hash_csr0::R](hash_csr0::R) reader structure"]
impl crate::Readable for HASH_CSR0 {}
#[doc = "`write(|w| ..)` method takes [hash_csr0::W](hash_csr0::W) writer structure"]
impl crate::Writable for HASH_CSR0 {}
#[doc = "These registers contain the complete internal register states of the hash processor. They are useful when a context swap has to be done because a high-priority task needs to use the hash processor while it is already used by another task. When such an event occurs, the HASH_CSRx registers have to be read and the read values have to be saved in the system memory space. Then the hash processor can be used by the preemptive task, and when the hash computation is complete, the saved context can be read from memory and written back into the HASH_CSRx registers."]
pub mod hash_csr0;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr1](hash_csr1) module"]
pub type HASH_CSR1 = crate::Reg<u32, _HASH_CSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR1;
#[doc = "`read()` method returns [hash_csr1::R](hash_csr1::R) reader structure"]
impl crate::Readable for HASH_CSR1 {}
#[doc = "`write(|w| ..)` method takes [hash_csr1::W](hash_csr1::W) writer structure"]
impl crate::Writable for HASH_CSR1 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr1;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr2](hash_csr2) module"]
pub type HASH_CSR2 = crate::Reg<u32, _HASH_CSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR2;
#[doc = "`read()` method returns [hash_csr2::R](hash_csr2::R) reader structure"]
impl crate::Readable for HASH_CSR2 {}
#[doc = "`write(|w| ..)` method takes [hash_csr2::W](hash_csr2::W) writer structure"]
impl crate::Writable for HASH_CSR2 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr2;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr3](hash_csr3) module"]
pub type HASH_CSR3 = crate::Reg<u32, _HASH_CSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR3;
#[doc = "`read()` method returns [hash_csr3::R](hash_csr3::R) reader structure"]
impl crate::Readable for HASH_CSR3 {}
#[doc = "`write(|w| ..)` method takes [hash_csr3::W](hash_csr3::W) writer structure"]
impl crate::Writable for HASH_CSR3 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr3;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr4](hash_csr4) module"]
pub type HASH_CSR4 = crate::Reg<u32, _HASH_CSR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR4;
#[doc = "`read()` method returns [hash_csr4::R](hash_csr4::R) reader structure"]
impl crate::Readable for HASH_CSR4 {}
#[doc = "`write(|w| ..)` method takes [hash_csr4::W](hash_csr4::W) writer structure"]
impl crate::Writable for HASH_CSR4 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr4;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr5](hash_csr5) module"]
pub type HASH_CSR5 = crate::Reg<u32, _HASH_CSR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR5;
#[doc = "`read()` method returns [hash_csr5::R](hash_csr5::R) reader structure"]
impl crate::Readable for HASH_CSR5 {}
#[doc = "`write(|w| ..)` method takes [hash_csr5::W](hash_csr5::W) writer structure"]
impl crate::Writable for HASH_CSR5 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr5;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr6](hash_csr6) module"]
pub type HASH_CSR6 = crate::Reg<u32, _HASH_CSR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR6;
#[doc = "`read()` method returns [hash_csr6::R](hash_csr6::R) reader structure"]
impl crate::Readable for HASH_CSR6 {}
#[doc = "`write(|w| ..)` method takes [hash_csr6::W](hash_csr6::W) writer structure"]
impl crate::Writable for HASH_CSR6 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr6;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr7](hash_csr7) module"]
pub type HASH_CSR7 = crate::Reg<u32, _HASH_CSR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR7;
#[doc = "`read()` method returns [hash_csr7::R](hash_csr7::R) reader structure"]
impl crate::Readable for HASH_CSR7 {}
#[doc = "`write(|w| ..)` method takes [hash_csr7::W](hash_csr7::W) writer structure"]
impl crate::Writable for HASH_CSR7 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr7;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr8](hash_csr8) module"]
pub type HASH_CSR8 = crate::Reg<u32, _HASH_CSR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR8;
#[doc = "`read()` method returns [hash_csr8::R](hash_csr8::R) reader structure"]
impl crate::Readable for HASH_CSR8 {}
#[doc = "`write(|w| ..)` method takes [hash_csr8::W](hash_csr8::W) writer structure"]
impl crate::Writable for HASH_CSR8 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr8;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr9](hash_csr9) module"]
pub type HASH_CSR9 = crate::Reg<u32, _HASH_CSR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR9;
#[doc = "`read()` method returns [hash_csr9::R](hash_csr9::R) reader structure"]
impl crate::Readable for HASH_CSR9 {}
#[doc = "`write(|w| ..)` method takes [hash_csr9::W](hash_csr9::W) writer structure"]
impl crate::Writable for HASH_CSR9 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr9;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr10](hash_csr10) module"]
pub type HASH_CSR10 = crate::Reg<u32, _HASH_CSR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR10;
#[doc = "`read()` method returns [hash_csr10::R](hash_csr10::R) reader structure"]
impl crate::Readable for HASH_CSR10 {}
#[doc = "`write(|w| ..)` method takes [hash_csr10::W](hash_csr10::W) writer structure"]
impl crate::Writable for HASH_CSR10 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr10;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr11](hash_csr11) module"]
pub type HASH_CSR11 = crate::Reg<u32, _HASH_CSR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR11;
#[doc = "`read()` method returns [hash_csr11::R](hash_csr11::R) reader structure"]
impl crate::Readable for HASH_CSR11 {}
#[doc = "`write(|w| ..)` method takes [hash_csr11::W](hash_csr11::W) writer structure"]
impl crate::Writable for HASH_CSR11 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr11;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr12](hash_csr12) module"]
pub type HASH_CSR12 = crate::Reg<u32, _HASH_CSR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR12;
#[doc = "`read()` method returns [hash_csr12::R](hash_csr12::R) reader structure"]
impl crate::Readable for HASH_CSR12 {}
#[doc = "`write(|w| ..)` method takes [hash_csr12::W](hash_csr12::W) writer structure"]
impl crate::Writable for HASH_CSR12 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr12;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr13](hash_csr13) module"]
pub type HASH_CSR13 = crate::Reg<u32, _HASH_CSR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR13;
#[doc = "`read()` method returns [hash_csr13::R](hash_csr13::R) reader structure"]
impl crate::Readable for HASH_CSR13 {}
#[doc = "`write(|w| ..)` method takes [hash_csr13::W](hash_csr13::W) writer structure"]
impl crate::Writable for HASH_CSR13 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr13;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr14](hash_csr14) module"]
pub type HASH_CSR14 = crate::Reg<u32, _HASH_CSR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR14;
#[doc = "`read()` method returns [hash_csr14::R](hash_csr14::R) reader structure"]
impl crate::Readable for HASH_CSR14 {}
#[doc = "`write(|w| ..)` method takes [hash_csr14::W](hash_csr14::W) writer structure"]
impl crate::Writable for HASH_CSR14 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr14;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr15](hash_csr15) module"]
pub type HASH_CSR15 = crate::Reg<u32, _HASH_CSR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR15;
#[doc = "`read()` method returns [hash_csr15::R](hash_csr15::R) reader structure"]
impl crate::Readable for HASH_CSR15 {}
#[doc = "`write(|w| ..)` method takes [hash_csr15::W](hash_csr15::W) writer structure"]
impl crate::Writable for HASH_CSR15 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr15;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr16](hash_csr16) module"]
pub type HASH_CSR16 = crate::Reg<u32, _HASH_CSR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR16;
#[doc = "`read()` method returns [hash_csr16::R](hash_csr16::R) reader structure"]
impl crate::Readable for HASH_CSR16 {}
#[doc = "`write(|w| ..)` method takes [hash_csr16::W](hash_csr16::W) writer structure"]
impl crate::Writable for HASH_CSR16 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr16;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr17](hash_csr17) module"]
pub type HASH_CSR17 = crate::Reg<u32, _HASH_CSR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR17;
#[doc = "`read()` method returns [hash_csr17::R](hash_csr17::R) reader structure"]
impl crate::Readable for HASH_CSR17 {}
#[doc = "`write(|w| ..)` method takes [hash_csr17::W](hash_csr17::W) writer structure"]
impl crate::Writable for HASH_CSR17 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr17;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr18](hash_csr18) module"]
pub type HASH_CSR18 = crate::Reg<u32, _HASH_CSR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR18;
#[doc = "`read()` method returns [hash_csr18::R](hash_csr18::R) reader structure"]
impl crate::Readable for HASH_CSR18 {}
#[doc = "`write(|w| ..)` method takes [hash_csr18::W](hash_csr18::W) writer structure"]
impl crate::Writable for HASH_CSR18 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr18;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr19](hash_csr19) module"]
pub type HASH_CSR19 = crate::Reg<u32, _HASH_CSR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR19;
#[doc = "`read()` method returns [hash_csr19::R](hash_csr19::R) reader structure"]
impl crate::Readable for HASH_CSR19 {}
#[doc = "`write(|w| ..)` method takes [hash_csr19::W](hash_csr19::W) writer structure"]
impl crate::Writable for HASH_CSR19 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr19;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr20](hash_csr20) module"]
pub type HASH_CSR20 = crate::Reg<u32, _HASH_CSR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR20;
#[doc = "`read()` method returns [hash_csr20::R](hash_csr20::R) reader structure"]
impl crate::Readable for HASH_CSR20 {}
#[doc = "`write(|w| ..)` method takes [hash_csr20::W](hash_csr20::W) writer structure"]
impl crate::Writable for HASH_CSR20 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr20;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr21](hash_csr21) module"]
pub type HASH_CSR21 = crate::Reg<u32, _HASH_CSR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR21;
#[doc = "`read()` method returns [hash_csr21::R](hash_csr21::R) reader structure"]
impl crate::Readable for HASH_CSR21 {}
#[doc = "`write(|w| ..)` method takes [hash_csr21::W](hash_csr21::W) writer structure"]
impl crate::Writable for HASH_CSR21 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr21;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr22](hash_csr22) module"]
pub type HASH_CSR22 = crate::Reg<u32, _HASH_CSR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR22;
#[doc = "`read()` method returns [hash_csr22::R](hash_csr22::R) reader structure"]
impl crate::Readable for HASH_CSR22 {}
#[doc = "`write(|w| ..)` method takes [hash_csr22::W](hash_csr22::W) writer structure"]
impl crate::Writable for HASH_CSR22 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr22;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr23](hash_csr23) module"]
pub type HASH_CSR23 = crate::Reg<u32, _HASH_CSR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR23;
#[doc = "`read()` method returns [hash_csr23::R](hash_csr23::R) reader structure"]
impl crate::Readable for HASH_CSR23 {}
#[doc = "`write(|w| ..)` method takes [hash_csr23::W](hash_csr23::W) writer structure"]
impl crate::Writable for HASH_CSR23 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr23;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr24](hash_csr24) module"]
pub type HASH_CSR24 = crate::Reg<u32, _HASH_CSR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR24;
#[doc = "`read()` method returns [hash_csr24::R](hash_csr24::R) reader structure"]
impl crate::Readable for HASH_CSR24 {}
#[doc = "`write(|w| ..)` method takes [hash_csr24::W](hash_csr24::W) writer structure"]
impl crate::Writable for HASH_CSR24 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr24;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr25](hash_csr25) module"]
pub type HASH_CSR25 = crate::Reg<u32, _HASH_CSR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR25;
#[doc = "`read()` method returns [hash_csr25::R](hash_csr25::R) reader structure"]
impl crate::Readable for HASH_CSR25 {}
#[doc = "`write(|w| ..)` method takes [hash_csr25::W](hash_csr25::W) writer structure"]
impl crate::Writable for HASH_CSR25 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr25;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr26](hash_csr26) module"]
pub type HASH_CSR26 = crate::Reg<u32, _HASH_CSR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR26;
#[doc = "`read()` method returns [hash_csr26::R](hash_csr26::R) reader structure"]
impl crate::Readable for HASH_CSR26 {}
#[doc = "`write(|w| ..)` method takes [hash_csr26::W](hash_csr26::W) writer structure"]
impl crate::Writable for HASH_CSR26 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr26;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr27](hash_csr27) module"]
pub type HASH_CSR27 = crate::Reg<u32, _HASH_CSR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR27;
#[doc = "`read()` method returns [hash_csr27::R](hash_csr27::R) reader structure"]
impl crate::Readable for HASH_CSR27 {}
#[doc = "`write(|w| ..)` method takes [hash_csr27::W](hash_csr27::W) writer structure"]
impl crate::Writable for HASH_CSR27 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr27;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr28](hash_csr28) module"]
pub type HASH_CSR28 = crate::Reg<u32, _HASH_CSR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR28;
#[doc = "`read()` method returns [hash_csr28::R](hash_csr28::R) reader structure"]
impl crate::Readable for HASH_CSR28 {}
#[doc = "`write(|w| ..)` method takes [hash_csr28::W](hash_csr28::W) writer structure"]
impl crate::Writable for HASH_CSR28 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr28;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr29](hash_csr29) module"]
pub type HASH_CSR29 = crate::Reg<u32, _HASH_CSR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR29;
#[doc = "`read()` method returns [hash_csr29::R](hash_csr29::R) reader structure"]
impl crate::Readable for HASH_CSR29 {}
#[doc = "`write(|w| ..)` method takes [hash_csr29::W](hash_csr29::W) writer structure"]
impl crate::Writable for HASH_CSR29 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr29;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr30](hash_csr30) module"]
pub type HASH_CSR30 = crate::Reg<u32, _HASH_CSR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR30;
#[doc = "`read()` method returns [hash_csr30::R](hash_csr30::R) reader structure"]
impl crate::Readable for HASH_CSR30 {}
#[doc = "`write(|w| ..)` method takes [hash_csr30::W](hash_csr30::W) writer structure"]
impl crate::Writable for HASH_CSR30 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr30;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr31](hash_csr31) module"]
pub type HASH_CSR31 = crate::Reg<u32, _HASH_CSR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR31;
#[doc = "`read()` method returns [hash_csr31::R](hash_csr31::R) reader structure"]
impl crate::Readable for HASH_CSR31 {}
#[doc = "`write(|w| ..)` method takes [hash_csr31::W](hash_csr31::W) writer structure"]
impl crate::Writable for HASH_CSR31 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr31;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr32](hash_csr32) module"]
pub type HASH_CSR32 = crate::Reg<u32, _HASH_CSR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR32;
#[doc = "`read()` method returns [hash_csr32::R](hash_csr32::R) reader structure"]
impl crate::Readable for HASH_CSR32 {}
#[doc = "`write(|w| ..)` method takes [hash_csr32::W](hash_csr32::W) writer structure"]
impl crate::Writable for HASH_CSR32 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr32;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr33](hash_csr33) module"]
pub type HASH_CSR33 = crate::Reg<u32, _HASH_CSR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR33;
#[doc = "`read()` method returns [hash_csr33::R](hash_csr33::R) reader structure"]
impl crate::Readable for HASH_CSR33 {}
#[doc = "`write(|w| ..)` method takes [hash_csr33::W](hash_csr33::W) writer structure"]
impl crate::Writable for HASH_CSR33 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr33;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr34](hash_csr34) module"]
pub type HASH_CSR34 = crate::Reg<u32, _HASH_CSR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR34;
#[doc = "`read()` method returns [hash_csr34::R](hash_csr34::R) reader structure"]
impl crate::Readable for HASH_CSR34 {}
#[doc = "`write(|w| ..)` method takes [hash_csr34::W](hash_csr34::W) writer structure"]
impl crate::Writable for HASH_CSR34 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr34;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr35](hash_csr35) module"]
pub type HASH_CSR35 = crate::Reg<u32, _HASH_CSR35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR35;
#[doc = "`read()` method returns [hash_csr35::R](hash_csr35::R) reader structure"]
impl crate::Readable for HASH_CSR35 {}
#[doc = "`write(|w| ..)` method takes [hash_csr35::W](hash_csr35::W) writer structure"]
impl crate::Writable for HASH_CSR35 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr35;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr36](hash_csr36) module"]
pub type HASH_CSR36 = crate::Reg<u32, _HASH_CSR36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR36;
#[doc = "`read()` method returns [hash_csr36::R](hash_csr36::R) reader structure"]
impl crate::Readable for HASH_CSR36 {}
#[doc = "`write(|w| ..)` method takes [hash_csr36::W](hash_csr36::W) writer structure"]
impl crate::Writable for HASH_CSR36 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr36;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr37](hash_csr37) module"]
pub type HASH_CSR37 = crate::Reg<u32, _HASH_CSR37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR37;
#[doc = "`read()` method returns [hash_csr37::R](hash_csr37::R) reader structure"]
impl crate::Readable for HASH_CSR37 {}
#[doc = "`write(|w| ..)` method takes [hash_csr37::W](hash_csr37::W) writer structure"]
impl crate::Writable for HASH_CSR37 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr37;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr38](hash_csr38) module"]
pub type HASH_CSR38 = crate::Reg<u32, _HASH_CSR38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR38;
#[doc = "`read()` method returns [hash_csr38::R](hash_csr38::R) reader structure"]
impl crate::Readable for HASH_CSR38 {}
#[doc = "`write(|w| ..)` method takes [hash_csr38::W](hash_csr38::W) writer structure"]
impl crate::Writable for HASH_CSR38 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr38;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr39](hash_csr39) module"]
pub type HASH_CSR39 = crate::Reg<u32, _HASH_CSR39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR39;
#[doc = "`read()` method returns [hash_csr39::R](hash_csr39::R) reader structure"]
impl crate::Readable for HASH_CSR39 {}
#[doc = "`write(|w| ..)` method takes [hash_csr39::W](hash_csr39::W) writer structure"]
impl crate::Writable for HASH_CSR39 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr39;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr40](hash_csr40) module"]
pub type HASH_CSR40 = crate::Reg<u32, _HASH_CSR40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR40;
#[doc = "`read()` method returns [hash_csr40::R](hash_csr40::R) reader structure"]
impl crate::Readable for HASH_CSR40 {}
#[doc = "`write(|w| ..)` method takes [hash_csr40::W](hash_csr40::W) writer structure"]
impl crate::Writable for HASH_CSR40 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr40;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr41](hash_csr41) module"]
pub type HASH_CSR41 = crate::Reg<u32, _HASH_CSR41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR41;
#[doc = "`read()` method returns [hash_csr41::R](hash_csr41::R) reader structure"]
impl crate::Readable for HASH_CSR41 {}
#[doc = "`write(|w| ..)` method takes [hash_csr41::W](hash_csr41::W) writer structure"]
impl crate::Writable for HASH_CSR41 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr41;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr42](hash_csr42) module"]
pub type HASH_CSR42 = crate::Reg<u32, _HASH_CSR42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR42;
#[doc = "`read()` method returns [hash_csr42::R](hash_csr42::R) reader structure"]
impl crate::Readable for HASH_CSR42 {}
#[doc = "`write(|w| ..)` method takes [hash_csr42::W](hash_csr42::W) writer structure"]
impl crate::Writable for HASH_CSR42 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr42;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr43](hash_csr43) module"]
pub type HASH_CSR43 = crate::Reg<u32, _HASH_CSR43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR43;
#[doc = "`read()` method returns [hash_csr43::R](hash_csr43::R) reader structure"]
impl crate::Readable for HASH_CSR43 {}
#[doc = "`write(|w| ..)` method takes [hash_csr43::W](hash_csr43::W) writer structure"]
impl crate::Writable for HASH_CSR43 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr43;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr44](hash_csr44) module"]
pub type HASH_CSR44 = crate::Reg<u32, _HASH_CSR44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR44;
#[doc = "`read()` method returns [hash_csr44::R](hash_csr44::R) reader structure"]
impl crate::Readable for HASH_CSR44 {}
#[doc = "`write(|w| ..)` method takes [hash_csr44::W](hash_csr44::W) writer structure"]
impl crate::Writable for HASH_CSR44 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr44;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr45](hash_csr45) module"]
pub type HASH_CSR45 = crate::Reg<u32, _HASH_CSR45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR45;
#[doc = "`read()` method returns [hash_csr45::R](hash_csr45::R) reader structure"]
impl crate::Readable for HASH_CSR45 {}
#[doc = "`write(|w| ..)` method takes [hash_csr45::W](hash_csr45::W) writer structure"]
impl crate::Writable for HASH_CSR45 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr45;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr46](hash_csr46) module"]
pub type HASH_CSR46 = crate::Reg<u32, _HASH_CSR46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR46;
#[doc = "`read()` method returns [hash_csr46::R](hash_csr46::R) reader structure"]
impl crate::Readable for HASH_CSR46 {}
#[doc = "`write(|w| ..)` method takes [hash_csr46::W](hash_csr46::W) writer structure"]
impl crate::Writable for HASH_CSR46 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr46;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr47](hash_csr47) module"]
pub type HASH_CSR47 = crate::Reg<u32, _HASH_CSR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR47;
#[doc = "`read()` method returns [hash_csr47::R](hash_csr47::R) reader structure"]
impl crate::Readable for HASH_CSR47 {}
#[doc = "`write(|w| ..)` method takes [hash_csr47::W](hash_csr47::W) writer structure"]
impl crate::Writable for HASH_CSR47 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr47;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr48](hash_csr48) module"]
pub type HASH_CSR48 = crate::Reg<u32, _HASH_CSR48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR48;
#[doc = "`read()` method returns [hash_csr48::R](hash_csr48::R) reader structure"]
impl crate::Readable for HASH_CSR48 {}
#[doc = "`write(|w| ..)` method takes [hash_csr48::W](hash_csr48::W) writer structure"]
impl crate::Writable for HASH_CSR48 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr48;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr49](hash_csr49) module"]
pub type HASH_CSR49 = crate::Reg<u32, _HASH_CSR49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR49;
#[doc = "`read()` method returns [hash_csr49::R](hash_csr49::R) reader structure"]
impl crate::Readable for HASH_CSR49 {}
#[doc = "`write(|w| ..)` method takes [hash_csr49::W](hash_csr49::W) writer structure"]
impl crate::Writable for HASH_CSR49 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr49;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr50](hash_csr50) module"]
pub type HASH_CSR50 = crate::Reg<u32, _HASH_CSR50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR50;
#[doc = "`read()` method returns [hash_csr50::R](hash_csr50::R) reader structure"]
impl crate::Readable for HASH_CSR50 {}
#[doc = "`write(|w| ..)` method takes [hash_csr50::W](hash_csr50::W) writer structure"]
impl crate::Writable for HASH_CSR50 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr50;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr51](hash_csr51) module"]
pub type HASH_CSR51 = crate::Reg<u32, _HASH_CSR51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR51;
#[doc = "`read()` method returns [hash_csr51::R](hash_csr51::R) reader structure"]
impl crate::Readable for HASH_CSR51 {}
#[doc = "`write(|w| ..)` method takes [hash_csr51::W](hash_csr51::W) writer structure"]
impl crate::Writable for HASH_CSR51 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr51;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr52](hash_csr52) module"]
pub type HASH_CSR52 = crate::Reg<u32, _HASH_CSR52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR52;
#[doc = "`read()` method returns [hash_csr52::R](hash_csr52::R) reader structure"]
impl crate::Readable for HASH_CSR52 {}
#[doc = "`write(|w| ..)` method takes [hash_csr52::W](hash_csr52::W) writer structure"]
impl crate::Writable for HASH_CSR52 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr52;
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr53](hash_csr53) module"]
pub type HASH_CSR53 = crate::Reg<u32, _HASH_CSR53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_CSR53;
#[doc = "`read()` method returns [hash_csr53::R](hash_csr53::R) reader structure"]
impl crate::Readable for HASH_CSR53 {}
#[doc = "`write(|w| ..)` method takes [hash_csr53::W](hash_csr53::W) writer structure"]
impl crate::Writable for HASH_CSR53 {}
#[doc = "HASH context swap registers"]
pub mod hash_csr53;
#[doc = "HASH digest register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr5](hash_hr5) module"]
pub type HASH_HR5 = crate::Reg<u32, _HASH_HR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR5;
#[doc = "`read()` method returns [hash_hr5::R](hash_hr5::R) reader structure"]
impl crate::Readable for HASH_HR5 {}
#[doc = "HASH digest register 5"]
pub mod hash_hr5;
#[doc = "HASH digest register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr6](hash_hr6) module"]
pub type HASH_HR6 = crate::Reg<u32, _HASH_HR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR6;
#[doc = "`read()` method returns [hash_hr6::R](hash_hr6::R) reader structure"]
impl crate::Readable for HASH_HR6 {}
#[doc = "HASH digest register 6"]
pub mod hash_hr6;
#[doc = "HASH digest register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr7](hash_hr7) module"]
pub type HASH_HR7 = crate::Reg<u32, _HASH_HR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HR7;
#[doc = "`read()` method returns [hash_hr7::R](hash_hr7::R) reader structure"]
impl crate::Readable for HASH_HR7 {}
#[doc = "HASH digest register 7"]
pub mod hash_hr7;
#[doc = "HASH Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hwcfgr](hash_hwcfgr) module"]
pub type HASH_HWCFGR = crate::Reg<u32, _HASH_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_HWCFGR;
#[doc = "`read()` method returns [hash_hwcfgr::R](hash_hwcfgr::R) reader structure"]
impl crate::Readable for HASH_HWCFGR {}
#[doc = "HASH Hardware Configuration Register"]
pub mod hash_hwcfgr;
#[doc = "HASH Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_verr](hash_verr) module"]
pub type HASH_VERR = crate::Reg<u32, _HASH_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_VERR;
#[doc = "`read()` method returns [hash_verr::R](hash_verr::R) reader structure"]
impl crate::Readable for HASH_VERR {}
#[doc = "HASH Version Register"]
pub mod hash_verr;
#[doc = "HASH Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_ipidr](hash_ipidr) module"]
pub type HASH_IPIDR = crate::Reg<u32, _HASH_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_IPIDR;
#[doc = "`read()` method returns [hash_ipidr::R](hash_ipidr::R) reader structure"]
impl crate::Readable for HASH_IPIDR {}
#[doc = "HASH Identification"]
pub mod hash_ipidr;
#[doc = "HASH Hardware Magic ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_mid](hash_mid) module"]
pub type HASH_MID = crate::Reg<u32, _HASH_MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH_MID;
#[doc = "`read()` method returns [hash_mid::R](hash_mid::R) reader structure"]
impl crate::Readable for HASH_MID {}
#[doc = "HASH Hardware Magic ID"]
pub mod hash_mid;
