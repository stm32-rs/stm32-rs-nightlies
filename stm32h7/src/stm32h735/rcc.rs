#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - RCC HSI configuration register"]
    pub hsicfgr: crate::Reg<hsicfgr::HSICFGR_SPEC>,
    #[doc = "0x08 - RCC Clock Recovery RC Register"]
    pub crrcr: crate::Reg<crrcr::CRRCR_SPEC>,
    #[doc = "0x0c - RCC CSI configuration register"]
    pub csicfgr: crate::Reg<csicfgr::CSICFGR_SPEC>,
    #[doc = "0x10 - RCC Clock Configuration Register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - RCC Domain 1 Clock Configuration Register"]
    pub d1cfgr: crate::Reg<d1cfgr::D1CFGR_SPEC>,
    #[doc = "0x1c - RCC Domain 2 Clock Configuration Register"]
    pub d2cfgr: crate::Reg<d2cfgr::D2CFGR_SPEC>,
    #[doc = "0x20 - RCC Domain 3 Clock Configuration Register"]
    pub d3cfgr: crate::Reg<d3cfgr::D3CFGR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - RCC PLLs Clock Source Selection Register"]
    pub pllckselr: crate::Reg<pllckselr::PLLCKSELR_SPEC>,
    #[doc = "0x2c - RCC PLLs Configuration Register"]
    pub pllcfgr: crate::Reg<pllcfgr::PLLCFGR_SPEC>,
    #[doc = "0x30 - RCC PLL1 Dividers Configuration Register"]
    pub pll1divr: crate::Reg<pll1divr::PLL1DIVR_SPEC>,
    #[doc = "0x34 - RCC PLL1 Fractional Divider Register"]
    pub pll1fracr: crate::Reg<pll1fracr::PLL1FRACR_SPEC>,
    #[doc = "0x38 - RCC PLL2 Dividers Configuration Register"]
    pub pll2divr: crate::Reg<pll2divr::PLL2DIVR_SPEC>,
    #[doc = "0x3c - RCC PLL2 Fractional Divider Register"]
    pub pll2fracr: crate::Reg<pll2fracr::PLL2FRACR_SPEC>,
    #[doc = "0x40 - RCC PLL3 Dividers Configuration Register"]
    pub pll3divr: crate::Reg<pll3divr::PLL3DIVR_SPEC>,
    #[doc = "0x44 - RCC PLL3 Fractional Divider Register"]
    pub pll3fracr: crate::Reg<pll3fracr::PLL3FRACR_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x4c - RCC Domain 1 Kernel Clock Configuration Register"]
    pub d1ccipr: crate::Reg<d1ccipr::D1CCIPR_SPEC>,
    #[doc = "0x50 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip1r: crate::Reg<d2ccip1r::D2CCIP1R_SPEC>,
    #[doc = "0x54 - RCC Domain 2 Kernel Clock Configuration Register"]
    pub d2ccip2r: crate::Reg<d2ccip2r::D2CCIP2R_SPEC>,
    #[doc = "0x58 - RCC Domain 3 Kernel Clock Configuration Register"]
    pub d3ccipr: crate::Reg<d3ccipr::D3CCIPR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - RCC Clock Source Interrupt Enable Register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x64 - RCC Clock Source Interrupt Flag Register"]
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    #[doc = "0x68 - RCC Clock Source Interrupt Clear Register"]
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - RCC Backup Domain Control Register"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x74 - RCC Clock Control and Status Register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x7c - RCC AHB3 Reset Register"]
    pub ahb3rstr: crate::Reg<ahb3rstr::AHB3RSTR_SPEC>,
    #[doc = "0x80 - RCC AHB1 Peripheral Reset Register"]
    pub ahb1rstr: crate::Reg<ahb1rstr::AHB1RSTR_SPEC>,
    #[doc = "0x84 - RCC AHB2 Peripheral Reset Register"]
    pub ahb2rstr: crate::Reg<ahb2rstr::AHB2RSTR_SPEC>,
    #[doc = "0x88 - RCC AHB4 Peripheral Reset Register"]
    pub ahb4rstr: crate::Reg<ahb4rstr::AHB4RSTR_SPEC>,
    #[doc = "0x8c - RCC APB3 Peripheral Reset Register"]
    pub apb3rstr: crate::Reg<apb3rstr::APB3RSTR_SPEC>,
    #[doc = "0x90 - RCC APB1 Peripheral Reset Register"]
    pub apb1lrstr: crate::Reg<apb1lrstr::APB1LRSTR_SPEC>,
    #[doc = "0x94 - RCC APB1 Peripheral Reset Register"]
    pub apb1hrstr: crate::Reg<apb1hrstr::APB1HRSTR_SPEC>,
    #[doc = "0x98 - RCC APB2 Peripheral Reset Register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    #[doc = "0x9c - RCC APB4 Peripheral Reset Register"]
    pub apb4rstr: crate::Reg<apb4rstr::APB4RSTR_SPEC>,
    #[doc = "0xa0 - RCC Global Control Register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0xa8 - RCC D3 Autonomous mode Register"]
    pub d3amr: crate::Reg<d3amr::D3AMR_SPEC>,
    _reserved36: [u8; 0x24],
    #[doc = "0xd0 - RCC Reset Status Register"]
    pub rsr: crate::Reg<rsr::RSR_SPEC>,
    #[doc = "0xd4 - RCC AHB3 Clock Register"]
    pub ahb3enr: crate::Reg<ahb3enr::AHB3ENR_SPEC>,
    #[doc = "0xd8 - RCC AHB1 Clock Register"]
    pub ahb1enr: crate::Reg<ahb1enr::AHB1ENR_SPEC>,
    #[doc = "0xdc - RCC AHB2 Clock Register"]
    pub ahb2enr: crate::Reg<ahb2enr::AHB2ENR_SPEC>,
    #[doc = "0xe0 - RCC AHB4 Clock Register"]
    pub ahb4enr: crate::Reg<ahb4enr::AHB4ENR_SPEC>,
    #[doc = "0xe4 - RCC APB3 Clock Register"]
    pub apb3enr: crate::Reg<apb3enr::APB3ENR_SPEC>,
    #[doc = "0xe8 - RCC APB1 Clock Register"]
    pub apb1lenr: crate::Reg<apb1lenr::APB1LENR_SPEC>,
    #[doc = "0xec - RCC APB1 Clock Register"]
    pub apb1henr: crate::Reg<apb1henr::APB1HENR_SPEC>,
    #[doc = "0xf0 - RCC APB2 Clock Register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    #[doc = "0xf4 - RCC APB4 Clock Register"]
    pub apb4enr: crate::Reg<apb4enr::APB4ENR_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0xfc - RCC AHB3 Sleep Clock Register"]
    pub ahb3lpenr: crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>,
    #[doc = "0x100 - RCC AHB1 Sleep Clock Register"]
    pub ahb1lpenr: crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>,
    #[doc = "0x104 - RCC AHB2 Sleep Clock Register"]
    pub ahb2lpenr: crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>,
    #[doc = "0x108 - RCC AHB4 Sleep Clock Register"]
    pub ahb4lpenr: crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>,
    #[doc = "0x10c - RCC APB3 Sleep Clock Register"]
    pub apb3lpenr: crate::Reg<apb3lpenr::APB3LPENR_SPEC>,
    #[doc = "0x110 - RCC APB1 Low Sleep Clock Register"]
    pub apb1llpenr: crate::Reg<apb1llpenr::APB1LLPENR_SPEC>,
    #[doc = "0x114 - RCC APB1 High Sleep Clock Register"]
    pub apb1hlpenr: crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>,
    #[doc = "0x118 - RCC APB2 Sleep Clock Register"]
    pub apb2lpenr: crate::Reg<apb2lpenr::APB2LPENR_SPEC>,
    #[doc = "0x11c - RCC APB4 Sleep Clock Register"]
    pub apb4lpenr: crate::Reg<apb4lpenr::APB4LPENR_SPEC>,
    _reserved55: [u8; 0x10],
    #[doc = "0x130 - RCC Reset Status Register"]
    pub c1_rsr: crate::Reg<c1_rsr::C1_RSR_SPEC>,
    #[doc = "0x134 - RCC AHB3 Clock Register"]
    pub c1_ahb3enr: crate::Reg<c1_ahb3enr::C1_AHB3ENR_SPEC>,
    #[doc = "0x138 - RCC AHB1 Clock Register"]
    pub c1_ahb1enr: crate::Reg<c1_ahb1enr::C1_AHB1ENR_SPEC>,
    #[doc = "0x13c - RCC AHB2 Clock Register"]
    pub c1_ahb2enr: crate::Reg<c1_ahb2enr::C1_AHB2ENR_SPEC>,
    #[doc = "0x140 - RCC AHB4 Clock Register"]
    pub c1_ahb4enr: crate::Reg<c1_ahb4enr::C1_AHB4ENR_SPEC>,
    #[doc = "0x144 - RCC APB3 Clock Register"]
    pub c1_apb3enr: crate::Reg<c1_apb3enr::C1_APB3ENR_SPEC>,
    #[doc = "0x148 - RCC APB1 Clock Register"]
    pub c1_apb1lenr: crate::Reg<c1_apb1lenr::C1_APB1LENR_SPEC>,
    #[doc = "0x14c - RCC APB1 Clock Register"]
    pub c1_apb1henr: crate::Reg<c1_apb1henr::C1_APB1HENR_SPEC>,
    #[doc = "0x150 - RCC APB2 Clock Register"]
    pub c1_apb2enr: crate::Reg<c1_apb2enr::C1_APB2ENR_SPEC>,
    #[doc = "0x154 - RCC APB4 Clock Register"]
    pub c1_apb4enr: crate::Reg<c1_apb4enr::C1_APB4ENR_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0x15c - RCC AHB3 Sleep Clock Register"]
    pub c1_ahb3lpenr: crate::Reg<c1_ahb3lpenr::C1_AHB3LPENR_SPEC>,
    #[doc = "0x160 - RCC AHB1 Sleep Clock Register"]
    pub c1_ahb1lpenr: crate::Reg<c1_ahb1lpenr::C1_AHB1LPENR_SPEC>,
    #[doc = "0x164 - RCC AHB2 Sleep Clock Register"]
    pub c1_ahb2lpenr: crate::Reg<c1_ahb2lpenr::C1_AHB2LPENR_SPEC>,
    #[doc = "0x168 - RCC AHB4 Sleep Clock Register"]
    pub c1_ahb4lpenr: crate::Reg<c1_ahb4lpenr::C1_AHB4LPENR_SPEC>,
    #[doc = "0x16c - RCC APB3 Sleep Clock Register"]
    pub c1_apb3lpenr: crate::Reg<c1_apb3lpenr::C1_APB3LPENR_SPEC>,
    #[doc = "0x170 - RCC APB1 Low Sleep Clock Register"]
    pub c1_apb1llpenr: crate::Reg<c1_apb1llpenr::C1_APB1LLPENR_SPEC>,
    #[doc = "0x174 - RCC APB1 High Sleep Clock Register"]
    pub c1_apb1hlpenr: crate::Reg<c1_apb1hlpenr::C1_APB1HLPENR_SPEC>,
    #[doc = "0x178 - RCC APB2 Sleep Clock Register"]
    pub c1_apb2lpenr: crate::Reg<c1_apb2lpenr::C1_APB2LPENR_SPEC>,
    #[doc = "0x17c - RCC APB4 Sleep Clock Register"]
    pub c1_apb4lpenr: crate::Reg<c1_apb4lpenr::C1_APB4LPENR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "CRRCR register accessor: an alias for `Reg<CRRCR_SPEC>`"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "RCC Clock Recovery RC Register"]
pub mod crrcr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "RCC Clock Configuration Register"]
pub mod cfgr;
#[doc = "D1CFGR register accessor: an alias for `Reg<D1CFGR_SPEC>`"]
pub type D1CFGR = crate::Reg<d1cfgr::D1CFGR_SPEC>;
#[doc = "RCC Domain 1 Clock Configuration Register"]
pub mod d1cfgr;
#[doc = "D2CFGR register accessor: an alias for `Reg<D2CFGR_SPEC>`"]
pub type D2CFGR = crate::Reg<d2cfgr::D2CFGR_SPEC>;
#[doc = "RCC Domain 2 Clock Configuration Register"]
pub mod d2cfgr;
#[doc = "D3CFGR register accessor: an alias for `Reg<D3CFGR_SPEC>`"]
pub type D3CFGR = crate::Reg<d3cfgr::D3CFGR_SPEC>;
#[doc = "RCC Domain 3 Clock Configuration Register"]
pub mod d3cfgr;
#[doc = "PLLCKSELR register accessor: an alias for `Reg<PLLCKSELR_SPEC>`"]
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELR_SPEC>;
#[doc = "RCC PLLs Clock Source Selection Register"]
pub mod pllckselr;
#[doc = "PLLCFGR register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "RCC PLLs Configuration Register"]
pub mod pllcfgr;
#[doc = "PLL1DIVR register accessor: an alias for `Reg<PLL1DIVR_SPEC>`"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
#[doc = "RCC PLL1 Dividers Configuration Register"]
pub mod pll1divr;
#[doc = "PLL1FRACR register accessor: an alias for `Reg<PLL1FRACR_SPEC>`"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
#[doc = "RCC PLL1 Fractional Divider Register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR register accessor: an alias for `Reg<PLL2DIVR_SPEC>`"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
#[doc = "RCC PLL2 Dividers Configuration Register"]
pub mod pll2divr;
#[doc = "PLL2FRACR register accessor: an alias for `Reg<PLL2FRACR_SPEC>`"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
#[doc = "RCC PLL2 Fractional Divider Register"]
pub mod pll2fracr;
#[doc = "PLL3DIVR register accessor: an alias for `Reg<PLL3DIVR_SPEC>`"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVR_SPEC>;
#[doc = "RCC PLL3 Dividers Configuration Register"]
pub mod pll3divr;
#[doc = "PLL3FRACR register accessor: an alias for `Reg<PLL3FRACR_SPEC>`"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACR_SPEC>;
#[doc = "RCC PLL3 Fractional Divider Register"]
pub mod pll3fracr;
#[doc = "D1CCIPR register accessor: an alias for `Reg<D1CCIPR_SPEC>`"]
pub type D1CCIPR = crate::Reg<d1ccipr::D1CCIPR_SPEC>;
#[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
pub mod d1ccipr;
#[doc = "D2CCIP1R register accessor: an alias for `Reg<D2CCIP1R_SPEC>`"]
pub type D2CCIP1R = crate::Reg<d2ccip1r::D2CCIP1R_SPEC>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip1r;
#[doc = "D2CCIP2R register accessor: an alias for `Reg<D2CCIP2R_SPEC>`"]
pub type D2CCIP2R = crate::Reg<d2ccip2r::D2CCIP2R_SPEC>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip2r;
#[doc = "D3CCIPR register accessor: an alias for `Reg<D3CCIPR_SPEC>`"]
pub type D3CCIPR = crate::Reg<d3ccipr::D3CCIPR_SPEC>;
#[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
pub mod d3ccipr;
#[doc = "CIER register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "RCC Clock Source Interrupt Enable Register"]
pub mod cier;
#[doc = "CIFR register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = "RCC Clock Source Interrupt Flag Register"]
pub mod cifr;
#[doc = "CICR register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = "RCC Clock Source Interrupt Clear Register"]
pub mod cicr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RCC Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "RCC Clock Control and Status Register"]
pub mod csr;
#[doc = "AHB3RSTR register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "RCC AHB3 Reset Register"]
pub mod ahb3rstr;
#[doc = "AHB1RSTR register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "RCC AHB1 Peripheral Reset Register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "RCC AHB2 Peripheral Reset Register"]
pub mod ahb2rstr;
#[doc = "AHB4RSTR register accessor: an alias for `Reg<AHB4RSTR_SPEC>`"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTR_SPEC>;
#[doc = "RCC AHB4 Peripheral Reset Register"]
pub mod ahb4rstr;
#[doc = "APB3RSTR register accessor: an alias for `Reg<APB3RSTR_SPEC>`"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = "RCC APB3 Peripheral Reset Register"]
pub mod apb3rstr;
#[doc = "APB1LRSTR register accessor: an alias for `Reg<APB1LRSTR_SPEC>`"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR register accessor: an alias for `Reg<APB1HRSTR_SPEC>`"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "RCC APB2 Peripheral Reset Register"]
pub mod apb2rstr;
#[doc = "APB4RSTR register accessor: an alias for `Reg<APB4RSTR_SPEC>`"]
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTR_SPEC>;
#[doc = "RCC APB4 Peripheral Reset Register"]
pub mod apb4rstr;
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "RCC Global Control Register"]
pub mod gcr;
#[doc = "D3AMR register accessor: an alias for `Reg<D3AMR_SPEC>`"]
pub type D3AMR = crate::Reg<d3amr::D3AMR_SPEC>;
#[doc = "RCC D3 Autonomous mode Register"]
pub mod d3amr;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "RCC Reset Status Register"]
pub mod rsr;
#[doc = "C1_RSR register accessor: an alias for `Reg<C1_RSR_SPEC>`"]
pub type C1_RSR = crate::Reg<c1_rsr::C1_RSR_SPEC>;
#[doc = "RCC Reset Status Register"]
pub mod c1_rsr;
#[doc = "C1_AHB3ENR register accessor: an alias for `Reg<C1_AHB3ENR_SPEC>`"]
pub type C1_AHB3ENR = crate::Reg<c1_ahb3enr::C1_AHB3ENR_SPEC>;
#[doc = "RCC AHB3 Clock Register"]
pub mod c1_ahb3enr;
#[doc = "AHB3ENR register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "RCC AHB3 Clock Register"]
pub mod ahb3enr;
#[doc = "AHB1ENR register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "RCC AHB1 Clock Register"]
pub mod ahb1enr;
#[doc = "C1_AHB1ENR register accessor: an alias for `Reg<C1_AHB1ENR_SPEC>`"]
pub type C1_AHB1ENR = crate::Reg<c1_ahb1enr::C1_AHB1ENR_SPEC>;
#[doc = "RCC AHB1 Clock Register"]
pub mod c1_ahb1enr;
#[doc = "C1_AHB2ENR register accessor: an alias for `Reg<C1_AHB2ENR_SPEC>`"]
pub type C1_AHB2ENR = crate::Reg<c1_ahb2enr::C1_AHB2ENR_SPEC>;
#[doc = "RCC AHB2 Clock Register"]
pub mod c1_ahb2enr;
#[doc = "AHB2ENR register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "RCC AHB2 Clock Register"]
pub mod ahb2enr;
#[doc = "AHB4ENR register accessor: an alias for `Reg<AHB4ENR_SPEC>`"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENR_SPEC>;
#[doc = "RCC AHB4 Clock Register"]
pub mod ahb4enr;
#[doc = "C1_AHB4ENR register accessor: an alias for `Reg<C1_AHB4ENR_SPEC>`"]
pub type C1_AHB4ENR = crate::Reg<c1_ahb4enr::C1_AHB4ENR_SPEC>;
#[doc = "RCC AHB4 Clock Register"]
pub mod c1_ahb4enr;
#[doc = "C1_APB3ENR register accessor: an alias for `Reg<C1_APB3ENR_SPEC>`"]
pub type C1_APB3ENR = crate::Reg<c1_apb3enr::C1_APB3ENR_SPEC>;
#[doc = "RCC APB3 Clock Register"]
pub mod c1_apb3enr;
#[doc = "APB3ENR register accessor: an alias for `Reg<APB3ENR_SPEC>`"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = "RCC APB3 Clock Register"]
pub mod apb3enr;
#[doc = "APB1LENR register accessor: an alias for `Reg<APB1LENR_SPEC>`"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1lenr;
#[doc = "C1_APB1LENR register accessor: an alias for `Reg<C1_APB1LENR_SPEC>`"]
pub type C1_APB1LENR = crate::Reg<c1_apb1lenr::C1_APB1LENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1lenr;
#[doc = "APB1HENR register accessor: an alias for `Reg<APB1HENR_SPEC>`"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1henr;
#[doc = "C1_APB1HENR register accessor: an alias for `Reg<C1_APB1HENR_SPEC>`"]
pub type C1_APB1HENR = crate::Reg<c1_apb1henr::C1_APB1HENR_SPEC>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1henr;
#[doc = "C1_APB2ENR register accessor: an alias for `Reg<C1_APB2ENR_SPEC>`"]
pub type C1_APB2ENR = crate::Reg<c1_apb2enr::C1_APB2ENR_SPEC>;
#[doc = "RCC APB2 Clock Register"]
pub mod c1_apb2enr;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "RCC APB2 Clock Register"]
pub mod apb2enr;
#[doc = "APB4ENR register accessor: an alias for `Reg<APB4ENR_SPEC>`"]
pub type APB4ENR = crate::Reg<apb4enr::APB4ENR_SPEC>;
#[doc = "RCC APB4 Clock Register"]
pub mod apb4enr;
#[doc = "C1_APB4ENR register accessor: an alias for `Reg<C1_APB4ENR_SPEC>`"]
pub type C1_APB4ENR = crate::Reg<c1_apb4enr::C1_APB4ENR_SPEC>;
#[doc = "RCC APB4 Clock Register"]
pub mod c1_apb4enr;
#[doc = "C1_AHB3LPENR register accessor: an alias for `Reg<C1_AHB3LPENR_SPEC>`"]
pub type C1_AHB3LPENR = crate::Reg<c1_ahb3lpenr::C1_AHB3LPENR_SPEC>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod c1_ahb3lpenr;
#[doc = "AHB3LPENR register accessor: an alias for `Reg<AHB3LPENR_SPEC>`"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR register accessor: an alias for `Reg<AHB1LPENR_SPEC>`"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod ahb1lpenr;
#[doc = "C1_AHB1LPENR register accessor: an alias for `Reg<C1_AHB1LPENR_SPEC>`"]
pub type C1_AHB1LPENR = crate::Reg<c1_ahb1lpenr::C1_AHB1LPENR_SPEC>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod c1_ahb1lpenr;
#[doc = "C1_AHB2LPENR register accessor: an alias for `Reg<C1_AHB2LPENR_SPEC>`"]
pub type C1_AHB2LPENR = crate::Reg<c1_ahb2lpenr::C1_AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod c1_ahb2lpenr;
#[doc = "AHB2LPENR register accessor: an alias for `Reg<AHB2LPENR_SPEC>`"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR register accessor: an alias for `Reg<AHB4LPENR_SPEC>`"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod ahb4lpenr;
#[doc = "C1_AHB4LPENR register accessor: an alias for `Reg<C1_AHB4LPENR_SPEC>`"]
pub type C1_AHB4LPENR = crate::Reg<c1_ahb4lpenr::C1_AHB4LPENR_SPEC>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod c1_ahb4lpenr;
#[doc = "C1_APB3LPENR register accessor: an alias for `Reg<C1_APB3LPENR_SPEC>`"]
pub type C1_APB3LPENR = crate::Reg<c1_apb3lpenr::C1_APB3LPENR_SPEC>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod c1_apb3lpenr;
#[doc = "APB3LPENR register accessor: an alias for `Reg<APB3LPENR_SPEC>`"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod apb3lpenr;
#[doc = "APB1LLPENR register accessor: an alias for `Reg<APB1LLPENR_SPEC>`"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod apb1llpenr;
#[doc = "C1_APB1LLPENR register accessor: an alias for `Reg<C1_APB1LLPENR_SPEC>`"]
pub type C1_APB1LLPENR = crate::Reg<c1_apb1llpenr::C1_APB1LLPENR_SPEC>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod c1_apb1llpenr;
#[doc = "C1_APB1HLPENR register accessor: an alias for `Reg<C1_APB1HLPENR_SPEC>`"]
pub type C1_APB1HLPENR = crate::Reg<c1_apb1hlpenr::C1_APB1HLPENR_SPEC>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod c1_apb1hlpenr;
#[doc = "APB1HLPENR register accessor: an alias for `Reg<APB1HLPENR_SPEC>`"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod apb2lpenr;
#[doc = "C1_APB2LPENR register accessor: an alias for `Reg<C1_APB2LPENR_SPEC>`"]
pub type C1_APB2LPENR = crate::Reg<c1_apb2lpenr::C1_APB2LPENR_SPEC>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod c1_apb2lpenr;
#[doc = "C1_APB4LPENR register accessor: an alias for `Reg<C1_APB4LPENR_SPEC>`"]
pub type C1_APB4LPENR = crate::Reg<c1_apb4lpenr::C1_APB4LPENR_SPEC>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod c1_apb4lpenr;
#[doc = "APB4LPENR register accessor: an alias for `Reg<APB4LPENR_SPEC>`"]
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENR_SPEC>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod apb4lpenr;
#[doc = "HSICFGR register accessor: an alias for `Reg<HSICFGR_SPEC>`"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
#[doc = "RCC HSI configuration register"]
pub mod hsicfgr;
#[doc = "CSICFGR register accessor: an alias for `Reg<CSICFGR_SPEC>`"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
#[doc = "RCC CSI configuration register"]
pub mod csicfgr;
