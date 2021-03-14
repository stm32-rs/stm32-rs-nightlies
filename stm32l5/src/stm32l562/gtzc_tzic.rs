#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZIC interrupt enable register 1"]
    pub ier1: IER1,
    #[doc = "0x04 - TZIC interrupt enable register 2"]
    pub ier2: IER2,
    #[doc = "0x08 - TZIC interrupt enable register 3"]
    pub ier3: IER3,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - TZIC interrupt status register 1"]
    pub sr1: SR1,
    #[doc = "0x14 - TZIC interrupt status register 2"]
    pub sr2: SR2,
    #[doc = "0x18 - TZIC interrupt status register 3"]
    pub sr3: SR3,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - TZIC interrupt clear register 1"]
    pub fcr1: FCR1,
    #[doc = "0x24 - TZIC interrupt clear register 2"]
    pub fcr2: FCR2,
    #[doc = "0x28 - TZIC interrupt clear register 3"]
    pub fcr3: FCR3,
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`read()` method returns [ier1::R](ier1::R) reader structure"]
impl crate::Readable for IER1 {}
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "TZIC interrupt enable register 1"]
pub mod ier1;
#[doc = "TZIC interrupt enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier2](ier2) module"]
pub type IER2 = crate::Reg<u32, _IER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER2;
#[doc = "`read()` method returns [ier2::R](ier2::R) reader structure"]
impl crate::Readable for IER2 {}
#[doc = "`write(|w| ..)` method takes [ier2::W](ier2::W) writer structure"]
impl crate::Writable for IER2 {}
#[doc = "TZIC interrupt enable register 2"]
pub mod ier2;
#[doc = "TZIC interrupt enable register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier3](ier3) module"]
pub type IER3 = crate::Reg<u32, _IER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER3;
#[doc = "`read()` method returns [ier3::R](ier3::R) reader structure"]
impl crate::Readable for IER3 {}
#[doc = "`write(|w| ..)` method takes [ier3::W](ier3::W) writer structure"]
impl crate::Writable for IER3 {}
#[doc = "TZIC interrupt enable register 3"]
pub mod ier3;
#[doc = "TZIC interrupt status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "TZIC interrupt status register 1"]
pub mod sr1;
#[doc = "TZIC interrupt status register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](sr2) module"]
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
#[doc = "`read()` method returns [sr2::R](sr2::R) reader structure"]
impl crate::Readable for SR2 {}
#[doc = "`write(|w| ..)` method takes [sr2::W](sr2::W) writer structure"]
impl crate::Writable for SR2 {}
#[doc = "TZIC interrupt status register 2"]
pub mod sr2;
#[doc = "TZIC interrupt status register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr3](sr3) module"]
pub type SR3 = crate::Reg<u32, _SR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR3;
#[doc = "`read()` method returns [sr3::R](sr3::R) reader structure"]
impl crate::Readable for SR3 {}
#[doc = "`write(|w| ..)` method takes [sr3::W](sr3::W) writer structure"]
impl crate::Writable for SR3 {}
#[doc = "TZIC interrupt status register 3"]
pub mod sr3;
#[doc = "TZIC interrupt clear register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr1](fcr1) module"]
pub type FCR1 = crate::Reg<u32, _FCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR1;
#[doc = "`write(|w| ..)` method takes [fcr1::W](fcr1::W) writer structure"]
impl crate::Writable for FCR1 {}
#[doc = "TZIC interrupt clear register 1"]
pub mod fcr1;
#[doc = "TZIC interrupt clear register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr2](fcr2) module"]
pub type FCR2 = crate::Reg<u32, _FCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR2;
#[doc = "`read()` method returns [fcr2::R](fcr2::R) reader structure"]
impl crate::Readable for FCR2 {}
#[doc = "`write(|w| ..)` method takes [fcr2::W](fcr2::W) writer structure"]
impl crate::Writable for FCR2 {}
#[doc = "TZIC interrupt clear register 2"]
pub mod fcr2;
#[doc = "TZIC interrupt clear register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr3](fcr3) module"]
pub type FCR3 = crate::Reg<u32, _FCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR3;
#[doc = "`read()` method returns [fcr3::R](fcr3::R) reader structure"]
impl crate::Readable for FCR3 {}
#[doc = "`write(|w| ..)` method takes [fcr3::W](fcr3::W) writer structure"]
impl crate::Writable for FCR3 {}
#[doc = "TZIC interrupt clear register 3"]
pub mod fcr3;
