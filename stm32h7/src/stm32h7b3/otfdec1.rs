#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTFDEC control register"]
    pub cr: CR,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - OTFDEC region x configuration register"]
    pub r1cfgr: R1CFGR,
    #[doc = "0x24 - OTFDEC region x start address register"]
    pub r1startaddr: R1STARTADDR,
    #[doc = "0x28 - OTFDEC region x end address register"]
    pub r1endaddr: R1ENDADDR,
    #[doc = "0x2c - OTFDEC region x nonce register 0"]
    pub r1noncer0: R1NONCER0,
    #[doc = "0x30 - OTFDEC region x nonce register 1"]
    pub r1noncer1: R1NONCER1,
    #[doc = "0x34 - OTFDEC region x key register 0"]
    pub r1keyr0: R1KEYR0,
    #[doc = "0x38 - OTFDEC region x key register 1"]
    pub r1keyr1: R1KEYR1,
    #[doc = "0x3c - OTFDEC region x key register 2"]
    pub r1keyr2: R1KEYR2,
    #[doc = "0x40 - OTFDEC region x key register 3"]
    pub r1keyr3: R1KEYR3,
    _reserved10: [u8; 12usize],
    #[doc = "0x50 - OTFDEC region x configuration register"]
    pub r2cfgr: R2CFGR,
    #[doc = "0x54 - OTFDEC region x start address register"]
    pub r2startaddr: R2STARTADDR,
    #[doc = "0x58 - OTFDEC region x end address register"]
    pub r2endaddr: R2ENDADDR,
    #[doc = "0x5c - OTFDEC region x nonce register 0"]
    pub r2noncer0: R2NONCER0,
    #[doc = "0x60 - OTFDEC region x nonce register 1"]
    pub r2noncer1: R2NONCER1,
    #[doc = "0x64 - OTFDEC region x key register 0"]
    pub r2keyr0: R2KEYR0,
    #[doc = "0x68 - OTFDEC region x key register 1"]
    pub r2keyr1: R2KEYR1,
    #[doc = "0x6c - OTFDEC region x key register 2"]
    pub r2keyr2: R2KEYR2,
    #[doc = "0x70 - OTFDEC region x key register 3"]
    pub r2keyr3: R2KEYR3,
    _reserved19: [u8; 12usize],
    #[doc = "0x80 - OTFDEC region x configuration register"]
    pub r3cfgr: R3CFGR,
    #[doc = "0x84 - OTFDEC region x start address register"]
    pub r3startaddr: R3STARTADDR,
    #[doc = "0x88 - OTFDEC region x end address register"]
    pub r3endaddr: R3ENDADDR,
    _reserved_22_r3noncer0: [u8; 4usize],
    #[doc = "0x90 - OTFDEC region x nonce register 1"]
    pub r3noncer1: R3NONCER1,
    #[doc = "0x94 - OTFDEC region x key register 0"]
    pub r3keyr0: R3KEYR0,
    #[doc = "0x98 - OTFDEC region x key register 1"]
    pub r3keyr1: R3KEYR1,
    #[doc = "0x9c - OTFDEC region x key register 2"]
    pub r3keyr2: R3KEYR2,
    #[doc = "0xa0 - OTFDEC region x key register 3"]
    pub r3keyr3: R3KEYR3,
    _reserved28: [u8; 12usize],
    #[doc = "0xb0 - OTFDEC region x configuration register"]
    pub r4cfgr: R4CFGR,
    #[doc = "0xb4 - OTFDEC region x start address register"]
    pub r4startaddr: R4STARTADDR,
    _reserved30: [u8; 4usize],
    #[doc = "0xbc - OTFDEC region x nonce register 0"]
    pub r4noncer0: R4NONCER0,
    #[doc = "0xc0 - OTFDEC region x nonce register 1"]
    pub r4noncer1: R4NONCER1,
    #[doc = "0xc4 - OTFDEC region x key register 0"]
    pub r4keyr0: R4KEYR0,
    #[doc = "0xc8 - OTFDEC region x key register 1"]
    pub r4keyr1: R4KEYR1,
    #[doc = "0xcc - OTFDEC region x key register 2"]
    pub r4keyr2: R4KEYR2,
    #[doc = "0xd0 - OTFDEC region x key register 3"]
    pub r4keyr3: R4KEYR3,
    _reserved36: [u8; 556usize],
    #[doc = "0x300 - OTFDEC interrupt status register"]
    pub isr: ISR,
    #[doc = "0x304 - OTFDEC interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x308 - OTFDEC interrupt enable register"]
    pub ier: IER,
}
impl RegisterBlock {
    #[doc = "0x8c - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub fn r3noncer0(&self) -> &R3NONCER0 {
        unsafe { &*(((self as *const Self) as *const u8).add(140usize) as *const R3NONCER0) }
    }
    #[doc = "0x8c - OTFDEC region x nonce register 0"]
    #[inline(always)]
    pub fn r3noncer0_mut(&self) -> &mut R3NONCER0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(140usize) as *mut R3NONCER0) }
    }
    #[doc = "0x8c - OTFDEC region x end address register"]
    #[inline(always)]
    pub fn r4endaddr(&self) -> &R4ENDADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(140usize) as *const R4ENDADDR) }
    }
    #[doc = "0x8c - OTFDEC region x end address register"]
    #[inline(always)]
    pub fn r4endaddr_mut(&self) -> &mut R4ENDADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(140usize) as *mut R4ENDADDR) }
    }
}
#[doc = "OTFDEC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "OTFDEC control register"]
pub mod cr;
#[doc = "OTFDEC region x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1cfgr](r1cfgr) module"]
pub type R1CFGR = crate::Reg<u32, _R1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1CFGR;
#[doc = "`read()` method returns [r1cfgr::R](r1cfgr::R) reader structure"]
impl crate::Readable for R1CFGR {}
#[doc = "`write(|w| ..)` method takes [r1cfgr::W](r1cfgr::W) writer structure"]
impl crate::Writable for R1CFGR {}
#[doc = "OTFDEC region x configuration register"]
pub mod r1cfgr;
#[doc = "OTFDEC region x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2cfgr](r2cfgr) module"]
pub type R2CFGR = crate::Reg<u32, _R2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2CFGR;
#[doc = "`read()` method returns [r2cfgr::R](r2cfgr::R) reader structure"]
impl crate::Readable for R2CFGR {}
#[doc = "`write(|w| ..)` method takes [r2cfgr::W](r2cfgr::W) writer structure"]
impl crate::Writable for R2CFGR {}
#[doc = "OTFDEC region x configuration register"]
pub mod r2cfgr;
#[doc = "OTFDEC region x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3cfgr](r3cfgr) module"]
pub type R3CFGR = crate::Reg<u32, _R3CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3CFGR;
#[doc = "`read()` method returns [r3cfgr::R](r3cfgr::R) reader structure"]
impl crate::Readable for R3CFGR {}
#[doc = "`write(|w| ..)` method takes [r3cfgr::W](r3cfgr::W) writer structure"]
impl crate::Writable for R3CFGR {}
#[doc = "OTFDEC region x configuration register"]
pub mod r3cfgr;
#[doc = "OTFDEC region x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4cfgr](r4cfgr) module"]
pub type R4CFGR = crate::Reg<u32, _R4CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4CFGR;
#[doc = "`read()` method returns [r4cfgr::R](r4cfgr::R) reader structure"]
impl crate::Readable for R4CFGR {}
#[doc = "`write(|w| ..)` method takes [r4cfgr::W](r4cfgr::W) writer structure"]
impl crate::Writable for R4CFGR {}
#[doc = "OTFDEC region x configuration register"]
pub mod r4cfgr;
#[doc = "OTFDEC region x start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1startaddr](r1startaddr) module"]
pub type R1STARTADDR = crate::Reg<u32, _R1STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1STARTADDR;
#[doc = "`read()` method returns [r1startaddr::R](r1startaddr::R) reader structure"]
impl crate::Readable for R1STARTADDR {}
#[doc = "`write(|w| ..)` method takes [r1startaddr::W](r1startaddr::W) writer structure"]
impl crate::Writable for R1STARTADDR {}
#[doc = "OTFDEC region x start address register"]
pub mod r1startaddr;
#[doc = "OTFDEC region x start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2startaddr](r2startaddr) module"]
pub type R2STARTADDR = crate::Reg<u32, _R2STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2STARTADDR;
#[doc = "`read()` method returns [r2startaddr::R](r2startaddr::R) reader structure"]
impl crate::Readable for R2STARTADDR {}
#[doc = "`write(|w| ..)` method takes [r2startaddr::W](r2startaddr::W) writer structure"]
impl crate::Writable for R2STARTADDR {}
#[doc = "OTFDEC region x start address register"]
pub mod r2startaddr;
#[doc = "OTFDEC region x start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3startaddr](r3startaddr) module"]
pub type R3STARTADDR = crate::Reg<u32, _R3STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3STARTADDR;
#[doc = "`read()` method returns [r3startaddr::R](r3startaddr::R) reader structure"]
impl crate::Readable for R3STARTADDR {}
#[doc = "`write(|w| ..)` method takes [r3startaddr::W](r3startaddr::W) writer structure"]
impl crate::Writable for R3STARTADDR {}
#[doc = "OTFDEC region x start address register"]
pub mod r3startaddr;
#[doc = "OTFDEC region x start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4startaddr](r4startaddr) module"]
pub type R4STARTADDR = crate::Reg<u32, _R4STARTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4STARTADDR;
#[doc = "`read()` method returns [r4startaddr::R](r4startaddr::R) reader structure"]
impl crate::Readable for R4STARTADDR {}
#[doc = "`write(|w| ..)` method takes [r4startaddr::W](r4startaddr::W) writer structure"]
impl crate::Writable for R4STARTADDR {}
#[doc = "OTFDEC region x start address register"]
pub mod r4startaddr;
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1endaddr](r1endaddr) module"]
pub type R1ENDADDR = crate::Reg<u32, _R1ENDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1ENDADDR;
#[doc = "`read()` method returns [r1endaddr::R](r1endaddr::R) reader structure"]
impl crate::Readable for R1ENDADDR {}
#[doc = "`write(|w| ..)` method takes [r1endaddr::W](r1endaddr::W) writer structure"]
impl crate::Writable for R1ENDADDR {}
#[doc = "OTFDEC region x end address register"]
pub mod r1endaddr;
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2endaddr](r2endaddr) module"]
pub type R2ENDADDR = crate::Reg<u32, _R2ENDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2ENDADDR;
#[doc = "`read()` method returns [r2endaddr::R](r2endaddr::R) reader structure"]
impl crate::Readable for R2ENDADDR {}
#[doc = "`write(|w| ..)` method takes [r2endaddr::W](r2endaddr::W) writer structure"]
impl crate::Writable for R2ENDADDR {}
#[doc = "OTFDEC region x end address register"]
pub mod r2endaddr;
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3endaddr](r3endaddr) module"]
pub type R3ENDADDR = crate::Reg<u32, _R3ENDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3ENDADDR;
#[doc = "`read()` method returns [r3endaddr::R](r3endaddr::R) reader structure"]
impl crate::Readable for R3ENDADDR {}
#[doc = "`write(|w| ..)` method takes [r3endaddr::W](r3endaddr::W) writer structure"]
impl crate::Writable for R3ENDADDR {}
#[doc = "OTFDEC region x end address register"]
pub mod r3endaddr;
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4endaddr](r4endaddr) module"]
pub type R4ENDADDR = crate::Reg<u32, _R4ENDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4ENDADDR;
#[doc = "`read()` method returns [r4endaddr::R](r4endaddr::R) reader structure"]
impl crate::Readable for R4ENDADDR {}
#[doc = "`write(|w| ..)` method takes [r4endaddr::W](r4endaddr::W) writer structure"]
impl crate::Writable for R4ENDADDR {}
#[doc = "OTFDEC region x end address register"]
pub mod r4endaddr;
#[doc = "OTFDEC region x nonce register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1noncer0](r1noncer0) module"]
pub type R1NONCER0 = crate::Reg<u32, _R1NONCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1NONCER0;
#[doc = "`read()` method returns [r1noncer0::R](r1noncer0::R) reader structure"]
impl crate::Readable for R1NONCER0 {}
#[doc = "`write(|w| ..)` method takes [r1noncer0::W](r1noncer0::W) writer structure"]
impl crate::Writable for R1NONCER0 {}
#[doc = "OTFDEC region x nonce register 0"]
pub mod r1noncer0;
#[doc = "OTFDEC region x nonce register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2noncer0](r2noncer0) module"]
pub type R2NONCER0 = crate::Reg<u32, _R2NONCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2NONCER0;
#[doc = "`read()` method returns [r2noncer0::R](r2noncer0::R) reader structure"]
impl crate::Readable for R2NONCER0 {}
#[doc = "`write(|w| ..)` method takes [r2noncer0::W](r2noncer0::W) writer structure"]
impl crate::Writable for R2NONCER0 {}
#[doc = "OTFDEC region x nonce register 0"]
pub mod r2noncer0;
#[doc = "OTFDEC region x nonce register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3noncer0](r3noncer0) module"]
pub type R3NONCER0 = crate::Reg<u32, _R3NONCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3NONCER0;
#[doc = "`read()` method returns [r3noncer0::R](r3noncer0::R) reader structure"]
impl crate::Readable for R3NONCER0 {}
#[doc = "`write(|w| ..)` method takes [r3noncer0::W](r3noncer0::W) writer structure"]
impl crate::Writable for R3NONCER0 {}
#[doc = "OTFDEC region x nonce register 0"]
pub mod r3noncer0;
#[doc = "OTFDEC region x nonce register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4noncer0](r4noncer0) module"]
pub type R4NONCER0 = crate::Reg<u32, _R4NONCER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4NONCER0;
#[doc = "`read()` method returns [r4noncer0::R](r4noncer0::R) reader structure"]
impl crate::Readable for R4NONCER0 {}
#[doc = "`write(|w| ..)` method takes [r4noncer0::W](r4noncer0::W) writer structure"]
impl crate::Writable for R4NONCER0 {}
#[doc = "OTFDEC region x nonce register 0"]
pub mod r4noncer0;
#[doc = "OTFDEC region x nonce register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1noncer1](r1noncer1) module"]
pub type R1NONCER1 = crate::Reg<u32, _R1NONCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1NONCER1;
#[doc = "`read()` method returns [r1noncer1::R](r1noncer1::R) reader structure"]
impl crate::Readable for R1NONCER1 {}
#[doc = "`write(|w| ..)` method takes [r1noncer1::W](r1noncer1::W) writer structure"]
impl crate::Writable for R1NONCER1 {}
#[doc = "OTFDEC region x nonce register 1"]
pub mod r1noncer1;
#[doc = "OTFDEC region x nonce register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2noncer1](r2noncer1) module"]
pub type R2NONCER1 = crate::Reg<u32, _R2NONCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2NONCER1;
#[doc = "`read()` method returns [r2noncer1::R](r2noncer1::R) reader structure"]
impl crate::Readable for R2NONCER1 {}
#[doc = "`write(|w| ..)` method takes [r2noncer1::W](r2noncer1::W) writer structure"]
impl crate::Writable for R2NONCER1 {}
#[doc = "OTFDEC region x nonce register 1"]
pub mod r2noncer1;
#[doc = "OTFDEC region x nonce register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3noncer1](r3noncer1) module"]
pub type R3NONCER1 = crate::Reg<u32, _R3NONCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3NONCER1;
#[doc = "`read()` method returns [r3noncer1::R](r3noncer1::R) reader structure"]
impl crate::Readable for R3NONCER1 {}
#[doc = "`write(|w| ..)` method takes [r3noncer1::W](r3noncer1::W) writer structure"]
impl crate::Writable for R3NONCER1 {}
#[doc = "OTFDEC region x nonce register 1"]
pub mod r3noncer1;
#[doc = "OTFDEC region x nonce register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4noncer1](r4noncer1) module"]
pub type R4NONCER1 = crate::Reg<u32, _R4NONCER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4NONCER1;
#[doc = "`read()` method returns [r4noncer1::R](r4noncer1::R) reader structure"]
impl crate::Readable for R4NONCER1 {}
#[doc = "`write(|w| ..)` method takes [r4noncer1::W](r4noncer1::W) writer structure"]
impl crate::Writable for R4NONCER1 {}
#[doc = "OTFDEC region x nonce register 1"]
pub mod r4noncer1;
#[doc = "OTFDEC region x key register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1keyr0](r1keyr0) module"]
pub type R1KEYR0 = crate::Reg<u32, _R1KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1KEYR0;
#[doc = "`write(|w| ..)` method takes [r1keyr0::W](r1keyr0::W) writer structure"]
impl crate::Writable for R1KEYR0 {}
#[doc = "OTFDEC region x key register 0"]
pub mod r1keyr0;
#[doc = "OTFDEC region x key register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2keyr0](r2keyr0) module"]
pub type R2KEYR0 = crate::Reg<u32, _R2KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2KEYR0;
#[doc = "`write(|w| ..)` method takes [r2keyr0::W](r2keyr0::W) writer structure"]
impl crate::Writable for R2KEYR0 {}
#[doc = "OTFDEC region x key register 0"]
pub mod r2keyr0;
#[doc = "OTFDEC region x key register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3keyr0](r3keyr0) module"]
pub type R3KEYR0 = crate::Reg<u32, _R3KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3KEYR0;
#[doc = "`write(|w| ..)` method takes [r3keyr0::W](r3keyr0::W) writer structure"]
impl crate::Writable for R3KEYR0 {}
#[doc = "OTFDEC region x key register 0"]
pub mod r3keyr0;
#[doc = "OTFDEC region x key register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4keyr0](r4keyr0) module"]
pub type R4KEYR0 = crate::Reg<u32, _R4KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4KEYR0;
#[doc = "`write(|w| ..)` method takes [r4keyr0::W](r4keyr0::W) writer structure"]
impl crate::Writable for R4KEYR0 {}
#[doc = "OTFDEC region x key register 0"]
pub mod r4keyr0;
#[doc = "OTFDEC region x key register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1keyr1](r1keyr1) module"]
pub type R1KEYR1 = crate::Reg<u32, _R1KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1KEYR1;
#[doc = "`write(|w| ..)` method takes [r1keyr1::W](r1keyr1::W) writer structure"]
impl crate::Writable for R1KEYR1 {}
#[doc = "OTFDEC region x key register 1"]
pub mod r1keyr1;
#[doc = "OTFDEC region x key register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2keyr1](r2keyr1) module"]
pub type R2KEYR1 = crate::Reg<u32, _R2KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2KEYR1;
#[doc = "`write(|w| ..)` method takes [r2keyr1::W](r2keyr1::W) writer structure"]
impl crate::Writable for R2KEYR1 {}
#[doc = "OTFDEC region x key register 1"]
pub mod r2keyr1;
#[doc = "OTFDEC region x key register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3keyr1](r3keyr1) module"]
pub type R3KEYR1 = crate::Reg<u32, _R3KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3KEYR1;
#[doc = "`write(|w| ..)` method takes [r3keyr1::W](r3keyr1::W) writer structure"]
impl crate::Writable for R3KEYR1 {}
#[doc = "OTFDEC region x key register 1"]
pub mod r3keyr1;
#[doc = "OTFDEC region x key register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4keyr1](r4keyr1) module"]
pub type R4KEYR1 = crate::Reg<u32, _R4KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4KEYR1;
#[doc = "`write(|w| ..)` method takes [r4keyr1::W](r4keyr1::W) writer structure"]
impl crate::Writable for R4KEYR1 {}
#[doc = "OTFDEC region x key register 1"]
pub mod r4keyr1;
#[doc = "OTFDEC region x key register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1keyr2](r1keyr2) module"]
pub type R1KEYR2 = crate::Reg<u32, _R1KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1KEYR2;
#[doc = "`write(|w| ..)` method takes [r1keyr2::W](r1keyr2::W) writer structure"]
impl crate::Writable for R1KEYR2 {}
#[doc = "OTFDEC region x key register 2"]
pub mod r1keyr2;
#[doc = "OTFDEC region x key register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2keyr2](r2keyr2) module"]
pub type R2KEYR2 = crate::Reg<u32, _R2KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2KEYR2;
#[doc = "`write(|w| ..)` method takes [r2keyr2::W](r2keyr2::W) writer structure"]
impl crate::Writable for R2KEYR2 {}
#[doc = "OTFDEC region x key register 2"]
pub mod r2keyr2;
#[doc = "OTFDEC region x key register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3keyr2](r3keyr2) module"]
pub type R3KEYR2 = crate::Reg<u32, _R3KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3KEYR2;
#[doc = "`write(|w| ..)` method takes [r3keyr2::W](r3keyr2::W) writer structure"]
impl crate::Writable for R3KEYR2 {}
#[doc = "OTFDEC region x key register 2"]
pub mod r3keyr2;
#[doc = "OTFDEC region x key register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4keyr2](r4keyr2) module"]
pub type R4KEYR2 = crate::Reg<u32, _R4KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4KEYR2;
#[doc = "`write(|w| ..)` method takes [r4keyr2::W](r4keyr2::W) writer structure"]
impl crate::Writable for R4KEYR2 {}
#[doc = "OTFDEC region x key register 2"]
pub mod r4keyr2;
#[doc = "OTFDEC region x key register 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1keyr3](r1keyr3) module"]
pub type R1KEYR3 = crate::Reg<u32, _R1KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1KEYR3;
#[doc = "`write(|w| ..)` method takes [r1keyr3::W](r1keyr3::W) writer structure"]
impl crate::Writable for R1KEYR3 {}
#[doc = "OTFDEC region x key register 3"]
pub mod r1keyr3;
#[doc = "OTFDEC region x key register 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2keyr3](r2keyr3) module"]
pub type R2KEYR3 = crate::Reg<u32, _R2KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2KEYR3;
#[doc = "`write(|w| ..)` method takes [r2keyr3::W](r2keyr3::W) writer structure"]
impl crate::Writable for R2KEYR3 {}
#[doc = "OTFDEC region x key register 3"]
pub mod r2keyr3;
#[doc = "OTFDEC region x key register 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3keyr3](r3keyr3) module"]
pub type R3KEYR3 = crate::Reg<u32, _R3KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3KEYR3;
#[doc = "`write(|w| ..)` method takes [r3keyr3::W](r3keyr3::W) writer structure"]
impl crate::Writable for R3KEYR3 {}
#[doc = "OTFDEC region x key register 3"]
pub mod r3keyr3;
#[doc = "OTFDEC region x key register 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4keyr3](r4keyr3) module"]
pub type R4KEYR3 = crate::Reg<u32, _R4KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4KEYR3;
#[doc = "`write(|w| ..)` method takes [r4keyr3::W](r4keyr3::W) writer structure"]
impl crate::Writable for R4KEYR3 {}
#[doc = "OTFDEC region x key register 3"]
pub mod r4keyr3;
#[doc = "OTFDEC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "OTFDEC interrupt status register"]
pub mod isr;
#[doc = "OTFDEC interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "OTFDEC interrupt clear register"]
pub mod icr;
#[doc = "OTFDEC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "OTFDEC interrupt enable register"]
pub mod ier;
