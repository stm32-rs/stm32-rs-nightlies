#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI pending register"]
    pub pr1: PR1,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - rising trigger selection register"]
    pub rtsr2: RTSR2,
    #[doc = "0x24 - falling trigger selection register"]
    pub ftsr2: FTSR2,
    #[doc = "0x28 - software interrupt event register"]
    pub swier2: SWIER2,
    #[doc = "0x2c - pending register"]
    pub pr2: PR2,
    _reserved8: [u8; 80usize],
    #[doc = "0x80 - CPUm wakeup with interrupt mask register"]
    pub c1imr1: C1IMR1,
    #[doc = "0x84 - CPUm wakeup with event mask register"]
    pub c1emr1: C1EMR1,
    _reserved10: [u8; 8usize],
    #[doc = "0x90 - CPUm wakeup with interrupt mask register"]
    pub c1imr2: C1IMR2,
    #[doc = "0x94 - CPUm wakeup with event mask register"]
    pub c1emr2: C1EMR2,
    _reserved12: [u8; 40usize],
    #[doc = "0xc0 - CPUm wakeup with interrupt mask register"]
    pub c2imr1: C2IMR1,
    #[doc = "0xc4 - CPUm wakeup with event mask register"]
    pub c2emr1: C2EMR1,
    _reserved14: [u8; 8usize],
    #[doc = "0xd0 - CPUm wakeup with interrupt mask register"]
    pub c2imr2: C2IMR2,
    #[doc = "0xd4 - CPUm wakeup with event mask register"]
    pub c2emr2: C2EMR2,
    _reserved16: [u8; 768usize],
    #[doc = "0x3d8 - EXTI Hardware configuration registers"]
    pub hwcfgr7: HWCFGR7,
    #[doc = "0x3dc - Hardware configuration registers"]
    pub hwcfgr6: HWCFGR6,
    #[doc = "0x3e0 - Hardware configuration registers"]
    pub hwcfgr5: HWCFGR5,
    #[doc = "0x3e4 - Hardware configuration registers"]
    pub hwcfgr4: HWCFGR4,
    #[doc = "0x3e8 - Hardware configuration registers"]
    pub hwcfgr3: HWCFGR3,
    #[doc = "0x3ec - Hardware configuration registers"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - Size ID register"]
    pub sidr: SIDR,
}
#[doc = "rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr1](rtsr1) module"]
pub type RTSR1 = crate::Reg<u32, _RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR1;
#[doc = "`read()` method returns [rtsr1::R](rtsr1::R) reader structure"]
impl crate::Readable for RTSR1 {}
#[doc = "`write(|w| ..)` method takes [rtsr1::W](rtsr1::W) writer structure"]
impl crate::Writable for RTSR1 {}
#[doc = "rising trigger selection register"]
pub mod rtsr1;
#[doc = "falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](ftsr1) module"]
pub type FTSR1 = crate::Reg<u32, _FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR1;
#[doc = "`read()` method returns [ftsr1::R](ftsr1::R) reader structure"]
impl crate::Readable for FTSR1 {}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](ftsr1::W) writer structure"]
impl crate::Writable for FTSR1 {}
#[doc = "falling trigger selection register"]
pub mod ftsr1;
#[doc = "software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](swier1) module"]
pub type SWIER1 = crate::Reg<u32, _SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER1;
#[doc = "`read()` method returns [swier1::R](swier1::R) reader structure"]
impl crate::Readable for SWIER1 {}
#[doc = "`write(|w| ..)` method takes [swier1::W](swier1::W) writer structure"]
impl crate::Writable for SWIER1 {}
#[doc = "software interrupt event register"]
pub mod swier1;
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](pr1) module"]
pub type PR1 = crate::Reg<u32, _PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1;
#[doc = "`read()` method returns [pr1::R](pr1::R) reader structure"]
impl crate::Readable for PR1 {}
#[doc = "`write(|w| ..)` method takes [pr1::W](pr1::W) writer structure"]
impl crate::Writable for PR1 {}
#[doc = "EXTI pending register"]
pub mod pr1;
#[doc = "rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](rtsr2) module"]
pub type RTSR2 = crate::Reg<u32, _RTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR2;
#[doc = "`read()` method returns [rtsr2::R](rtsr2::R) reader structure"]
impl crate::Readable for RTSR2 {}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](rtsr2::W) writer structure"]
impl crate::Writable for RTSR2 {}
#[doc = "rising trigger selection register"]
pub mod rtsr2;
#[doc = "falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](ftsr2) module"]
pub type FTSR2 = crate::Reg<u32, _FTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR2;
#[doc = "`read()` method returns [ftsr2::R](ftsr2::R) reader structure"]
impl crate::Readable for FTSR2 {}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](ftsr2::W) writer structure"]
impl crate::Writable for FTSR2 {}
#[doc = "falling trigger selection register"]
pub mod ftsr2;
#[doc = "software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](swier2) module"]
pub type SWIER2 = crate::Reg<u32, _SWIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER2;
#[doc = "`read()` method returns [swier2::R](swier2::R) reader structure"]
impl crate::Readable for SWIER2 {}
#[doc = "`write(|w| ..)` method takes [swier2::W](swier2::W) writer structure"]
impl crate::Writable for SWIER2 {}
#[doc = "software interrupt event register"]
pub mod swier2;
#[doc = "pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](pr2) module"]
pub type PR2 = crate::Reg<u32, _PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2;
#[doc = "`read()` method returns [pr2::R](pr2::R) reader structure"]
impl crate::Readable for PR2 {}
#[doc = "`write(|w| ..)` method takes [pr2::W](pr2::W) writer structure"]
impl crate::Writable for PR2 {}
#[doc = "pending register"]
pub mod pr2;
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr1](c1imr1) module"]
pub type C1IMR1 = crate::Reg<u32, _C1IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IMR1;
#[doc = "`read()` method returns [c1imr1::R](c1imr1::R) reader structure"]
impl crate::Readable for C1IMR1 {}
#[doc = "`write(|w| ..)` method takes [c1imr1::W](c1imr1::W) writer structure"]
impl crate::Writable for C1IMR1 {}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c1imr1;
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](c2imr1) module"]
pub type C2IMR1 = crate::Reg<u32, _C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR1;
#[doc = "`read()` method returns [c2imr1::R](c2imr1::R) reader structure"]
impl crate::Readable for C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](c2imr1::W) writer structure"]
impl crate::Writable for C2IMR1 {}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr1;
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr1](c1emr1) module"]
pub type C1EMR1 = crate::Reg<u32, _C1EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1EMR1;
#[doc = "`read()` method returns [c1emr1::R](c1emr1::R) reader structure"]
impl crate::Readable for C1EMR1 {}
#[doc = "`write(|w| ..)` method takes [c1emr1::W](c1emr1::W) writer structure"]
impl crate::Writable for C1EMR1 {}
#[doc = "CPUm wakeup with event mask register"]
pub mod c1emr1;
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr1](c2emr1) module"]
pub type C2EMR1 = crate::Reg<u32, _C2EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR1;
#[doc = "`read()` method returns [c2emr1::R](c2emr1::R) reader structure"]
impl crate::Readable for C2EMR1 {}
#[doc = "`write(|w| ..)` method takes [c2emr1::W](c2emr1::W) writer structure"]
impl crate::Writable for C2EMR1 {}
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr1;
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1imr2](c1imr2) module"]
pub type C1IMR2 = crate::Reg<u32, _C1IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IMR2;
#[doc = "`read()` method returns [c1imr2::R](c1imr2::R) reader structure"]
impl crate::Readable for C1IMR2 {}
#[doc = "`write(|w| ..)` method takes [c1imr2::W](c1imr2::W) writer structure"]
impl crate::Writable for C1IMR2 {}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c1imr2;
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](c2imr2) module"]
pub type C2IMR2 = crate::Reg<u32, _C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR2;
#[doc = "`read()` method returns [c2imr2::R](c2imr2::R) reader structure"]
impl crate::Readable for C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](c2imr2::W) writer structure"]
impl crate::Writable for C2IMR2 {}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1emr2](c1emr2) module"]
pub type C1EMR2 = crate::Reg<u32, _C1EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1EMR2;
#[doc = "`read()` method returns [c1emr2::R](c1emr2::R) reader structure"]
impl crate::Readable for C1EMR2 {}
#[doc = "`write(|w| ..)` method takes [c1emr2::W](c1emr2::W) writer structure"]
impl crate::Writable for C1EMR2 {}
#[doc = "CPUm wakeup with event mask register"]
pub mod c1emr2;
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr2](c2emr2) module"]
pub type C2EMR2 = crate::Reg<u32, _C2EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR2;
#[doc = "`read()` method returns [c2emr2::R](c2emr2::R) reader structure"]
impl crate::Readable for C2EMR2 {}
#[doc = "`write(|w| ..)` method takes [c2emr2::W](c2emr2::W) writer structure"]
impl crate::Writable for C2EMR2 {}
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr2;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr5](hwcfgr5) module"]
pub type HWCFGR5 = crate::Reg<u32, _HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR5;
#[doc = "`read()` method returns [hwcfgr5::R](hwcfgr5::R) reader structure"]
impl crate::Readable for HWCFGR5 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr6](hwcfgr6) module"]
pub type HWCFGR6 = crate::Reg<u32, _HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR6;
#[doc = "`read()` method returns [hwcfgr6::R](hwcfgr6::R) reader structure"]
impl crate::Readable for HWCFGR6 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "EXTI Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr7](hwcfgr7) module"]
pub type HWCFGR7 = crate::Reg<u32, _HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR7;
#[doc = "`read()` method returns [hwcfgr7::R](hwcfgr7::R) reader structure"]
impl crate::Readable for HWCFGR7 {}
#[doc = "EXTI Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](hwcfgr2) module"]
pub type HWCFGR2 = crate::Reg<u32, _HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR2;
#[doc = "`read()` method returns [hwcfgr2::R](hwcfgr2::R) reader structure"]
impl crate::Readable for HWCFGR2 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr3](hwcfgr3) module"]
pub type HWCFGR3 = crate::Reg<u32, _HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR3;
#[doc = "`read()` method returns [hwcfgr3::R](hwcfgr3::R) reader structure"]
impl crate::Readable for HWCFGR3 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr4](hwcfgr4) module"]
pub type HWCFGR4 = crate::Reg<u32, _HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR4;
#[doc = "`read()` method returns [hwcfgr4::R](hwcfgr4::R) reader structure"]
impl crate::Readable for HWCFGR4 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "Hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](hwcfgr1) module"]
pub type HWCFGR1 = crate::Reg<u32, _HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR1;
#[doc = "`read()` method returns [hwcfgr1::R](hwcfgr1::R) reader structure"]
impl crate::Readable for HWCFGR1 {}
#[doc = "Hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "EXTI IP Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "Identification register"]
pub mod ipidr;
#[doc = "Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "Size ID register"]
pub mod sidr;
