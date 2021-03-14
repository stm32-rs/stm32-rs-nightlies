#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains only register bits for configurable events."]
    pub exti_rtsr1: EXTI_RTSR1,
    #[doc = "0x04 - Contains only register bits for configurable events."]
    pub exti_ftsr1: EXTI_FTSR1,
    #[doc = "0x08 - Contains only register bits for configurable events."]
    pub exti_swier1: EXTI_SWIER1,
    #[doc = "0x0c - Contains only register bits for configurable events."]
    pub exti_rpr1: EXTI_RPR1,
    #[doc = "0x10 - Contains only register bits for configurable events."]
    pub exti_fpr1: EXTI_FPR1,
    #[doc = "0x14 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr1: EXTI_TZENR1,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Contains only register bits for configurable events."]
    pub exti_rtsr2: EXTI_RTSR2,
    #[doc = "0x24 - Contains only register bits for configurable events."]
    pub exti_ftsr2: EXTI_FTSR2,
    #[doc = "0x28 - Contains only register bits for configurable events."]
    pub exti_swier2: EXTI_SWIER2,
    #[doc = "0x2c - Contains only register bits for configurable events."]
    pub exti_rpr2: EXTI_RPR2,
    #[doc = "0x30 - Contains only register bits for configurable events."]
    pub exti_fpr2: EXTI_FPR2,
    #[doc = "0x34 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr2: EXTI_TZENR2,
    _reserved12: [u8; 8usize],
    #[doc = "0x40 - Contains only register bits for configurable events."]
    pub exti_rtsr3: EXTI_RTSR3,
    #[doc = "0x44 - Contains only register bits for configurable events."]
    pub exti_ftsr3: EXTI_FTSR3,
    #[doc = "0x48 - Contains only register bits for configurable events."]
    pub exti_swier3: EXTI_SWIER3,
    #[doc = "0x4c - Contains only register bits for configurable events."]
    pub exti_rpr3: EXTI_RPR3,
    #[doc = "0x50 - Contains only register bits for configurable events."]
    pub exti_fpr3: EXTI_FPR3,
    #[doc = "0x54 - This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
    pub exti_tzenr3: EXTI_TZENR3,
    _reserved18: [u8; 8usize],
    #[doc = "0x60 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr1: EXTI_EXTICR1,
    #[doc = "0x64 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr2: EXTI_EXTICR2,
    #[doc = "0x68 - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr3: EXTI_EXTICR3,
    #[doc = "0x6c - EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
    pub exti_exticr4: EXTI_EXTICR4,
    _reserved22: [u8; 16usize],
    #[doc = "0x80 - Contains register bits for configurable events and Direct events."]
    pub exti_imr1: EXTI_IMR1,
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    pub exti_emr1: EXTI_EMR1,
    _reserved24: [u8; 8usize],
    #[doc = "0x90 - Contains register bits for configurable events and direct events."]
    pub exti_imr2: EXTI_IMR2,
    #[doc = "0x94 - EXTI CPU wakeup with event mask register"]
    pub exti_emr2: EXTI_EMR2,
    _reserved26: [u8; 8usize],
    #[doc = "0xa0 - Contains register bits for configurable events and direct events."]
    pub exti_imr3: EXTI_IMR3,
    #[doc = "0xa4 - EXTI CPU wakeup with event mask register"]
    pub exti_emr3: EXTI_EMR3,
    _reserved28: [u8; 24usize],
    #[doc = "0xc0 - Contains register bits for configurable events and Direct events."]
    pub exti_c2imr1: EXTI_C2IMR1,
    #[doc = "0xc4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr1: EXTI_C2EMR1,
    _reserved30: [u8; 8usize],
    #[doc = "0xd0 - Contains register bits for configurable events and direct events."]
    pub exti_c2imr2: EXTI_C2IMR2,
    #[doc = "0xd4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr2: EXTI_C2EMR2,
    _reserved32: [u8; 8usize],
    #[doc = "0xe0 - Contains register bits for configurable events and direct events."]
    pub exti_c2imr3: EXTI_C2IMR3,
    #[doc = "0xe4 - EXTI CPU2 wakeup with event mask register"]
    pub exti_c2emr3: EXTI_C2EMR3,
    _reserved34: [u8; 728usize],
    #[doc = "0x3c0 - EXTI hardware configuration register 13"]
    pub exti_hwcfgr13: EXTI_HWCFGR13,
    #[doc = "0x3c4 - EXTI hardware configuration register 12"]
    pub exti_hwcfgr12: EXTI_HWCFGR12,
    #[doc = "0x3c8 - EXTI hardware configuration register 11"]
    pub exti_hwcfgr11: EXTI_HWCFGR11,
    #[doc = "0x3cc - EXTI hardware configuration register 10"]
    pub exti_hwcfgr10: EXTI_HWCFGR10,
    #[doc = "0x3d0 - EXTI hardware configuration register 9"]
    pub exti_hwcfgr9: EXTI_HWCFGR9,
    #[doc = "0x3d4 - EXTI hardware configuration register 8"]
    pub exti_hwcfgr8: EXTI_HWCFGR8,
    #[doc = "0x3d8 - EXTI hardware configuration register 7"]
    pub exti_hwcfgr7: EXTI_HWCFGR7,
    #[doc = "0x3dc - EXTI hardware configuration register 6"]
    pub exti_hwcfgr6: EXTI_HWCFGR6,
    #[doc = "0x3e0 - EXTI hardware configuration register 5"]
    pub exti_hwcfgr5: EXTI_HWCFGR5,
    #[doc = "0x3e4 - EXTI hardware configuration register 4"]
    pub exti_hwcfgr4: EXTI_HWCFGR4,
    #[doc = "0x3e8 - EXTI hardware configuration register 3"]
    pub exti_hwcfgr3: EXTI_HWCFGR3,
    #[doc = "0x3ec - EXTI hardware configuration register 2"]
    pub exti_hwcfgr2: EXTI_HWCFGR2,
    #[doc = "0x3f0 - EXTI hardware configuration register 1"]
    pub exti_hwcfgr1: EXTI_HWCFGR1,
    #[doc = "0x3f4 - EXTI IP version register"]
    pub exti_verr: EXTI_VERR,
    #[doc = "0x3f8 - EXTI identification register"]
    pub exti_ipidr: EXTI_IPIDR,
    #[doc = "0x3fc - EXTI size ID register"]
    pub exti_sidr: EXTI_SIDR,
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr1](exti_rtsr1) module"]
pub type EXTI_RTSR1 = crate::Reg<u32, _EXTI_RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RTSR1;
#[doc = "`read()` method returns [exti_rtsr1::R](exti_rtsr1::R) reader structure"]
impl crate::Readable for EXTI_RTSR1 {}
#[doc = "`write(|w| ..)` method takes [exti_rtsr1::W](exti_rtsr1::W) writer structure"]
impl crate::Writable for EXTI_RTSR1 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr1;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr1](exti_ftsr1) module"]
pub type EXTI_FTSR1 = crate::Reg<u32, _EXTI_FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FTSR1;
#[doc = "`read()` method returns [exti_ftsr1::R](exti_ftsr1::R) reader structure"]
impl crate::Readable for EXTI_FTSR1 {}
#[doc = "`write(|w| ..)` method takes [exti_ftsr1::W](exti_ftsr1::W) writer structure"]
impl crate::Writable for EXTI_FTSR1 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr1;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier1](exti_swier1) module"]
pub type EXTI_SWIER1 = crate::Reg<u32, _EXTI_SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_SWIER1;
#[doc = "`read()` method returns [exti_swier1::R](exti_swier1::R) reader structure"]
impl crate::Readable for EXTI_SWIER1 {}
#[doc = "`write(|w| ..)` method takes [exti_swier1::W](exti_swier1::W) writer structure"]
impl crate::Writable for EXTI_SWIER1 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier1;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr1](exti_rpr1) module"]
pub type EXTI_RPR1 = crate::Reg<u32, _EXTI_RPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RPR1;
#[doc = "`read()` method returns [exti_rpr1::R](exti_rpr1::R) reader structure"]
impl crate::Readable for EXTI_RPR1 {}
#[doc = "`write(|w| ..)` method takes [exti_rpr1::W](exti_rpr1::W) writer structure"]
impl crate::Writable for EXTI_RPR1 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr1;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_fpr1](exti_fpr1) module"]
pub type EXTI_FPR1 = crate::Reg<u32, _EXTI_FPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FPR1;
#[doc = "`read()` method returns [exti_fpr1::R](exti_fpr1::R) reader structure"]
impl crate::Readable for EXTI_FPR1 {}
#[doc = "`write(|w| ..)` method takes [exti_fpr1::W](exti_fpr1::W) writer structure"]
impl crate::Writable for EXTI_FPR1 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr1;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr1](exti_tzenr1) module"]
pub type EXTI_TZENR1 = crate::Reg<u32, _EXTI_TZENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_TZENR1;
#[doc = "`read()` method returns [exti_tzenr1::R](exti_tzenr1::R) reader structure"]
impl crate::Readable for EXTI_TZENR1 {}
#[doc = "`write(|w| ..)` method takes [exti_tzenr1::W](exti_tzenr1::W) writer structure"]
impl crate::Writable for EXTI_TZENR1 {}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr1;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr2](exti_rtsr2) module"]
pub type EXTI_RTSR2 = crate::Reg<u32, _EXTI_RTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RTSR2;
#[doc = "`read()` method returns [exti_rtsr2::R](exti_rtsr2::R) reader structure"]
impl crate::Readable for EXTI_RTSR2 {}
#[doc = "`write(|w| ..)` method takes [exti_rtsr2::W](exti_rtsr2::W) writer structure"]
impl crate::Writable for EXTI_RTSR2 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr2;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr2](exti_ftsr2) module"]
pub type EXTI_FTSR2 = crate::Reg<u32, _EXTI_FTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FTSR2;
#[doc = "`read()` method returns [exti_ftsr2::R](exti_ftsr2::R) reader structure"]
impl crate::Readable for EXTI_FTSR2 {}
#[doc = "`write(|w| ..)` method takes [exti_ftsr2::W](exti_ftsr2::W) writer structure"]
impl crate::Writable for EXTI_FTSR2 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr2;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier2](exti_swier2) module"]
pub type EXTI_SWIER2 = crate::Reg<u32, _EXTI_SWIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_SWIER2;
#[doc = "`read()` method returns [exti_swier2::R](exti_swier2::R) reader structure"]
impl crate::Readable for EXTI_SWIER2 {}
#[doc = "`write(|w| ..)` method takes [exti_swier2::W](exti_swier2::W) writer structure"]
impl crate::Writable for EXTI_SWIER2 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier2;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr2](exti_rpr2) module"]
pub type EXTI_RPR2 = crate::Reg<u32, _EXTI_RPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RPR2;
#[doc = "`read()` method returns [exti_rpr2::R](exti_rpr2::R) reader structure"]
impl crate::Readable for EXTI_RPR2 {}
#[doc = "`write(|w| ..)` method takes [exti_rpr2::W](exti_rpr2::W) writer structure"]
impl crate::Writable for EXTI_RPR2 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr2;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_fpr2](exti_fpr2) module"]
pub type EXTI_FPR2 = crate::Reg<u32, _EXTI_FPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FPR2;
#[doc = "`read()` method returns [exti_fpr2::R](exti_fpr2::R) reader structure"]
impl crate::Readable for EXTI_FPR2 {}
#[doc = "`write(|w| ..)` method takes [exti_fpr2::W](exti_fpr2::W) writer structure"]
impl crate::Writable for EXTI_FPR2 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr2;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr2](exti_tzenr2) module"]
pub type EXTI_TZENR2 = crate::Reg<u32, _EXTI_TZENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_TZENR2;
#[doc = "`read()` method returns [exti_tzenr2::R](exti_tzenr2::R) reader structure"]
impl crate::Readable for EXTI_TZENR2 {}
#[doc = "`write(|w| ..)` method takes [exti_tzenr2::W](exti_tzenr2::W) writer structure"]
impl crate::Writable for EXTI_TZENR2 {}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr2;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr3](exti_rtsr3) module"]
pub type EXTI_RTSR3 = crate::Reg<u32, _EXTI_RTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RTSR3;
#[doc = "`read()` method returns [exti_rtsr3::R](exti_rtsr3::R) reader structure"]
impl crate::Readable for EXTI_RTSR3 {}
#[doc = "`write(|w| ..)` method takes [exti_rtsr3::W](exti_rtsr3::W) writer structure"]
impl crate::Writable for EXTI_RTSR3 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rtsr3;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr3](exti_ftsr3) module"]
pub type EXTI_FTSR3 = crate::Reg<u32, _EXTI_FTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FTSR3;
#[doc = "`read()` method returns [exti_ftsr3::R](exti_ftsr3::R) reader structure"]
impl crate::Readable for EXTI_FTSR3 {}
#[doc = "`write(|w| ..)` method takes [exti_ftsr3::W](exti_ftsr3::W) writer structure"]
impl crate::Writable for EXTI_FTSR3 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_ftsr3;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier3](exti_swier3) module"]
pub type EXTI_SWIER3 = crate::Reg<u32, _EXTI_SWIER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_SWIER3;
#[doc = "`read()` method returns [exti_swier3::R](exti_swier3::R) reader structure"]
impl crate::Readable for EXTI_SWIER3 {}
#[doc = "`write(|w| ..)` method takes [exti_swier3::W](exti_swier3::W) writer structure"]
impl crate::Writable for EXTI_SWIER3 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_swier3;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr3](exti_rpr3) module"]
pub type EXTI_RPR3 = crate::Reg<u32, _EXTI_RPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_RPR3;
#[doc = "`read()` method returns [exti_rpr3::R](exti_rpr3::R) reader structure"]
impl crate::Readable for EXTI_RPR3 {}
#[doc = "`write(|w| ..)` method takes [exti_rpr3::W](exti_rpr3::W) writer structure"]
impl crate::Writable for EXTI_RPR3 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_rpr3;
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_fpr3](exti_fpr3) module"]
pub type EXTI_FPR3 = crate::Reg<u32, _EXTI_FPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_FPR3;
#[doc = "`read()` method returns [exti_fpr3::R](exti_fpr3::R) reader structure"]
impl crate::Readable for EXTI_FPR3 {}
#[doc = "`write(|w| ..)` method takes [exti_fpr3::W](exti_fpr3::W) writer structure"]
impl crate::Writable for EXTI_FPR3 {}
#[doc = "Contains only register bits for configurable events."]
pub mod exti_fpr3;
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr3](exti_tzenr3) module"]
pub type EXTI_TZENR3 = crate::Reg<u32, _EXTI_TZENR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_TZENR3;
#[doc = "`read()` method returns [exti_tzenr3::R](exti_tzenr3::R) reader structure"]
impl crate::Readable for EXTI_TZENR3 {}
#[doc = "`write(|w| ..)` method takes [exti_tzenr3::W](exti_tzenr3::W) writer structure"]
impl crate::Writable for EXTI_TZENR3 {}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events."]
pub mod exti_tzenr3;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr1](exti_exticr1) module"]
pub type EXTI_EXTICR1 = crate::Reg<u32, _EXTI_EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EXTICR1;
#[doc = "`read()` method returns [exti_exticr1::R](exti_exticr1::R) reader structure"]
impl crate::Readable for EXTI_EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exti_exticr1::W](exti_exticr1::W) writer structure"]
impl crate::Writable for EXTI_EXTICR1 {}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr1;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr2](exti_exticr2) module"]
pub type EXTI_EXTICR2 = crate::Reg<u32, _EXTI_EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EXTICR2;
#[doc = "`read()` method returns [exti_exticr2::R](exti_exticr2::R) reader structure"]
impl crate::Readable for EXTI_EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exti_exticr2::W](exti_exticr2::W) writer structure"]
impl crate::Writable for EXTI_EXTICR2 {}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr2;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr3](exti_exticr3) module"]
pub type EXTI_EXTICR3 = crate::Reg<u32, _EXTI_EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EXTICR3;
#[doc = "`read()` method returns [exti_exticr3::R](exti_exticr3::R) reader structure"]
impl crate::Readable for EXTI_EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exti_exticr3::W](exti_exticr3::W) writer structure"]
impl crate::Writable for EXTI_EXTICR3 {}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr3;
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr4](exti_exticr4) module"]
pub type EXTI_EXTICR4 = crate::Reg<u32, _EXTI_EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EXTICR4;
#[doc = "`read()` method returns [exti_exticr4::R](exti_exticr4::R) reader structure"]
impl crate::Readable for EXTI_EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exti_exticr4::W](exti_exticr4::W) writer structure"]
impl crate::Writable for EXTI_EXTICR4 {}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration."]
pub mod exti_exticr4;
#[doc = "Contains register bits for configurable events and Direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_imr1](exti_imr1) module"]
pub type EXTI_IMR1 = crate::Reg<u32, _EXTI_IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_IMR1;
#[doc = "`read()` method returns [exti_imr1::R](exti_imr1::R) reader structure"]
impl crate::Readable for EXTI_IMR1 {}
#[doc = "`write(|w| ..)` method takes [exti_imr1::W](exti_imr1::W) writer structure"]
impl crate::Writable for EXTI_IMR1 {}
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_imr1;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_emr1](exti_emr1) module"]
pub type EXTI_EMR1 = crate::Reg<u32, _EXTI_EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EMR1;
#[doc = "`read()` method returns [exti_emr1::R](exti_emr1::R) reader structure"]
impl crate::Readable for EXTI_EMR1 {}
#[doc = "`write(|w| ..)` method takes [exti_emr1::W](exti_emr1::W) writer structure"]
impl crate::Writable for EXTI_EMR1 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr1;
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_imr2](exti_imr2) module"]
pub type EXTI_IMR2 = crate::Reg<u32, _EXTI_IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_IMR2;
#[doc = "`read()` method returns [exti_imr2::R](exti_imr2::R) reader structure"]
impl crate::Readable for EXTI_IMR2 {}
#[doc = "`write(|w| ..)` method takes [exti_imr2::W](exti_imr2::W) writer structure"]
impl crate::Writable for EXTI_IMR2 {}
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr2;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_emr2](exti_emr2) module"]
pub type EXTI_EMR2 = crate::Reg<u32, _EXTI_EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EMR2;
#[doc = "`read()` method returns [exti_emr2::R](exti_emr2::R) reader structure"]
impl crate::Readable for EXTI_EMR2 {}
#[doc = "`write(|w| ..)` method takes [exti_emr2::W](exti_emr2::W) writer structure"]
impl crate::Writable for EXTI_EMR2 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr2;
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_imr3](exti_imr3) module"]
pub type EXTI_IMR3 = crate::Reg<u32, _EXTI_IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_IMR3;
#[doc = "`read()` method returns [exti_imr3::R](exti_imr3::R) reader structure"]
impl crate::Readable for EXTI_IMR3 {}
#[doc = "`write(|w| ..)` method takes [exti_imr3::W](exti_imr3::W) writer structure"]
impl crate::Writable for EXTI_IMR3 {}
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_imr3;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_emr3](exti_emr3) module"]
pub type EXTI_EMR3 = crate::Reg<u32, _EXTI_EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EMR3;
#[doc = "`read()` method returns [exti_emr3::R](exti_emr3::R) reader structure"]
impl crate::Readable for EXTI_EMR3 {}
#[doc = "`write(|w| ..)` method takes [exti_emr3::W](exti_emr3::W) writer structure"]
impl crate::Writable for EXTI_EMR3 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod exti_emr3;
#[doc = "Contains register bits for configurable events and Direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr1](exti_c2imr1) module"]
pub type EXTI_C2IMR1 = crate::Reg<u32, _EXTI_C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2IMR1;
#[doc = "`read()` method returns [exti_c2imr1::R](exti_c2imr1::R) reader structure"]
impl crate::Readable for EXTI_C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [exti_c2imr1::W](exti_c2imr1::W) writer structure"]
impl crate::Writable for EXTI_C2IMR1 {}
#[doc = "Contains register bits for configurable events and Direct events."]
pub mod exti_c2imr1;
#[doc = "EXTI CPU2 wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2emr1](exti_c2emr1) module"]
pub type EXTI_C2EMR1 = crate::Reg<u32, _EXTI_C2EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2EMR1;
#[doc = "`read()` method returns [exti_c2emr1::R](exti_c2emr1::R) reader structure"]
impl crate::Readable for EXTI_C2EMR1 {}
#[doc = "`write(|w| ..)` method takes [exti_c2emr1::W](exti_c2emr1::W) writer structure"]
impl crate::Writable for EXTI_C2EMR1 {}
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr1;
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr2](exti_c2imr2) module"]
pub type EXTI_C2IMR2 = crate::Reg<u32, _EXTI_C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2IMR2;
#[doc = "`read()` method returns [exti_c2imr2::R](exti_c2imr2::R) reader structure"]
impl crate::Readable for EXTI_C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [exti_c2imr2::W](exti_c2imr2::W) writer structure"]
impl crate::Writable for EXTI_C2IMR2 {}
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr2;
#[doc = "EXTI CPU2 wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2emr2](exti_c2emr2) module"]
pub type EXTI_C2EMR2 = crate::Reg<u32, _EXTI_C2EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2EMR2;
#[doc = "`read()` method returns [exti_c2emr2::R](exti_c2emr2::R) reader structure"]
impl crate::Readable for EXTI_C2EMR2 {}
#[doc = "`write(|w| ..)` method takes [exti_c2emr2::W](exti_c2emr2::W) writer structure"]
impl crate::Writable for EXTI_C2EMR2 {}
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr2;
#[doc = "Contains register bits for configurable events and direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr3](exti_c2imr3) module"]
pub type EXTI_C2IMR3 = crate::Reg<u32, _EXTI_C2IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2IMR3;
#[doc = "`read()` method returns [exti_c2imr3::R](exti_c2imr3::R) reader structure"]
impl crate::Readable for EXTI_C2IMR3 {}
#[doc = "`write(|w| ..)` method takes [exti_c2imr3::W](exti_c2imr3::W) writer structure"]
impl crate::Writable for EXTI_C2IMR3 {}
#[doc = "Contains register bits for configurable events and direct events."]
pub mod exti_c2imr3;
#[doc = "EXTI CPU2 wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2emr3](exti_c2emr3) module"]
pub type EXTI_C2EMR3 = crate::Reg<u32, _EXTI_C2EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_C2EMR3;
#[doc = "`read()` method returns [exti_c2emr3::R](exti_c2emr3::R) reader structure"]
impl crate::Readable for EXTI_C2EMR3 {}
#[doc = "`write(|w| ..)` method takes [exti_c2emr3::W](exti_c2emr3::W) writer structure"]
impl crate::Writable for EXTI_C2EMR3 {}
#[doc = "EXTI CPU2 wakeup with event mask register"]
pub mod exti_c2emr3;
#[doc = "EXTI hardware configuration register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr13](exti_hwcfgr13) module"]
pub type EXTI_HWCFGR13 = crate::Reg<u32, _EXTI_HWCFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR13;
#[doc = "`read()` method returns [exti_hwcfgr13::R](exti_hwcfgr13::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR13 {}
#[doc = "EXTI hardware configuration register 13"]
pub mod exti_hwcfgr13;
#[doc = "EXTI hardware configuration register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr12](exti_hwcfgr12) module"]
pub type EXTI_HWCFGR12 = crate::Reg<u32, _EXTI_HWCFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR12;
#[doc = "`read()` method returns [exti_hwcfgr12::R](exti_hwcfgr12::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR12 {}
#[doc = "EXTI hardware configuration register 12"]
pub mod exti_hwcfgr12;
#[doc = "EXTI hardware configuration register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr11](exti_hwcfgr11) module"]
pub type EXTI_HWCFGR11 = crate::Reg<u32, _EXTI_HWCFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR11;
#[doc = "`read()` method returns [exti_hwcfgr11::R](exti_hwcfgr11::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR11 {}
#[doc = "EXTI hardware configuration register 11"]
pub mod exti_hwcfgr11;
#[doc = "EXTI hardware configuration register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr10](exti_hwcfgr10) module"]
pub type EXTI_HWCFGR10 = crate::Reg<u32, _EXTI_HWCFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR10;
#[doc = "`read()` method returns [exti_hwcfgr10::R](exti_hwcfgr10::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR10 {}
#[doc = "EXTI hardware configuration register 10"]
pub mod exti_hwcfgr10;
#[doc = "EXTI hardware configuration register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr9](exti_hwcfgr9) module"]
pub type EXTI_HWCFGR9 = crate::Reg<u32, _EXTI_HWCFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR9;
#[doc = "`read()` method returns [exti_hwcfgr9::R](exti_hwcfgr9::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR9 {}
#[doc = "EXTI hardware configuration register 9"]
pub mod exti_hwcfgr9;
#[doc = "EXTI hardware configuration register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr8](exti_hwcfgr8) module"]
pub type EXTI_HWCFGR8 = crate::Reg<u32, _EXTI_HWCFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR8;
#[doc = "`read()` method returns [exti_hwcfgr8::R](exti_hwcfgr8::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR8 {}
#[doc = "EXTI hardware configuration register 8"]
pub mod exti_hwcfgr8;
#[doc = "EXTI hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr7](exti_hwcfgr7) module"]
pub type EXTI_HWCFGR7 = crate::Reg<u32, _EXTI_HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR7;
#[doc = "`read()` method returns [exti_hwcfgr7::R](exti_hwcfgr7::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR7 {}
#[doc = "EXTI hardware configuration register 7"]
pub mod exti_hwcfgr7;
#[doc = "EXTI hardware configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr6](exti_hwcfgr6) module"]
pub type EXTI_HWCFGR6 = crate::Reg<u32, _EXTI_HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR6;
#[doc = "`read()` method returns [exti_hwcfgr6::R](exti_hwcfgr6::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR6 {}
#[doc = "EXTI hardware configuration register 6"]
pub mod exti_hwcfgr6;
#[doc = "EXTI hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr5](exti_hwcfgr5) module"]
pub type EXTI_HWCFGR5 = crate::Reg<u32, _EXTI_HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR5;
#[doc = "`read()` method returns [exti_hwcfgr5::R](exti_hwcfgr5::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR5 {}
#[doc = "EXTI hardware configuration register 5"]
pub mod exti_hwcfgr5;
#[doc = "EXTI hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr4](exti_hwcfgr4) module"]
pub type EXTI_HWCFGR4 = crate::Reg<u32, _EXTI_HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR4;
#[doc = "`read()` method returns [exti_hwcfgr4::R](exti_hwcfgr4::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR4 {}
#[doc = "EXTI hardware configuration register 4"]
pub mod exti_hwcfgr4;
#[doc = "EXTI hardware configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr3](exti_hwcfgr3) module"]
pub type EXTI_HWCFGR3 = crate::Reg<u32, _EXTI_HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR3;
#[doc = "`read()` method returns [exti_hwcfgr3::R](exti_hwcfgr3::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR3 {}
#[doc = "EXTI hardware configuration register 3"]
pub mod exti_hwcfgr3;
#[doc = "EXTI hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr2](exti_hwcfgr2) module"]
pub type EXTI_HWCFGR2 = crate::Reg<u32, _EXTI_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR2;
#[doc = "`read()` method returns [exti_hwcfgr2::R](exti_hwcfgr2::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR2 {}
#[doc = "EXTI hardware configuration register 2"]
pub mod exti_hwcfgr2;
#[doc = "EXTI hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr1](exti_hwcfgr1) module"]
pub type EXTI_HWCFGR1 = crate::Reg<u32, _EXTI_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_HWCFGR1;
#[doc = "`read()` method returns [exti_hwcfgr1::R](exti_hwcfgr1::R) reader structure"]
impl crate::Readable for EXTI_HWCFGR1 {}
#[doc = "EXTI hardware configuration register 1"]
pub mod exti_hwcfgr1;
#[doc = "EXTI IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_verr](exti_verr) module"]
pub type EXTI_VERR = crate::Reg<u32, _EXTI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_VERR;
#[doc = "`read()` method returns [exti_verr::R](exti_verr::R) reader structure"]
impl crate::Readable for EXTI_VERR {}
#[doc = "EXTI IP version register"]
pub mod exti_verr;
#[doc = "EXTI identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ipidr](exti_ipidr) module"]
pub type EXTI_IPIDR = crate::Reg<u32, _EXTI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_IPIDR;
#[doc = "`read()` method returns [exti_ipidr::R](exti_ipidr::R) reader structure"]
impl crate::Readable for EXTI_IPIDR {}
#[doc = "EXTI identification register"]
pub mod exti_ipidr;
#[doc = "EXTI size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_sidr](exti_sidr) module"]
pub type EXTI_SIDR = crate::Reg<u32, _EXTI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_SIDR;
#[doc = "`read()` method returns [exti_sidr::R](exti_sidr::R) reader structure"]
impl crate::Readable for EXTI_SIDR {}
#[doc = "EXTI size ID register"]
pub mod exti_sidr;
