#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: CR,
    #[doc = "0x04 - RCC HSI configuration register"]
    pub hsicfgr: HSICFGR,
    #[doc = "0x08 - RCC Clock Recovery RC Register"]
    pub crrcr: CRRCR,
    #[doc = "0x0c - RCC CSI configuration register"]
    pub csicfgr: CSICFGR,
    #[doc = "0x10 - RCC Clock Configuration Register"]
    pub cfgr: CFGR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - RCC Domain 1 Clock Configuration Register"]
    pub d1cfgr: D1CFGR,
    #[doc = "0x1c - RCC Domain 2 Clock Configuration Register"]
    pub d2cfgr: D2CFGR,
    #[doc = "0x20 - RCC Domain 3 Clock Configuration Register"]
    pub d3cfgr: D3CFGR,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - RCC PLLs Clock Source Selection Register"]
    pub pllckselr: PLLCKSELR,
    #[doc = "0x2c - RCC PLLs Configuration Register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x30 - RCC PLL1 Dividers Configuration Register"]
    pub pll1divr: PLL1DIVR,
    #[doc = "0x34 - RCC PLL1 Fractional Divider Register"]
    pub pll1fracr: PLL1FRACR,
    #[doc = "0x38 - RCC PLL2 Dividers Configuration Register"]
    pub pll2divr: PLL2DIVR,
    #[doc = "0x3c - RCC PLL2 Fractional Divider Register"]
    pub pll2fracr: PLL2FRACR,
    #[doc = "0x40 - RCC PLL3 Dividers Configuration Register"]
    pub pll3divr: PLL3DIVR,
    #[doc = "0x44 - RCC PLL3 Fractional Divider Register"]
    pub pll3fracr: PLL3FRACR,
    _reserved16: [u8; 4usize],
    #[doc = "0x4c - RCC Domain 1 Kernel Clock Configuration Register"]
    pub d1ccipr: D1CCIPR,
    #[doc = "0x50 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip1r: D2CCIP1R,
    #[doc = "0x54 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip2r: D2CCIP2R,
    #[doc = "0x58 - RCC Domain 3 Kernel Clock Configuration Register"]
    pub d3ccipr: D3CCIPR,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - RCC Clock Source Interrupt Enable Register"]
    pub cier: CIER,
    #[doc = "0x64 - RCC Clock Source Interrupt Flag Register"]
    pub cifr: CIFR,
    #[doc = "0x68 - RCC Clock Source Interrupt Clear Register"]
    pub cicr: CICR,
    _reserved23: [u8; 4usize],
    #[doc = "0x70 - RCC Backup Domain Control Register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - RCC Clock Control and Status Register"]
    pub csr: CSR,
    _reserved25: [u8; 4usize],
    #[doc = "0x7c - RCC AHB3 Reset Register"]
    pub ahb3rstr: AHB3RSTR,
    #[doc = "0x80 - RCC AHB1 Peripheral Reset Register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x84 - RCC AHB2 Peripheral Reset Register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x88 - RCC AHB4 Peripheral Reset Register"]
    pub ahb4rstr: AHB4RSTR,
    #[doc = "0x8c - RCC APB3 Peripheral Reset Register"]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x90 - RCC APB1 Peripheral Reset Register"]
    pub apb1lrstr: APB1LRSTR,
    #[doc = "0x94 - RCC APB1 Peripheral Reset Register"]
    pub apb1hrstr: APB1HRSTR,
    #[doc = "0x98 - RCC APB2 Peripheral Reset Register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x9c - RCC APB4 Peripheral Reset Register"]
    pub apb4rstr: APB4RSTR,
    #[doc = "0xa0 - RCC Global Control Register"]
    pub gcr: GCR,
    _reserved35: [u8; 4usize],
    #[doc = "0xa8 - RCC D3 Autonomous mode Register"]
    pub d3amr: D3AMR,
    _reserved36: [u8; 36usize],
    #[doc = "0xd0 - RCC Reset Status Register"]
    pub rsr: RSR,
    #[doc = "0xd4 - RCC AHB3 Clock Register"]
    pub ahb3enr: AHB3ENR,
    #[doc = "0xd8 - RCC AHB1 Clock Register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0xdc - RCC AHB2 Clock Register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0xe0 - RCC AHB4 Clock Register"]
    pub ahb4enr: AHB4ENR,
    #[doc = "0xe4 - RCC APB3 Clock Register"]
    pub apb3enr: APB3ENR,
    #[doc = "0xe8 - RCC APB1 Clock Register"]
    pub apb1lenr: APB1LENR,
    #[doc = "0xec - RCC APB1 Clock Register"]
    pub apb1henr: APB1HENR,
    #[doc = "0xf0 - RCC APB2 Clock Register"]
    pub apb2enr: APB2ENR,
    #[doc = "0xf4 - RCC APB4 Clock Register"]
    pub apb4enr: APB4ENR,
    _reserved46: [u8; 4usize],
    #[doc = "0xfc - RCC AHB3 Sleep Clock Register"]
    pub ahb3lpenr: AHB3LPENR,
    #[doc = "0x100 - RCC AHB1 Sleep Clock Register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x104 - RCC AHB2 Sleep Clock Register"]
    pub ahb2lpenr: AHB2LPENR,
    #[doc = "0x108 - RCC AHB4 Sleep Clock Register"]
    pub ahb4lpenr: AHB4LPENR,
    #[doc = "0x10c - RCC APB3 Sleep Clock Register"]
    pub apb3lpenr: APB3LPENR,
    #[doc = "0x110 - RCC APB1 Low Sleep Clock Register"]
    pub apb1llpenr: APB1LLPENR,
    #[doc = "0x114 - RCC APB1 High Sleep Clock Register"]
    pub apb1hlpenr: APB1HLPENR,
    #[doc = "0x118 - RCC APB2 Sleep Clock Register"]
    pub apb2lpenr: APB2LPENR,
    #[doc = "0x11c - RCC APB4 Sleep Clock Register"]
    pub apb4lpenr: APB4LPENR,
    _reserved55: [u8; 16usize],
    #[doc = "0x130 - RCC Reset Status Register"]
    pub c1_rsr: C1_RSR,
    #[doc = "0x134 - RCC AHB3 Clock Register"]
    pub c1_ahb3enr: C1_AHB3ENR,
    #[doc = "0x138 - RCC AHB1 Clock Register"]
    pub c1_ahb1enr: C1_AHB1ENR,
    #[doc = "0x13c - RCC AHB2 Clock Register"]
    pub c1_ahb2enr: C1_AHB2ENR,
    #[doc = "0x140 - RCC AHB4 Clock Register"]
    pub c1_ahb4enr: C1_AHB4ENR,
    #[doc = "0x144 - RCC APB3 Clock Register"]
    pub c1_apb3enr: C1_APB3ENR,
    #[doc = "0x148 - RCC APB1 Clock Register"]
    pub c1_apb1lenr: C1_APB1LENR,
    #[doc = "0x14c - RCC APB1 Clock Register"]
    pub c1_apb1henr: C1_APB1HENR,
    #[doc = "0x150 - RCC APB2 Clock Register"]
    pub c1_apb2enr: C1_APB2ENR,
    #[doc = "0x154 - RCC APB4 Clock Register"]
    pub c1_apb4enr: C1_APB4ENR,
    _reserved65: [u8; 4usize],
    #[doc = "0x15c - RCC AHB3 Sleep Clock Register"]
    pub c1_ahb3lpenr: C1_AHB3LPENR,
    #[doc = "0x160 - RCC AHB1 Sleep Clock Register"]
    pub c1_ahb1lpenr: C1_AHB1LPENR,
    #[doc = "0x164 - RCC AHB2 Sleep Clock Register"]
    pub c1_ahb2lpenr: C1_AHB2LPENR,
    #[doc = "0x168 - RCC AHB4 Sleep Clock Register"]
    pub c1_ahb4lpenr: C1_AHB4LPENR,
    #[doc = "0x16c - RCC APB3 Sleep Clock Register"]
    pub c1_apb3lpenr: C1_APB3LPENR,
    #[doc = "0x170 - RCC APB1 Low Sleep Clock Register"]
    pub c1_apb1llpenr: C1_APB1LLPENR,
    #[doc = "0x174 - RCC APB1 High Sleep Clock Register"]
    pub c1_apb1hlpenr: C1_APB1HLPENR,
    #[doc = "0x178 - RCC APB2 Sleep Clock Register"]
    pub c1_apb2lpenr: C1_APB2LPENR,
    #[doc = "0x17c - RCC APB4 Sleep Clock Register"]
    pub c1_apb4lpenr: C1_APB4LPENR,
}
#[doc = "clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "clock control register"]
pub mod cr;
#[doc = "RCC Clock Recovery RC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crrcr](crrcr) module"]
pub type CRRCR = crate::Reg<u32, _CRRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRRCR;
#[doc = "`read()` method returns [crrcr::R](crrcr::R) reader structure"]
impl crate::Readable for CRRCR {}
#[doc = "RCC Clock Recovery RC Register"]
pub mod crrcr;
#[doc = "RCC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "RCC Clock Configuration Register"]
pub mod cfgr;
#[doc = "RCC Domain 1 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1cfgr](d1cfgr) module"]
pub type D1CFGR = crate::Reg<u32, _D1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D1CFGR;
#[doc = "`read()` method returns [d1cfgr::R](d1cfgr::R) reader structure"]
impl crate::Readable for D1CFGR {}
#[doc = "`write(|w| ..)` method takes [d1cfgr::W](d1cfgr::W) writer structure"]
impl crate::Writable for D1CFGR {}
#[doc = "RCC Domain 1 Clock Configuration Register"]
pub mod d1cfgr;
#[doc = "RCC Domain 2 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2cfgr](d2cfgr) module"]
pub type D2CFGR = crate::Reg<u32, _D2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2CFGR;
#[doc = "`read()` method returns [d2cfgr::R](d2cfgr::R) reader structure"]
impl crate::Readable for D2CFGR {}
#[doc = "`write(|w| ..)` method takes [d2cfgr::W](d2cfgr::W) writer structure"]
impl crate::Writable for D2CFGR {}
#[doc = "RCC Domain 2 Clock Configuration Register"]
pub mod d2cfgr;
#[doc = "RCC Domain 3 Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3cfgr](d3cfgr) module"]
pub type D3CFGR = crate::Reg<u32, _D3CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3CFGR;
#[doc = "`read()` method returns [d3cfgr::R](d3cfgr::R) reader structure"]
impl crate::Readable for D3CFGR {}
#[doc = "`write(|w| ..)` method takes [d3cfgr::W](d3cfgr::W) writer structure"]
impl crate::Writable for D3CFGR {}
#[doc = "RCC Domain 3 Clock Configuration Register"]
pub mod d3cfgr;
#[doc = "RCC PLLs Clock Source Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllckselr](pllckselr) module"]
pub type PLLCKSELR = crate::Reg<u32, _PLLCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCKSELR;
#[doc = "`read()` method returns [pllckselr::R](pllckselr::R) reader structure"]
impl crate::Readable for PLLCKSELR {}
#[doc = "`write(|w| ..)` method takes [pllckselr::W](pllckselr::W) writer structure"]
impl crate::Writable for PLLCKSELR {}
#[doc = "RCC PLLs Clock Source Selection Register"]
pub mod pllckselr;
#[doc = "RCC PLLs Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](pllcfgr) module"]
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
#[doc = "`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure"]
impl crate::Readable for PLLCFGR {}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure"]
impl crate::Writable for PLLCFGR {}
#[doc = "RCC PLLs Configuration Register"]
pub mod pllcfgr;
#[doc = "RCC PLL1 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1divr](pll1divr) module"]
pub type PLL1DIVR = crate::Reg<u32, _PLL1DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1DIVR;
#[doc = "`read()` method returns [pll1divr::R](pll1divr::R) reader structure"]
impl crate::Readable for PLL1DIVR {}
#[doc = "`write(|w| ..)` method takes [pll1divr::W](pll1divr::W) writer structure"]
impl crate::Writable for PLL1DIVR {}
#[doc = "RCC PLL1 Dividers Configuration Register"]
pub mod pll1divr;
#[doc = "RCC PLL1 Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1fracr](pll1fracr) module"]
pub type PLL1FRACR = crate::Reg<u32, _PLL1FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1FRACR;
#[doc = "`read()` method returns [pll1fracr::R](pll1fracr::R) reader structure"]
impl crate::Readable for PLL1FRACR {}
#[doc = "`write(|w| ..)` method takes [pll1fracr::W](pll1fracr::W) writer structure"]
impl crate::Writable for PLL1FRACR {}
#[doc = "RCC PLL1 Fractional Divider Register"]
pub mod pll1fracr;
#[doc = "RCC PLL2 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2divr](pll2divr) module"]
pub type PLL2DIVR = crate::Reg<u32, _PLL2DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL2DIVR;
#[doc = "`read()` method returns [pll2divr::R](pll2divr::R) reader structure"]
impl crate::Readable for PLL2DIVR {}
#[doc = "`write(|w| ..)` method takes [pll2divr::W](pll2divr::W) writer structure"]
impl crate::Writable for PLL2DIVR {}
#[doc = "RCC PLL2 Dividers Configuration Register"]
pub mod pll2divr;
#[doc = "RCC PLL2 Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll2fracr](pll2fracr) module"]
pub type PLL2FRACR = crate::Reg<u32, _PLL2FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL2FRACR;
#[doc = "`read()` method returns [pll2fracr::R](pll2fracr::R) reader structure"]
impl crate::Readable for PLL2FRACR {}
#[doc = "`write(|w| ..)` method takes [pll2fracr::W](pll2fracr::W) writer structure"]
impl crate::Writable for PLL2FRACR {}
#[doc = "RCC PLL2 Fractional Divider Register"]
pub mod pll2fracr;
#[doc = "RCC PLL3 Dividers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3divr](pll3divr) module"]
pub type PLL3DIVR = crate::Reg<u32, _PLL3DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL3DIVR;
#[doc = "`read()` method returns [pll3divr::R](pll3divr::R) reader structure"]
impl crate::Readable for PLL3DIVR {}
#[doc = "`write(|w| ..)` method takes [pll3divr::W](pll3divr::W) writer structure"]
impl crate::Writable for PLL3DIVR {}
#[doc = "RCC PLL3 Dividers Configuration Register"]
pub mod pll3divr;
#[doc = "RCC PLL3 Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll3fracr](pll3fracr) module"]
pub type PLL3FRACR = crate::Reg<u32, _PLL3FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL3FRACR;
#[doc = "`read()` method returns [pll3fracr::R](pll3fracr::R) reader structure"]
impl crate::Readable for PLL3FRACR {}
#[doc = "`write(|w| ..)` method takes [pll3fracr::W](pll3fracr::W) writer structure"]
impl crate::Writable for PLL3FRACR {}
#[doc = "RCC PLL3 Fractional Divider Register"]
pub mod pll3fracr;
#[doc = "RCC Domain 1 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1ccipr](d1ccipr) module"]
pub type D1CCIPR = crate::Reg<u32, _D1CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D1CCIPR;
#[doc = "`read()` method returns [d1ccipr::R](d1ccipr::R) reader structure"]
impl crate::Readable for D1CCIPR {}
#[doc = "`write(|w| ..)` method takes [d1ccipr::W](d1ccipr::W) writer structure"]
impl crate::Writable for D1CCIPR {}
#[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
pub mod d1ccipr;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2ccip1r](d2ccip1r) module"]
pub type D2CCIP1R = crate::Reg<u32, _D2CCIP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2CCIP1R;
#[doc = "`read()` method returns [d2ccip1r::R](d2ccip1r::R) reader structure"]
impl crate::Readable for D2CCIP1R {}
#[doc = "`write(|w| ..)` method takes [d2ccip1r::W](d2ccip1r::W) writer structure"]
impl crate::Writable for D2CCIP1R {}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip1r;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2ccip2r](d2ccip2r) module"]
pub type D2CCIP2R = crate::Reg<u32, _D2CCIP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D2CCIP2R;
#[doc = "`read()` method returns [d2ccip2r::R](d2ccip2r::R) reader structure"]
impl crate::Readable for D2CCIP2R {}
#[doc = "`write(|w| ..)` method takes [d2ccip2r::W](d2ccip2r::W) writer structure"]
impl crate::Writable for D2CCIP2R {}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip2r;
#[doc = "RCC Domain 3 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3ccipr](d3ccipr) module"]
pub type D3CCIPR = crate::Reg<u32, _D3CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3CCIPR;
#[doc = "`read()` method returns [d3ccipr::R](d3ccipr::R) reader structure"]
impl crate::Readable for D3CCIPR {}
#[doc = "`write(|w| ..)` method takes [d3ccipr::W](d3ccipr::W) writer structure"]
impl crate::Writable for D3CCIPR {}
#[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
pub mod d3ccipr;
#[doc = "RCC Clock Source Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](cier) module"]
pub type CIER = crate::Reg<u32, _CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIER;
#[doc = "`read()` method returns [cier::R](cier::R) reader structure"]
impl crate::Readable for CIER {}
#[doc = "`write(|w| ..)` method takes [cier::W](cier::W) writer structure"]
impl crate::Writable for CIER {}
#[doc = "RCC Clock Source Interrupt Enable Register"]
pub mod cier;
#[doc = "RCC Clock Source Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](cifr) module"]
pub type CIFR = crate::Reg<u32, _CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIFR;
#[doc = "`read()` method returns [cifr::R](cifr::R) reader structure"]
impl crate::Readable for CIFR {}
#[doc = "RCC Clock Source Interrupt Flag Register"]
pub mod cifr;
#[doc = "RCC Clock Source Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](cicr) module"]
pub type CICR = crate::Reg<u32, _CICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CICR;
#[doc = "`read()` method returns [cicr::R](cicr::R) reader structure"]
impl crate::Readable for CICR {}
#[doc = "`write(|w| ..)` method takes [cicr::W](cicr::W) writer structure"]
impl crate::Writable for CICR {}
#[doc = "RCC Clock Source Interrupt Clear Register"]
pub mod cicr;
#[doc = "RCC Backup Domain Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](bdcr) module"]
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
#[doc = "`read()` method returns [bdcr::R](bdcr::R) reader structure"]
impl crate::Readable for BDCR {}
#[doc = "`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure"]
impl crate::Writable for BDCR {}
#[doc = "RCC Backup Domain Control Register"]
pub mod bdcr;
#[doc = "RCC Clock Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "RCC Clock Control and Status Register"]
pub mod csr;
#[doc = "RCC AHB3 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](ahb3rstr) module"]
pub type AHB3RSTR = crate::Reg<u32, _AHB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3RSTR;
#[doc = "`read()` method returns [ahb3rstr::R](ahb3rstr::R) reader structure"]
impl crate::Readable for AHB3RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](ahb3rstr::W) writer structure"]
impl crate::Writable for AHB3RSTR {}
#[doc = "RCC AHB3 Reset Register"]
pub mod ahb3rstr;
#[doc = "RCC AHB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](ahb1rstr) module"]
pub type AHB1RSTR = crate::Reg<u32, _AHB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1RSTR;
#[doc = "`read()` method returns [ahb1rstr::R](ahb1rstr::R) reader structure"]
impl crate::Readable for AHB1RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](ahb1rstr::W) writer structure"]
impl crate::Writable for AHB1RSTR {}
#[doc = "RCC AHB1 Peripheral Reset Register"]
pub mod ahb1rstr;
#[doc = "RCC AHB2 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](ahb2rstr) module"]
pub type AHB2RSTR = crate::Reg<u32, _AHB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2RSTR;
#[doc = "`read()` method returns [ahb2rstr::R](ahb2rstr::R) reader structure"]
impl crate::Readable for AHB2RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](ahb2rstr::W) writer structure"]
impl crate::Writable for AHB2RSTR {}
#[doc = "RCC AHB2 Peripheral Reset Register"]
pub mod ahb2rstr;
#[doc = "RCC AHB4 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4rstr](ahb4rstr) module"]
pub type AHB4RSTR = crate::Reg<u32, _AHB4RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB4RSTR;
#[doc = "`read()` method returns [ahb4rstr::R](ahb4rstr::R) reader structure"]
impl crate::Readable for AHB4RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb4rstr::W](ahb4rstr::W) writer structure"]
impl crate::Writable for AHB4RSTR {}
#[doc = "RCC AHB4 Peripheral Reset Register"]
pub mod ahb4rstr;
#[doc = "RCC APB3 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](apb3rstr) module"]
pub type APB3RSTR = crate::Reg<u32, _APB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3RSTR;
#[doc = "`read()` method returns [apb3rstr::R](apb3rstr::R) reader structure"]
impl crate::Readable for APB3RSTR {}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](apb3rstr::W) writer structure"]
impl crate::Writable for APB3RSTR {}
#[doc = "RCC APB3 Peripheral Reset Register"]
pub mod apb3rstr;
#[doc = "RCC APB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lrstr](apb1lrstr) module"]
pub type APB1LRSTR = crate::Reg<u32, _APB1LRSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LRSTR;
#[doc = "`read()` method returns [apb1lrstr::R](apb1lrstr::R) reader structure"]
impl crate::Readable for APB1LRSTR {}
#[doc = "`write(|w| ..)` method takes [apb1lrstr::W](apb1lrstr::W) writer structure"]
impl crate::Writable for APB1LRSTR {}
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1lrstr;
#[doc = "RCC APB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hrstr](apb1hrstr) module"]
pub type APB1HRSTR = crate::Reg<u32, _APB1HRSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1HRSTR;
#[doc = "`read()` method returns [apb1hrstr::R](apb1hrstr::R) reader structure"]
impl crate::Readable for APB1HRSTR {}
#[doc = "`write(|w| ..)` method takes [apb1hrstr::W](apb1hrstr::W) writer structure"]
impl crate::Writable for APB1HRSTR {}
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1hrstr;
#[doc = "RCC APB2 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](apb2rstr) module"]
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
#[doc = "`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure"]
impl crate::Readable for APB2RSTR {}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure"]
impl crate::Writable for APB2RSTR {}
#[doc = "RCC APB2 Peripheral Reset Register"]
pub mod apb2rstr;
#[doc = "RCC APB4 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4rstr](apb4rstr) module"]
pub type APB4RSTR = crate::Reg<u32, _APB4RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB4RSTR;
#[doc = "`read()` method returns [apb4rstr::R](apb4rstr::R) reader structure"]
impl crate::Readable for APB4RSTR {}
#[doc = "`write(|w| ..)` method takes [apb4rstr::W](apb4rstr::W) writer structure"]
impl crate::Writable for APB4RSTR {}
#[doc = "RCC APB4 Peripheral Reset Register"]
pub mod apb4rstr;
#[doc = "RCC Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](gcr) module"]
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
#[doc = "`read()` method returns [gcr::R](gcr::R) reader structure"]
impl crate::Readable for GCR {}
#[doc = "`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure"]
impl crate::Writable for GCR {}
#[doc = "RCC Global Control Register"]
pub mod gcr;
#[doc = "RCC D3 Autonomous mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3amr](d3amr) module"]
pub type D3AMR = crate::Reg<u32, _D3AMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D3AMR;
#[doc = "`read()` method returns [d3amr::R](d3amr::R) reader structure"]
impl crate::Readable for D3AMR {}
#[doc = "`write(|w| ..)` method takes [d3amr::W](d3amr::W) writer structure"]
impl crate::Writable for D3AMR {}
#[doc = "RCC D3 Autonomous mode Register"]
pub mod d3amr;
#[doc = "RCC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "RCC Reset Status Register"]
pub mod rsr;
#[doc = "RCC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_rsr](c1_rsr) module"]
pub type C1_RSR = crate::Reg<u32, _C1_RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_RSR;
#[doc = "`read()` method returns [c1_rsr::R](c1_rsr::R) reader structure"]
impl crate::Readable for C1_RSR {}
#[doc = "`write(|w| ..)` method takes [c1_rsr::W](c1_rsr::W) writer structure"]
impl crate::Writable for C1_RSR {}
#[doc = "RCC Reset Status Register"]
pub mod c1_rsr;
#[doc = "RCC AHB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb3enr](c1_ahb3enr) module"]
pub type C1_AHB3ENR = crate::Reg<u32, _C1_AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB3ENR;
#[doc = "`read()` method returns [c1_ahb3enr::R](c1_ahb3enr::R) reader structure"]
impl crate::Readable for C1_AHB3ENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb3enr::W](c1_ahb3enr::W) writer structure"]
impl crate::Writable for C1_AHB3ENR {}
#[doc = "RCC AHB3 Clock Register"]
pub mod c1_ahb3enr;
#[doc = "RCC AHB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](ahb3enr) module"]
pub type AHB3ENR = crate::Reg<u32, _AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3ENR;
#[doc = "`read()` method returns [ahb3enr::R](ahb3enr::R) reader structure"]
impl crate::Readable for AHB3ENR {}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](ahb3enr::W) writer structure"]
impl crate::Writable for AHB3ENR {}
#[doc = "RCC AHB3 Clock Register"]
pub mod ahb3enr;
#[doc = "RCC AHB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](ahb1enr) module"]
pub type AHB1ENR = crate::Reg<u32, _AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1ENR;
#[doc = "`read()` method returns [ahb1enr::R](ahb1enr::R) reader structure"]
impl crate::Readable for AHB1ENR {}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](ahb1enr::W) writer structure"]
impl crate::Writable for AHB1ENR {}
#[doc = "RCC AHB1 Clock Register"]
pub mod ahb1enr;
#[doc = "RCC AHB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb1enr](c1_ahb1enr) module"]
pub type C1_AHB1ENR = crate::Reg<u32, _C1_AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB1ENR;
#[doc = "`read()` method returns [c1_ahb1enr::R](c1_ahb1enr::R) reader structure"]
impl crate::Readable for C1_AHB1ENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb1enr::W](c1_ahb1enr::W) writer structure"]
impl crate::Writable for C1_AHB1ENR {}
#[doc = "RCC AHB1 Clock Register"]
pub mod c1_ahb1enr;
#[doc = "RCC AHB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb2enr](c1_ahb2enr) module"]
pub type C1_AHB2ENR = crate::Reg<u32, _C1_AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB2ENR;
#[doc = "`read()` method returns [c1_ahb2enr::R](c1_ahb2enr::R) reader structure"]
impl crate::Readable for C1_AHB2ENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb2enr::W](c1_ahb2enr::W) writer structure"]
impl crate::Writable for C1_AHB2ENR {}
#[doc = "RCC AHB2 Clock Register"]
pub mod c1_ahb2enr;
#[doc = "RCC AHB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](ahb2enr) module"]
pub type AHB2ENR = crate::Reg<u32, _AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2ENR;
#[doc = "`read()` method returns [ahb2enr::R](ahb2enr::R) reader structure"]
impl crate::Readable for AHB2ENR {}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](ahb2enr::W) writer structure"]
impl crate::Writable for AHB2ENR {}
#[doc = "RCC AHB2 Clock Register"]
pub mod ahb2enr;
#[doc = "RCC AHB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4enr](ahb4enr) module"]
pub type AHB4ENR = crate::Reg<u32, _AHB4ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB4ENR;
#[doc = "`read()` method returns [ahb4enr::R](ahb4enr::R) reader structure"]
impl crate::Readable for AHB4ENR {}
#[doc = "`write(|w| ..)` method takes [ahb4enr::W](ahb4enr::W) writer structure"]
impl crate::Writable for AHB4ENR {}
#[doc = "RCC AHB4 Clock Register"]
pub mod ahb4enr;
#[doc = "RCC AHB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb4enr](c1_ahb4enr) module"]
pub type C1_AHB4ENR = crate::Reg<u32, _C1_AHB4ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB4ENR;
#[doc = "`read()` method returns [c1_ahb4enr::R](c1_ahb4enr::R) reader structure"]
impl crate::Readable for C1_AHB4ENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb4enr::W](c1_ahb4enr::W) writer structure"]
impl crate::Writable for C1_AHB4ENR {}
#[doc = "RCC AHB4 Clock Register"]
pub mod c1_ahb4enr;
#[doc = "RCC APB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb3enr](c1_apb3enr) module"]
pub type C1_APB3ENR = crate::Reg<u32, _C1_APB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB3ENR;
#[doc = "`read()` method returns [c1_apb3enr::R](c1_apb3enr::R) reader structure"]
impl crate::Readable for C1_APB3ENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb3enr::W](c1_apb3enr::W) writer structure"]
impl crate::Writable for C1_APB3ENR {}
#[doc = "RCC APB3 Clock Register"]
pub mod c1_apb3enr;
#[doc = "RCC APB3 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3enr](apb3enr) module"]
pub type APB3ENR = crate::Reg<u32, _APB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3ENR;
#[doc = "`read()` method returns [apb3enr::R](apb3enr::R) reader structure"]
impl crate::Readable for APB3ENR {}
#[doc = "`write(|w| ..)` method takes [apb3enr::W](apb3enr::W) writer structure"]
impl crate::Writable for APB3ENR {}
#[doc = "RCC APB3 Clock Register"]
pub mod apb3enr;
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lenr](apb1lenr) module"]
pub type APB1LENR = crate::Reg<u32, _APB1LENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LENR;
#[doc = "`read()` method returns [apb1lenr::R](apb1lenr::R) reader structure"]
impl crate::Readable for APB1LENR {}
#[doc = "`write(|w| ..)` method takes [apb1lenr::W](apb1lenr::W) writer structure"]
impl crate::Writable for APB1LENR {}
#[doc = "RCC APB1 Clock Register"]
pub mod apb1lenr;
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1lenr](c1_apb1lenr) module"]
pub type C1_APB1LENR = crate::Reg<u32, _C1_APB1LENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB1LENR;
#[doc = "`read()` method returns [c1_apb1lenr::R](c1_apb1lenr::R) reader structure"]
impl crate::Readable for C1_APB1LENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb1lenr::W](c1_apb1lenr::W) writer structure"]
impl crate::Writable for C1_APB1LENR {}
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1lenr;
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1henr](apb1henr) module"]
pub type APB1HENR = crate::Reg<u32, _APB1HENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1HENR;
#[doc = "`read()` method returns [apb1henr::R](apb1henr::R) reader structure"]
impl crate::Readable for APB1HENR {}
#[doc = "`write(|w| ..)` method takes [apb1henr::W](apb1henr::W) writer structure"]
impl crate::Writable for APB1HENR {}
#[doc = "RCC APB1 Clock Register"]
pub mod apb1henr;
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1henr](c1_apb1henr) module"]
pub type C1_APB1HENR = crate::Reg<u32, _C1_APB1HENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB1HENR;
#[doc = "`read()` method returns [c1_apb1henr::R](c1_apb1henr::R) reader structure"]
impl crate::Readable for C1_APB1HENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb1henr::W](c1_apb1henr::W) writer structure"]
impl crate::Writable for C1_APB1HENR {}
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1henr;
#[doc = "RCC APB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb2enr](c1_apb2enr) module"]
pub type C1_APB2ENR = crate::Reg<u32, _C1_APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB2ENR;
#[doc = "`read()` method returns [c1_apb2enr::R](c1_apb2enr::R) reader structure"]
impl crate::Readable for C1_APB2ENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb2enr::W](c1_apb2enr::W) writer structure"]
impl crate::Writable for C1_APB2ENR {}
#[doc = "RCC APB2 Clock Register"]
pub mod c1_apb2enr;
#[doc = "RCC APB2 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](apb2enr) module"]
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
#[doc = "`read()` method returns [apb2enr::R](apb2enr::R) reader structure"]
impl crate::Readable for APB2ENR {}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure"]
impl crate::Writable for APB2ENR {}
#[doc = "RCC APB2 Clock Register"]
pub mod apb2enr;
#[doc = "RCC APB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4enr](apb4enr) module"]
pub type APB4ENR = crate::Reg<u32, _APB4ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB4ENR;
#[doc = "`read()` method returns [apb4enr::R](apb4enr::R) reader structure"]
impl crate::Readable for APB4ENR {}
#[doc = "`write(|w| ..)` method takes [apb4enr::W](apb4enr::W) writer structure"]
impl crate::Writable for APB4ENR {}
#[doc = "RCC APB4 Clock Register"]
pub mod apb4enr;
#[doc = "RCC APB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb4enr](c1_apb4enr) module"]
pub type C1_APB4ENR = crate::Reg<u32, _C1_APB4ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB4ENR;
#[doc = "`read()` method returns [c1_apb4enr::R](c1_apb4enr::R) reader structure"]
impl crate::Readable for C1_APB4ENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb4enr::W](c1_apb4enr::W) writer structure"]
impl crate::Writable for C1_APB4ENR {}
#[doc = "RCC APB4 Clock Register"]
pub mod c1_apb4enr;
#[doc = "RCC AHB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb3lpenr](c1_ahb3lpenr) module"]
pub type C1_AHB3LPENR = crate::Reg<u32, _C1_AHB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB3LPENR;
#[doc = "`read()` method returns [c1_ahb3lpenr::R](c1_ahb3lpenr::R) reader structure"]
impl crate::Readable for C1_AHB3LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb3lpenr::W](c1_ahb3lpenr::W) writer structure"]
impl crate::Writable for C1_AHB3LPENR {}
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod c1_ahb3lpenr;
#[doc = "RCC AHB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](ahb3lpenr) module"]
pub type AHB3LPENR = crate::Reg<u32, _AHB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3LPENR;
#[doc = "`read()` method returns [ahb3lpenr::R](ahb3lpenr::R) reader structure"]
impl crate::Readable for AHB3LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb3lpenr::W](ahb3lpenr::W) writer structure"]
impl crate::Writable for AHB3LPENR {}
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod ahb3lpenr;
#[doc = "RCC AHB1 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](ahb1lpenr) module"]
pub type AHB1LPENR = crate::Reg<u32, _AHB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1LPENR;
#[doc = "`read()` method returns [ahb1lpenr::R](ahb1lpenr::R) reader structure"]
impl crate::Readable for AHB1LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](ahb1lpenr::W) writer structure"]
impl crate::Writable for AHB1LPENR {}
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod ahb1lpenr;
#[doc = "RCC AHB1 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb1lpenr](c1_ahb1lpenr) module"]
pub type C1_AHB1LPENR = crate::Reg<u32, _C1_AHB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB1LPENR;
#[doc = "`read()` method returns [c1_ahb1lpenr::R](c1_ahb1lpenr::R) reader structure"]
impl crate::Readable for C1_AHB1LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb1lpenr::W](c1_ahb1lpenr::W) writer structure"]
impl crate::Writable for C1_AHB1LPENR {}
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod c1_ahb1lpenr;
#[doc = "RCC AHB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb2lpenr](c1_ahb2lpenr) module"]
pub type C1_AHB2LPENR = crate::Reg<u32, _C1_AHB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB2LPENR;
#[doc = "`read()` method returns [c1_ahb2lpenr::R](c1_ahb2lpenr::R) reader structure"]
impl crate::Readable for C1_AHB2LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb2lpenr::W](c1_ahb2lpenr::W) writer structure"]
impl crate::Writable for C1_AHB2LPENR {}
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod c1_ahb2lpenr;
#[doc = "RCC AHB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](ahb2lpenr) module"]
pub type AHB2LPENR = crate::Reg<u32, _AHB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2LPENR;
#[doc = "`read()` method returns [ahb2lpenr::R](ahb2lpenr::R) reader structure"]
impl crate::Readable for AHB2LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](ahb2lpenr::W) writer structure"]
impl crate::Writable for AHB2LPENR {}
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod ahb2lpenr;
#[doc = "RCC AHB4 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4lpenr](ahb4lpenr) module"]
pub type AHB4LPENR = crate::Reg<u32, _AHB4LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB4LPENR;
#[doc = "`read()` method returns [ahb4lpenr::R](ahb4lpenr::R) reader structure"]
impl crate::Readable for AHB4LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb4lpenr::W](ahb4lpenr::W) writer structure"]
impl crate::Writable for AHB4LPENR {}
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod ahb4lpenr;
#[doc = "RCC AHB4 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb4lpenr](c1_ahb4lpenr) module"]
pub type C1_AHB4LPENR = crate::Reg<u32, _C1_AHB4LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_AHB4LPENR;
#[doc = "`read()` method returns [c1_ahb4lpenr::R](c1_ahb4lpenr::R) reader structure"]
impl crate::Readable for C1_AHB4LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_ahb4lpenr::W](c1_ahb4lpenr::W) writer structure"]
impl crate::Writable for C1_AHB4LPENR {}
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod c1_ahb4lpenr;
#[doc = "RCC APB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb3lpenr](c1_apb3lpenr) module"]
pub type C1_APB3LPENR = crate::Reg<u32, _C1_APB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB3LPENR;
#[doc = "`read()` method returns [c1_apb3lpenr::R](c1_apb3lpenr::R) reader structure"]
impl crate::Readable for C1_APB3LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb3lpenr::W](c1_apb3lpenr::W) writer structure"]
impl crate::Writable for C1_APB3LPENR {}
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod c1_apb3lpenr;
#[doc = "RCC APB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3lpenr](apb3lpenr) module"]
pub type APB3LPENR = crate::Reg<u32, _APB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3LPENR;
#[doc = "`read()` method returns [apb3lpenr::R](apb3lpenr::R) reader structure"]
impl crate::Readable for APB3LPENR {}
#[doc = "`write(|w| ..)` method takes [apb3lpenr::W](apb3lpenr::W) writer structure"]
impl crate::Writable for APB3LPENR {}
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod apb3lpenr;
#[doc = "RCC APB1 Low Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1llpenr](apb1llpenr) module"]
pub type APB1LLPENR = crate::Reg<u32, _APB1LLPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LLPENR;
#[doc = "`read()` method returns [apb1llpenr::R](apb1llpenr::R) reader structure"]
impl crate::Readable for APB1LLPENR {}
#[doc = "`write(|w| ..)` method takes [apb1llpenr::W](apb1llpenr::W) writer structure"]
impl crate::Writable for APB1LLPENR {}
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod apb1llpenr;
#[doc = "RCC APB1 Low Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1llpenr](c1_apb1llpenr) module"]
pub type C1_APB1LLPENR = crate::Reg<u32, _C1_APB1LLPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB1LLPENR;
#[doc = "`read()` method returns [c1_apb1llpenr::R](c1_apb1llpenr::R) reader structure"]
impl crate::Readable for C1_APB1LLPENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb1llpenr::W](c1_apb1llpenr::W) writer structure"]
impl crate::Writable for C1_APB1LLPENR {}
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod c1_apb1llpenr;
#[doc = "RCC APB1 High Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1hlpenr](c1_apb1hlpenr) module"]
pub type C1_APB1HLPENR = crate::Reg<u32, _C1_APB1HLPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB1HLPENR;
#[doc = "`read()` method returns [c1_apb1hlpenr::R](c1_apb1hlpenr::R) reader structure"]
impl crate::Readable for C1_APB1HLPENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb1hlpenr::W](c1_apb1hlpenr::W) writer structure"]
impl crate::Writable for C1_APB1HLPENR {}
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod c1_apb1hlpenr;
#[doc = "RCC APB1 High Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hlpenr](apb1hlpenr) module"]
pub type APB1HLPENR = crate::Reg<u32, _APB1HLPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1HLPENR;
#[doc = "`read()` method returns [apb1hlpenr::R](apb1hlpenr::R) reader structure"]
impl crate::Readable for APB1HLPENR {}
#[doc = "`write(|w| ..)` method takes [apb1hlpenr::W](apb1hlpenr::W) writer structure"]
impl crate::Writable for APB1HLPENR {}
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod apb1hlpenr;
#[doc = "RCC APB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](apb2lpenr) module"]
pub type APB2LPENR = crate::Reg<u32, _APB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2LPENR;
#[doc = "`read()` method returns [apb2lpenr::R](apb2lpenr::R) reader structure"]
impl crate::Readable for APB2LPENR {}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](apb2lpenr::W) writer structure"]
impl crate::Writable for APB2LPENR {}
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod apb2lpenr;
#[doc = "RCC APB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb2lpenr](c1_apb2lpenr) module"]
pub type C1_APB2LPENR = crate::Reg<u32, _C1_APB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB2LPENR;
#[doc = "`read()` method returns [c1_apb2lpenr::R](c1_apb2lpenr::R) reader structure"]
impl crate::Readable for C1_APB2LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb2lpenr::W](c1_apb2lpenr::W) writer structure"]
impl crate::Writable for C1_APB2LPENR {}
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod c1_apb2lpenr;
#[doc = "RCC APB4 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb4lpenr](c1_apb4lpenr) module"]
pub type C1_APB4LPENR = crate::Reg<u32, _C1_APB4LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1_APB4LPENR;
#[doc = "`read()` method returns [c1_apb4lpenr::R](c1_apb4lpenr::R) reader structure"]
impl crate::Readable for C1_APB4LPENR {}
#[doc = "`write(|w| ..)` method takes [c1_apb4lpenr::W](c1_apb4lpenr::W) writer structure"]
impl crate::Writable for C1_APB4LPENR {}
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod c1_apb4lpenr;
#[doc = "RCC APB4 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4lpenr](apb4lpenr) module"]
pub type APB4LPENR = crate::Reg<u32, _APB4LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB4LPENR;
#[doc = "`read()` method returns [apb4lpenr::R](apb4lpenr::R) reader structure"]
impl crate::Readable for APB4LPENR {}
#[doc = "`write(|w| ..)` method takes [apb4lpenr::W](apb4lpenr::W) writer structure"]
impl crate::Writable for APB4LPENR {}
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod apb4lpenr;
#[doc = "RCC HSI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsicfgr](hsicfgr) module"]
pub type HSICFGR = crate::Reg<u32, _HSICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSICFGR;
#[doc = "`read()` method returns [hsicfgr::R](hsicfgr::R) reader structure"]
impl crate::Readable for HSICFGR {}
#[doc = "`write(|w| ..)` method takes [hsicfgr::W](hsicfgr::W) writer structure"]
impl crate::Writable for HSICFGR {}
#[doc = "RCC HSI configuration register"]
pub mod hsicfgr;
#[doc = "RCC CSI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicfgr](csicfgr) module"]
pub type CSICFGR = crate::Reg<u32, _CSICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICFGR;
#[doc = "`read()` method returns [csicfgr::R](csicfgr::R) reader structure"]
impl crate::Readable for CSICFGR {}
#[doc = "`write(|w| ..)` method takes [csicfgr::W](csicfgr::W) writer structure"]
impl crate::Writable for CSICFGR {}
#[doc = "RCC CSI configuration register"]
pub mod csicfgr;
