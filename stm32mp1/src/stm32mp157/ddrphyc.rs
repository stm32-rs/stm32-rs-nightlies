#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DDRPHYC revision ID register"]
    pub ddrphyc_ridr: DDRPHYC_RIDR,
    #[doc = "0x04 - DDRPHYC PHY initialization register"]
    pub ddrphyc_pir: DDRPHYC_PIR,
    #[doc = "0x08 - DDRPHYC PHY global control register"]
    pub ddrphyc_pgcr: DDRPHYC_PGCR,
    #[doc = "0x0c - DDRPHYC PHY global status register"]
    pub ddrphyc_pgsr: DDRPHYC_PGSR,
    #[doc = "0x10 - DDRPHYC DDR global control register"]
    pub ddrphyc_dllgcr: DDRPHYC_DLLGCR,
    #[doc = "0x14 - DDRPHYC AC DLL control register"]
    pub ddrphyc_acdllcr: DDRPHYC_ACDLLCR,
    #[doc = "0x18 - DDRPHYC PT register 0"]
    pub ddrphyc_ptr0: DDRPHYC_PTR0,
    #[doc = "0x1c - DDRPHYC PT register 1"]
    pub ddrphyc_ptr1: DDRPHYC_PTR1,
    #[doc = "0x20 - DDRPHYC PT register 2"]
    pub ddrphyc_ptr2: DDRPHYC_PTR2,
    #[doc = "0x24 - DDRPHYC ACIOC register"]
    pub ddrphyc_aciocr: DDRPHYC_ACIOCR,
    #[doc = "0x28 - DDRPHYC DXCC register"]
    pub ddrphyc_dxccr: DDRPHYC_DXCCR,
    #[doc = "0x2c - DDRPHYC DSGC register"]
    pub ddrphyc_dsgcr: DDRPHYC_DSGCR,
    #[doc = "0x30 - DDRPHYC DC register"]
    pub ddrphyc_dcr: DDRPHYC_DCR,
    #[doc = "0x34 - DDRPHYC DTP register 0"]
    pub ddrphyc_dtpr0: DDRPHYC_DTPR0,
    #[doc = "0x38 - DDRPHYC DTP register 1"]
    pub ddrphyc_dtpr1: DDRPHYC_DTPR1,
    #[doc = "0x3c - DDRPHYC DTP register 2"]
    pub ddrphyc_dtpr2: DDRPHYC_DTPR2,
    #[doc = "0x40 - DDRPHYC MR0 register for DDR3"]
    pub ddrphyc_ddr3_mr0: DDRPHYC_DDR3_MR0,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - DDRPHYC MR1 register for DDR3"]
    pub ddrphyc_ddr3_mr1: DDRPHYC_DDR3_MR1,
    _reserved18: [u8; 2usize],
    #[doc = "0x48 - DDRPHYC MR2 register for DDR3"]
    pub ddrphyc_ddr3_mr2: DDRPHYC_DDR3_MR2,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - DDRPHYC MR3 register for DDR3"]
    pub ddrphyc_ddr3_mr3: DDRPHYC_DDR3_MR3,
    _reserved20: [u8; 3usize],
    #[doc = "0x50 - DDRPHYC ODTC register"]
    pub ddrphyc_odtcr: DDRPHYC_ODTCR,
    #[doc = "0x54 - DDRPHYC DTA register"]
    pub ddrphyc_dtar: DDRPHYC_DTAR,
    #[doc = "0x58 - DDRPHYC DTD register 0"]
    pub ddrphyc_dtdr0: DDRPHYC_DTDR0,
    #[doc = "0x5c - DDRPHYC DTD register 1"]
    pub ddrphyc_dtdr1: DDRPHYC_DTDR1,
    _reserved24: [u8; 280usize],
    #[doc = "0x178 - DDRPHYC general purpose register 0"]
    pub ddrphyc_gpr0: DDRPHYC_GPR0,
    #[doc = "0x17c - DDRPHYC general purpose register 1"]
    pub ddrphyc_gpr1: DDRPHYC_GPR1,
    #[doc = "0x180 - DDRPHYC ZQ0C register 0"]
    pub ddrphyc_zq0cr0: DDRPHYC_ZQ0CR0,
    #[doc = "0x184 - DDRPHYC ZQ0CR1 register"]
    pub ddrphyc_zq0cr1: DDRPHYC_ZQ0CR1,
    _reserved28: [u8; 3usize],
    #[doc = "0x188 - DDRPHYC ZQ0S register 0"]
    pub ddrphyc_zq0sr0: DDRPHYC_ZQ0SR0,
    #[doc = "0x18c - DDRPHYC ZQ0S register 1"]
    pub ddrphyc_zq0sr1: DDRPHYC_ZQ0SR1,
    _reserved30: [u8; 51usize],
    #[doc = "0x1c0 - DDRPHYC byte lane 0 GC register"]
    pub ddrphyc_dx0gcr: DDRPHYC_DX0GCR,
    #[doc = "0x1c4 - DDRPHYC byte lane 0 GS register 0"]
    pub ddrphyc_dx0gsr0: DDRPHYC_DX0GSR0,
    _reserved32: [u8; 2usize],
    #[doc = "0x1c8 - DDRPHYC byte lane 0 GS register 1"]
    pub ddrphyc_dx0gsr1: DDRPHYC_DX0GSR1,
    #[doc = "0x1cc - DDRPHYC byte lane 0 DLLC register"]
    pub ddrphyc_dx0dllcr: DDRPHYC_DX0DLLCR,
    #[doc = "0x1d0 - DDRPHYC byte lane 0 DQT register"]
    pub ddrphyc_dx0dqtr: DDRPHYC_DX0DQTR,
    #[doc = "0x1d4 - DDRPHYC byte lane 0 DQST register"]
    pub ddrphyc_dx0dqstr: DDRPHYC_DX0DQSTR,
    _reserved36: [u8; 40usize],
    #[doc = "0x200 - DDRPHYC byte lane 1 GC register"]
    pub ddrphyc_dx1gcr: DDRPHYC_DX1GCR,
    #[doc = "0x204 - DDRPHYC byte lane 1 GS register 0"]
    pub ddrphyc_dx1gsr0: DDRPHYC_DX1GSR0,
    _reserved38: [u8; 2usize],
    #[doc = "0x208 - DDRPHYC byte lane 1 GS register 1"]
    pub ddrphyc_dx1gsr1: DDRPHYC_DX1GSR1,
    #[doc = "0x20c - DDRPHYC byte lane 1 DLLC register"]
    pub ddrphyc_dx1dllcr: DDRPHYC_DX1DLLCR,
    #[doc = "0x210 - DDRPHYC byte lane 1 DQT register"]
    pub ddrphyc_dx1dqtr: DDRPHYC_DX1DQTR,
    #[doc = "0x214 - DDRPHYC byte lane 1 DQST register"]
    pub ddrphyc_dx1dqstr: DDRPHYC_DX1DQSTR,
    _reserved42: [u8; 40usize],
    #[doc = "0x240 - DDRPHYC byte lane 2 GC register"]
    pub ddrphyc_dx2gcr: DDRPHYC_DX2GCR,
    #[doc = "0x244 - DDRPHYC byte lane 2 GS register 0"]
    pub ddrphyc_dx2gsr0: DDRPHYC_DX2GSR0,
    _reserved44: [u8; 2usize],
    #[doc = "0x248 - DDRPHYC byte lane 2 GS register 1"]
    pub ddrphyc_dx2gsr1: DDRPHYC_DX2GSR1,
    #[doc = "0x24c - DDRPHYC byte lane 2 DLLC register"]
    pub ddrphyc_dx2dllcr: DDRPHYC_DX2DLLCR,
    #[doc = "0x250 - DDRPHYC byte lane 2 DQT register"]
    pub ddrphyc_dx2dqtr: DDRPHYC_DX2DQTR,
    #[doc = "0x254 - DDRPHYC byte lane 2 DQST register"]
    pub ddrphyc_dx2dqstr: DDRPHYC_DX2DQSTR,
    _reserved48: [u8; 40usize],
    #[doc = "0x280 - DDRPHYC byte lane 3 GC register"]
    pub ddrphyc_dx3gcr: DDRPHYC_DX3GCR,
    #[doc = "0x284 - DDRPHYC byte lane 3 GS register 0"]
    pub ddrphyc_dx3gsr0: DDRPHYC_DX3GSR0,
    _reserved50: [u8; 2usize],
    #[doc = "0x288 - DDRPHYC byte lane 3 GS register 1"]
    pub ddrphyc_dx3gsr1: DDRPHYC_DX3GSR1,
    #[doc = "0x28c - DDRPHYC byte lane 3 DLLC register"]
    pub ddrphyc_dx3dllcr: DDRPHYC_DX3DLLCR,
    #[doc = "0x290 - DDRPHYC byte lane 3 DQT register"]
    pub ddrphyc_dx3dqtr: DDRPHYC_DX3DQTR,
    #[doc = "0x294 - DDRPHYC byte lane 3 DQST register"]
    pub ddrphyc_dx3dqstr: DDRPHYC_DX3DQSTR,
}
#[doc = "DDRPHYC revision ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ridr](ddrphyc_ridr) module"]
pub type DDRPHYC_RIDR = crate::Reg<u32, _DDRPHYC_RIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_RIDR;
#[doc = "`read()` method returns [ddrphyc_ridr::R](ddrphyc_ridr::R) reader structure"]
impl crate::Readable for DDRPHYC_RIDR {}
#[doc = "DDRPHYC revision ID register"]
pub mod ddrphyc_ridr;
#[doc = "DDRPHYC PHY initialization register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pir](ddrphyc_pir) module"]
pub type DDRPHYC_PIR = crate::Reg<u32, _DDRPHYC_PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PIR;
#[doc = "`write(|w| ..)` method takes [ddrphyc_pir::W](ddrphyc_pir::W) writer structure"]
impl crate::Writable for DDRPHYC_PIR {}
#[doc = "DDRPHYC PHY initialization register"]
pub mod ddrphyc_pir;
#[doc = "DDRPHYC PHY global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgcr](ddrphyc_pgcr) module"]
pub type DDRPHYC_PGCR = crate::Reg<u32, _DDRPHYC_PGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PGCR;
#[doc = "`read()` method returns [ddrphyc_pgcr::R](ddrphyc_pgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_PGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pgcr::W](ddrphyc_pgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_PGCR {}
#[doc = "DDRPHYC PHY global control register"]
pub mod ddrphyc_pgcr;
#[doc = "DDRPHYC PHY global status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgsr](ddrphyc_pgsr) module"]
pub type DDRPHYC_PGSR = crate::Reg<u32, _DDRPHYC_PGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PGSR;
#[doc = "`read()` method returns [ddrphyc_pgsr::R](ddrphyc_pgsr::R) reader structure"]
impl crate::Readable for DDRPHYC_PGSR {}
#[doc = "DDRPHYC PHY global status register"]
pub mod ddrphyc_pgsr;
#[doc = "DDRPHYC DDR global control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dllgcr](ddrphyc_dllgcr) module"]
pub type DDRPHYC_DLLGCR = crate::Reg<u32, _DDRPHYC_DLLGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DLLGCR;
#[doc = "`read()` method returns [ddrphyc_dllgcr::R](ddrphyc_dllgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DLLGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dllgcr::W](ddrphyc_dllgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DLLGCR {}
#[doc = "DDRPHYC DDR global control register"]
pub mod ddrphyc_dllgcr;
#[doc = "DDRPHYC AC DLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_acdllcr](ddrphyc_acdllcr) module"]
pub type DDRPHYC_ACDLLCR = crate::Reg<u32, _DDRPHYC_ACDLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ACDLLCR;
#[doc = "`read()` method returns [ddrphyc_acdllcr::R](ddrphyc_acdllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_ACDLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_acdllcr::W](ddrphyc_acdllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_ACDLLCR {}
#[doc = "DDRPHYC AC DLL control register"]
pub mod ddrphyc_acdllcr;
#[doc = "DDRPHYC PT register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr0](ddrphyc_ptr0) module"]
pub type DDRPHYC_PTR0 = crate::Reg<u32, _DDRPHYC_PTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR0;
#[doc = "`read()` method returns [ddrphyc_ptr0::R](ddrphyc_ptr0::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr0::W](ddrphyc_ptr0::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR0 {}
#[doc = "DDRPHYC PT register 0"]
pub mod ddrphyc_ptr0;
#[doc = "DDRPHYC PT register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr1](ddrphyc_ptr1) module"]
pub type DDRPHYC_PTR1 = crate::Reg<u32, _DDRPHYC_PTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR1;
#[doc = "`read()` method returns [ddrphyc_ptr1::R](ddrphyc_ptr1::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr1::W](ddrphyc_ptr1::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR1 {}
#[doc = "DDRPHYC PT register 1"]
pub mod ddrphyc_ptr1;
#[doc = "DDRPHYC PT register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr2](ddrphyc_ptr2) module"]
pub type DDRPHYC_PTR2 = crate::Reg<u32, _DDRPHYC_PTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR2;
#[doc = "`read()` method returns [ddrphyc_ptr2::R](ddrphyc_ptr2::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr2::W](ddrphyc_ptr2::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR2 {}
#[doc = "DDRPHYC PT register 2"]
pub mod ddrphyc_ptr2;
#[doc = "DDRPHYC ACIOC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_aciocr](ddrphyc_aciocr) module"]
pub type DDRPHYC_ACIOCR = crate::Reg<u32, _DDRPHYC_ACIOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ACIOCR;
#[doc = "`read()` method returns [ddrphyc_aciocr::R](ddrphyc_aciocr::R) reader structure"]
impl crate::Readable for DDRPHYC_ACIOCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_aciocr::W](ddrphyc_aciocr::W) writer structure"]
impl crate::Writable for DDRPHYC_ACIOCR {}
#[doc = "DDRPHYC ACIOC register"]
pub mod ddrphyc_aciocr;
#[doc = "DDRPHYC DXCC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dxccr](ddrphyc_dxccr) module"]
pub type DDRPHYC_DXCCR = crate::Reg<u32, _DDRPHYC_DXCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DXCCR;
#[doc = "`read()` method returns [ddrphyc_dxccr::R](ddrphyc_dxccr::R) reader structure"]
impl crate::Readable for DDRPHYC_DXCCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dxccr::W](ddrphyc_dxccr::W) writer structure"]
impl crate::Writable for DDRPHYC_DXCCR {}
#[doc = "DDRPHYC DXCC register"]
pub mod ddrphyc_dxccr;
#[doc = "DDRPHYC DSGC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dsgcr](ddrphyc_dsgcr) module"]
pub type DDRPHYC_DSGCR = crate::Reg<u32, _DDRPHYC_DSGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DSGCR;
#[doc = "`read()` method returns [ddrphyc_dsgcr::R](ddrphyc_dsgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DSGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dsgcr::W](ddrphyc_dsgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DSGCR {}
#[doc = "DDRPHYC DSGC register"]
pub mod ddrphyc_dsgcr;
#[doc = "DDRPHYC DC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcr](ddrphyc_dcr) module"]
pub type DDRPHYC_DCR = crate::Reg<u32, _DDRPHYC_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCR;
#[doc = "`read()` method returns [ddrphyc_dcr::R](ddrphyc_dcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcr::W](ddrphyc_dcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCR {}
#[doc = "DDRPHYC DC register"]
pub mod ddrphyc_dcr;
#[doc = "DDRPHYC DTP register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr0](ddrphyc_dtpr0) module"]
pub type DDRPHYC_DTPR0 = crate::Reg<u32, _DDRPHYC_DTPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR0;
#[doc = "`read()` method returns [ddrphyc_dtpr0::R](ddrphyc_dtpr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr0::W](ddrphyc_dtpr0::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR0 {}
#[doc = "DDRPHYC DTP register 0"]
pub mod ddrphyc_dtpr0;
#[doc = "DDRPHYC DTP register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr1](ddrphyc_dtpr1) module"]
pub type DDRPHYC_DTPR1 = crate::Reg<u32, _DDRPHYC_DTPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR1;
#[doc = "`read()` method returns [ddrphyc_dtpr1::R](ddrphyc_dtpr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr1::W](ddrphyc_dtpr1::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR1 {}
#[doc = "DDRPHYC DTP register 1"]
pub mod ddrphyc_dtpr1;
#[doc = "DDRPHYC DTP register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr2](ddrphyc_dtpr2) module"]
pub type DDRPHYC_DTPR2 = crate::Reg<u32, _DDRPHYC_DTPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR2;
#[doc = "`read()` method returns [ddrphyc_dtpr2::R](ddrphyc_dtpr2::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr2::W](ddrphyc_dtpr2::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR2 {}
#[doc = "DDRPHYC DTP register 2"]
pub mod ddrphyc_dtpr2;
#[doc = "DDRPHYC MR0 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr0](ddrphyc_ddr3_mr0) module"]
pub type DDRPHYC_DDR3_MR0 = crate::Reg<u16, _DDRPHYC_DDR3_MR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DDR3_MR0;
#[doc = "`read()` method returns [ddrphyc_ddr3_mr0::R](ddrphyc_ddr3_mr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr0::W](ddrphyc_ddr3_mr0::W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR0 {}
#[doc = "DDRPHYC MR0 register for DDR3"]
pub mod ddrphyc_ddr3_mr0;
#[doc = "DDRPHYC MR1 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr1](ddrphyc_ddr3_mr1) module"]
pub type DDRPHYC_DDR3_MR1 = crate::Reg<u16, _DDRPHYC_DDR3_MR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DDR3_MR1;
#[doc = "`read()` method returns [ddrphyc_ddr3_mr1::R](ddrphyc_ddr3_mr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr1::W](ddrphyc_ddr3_mr1::W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR1 {}
#[doc = "DDRPHYC MR1 register for DDR3"]
pub mod ddrphyc_ddr3_mr1;
#[doc = "DDRPHYC MR2 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr2](ddrphyc_ddr3_mr2) module"]
pub type DDRPHYC_DDR3_MR2 = crate::Reg<u16, _DDRPHYC_DDR3_MR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DDR3_MR2;
#[doc = "`read()` method returns [ddrphyc_ddr3_mr2::R](ddrphyc_ddr3_mr2::R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr2::W](ddrphyc_ddr3_mr2::W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR2 {}
#[doc = "DDRPHYC MR2 register for DDR3"]
pub mod ddrphyc_ddr3_mr2;
#[doc = "DDRPHYC MR3 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr3](ddrphyc_ddr3_mr3) module"]
pub type DDRPHYC_DDR3_MR3 = crate::Reg<u8, _DDRPHYC_DDR3_MR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DDR3_MR3;
#[doc = "`read()` method returns [ddrphyc_ddr3_mr3::R](ddrphyc_ddr3_mr3::R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR3 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr3::W](ddrphyc_ddr3_mr3::W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR3 {}
#[doc = "DDRPHYC MR3 register for DDR3"]
pub mod ddrphyc_ddr3_mr3;
#[doc = "DDRPHYC ODTC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_odtcr](ddrphyc_odtcr) module"]
pub type DDRPHYC_ODTCR = crate::Reg<u32, _DDRPHYC_ODTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ODTCR;
#[doc = "`read()` method returns [ddrphyc_odtcr::R](ddrphyc_odtcr::R) reader structure"]
impl crate::Readable for DDRPHYC_ODTCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_odtcr::W](ddrphyc_odtcr::W) writer structure"]
impl crate::Writable for DDRPHYC_ODTCR {}
#[doc = "DDRPHYC ODTC register"]
pub mod ddrphyc_odtcr;
#[doc = "DDRPHYC DTA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtar](ddrphyc_dtar) module"]
pub type DDRPHYC_DTAR = crate::Reg<u32, _DDRPHYC_DTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTAR;
#[doc = "`read()` method returns [ddrphyc_dtar::R](ddrphyc_dtar::R) reader structure"]
impl crate::Readable for DDRPHYC_DTAR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtar::W](ddrphyc_dtar::W) writer structure"]
impl crate::Writable for DDRPHYC_DTAR {}
#[doc = "DDRPHYC DTA register"]
pub mod ddrphyc_dtar;
#[doc = "DDRPHYC DTD register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr0](ddrphyc_dtdr0) module"]
pub type DDRPHYC_DTDR0 = crate::Reg<u32, _DDRPHYC_DTDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTDR0;
#[doc = "`read()` method returns [ddrphyc_dtdr0::R](ddrphyc_dtdr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr0::W](ddrphyc_dtdr0::W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR0 {}
#[doc = "DDRPHYC DTD register 0"]
pub mod ddrphyc_dtdr0;
#[doc = "DDRPHYC DTD register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr1](ddrphyc_dtdr1) module"]
pub type DDRPHYC_DTDR1 = crate::Reg<u32, _DDRPHYC_DTDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTDR1;
#[doc = "`read()` method returns [ddrphyc_dtdr1::R](ddrphyc_dtdr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr1::W](ddrphyc_dtdr1::W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR1 {}
#[doc = "DDRPHYC DTD register 1"]
pub mod ddrphyc_dtdr1;
#[doc = "DDRPHYC general purpose register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_gpr0](ddrphyc_gpr0) module"]
pub type DDRPHYC_GPR0 = crate::Reg<u32, _DDRPHYC_GPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_GPR0;
#[doc = "`read()` method returns [ddrphyc_gpr0::R](ddrphyc_gpr0::R) reader structure"]
impl crate::Readable for DDRPHYC_GPR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_gpr0::W](ddrphyc_gpr0::W) writer structure"]
impl crate::Writable for DDRPHYC_GPR0 {}
#[doc = "DDRPHYC general purpose register 0"]
pub mod ddrphyc_gpr0;
#[doc = "DDRPHYC general purpose register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_gpr1](ddrphyc_gpr1) module"]
pub type DDRPHYC_GPR1 = crate::Reg<u32, _DDRPHYC_GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_GPR1;
#[doc = "`read()` method returns [ddrphyc_gpr1::R](ddrphyc_gpr1::R) reader structure"]
impl crate::Readable for DDRPHYC_GPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_gpr1::W](ddrphyc_gpr1::W) writer structure"]
impl crate::Writable for DDRPHYC_GPR1 {}
#[doc = "DDRPHYC general purpose register 1"]
pub mod ddrphyc_gpr1;
#[doc = "DDRPHYC ZQ0C register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr0](ddrphyc_zq0cr0) module"]
pub type DDRPHYC_ZQ0CR0 = crate::Reg<u32, _DDRPHYC_ZQ0CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0CR0;
#[doc = "`read()` method returns [ddrphyc_zq0cr0::R](ddrphyc_zq0cr0::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr0::W](ddrphyc_zq0cr0::W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR0 {}
#[doc = "DDRPHYC ZQ0C register 0"]
pub mod ddrphyc_zq0cr0;
#[doc = "DDRPHYC ZQ0CR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr1](ddrphyc_zq0cr1) module"]
pub type DDRPHYC_ZQ0CR1 = crate::Reg<u8, _DDRPHYC_ZQ0CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0CR1;
#[doc = "`read()` method returns [ddrphyc_zq0cr1::R](ddrphyc_zq0cr1::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr1::W](ddrphyc_zq0cr1::W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR1 {}
#[doc = "DDRPHYC ZQ0CR1 register"]
pub mod ddrphyc_zq0cr1;
#[doc = "DDRPHYC ZQ0S register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr0](ddrphyc_zq0sr0) module"]
pub type DDRPHYC_ZQ0SR0 = crate::Reg<u32, _DDRPHYC_ZQ0SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0SR0;
#[doc = "`read()` method returns [ddrphyc_zq0sr0::R](ddrphyc_zq0sr0::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR0 {}
#[doc = "DDRPHYC ZQ0S register 0"]
pub mod ddrphyc_zq0sr0;
#[doc = "DDRPHYC ZQ0S register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr1](ddrphyc_zq0sr1) module"]
pub type DDRPHYC_ZQ0SR1 = crate::Reg<u8, _DDRPHYC_ZQ0SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0SR1;
#[doc = "`read()` method returns [ddrphyc_zq0sr1::R](ddrphyc_zq0sr1::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR1 {}
#[doc = "DDRPHYC ZQ0S register 1"]
pub mod ddrphyc_zq0sr1;
#[doc = "DDRPHYC byte lane 0 GC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gcr](ddrphyc_dx0gcr) module"]
pub type DDRPHYC_DX0GCR = crate::Reg<u32, _DDRPHYC_DX0GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GCR;
#[doc = "`read()` method returns [ddrphyc_dx0gcr::R](ddrphyc_dx0gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0gcr::W](ddrphyc_dx0gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0GCR {}
#[doc = "DDRPHYC byte lane 0 GC register"]
pub mod ddrphyc_dx0gcr;
#[doc = "DDRPHYC byte lane 0 GS register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gsr0](ddrphyc_dx0gsr0) module"]
pub type DDRPHYC_DX0GSR0 = crate::Reg<u16, _DDRPHYC_DX0GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GSR0;
#[doc = "`read()` method returns [ddrphyc_dx0gsr0::R](ddrphyc_dx0gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GSR0 {}
#[doc = "DDRPHYC byte lane 0 GS register 0"]
pub mod ddrphyc_dx0gsr0;
#[doc = "DDRPHYC byte lane 0 GS register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gsr1](ddrphyc_dx0gsr1) module"]
pub type DDRPHYC_DX0GSR1 = crate::Reg<u32, _DDRPHYC_DX0GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GSR1;
#[doc = "`read()` method returns [ddrphyc_dx0gsr1::R](ddrphyc_dx0gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GSR1 {}
#[doc = "DDRPHYC byte lane 0 GS register 1"]
pub mod ddrphyc_dx0gsr1;
#[doc = "DDRPHYC byte lane 0 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dllcr](ddrphyc_dx0dllcr) module"]
pub type DDRPHYC_DX0DLLCR = crate::Reg<u32, _DDRPHYC_DX0DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx0dllcr::R](ddrphyc_dx0dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dllcr::W](ddrphyc_dx0dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DLLCR {}
#[doc = "DDRPHYC byte lane 0 DLLC register"]
pub mod ddrphyc_dx0dllcr;
#[doc = "DDRPHYC byte lane 0 DQT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dqtr](ddrphyc_dx0dqtr) module"]
pub type DDRPHYC_DX0DQTR = crate::Reg<u32, _DDRPHYC_DX0DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DQTR;
#[doc = "`read()` method returns [ddrphyc_dx0dqtr::R](ddrphyc_dx0dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dqtr::W](ddrphyc_dx0dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DQTR {}
#[doc = "DDRPHYC byte lane 0 DQT register"]
pub mod ddrphyc_dx0dqtr;
#[doc = "DDRPHYC byte lane 0 DQST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dqstr](ddrphyc_dx0dqstr) module"]
pub type DDRPHYC_DX0DQSTR = crate::Reg<u32, _DDRPHYC_DX0DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx0dqstr::R](ddrphyc_dx0dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dqstr::W](ddrphyc_dx0dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DQSTR {}
#[doc = "DDRPHYC byte lane 0 DQST register"]
pub mod ddrphyc_dx0dqstr;
#[doc = "DDRPHYC byte lane 1 GC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gcr](ddrphyc_dx1gcr) module"]
pub type DDRPHYC_DX1GCR = crate::Reg<u32, _DDRPHYC_DX1GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GCR;
#[doc = "`read()` method returns [ddrphyc_dx1gcr::R](ddrphyc_dx1gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1gcr::W](ddrphyc_dx1gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1GCR {}
#[doc = "DDRPHYC byte lane 1 GC register"]
pub mod ddrphyc_dx1gcr;
#[doc = "DDRPHYC byte lane 1 GS register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gsr0](ddrphyc_dx1gsr0) module"]
pub type DDRPHYC_DX1GSR0 = crate::Reg<u16, _DDRPHYC_DX1GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GSR0;
#[doc = "`read()` method returns [ddrphyc_dx1gsr0::R](ddrphyc_dx1gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GSR0 {}
#[doc = "DDRPHYC byte lane 1 GS register 0"]
pub mod ddrphyc_dx1gsr0;
#[doc = "DDRPHYC byte lane 1 GS register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gsr1](ddrphyc_dx1gsr1) module"]
pub type DDRPHYC_DX1GSR1 = crate::Reg<u32, _DDRPHYC_DX1GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GSR1;
#[doc = "`read()` method returns [ddrphyc_dx1gsr1::R](ddrphyc_dx1gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GSR1 {}
#[doc = "DDRPHYC byte lane 1 GS register 1"]
pub mod ddrphyc_dx1gsr1;
#[doc = "DDRPHYC byte lane 1 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dllcr](ddrphyc_dx1dllcr) module"]
pub type DDRPHYC_DX1DLLCR = crate::Reg<u32, _DDRPHYC_DX1DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx1dllcr::R](ddrphyc_dx1dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dllcr::W](ddrphyc_dx1dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DLLCR {}
#[doc = "DDRPHYC byte lane 1 DLLC register"]
pub mod ddrphyc_dx1dllcr;
#[doc = "DDRPHYC byte lane 1 DQT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dqtr](ddrphyc_dx1dqtr) module"]
pub type DDRPHYC_DX1DQTR = crate::Reg<u32, _DDRPHYC_DX1DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DQTR;
#[doc = "`read()` method returns [ddrphyc_dx1dqtr::R](ddrphyc_dx1dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dqtr::W](ddrphyc_dx1dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DQTR {}
#[doc = "DDRPHYC byte lane 1 DQT register"]
pub mod ddrphyc_dx1dqtr;
#[doc = "DDRPHYC byte lane 1 DQST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dqstr](ddrphyc_dx1dqstr) module"]
pub type DDRPHYC_DX1DQSTR = crate::Reg<u32, _DDRPHYC_DX1DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx1dqstr::R](ddrphyc_dx1dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dqstr::W](ddrphyc_dx1dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DQSTR {}
#[doc = "DDRPHYC byte lane 1 DQST register"]
pub mod ddrphyc_dx1dqstr;
#[doc = "DDRPHYC byte lane 2 GC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gcr](ddrphyc_dx2gcr) module"]
pub type DDRPHYC_DX2GCR = crate::Reg<u32, _DDRPHYC_DX2GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GCR;
#[doc = "`read()` method returns [ddrphyc_dx2gcr::R](ddrphyc_dx2gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2gcr::W](ddrphyc_dx2gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2GCR {}
#[doc = "DDRPHYC byte lane 2 GC register"]
pub mod ddrphyc_dx2gcr;
#[doc = "DDRPHYC byte lane 2 GS register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gsr0](ddrphyc_dx2gsr0) module"]
pub type DDRPHYC_DX2GSR0 = crate::Reg<u16, _DDRPHYC_DX2GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GSR0;
#[doc = "`read()` method returns [ddrphyc_dx2gsr0::R](ddrphyc_dx2gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GSR0 {}
#[doc = "DDRPHYC byte lane 2 GS register 0"]
pub mod ddrphyc_dx2gsr0;
#[doc = "DDRPHYC byte lane 2 GS register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gsr1](ddrphyc_dx2gsr1) module"]
pub type DDRPHYC_DX2GSR1 = crate::Reg<u32, _DDRPHYC_DX2GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GSR1;
#[doc = "`read()` method returns [ddrphyc_dx2gsr1::R](ddrphyc_dx2gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GSR1 {}
#[doc = "DDRPHYC byte lane 2 GS register 1"]
pub mod ddrphyc_dx2gsr1;
#[doc = "DDRPHYC byte lane 2 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dllcr](ddrphyc_dx2dllcr) module"]
pub type DDRPHYC_DX2DLLCR = crate::Reg<u32, _DDRPHYC_DX2DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx2dllcr::R](ddrphyc_dx2dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dllcr::W](ddrphyc_dx2dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DLLCR {}
#[doc = "DDRPHYC byte lane 2 DLLC register"]
pub mod ddrphyc_dx2dllcr;
#[doc = "DDRPHYC byte lane 2 DQT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dqtr](ddrphyc_dx2dqtr) module"]
pub type DDRPHYC_DX2DQTR = crate::Reg<u32, _DDRPHYC_DX2DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DQTR;
#[doc = "`read()` method returns [ddrphyc_dx2dqtr::R](ddrphyc_dx2dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dqtr::W](ddrphyc_dx2dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DQTR {}
#[doc = "DDRPHYC byte lane 2 DQT register"]
pub mod ddrphyc_dx2dqtr;
#[doc = "DDRPHYC byte lane 2 DQST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dqstr](ddrphyc_dx2dqstr) module"]
pub type DDRPHYC_DX2DQSTR = crate::Reg<u32, _DDRPHYC_DX2DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx2dqstr::R](ddrphyc_dx2dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dqstr::W](ddrphyc_dx2dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DQSTR {}
#[doc = "DDRPHYC byte lane 2 DQST register"]
pub mod ddrphyc_dx2dqstr;
#[doc = "DDRPHYC byte lane 3 GC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gcr](ddrphyc_dx3gcr) module"]
pub type DDRPHYC_DX3GCR = crate::Reg<u32, _DDRPHYC_DX3GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GCR;
#[doc = "`read()` method returns [ddrphyc_dx3gcr::R](ddrphyc_dx3gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3gcr::W](ddrphyc_dx3gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3GCR {}
#[doc = "DDRPHYC byte lane 3 GC register"]
pub mod ddrphyc_dx3gcr;
#[doc = "DDRPHYC byte lane 3 GS register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gsr0](ddrphyc_dx3gsr0) module"]
pub type DDRPHYC_DX3GSR0 = crate::Reg<u16, _DDRPHYC_DX3GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GSR0;
#[doc = "`read()` method returns [ddrphyc_dx3gsr0::R](ddrphyc_dx3gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR0 {}
#[doc = "DDRPHYC byte lane 3 GS register 0"]
pub mod ddrphyc_dx3gsr0;
#[doc = "DDRPHYC byte lane 3 GS register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gsr1](ddrphyc_dx3gsr1) module"]
pub type DDRPHYC_DX3GSR1 = crate::Reg<u32, _DDRPHYC_DX3GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GSR1;
#[doc = "`read()` method returns [ddrphyc_dx3gsr1::R](ddrphyc_dx3gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR1 {}
#[doc = "DDRPHYC byte lane 3 GS register 1"]
pub mod ddrphyc_dx3gsr1;
#[doc = "DDRPHYC byte lane 3 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dllcr](ddrphyc_dx3dllcr) module"]
pub type DDRPHYC_DX3DLLCR = crate::Reg<u32, _DDRPHYC_DX3DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx3dllcr::R](ddrphyc_dx3dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dllcr::W](ddrphyc_dx3dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DLLCR {}
#[doc = "DDRPHYC byte lane 3 DLLC register"]
pub mod ddrphyc_dx3dllcr;
#[doc = "DDRPHYC byte lane 3 DQT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqtr](ddrphyc_dx3dqtr) module"]
pub type DDRPHYC_DX3DQTR = crate::Reg<u32, _DDRPHYC_DX3DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DQTR;
#[doc = "`read()` method returns [ddrphyc_dx3dqtr::R](ddrphyc_dx3dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqtr::W](ddrphyc_dx3dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQTR {}
#[doc = "DDRPHYC byte lane 3 DQT register"]
pub mod ddrphyc_dx3dqtr;
#[doc = "DDRPHYC byte lane 3 DQST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqstr](ddrphyc_dx3dqstr) module"]
pub type DDRPHYC_DX3DQSTR = crate::Reg<u32, _DDRPHYC_DX3DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx3dqstr::R](ddrphyc_dx3dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqstr::W](ddrphyc_dx3dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQSTR {}
#[doc = "DDRPHYC byte lane 3 DQST register"]
pub mod ddrphyc_dx3dqstr;
