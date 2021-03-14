#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Power control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Power control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - Power control register 4"]
    pub cr4: CR4,
    #[doc = "0x10 - Power status register 1"]
    pub sr1: SR1,
    #[doc = "0x14 - Power status register 2"]
    pub sr2: SR2,
    #[doc = "0x18 - Power status clear register"]
    pub scr: SCR,
    #[doc = "0x1c - Power control register 5"]
    pub cr5: CR5,
    #[doc = "0x20 - Power Port A pull-up control register"]
    pub pucra: PUCRA,
    #[doc = "0x24 - Power Port A pull-down control register"]
    pub pdcra: PDCRA,
    #[doc = "0x28 - Power Port B pull-up control register"]
    pub pucrb: PUCRB,
    #[doc = "0x2c - Power Port B pull-down control register"]
    pub pdcrb: PDCRB,
    #[doc = "0x30 - Power Port C pull-up control register"]
    pub pucrc: PUCRC,
    #[doc = "0x34 - Power Port C pull-down control register"]
    pub pdcrc: PDCRC,
    #[doc = "0x38 - Power Port D pull-up control register"]
    pub pucrd: PUCRD,
    #[doc = "0x3c - Power Port D pull-down control register"]
    pub pdcrd: PDCRD,
    #[doc = "0x40 - Power Port E pull-up control register"]
    pub pucre: PUCRE,
    #[doc = "0x44 - Power Port E pull-down control register"]
    pub pdcre: PDCRE,
    _reserved18: [u8; 16usize],
    #[doc = "0x58 - Power Port H pull-up control register"]
    pub pucrh: PUCRH,
    #[doc = "0x5c - Power Port H pull-down control register"]
    pub pdcrh: PDCRH,
    _reserved20: [u8; 32usize],
    #[doc = "0x80 - CPU2 Power control register 1"]
    pub c2cr1: C2CR1,
    #[doc = "0x84 - CPU2 Power control register 3"]
    pub c2cr3: C2CR3,
    #[doc = "0x88 - Power status clear register"]
    pub extscr: EXTSCR,
}
#[doc = "Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](cr3) module"]
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
#[doc = "`read()` method returns [cr3::R](cr3::R) reader structure"]
impl crate::Readable for CR3 {}
#[doc = "`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure"]
impl crate::Writable for CR3 {}
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "Power control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr4](cr4) module"]
pub type CR4 = crate::Reg<u32, _CR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR4;
#[doc = "`read()` method returns [cr4::R](cr4::R) reader structure"]
impl crate::Readable for CR4 {}
#[doc = "`write(|w| ..)` method takes [cr4::W](cr4::W) writer structure"]
impl crate::Writable for CR4 {}
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "Power status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "Power status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](sr2) module"]
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
#[doc = "`read()` method returns [sr2::R](sr2::R) reader structure"]
impl crate::Readable for SR2 {}
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "Power status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "Power control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr5](cr5) module"]
pub type CR5 = crate::Reg<u32, _CR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR5;
#[doc = "`read()` method returns [cr5::R](cr5::R) reader structure"]
impl crate::Readable for CR5 {}
#[doc = "`write(|w| ..)` method takes [cr5::W](cr5::W) writer structure"]
impl crate::Writable for CR5 {}
#[doc = "Power control register 5"]
pub mod cr5;
#[doc = "Power Port A pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucra](pucra) module"]
pub type PUCRA = crate::Reg<u32, _PUCRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRA;
#[doc = "`read()` method returns [pucra::R](pucra::R) reader structure"]
impl crate::Readable for PUCRA {}
#[doc = "`write(|w| ..)` method takes [pucra::W](pucra::W) writer structure"]
impl crate::Writable for PUCRA {}
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "Power Port A pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcra](pdcra) module"]
pub type PDCRA = crate::Reg<u32, _PDCRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRA;
#[doc = "`read()` method returns [pdcra::R](pdcra::R) reader structure"]
impl crate::Readable for PDCRA {}
#[doc = "`write(|w| ..)` method takes [pdcra::W](pdcra::W) writer structure"]
impl crate::Writable for PDCRA {}
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "Power Port B pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrb](pucrb) module"]
pub type PUCRB = crate::Reg<u32, _PUCRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRB;
#[doc = "`read()` method returns [pucrb::R](pucrb::R) reader structure"]
impl crate::Readable for PUCRB {}
#[doc = "`write(|w| ..)` method takes [pucrb::W](pucrb::W) writer structure"]
impl crate::Writable for PUCRB {}
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "Power Port B pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrb](pdcrb) module"]
pub type PDCRB = crate::Reg<u32, _PDCRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRB;
#[doc = "`read()` method returns [pdcrb::R](pdcrb::R) reader structure"]
impl crate::Readable for PDCRB {}
#[doc = "`write(|w| ..)` method takes [pdcrb::W](pdcrb::W) writer structure"]
impl crate::Writable for PDCRB {}
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "Power Port C pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrc](pucrc) module"]
pub type PUCRC = crate::Reg<u32, _PUCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRC;
#[doc = "`read()` method returns [pucrc::R](pucrc::R) reader structure"]
impl crate::Readable for PUCRC {}
#[doc = "`write(|w| ..)` method takes [pucrc::W](pucrc::W) writer structure"]
impl crate::Writable for PUCRC {}
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "Power Port C pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrc](pdcrc) module"]
pub type PDCRC = crate::Reg<u32, _PDCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRC;
#[doc = "`read()` method returns [pdcrc::R](pdcrc::R) reader structure"]
impl crate::Readable for PDCRC {}
#[doc = "`write(|w| ..)` method takes [pdcrc::W](pdcrc::W) writer structure"]
impl crate::Writable for PDCRC {}
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "Power Port D pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrd](pucrd) module"]
pub type PUCRD = crate::Reg<u32, _PUCRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRD;
#[doc = "`read()` method returns [pucrd::R](pucrd::R) reader structure"]
impl crate::Readable for PUCRD {}
#[doc = "`write(|w| ..)` method takes [pucrd::W](pucrd::W) writer structure"]
impl crate::Writable for PUCRD {}
#[doc = "Power Port D pull-up control register"]
pub mod pucrd;
#[doc = "Power Port D pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrd](pdcrd) module"]
pub type PDCRD = crate::Reg<u32, _PDCRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRD;
#[doc = "`read()` method returns [pdcrd::R](pdcrd::R) reader structure"]
impl crate::Readable for PDCRD {}
#[doc = "`write(|w| ..)` method takes [pdcrd::W](pdcrd::W) writer structure"]
impl crate::Writable for PDCRD {}
#[doc = "Power Port D pull-down control register"]
pub mod pdcrd;
#[doc = "Power Port E pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucre](pucre) module"]
pub type PUCRE = crate::Reg<u32, _PUCRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRE;
#[doc = "`read()` method returns [pucre::R](pucre::R) reader structure"]
impl crate::Readable for PUCRE {}
#[doc = "`write(|w| ..)` method takes [pucre::W](pucre::W) writer structure"]
impl crate::Writable for PUCRE {}
#[doc = "Power Port E pull-up control register"]
pub mod pucre;
#[doc = "Power Port E pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcre](pdcre) module"]
pub type PDCRE = crate::Reg<u32, _PDCRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRE;
#[doc = "`read()` method returns [pdcre::R](pdcre::R) reader structure"]
impl crate::Readable for PDCRE {}
#[doc = "`write(|w| ..)` method takes [pdcre::W](pdcre::W) writer structure"]
impl crate::Writable for PDCRE {}
#[doc = "Power Port E pull-down control register"]
pub mod pdcre;
#[doc = "Power Port H pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrh](pucrh) module"]
pub type PUCRH = crate::Reg<u32, _PUCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRH;
#[doc = "`read()` method returns [pucrh::R](pucrh::R) reader structure"]
impl crate::Readable for PUCRH {}
#[doc = "`write(|w| ..)` method takes [pucrh::W](pucrh::W) writer structure"]
impl crate::Writable for PUCRH {}
#[doc = "Power Port H pull-up control register"]
pub mod pucrh;
#[doc = "Power Port H pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrh](pdcrh) module"]
pub type PDCRH = crate::Reg<u32, _PDCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRH;
#[doc = "`read()` method returns [pdcrh::R](pdcrh::R) reader structure"]
impl crate::Readable for PDCRH {}
#[doc = "`write(|w| ..)` method takes [pdcrh::W](pdcrh::W) writer structure"]
impl crate::Writable for PDCRH {}
#[doc = "Power Port H pull-down control register"]
pub mod pdcrh;
#[doc = "CPU2 Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr1](c2cr1) module"]
pub type C2CR1 = crate::Reg<u32, _C2CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR1;
#[doc = "`read()` method returns [c2cr1::R](c2cr1::R) reader structure"]
impl crate::Readable for C2CR1 {}
#[doc = "`write(|w| ..)` method takes [c2cr1::W](c2cr1::W) writer structure"]
impl crate::Writable for C2CR1 {}
#[doc = "CPU2 Power control register 1"]
pub mod c2cr1;
#[doc = "CPU2 Power control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr3](c2cr3) module"]
pub type C2CR3 = crate::Reg<u32, _C2CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR3;
#[doc = "`read()` method returns [c2cr3::R](c2cr3::R) reader structure"]
impl crate::Readable for C2CR3 {}
#[doc = "`write(|w| ..)` method takes [c2cr3::W](c2cr3::W) writer structure"]
impl crate::Writable for C2CR3 {}
#[doc = "CPU2 Power control register 3"]
pub mod c2cr3;
#[doc = "Power status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extscr](extscr) module"]
pub type EXTSCR = crate::Reg<u32, _EXTSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTSCR;
#[doc = "`read()` method returns [extscr::R](extscr::R) reader structure"]
impl crate::Readable for EXTSCR {}
#[doc = "`write(|w| ..)` method takes [extscr::W](extscr::W) writer structure"]
impl crate::Writable for EXTSCR {}
#[doc = "Power status clear register"]
pub mod extscr;
