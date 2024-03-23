#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcc_cr: RCC_CR,
    _reserved1: [u8; 0x04],
    rcc_icscr1: RCC_ICSCR1,
    rcc_icscr2: RCC_ICSCR2,
    rcc_icscr3: RCC_ICSCR3,
    rcc_crrcr: RCC_CRRCR,
    _reserved5: [u8; 0x04],
    rcc_cfgr1: RCC_CFGR1,
    rcc_cfgr2: RCC_CFGR2,
    rcc_cfgr3: RCC_CFGR3,
    rcc_pll1cfgr: RCC_PLL1CFGR,
    rcc_pll2cfgr: RCC_PLL2CFGR,
    rcc_pll3cfgr: RCC_PLL3CFGR,
    rcc_pll1divr: RCC_PLL1DIVR,
    rcc_pll1fracr: RCC_PLL1FRACR,
    rcc_pll2divr: RCC_PLL2DIVR,
    rcc_pll2fracr: RCC_PLL2FRACR,
    rcc_pll3divr: RCC_PLL3DIVR,
    rcc_pll3fracr: RCC_PLL3FRACR,
    _reserved17: [u8; 0x04],
    rcc_cier: RCC_CIER,
    rcc_cifr: RCC_CIFR,
    rcc_cicr: RCC_CICR,
    _reserved20: [u8; 0x04],
    rcc_ahb1rstr: RCC_AHB1RSTR,
    rcc_ahb2rstr1: RCC_AHB2RSTR1,
    rcc_ahb2rstr2: RCC_AHB2RSTR2,
    rcc_ahb3rstr: RCC_AHB3RSTR,
    _reserved24: [u8; 0x04],
    rcc_apb1rstr1: RCC_APB1RSTR1,
    rcc_apb1rstr2: RCC_APB1RSTR2,
    rcc_apb2rstr: RCC_APB2RSTR,
    rcc_apb3rstr: RCC_APB3RSTR,
    _reserved28: [u8; 0x04],
    rcc_ahb1enr: RCC_AHB1ENR,
    rcc_ahb2enr1: RCC_AHB2ENR1,
    rcc_ahb2enr2: RCC_AHB2ENR2,
    rcc_ahb3enr: RCC_AHB3ENR,
    _reserved32: [u8; 0x04],
    rcc_apb1enr1: RCC_APB1ENR1,
    rcc_apb1enr2: RCC_APB1ENR2,
    rcc_apb2enr: RCC_APB2ENR,
    rcc_apb3enr: RCC_APB3ENR,
    _reserved36: [u8; 0x04],
    rcc_ahb1smenr: RCC_AHB1SMENR,
    rcc_ahb2smenr1: RCC_AHB2SMENR1,
    rcc_ahb2smenr2: RCC_AHB2SMENR2,
    rcc_ahb3smenr: RCC_AHB3SMENR,
    _reserved40: [u8; 0x04],
    rcc_apb1smenr1: RCC_APB1SMENR1,
    rcc_apb1smenr2: RCC_APB1SMENR2,
    rcc_apb2smenr: RCC_APB2SMENR,
    rcc_apb3smenr: RCC_APB3SMENR,
    _reserved44: [u8; 0x04],
    rcc_srdamr: RCC_SRDAMR,
    _reserved45: [u8; 0x04],
    rcc_ccipr1: RCC_CCIPR1,
    rcc_ccipr2: RCC_CCIPR2,
    rcc_ccipr3: RCC_CCIPR3,
    _reserved48: [u8; 0x04],
    rcc_bdcr: RCC_BDCR,
    rcc_csr: RCC_CSR,
    _reserved50: [u8; 0x18],
    rcc_seccfgr: RCC_SECCFGR,
    rcc_privcfgr: RCC_PRIVCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - RCC clock control register"]
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RCC_CR {
        &self.rcc_cr
    }
    #[doc = "0x08 - RCC internal clock sources calibration register 1"]
    #[inline(always)]
    pub const fn rcc_icscr1(&self) -> &RCC_ICSCR1 {
        &self.rcc_icscr1
    }
    #[doc = "0x0c - RCC internal clock sources calibration register 2"]
    #[inline(always)]
    pub const fn rcc_icscr2(&self) -> &RCC_ICSCR2 {
        &self.rcc_icscr2
    }
    #[doc = "0x10 - RCC internal clock sources calibration register 3"]
    #[inline(always)]
    pub const fn rcc_icscr3(&self) -> &RCC_ICSCR3 {
        &self.rcc_icscr3
    }
    #[doc = "0x14 - RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RCC_CRRCR {
        &self.rcc_crrcr
    }
    #[doc = "0x1c - RCC clock configuration register 1"]
    #[inline(always)]
    pub const fn rcc_cfgr1(&self) -> &RCC_CFGR1 {
        &self.rcc_cfgr1
    }
    #[doc = "0x20 - RCC clock configuration register 2"]
    #[inline(always)]
    pub const fn rcc_cfgr2(&self) -> &RCC_CFGR2 {
        &self.rcc_cfgr2
    }
    #[doc = "0x24 - RCC clock configuration register 3"]
    #[inline(always)]
    pub const fn rcc_cfgr3(&self) -> &RCC_CFGR3 {
        &self.rcc_cfgr3
    }
    #[doc = "0x28 - RCC PLL1 configuration register"]
    #[inline(always)]
    pub const fn rcc_pll1cfgr(&self) -> &RCC_PLL1CFGR {
        &self.rcc_pll1cfgr
    }
    #[doc = "0x2c - RCC PLL2 configuration register"]
    #[inline(always)]
    pub const fn rcc_pll2cfgr(&self) -> &RCC_PLL2CFGR {
        &self.rcc_pll2cfgr
    }
    #[doc = "0x30 - RCC PLL3 configuration register"]
    #[inline(always)]
    pub const fn rcc_pll3cfgr(&self) -> &RCC_PLL3CFGR {
        &self.rcc_pll3cfgr
    }
    #[doc = "0x34 - RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn rcc_pll1divr(&self) -> &RCC_PLL1DIVR {
        &self.rcc_pll1divr
    }
    #[doc = "0x38 - RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn rcc_pll1fracr(&self) -> &RCC_PLL1FRACR {
        &self.rcc_pll1fracr
    }
    #[doc = "0x3c - RCC PLL2 dividers configuration register"]
    #[inline(always)]
    pub const fn rcc_pll2divr(&self) -> &RCC_PLL2DIVR {
        &self.rcc_pll2divr
    }
    #[doc = "0x40 - RCC PLL2 fractional divider register"]
    #[inline(always)]
    pub const fn rcc_pll2fracr(&self) -> &RCC_PLL2FRACR {
        &self.rcc_pll2fracr
    }
    #[doc = "0x44 - RCC PLL3 dividers configuration register"]
    #[inline(always)]
    pub const fn rcc_pll3divr(&self) -> &RCC_PLL3DIVR {
        &self.rcc_pll3divr
    }
    #[doc = "0x48 - RCC PLL3 fractional divider register"]
    #[inline(always)]
    pub const fn rcc_pll3fracr(&self) -> &RCC_PLL3FRACR {
        &self.rcc_pll3fracr
    }
    #[doc = "0x50 - RCC clock interrupt enable register"]
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RCC_CIER {
        &self.rcc_cier
    }
    #[doc = "0x54 - RCC clock interrupt flag register"]
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RCC_CIFR {
        &self.rcc_cifr
    }
    #[doc = "0x58 - RCC clock interrupt clear register"]
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RCC_CICR {
        &self.rcc_cicr
    }
    #[doc = "0x60 - RCC AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahb1rstr(&self) -> &RCC_AHB1RSTR {
        &self.rcc_ahb1rstr
    }
    #[doc = "0x64 - RCC AHB2 peripheral reset register 1"]
    #[inline(always)]
    pub const fn rcc_ahb2rstr1(&self) -> &RCC_AHB2RSTR1 {
        &self.rcc_ahb2rstr1
    }
    #[doc = "0x68 - RCC AHB2 peripheral reset register 2"]
    #[inline(always)]
    pub const fn rcc_ahb2rstr2(&self) -> &RCC_AHB2RSTR2 {
        &self.rcc_ahb2rstr2
    }
    #[doc = "0x6c - RCC AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahb3rstr(&self) -> &RCC_AHB3RSTR {
        &self.rcc_ahb3rstr
    }
    #[doc = "0x74 - RCC APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn rcc_apb1rstr1(&self) -> &RCC_APB1RSTR1 {
        &self.rcc_apb1rstr1
    }
    #[doc = "0x78 - RCC APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn rcc_apb1rstr2(&self) -> &RCC_APB1RSTR2 {
        &self.rcc_apb1rstr2
    }
    #[doc = "0x7c - RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_apb2rstr(&self) -> &RCC_APB2RSTR {
        &self.rcc_apb2rstr
    }
    #[doc = "0x80 - RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_apb3rstr(&self) -> &RCC_APB3RSTR {
        &self.rcc_apb3rstr
    }
    #[doc = "0x88 - RCC AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahb1enr(&self) -> &RCC_AHB1ENR {
        &self.rcc_ahb1enr
    }
    #[doc = "0x8c - RCC AHB2 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn rcc_ahb2enr1(&self) -> &RCC_AHB2ENR1 {
        &self.rcc_ahb2enr1
    }
    #[doc = "0x90 - RCC AHB2 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn rcc_ahb2enr2(&self) -> &RCC_AHB2ENR2 {
        &self.rcc_ahb2enr2
    }
    #[doc = "0x94 - RCC AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahb3enr(&self) -> &RCC_AHB3ENR {
        &self.rcc_ahb3enr
    }
    #[doc = "0x9c - RCC APB1 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn rcc_apb1enr1(&self) -> &RCC_APB1ENR1 {
        &self.rcc_apb1enr1
    }
    #[doc = "0xa0 - RCC APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn rcc_apb1enr2(&self) -> &RCC_APB1ENR2 {
        &self.rcc_apb1enr2
    }
    #[doc = "0xa4 - RCC APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_apb2enr(&self) -> &RCC_APB2ENR {
        &self.rcc_apb2enr
    }
    #[doc = "0xa8 - RCC APB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_apb3enr(&self) -> &RCC_APB3ENR {
        &self.rcc_apb3enr
    }
    #[doc = "0xb0 - RCC AHB1 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_ahb1smenr(&self) -> &RCC_AHB1SMENR {
        &self.rcc_ahb1smenr
    }
    #[doc = "0xb4 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn rcc_ahb2smenr1(&self) -> &RCC_AHB2SMENR1 {
        &self.rcc_ahb2smenr1
    }
    #[doc = "0xb8 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn rcc_ahb2smenr2(&self) -> &RCC_AHB2SMENR2 {
        &self.rcc_ahb2smenr2
    }
    #[doc = "0xbc - RCC AHB3 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_ahb3smenr(&self) -> &RCC_AHB3SMENR {
        &self.rcc_ahb3smenr
    }
    #[doc = "0xc4 - RCC APB1 peripheral clock enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn rcc_apb1smenr1(&self) -> &RCC_APB1SMENR1 {
        &self.rcc_apb1smenr1
    }
    #[doc = "0xc8 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn rcc_apb1smenr2(&self) -> &RCC_APB1SMENR2 {
        &self.rcc_apb1smenr2
    }
    #[doc = "0xcc - RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_apb2smenr(&self) -> &RCC_APB2SMENR {
        &self.rcc_apb2smenr
    }
    #[doc = "0xd0 - RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_apb3smenr(&self) -> &RCC_APB3SMENR {
        &self.rcc_apb3smenr
    }
    #[doc = "0xd8 - RCC SmartRun domain peripheral autonomous mode register"]
    #[inline(always)]
    pub const fn rcc_srdamr(&self) -> &RCC_SRDAMR {
        &self.rcc_srdamr
    }
    #[doc = "0xe0 - RCC peripherals independent clock configuration register 1"]
    #[inline(always)]
    pub const fn rcc_ccipr1(&self) -> &RCC_CCIPR1 {
        &self.rcc_ccipr1
    }
    #[doc = "0xe4 - RCC peripherals independent clock configuration register 2"]
    #[inline(always)]
    pub const fn rcc_ccipr2(&self) -> &RCC_CCIPR2 {
        &self.rcc_ccipr2
    }
    #[doc = "0xe8 - RCC peripherals independent clock configuration register 3"]
    #[inline(always)]
    pub const fn rcc_ccipr3(&self) -> &RCC_CCIPR3 {
        &self.rcc_ccipr3
    }
    #[doc = "0xf0 - RCC backup domain control register"]
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RCC_BDCR {
        &self.rcc_bdcr
    }
    #[doc = "0xf4 - RCC control/status register"]
    #[inline(always)]
    pub const fn rcc_csr(&self) -> &RCC_CSR {
        &self.rcc_csr
    }
    #[doc = "0x110 - RCC secure configuration register"]
    #[inline(always)]
    pub const fn rcc_seccfgr(&self) -> &RCC_SECCFGR {
        &self.rcc_seccfgr
    }
    #[doc = "0x114 - RCC privilege configuration register"]
    #[inline(always)]
    pub const fn rcc_privcfgr(&self) -> &RCC_PRIVCFGR {
        &self.rcc_privcfgr
    }
}
#[doc = "RCC_CR (rw) register accessor: RCC clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cr`]
module"]
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CRrs>;
#[doc = "RCC clock control register"]
pub mod rcc_cr;
#[doc = "RCC_ICSCR1 (rw) register accessor: RCC internal clock sources calibration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_icscr1`]
module"]
pub type RCC_ICSCR1 = crate::Reg<rcc_icscr1::RCC_ICSCR1rs>;
#[doc = "RCC internal clock sources calibration register 1"]
pub mod rcc_icscr1;
#[doc = "RCC_ICSCR2 (rw) register accessor: RCC internal clock sources calibration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_icscr2`]
module"]
pub type RCC_ICSCR2 = crate::Reg<rcc_icscr2::RCC_ICSCR2rs>;
#[doc = "RCC internal clock sources calibration register 2"]
pub mod rcc_icscr2;
#[doc = "RCC_ICSCR3 (rw) register accessor: RCC internal clock sources calibration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_icscr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_icscr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_icscr3`]
module"]
pub type RCC_ICSCR3 = crate::Reg<rcc_icscr3::RCC_ICSCR3rs>;
#[doc = "RCC internal clock sources calibration register 3"]
pub mod rcc_icscr3;
#[doc = "RCC_CRRCR (r) register accessor: RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_crrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_crrcr`]
module"]
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCRrs>;
#[doc = "RCC clock recovery RC register"]
pub mod rcc_crrcr;
#[doc = "RCC_CFGR1 (rw) register accessor: RCC clock configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cfgr1`]
module"]
pub type RCC_CFGR1 = crate::Reg<rcc_cfgr1::RCC_CFGR1rs>;
#[doc = "RCC clock configuration register 1"]
pub mod rcc_cfgr1;
#[doc = "RCC_CFGR2 (rw) register accessor: RCC clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cfgr2`]
module"]
pub type RCC_CFGR2 = crate::Reg<rcc_cfgr2::RCC_CFGR2rs>;
#[doc = "RCC clock configuration register 2"]
pub mod rcc_cfgr2;
#[doc = "RCC_CFGR3 (rw) register accessor: RCC clock configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cfgr3`]
module"]
pub type RCC_CFGR3 = crate::Reg<rcc_cfgr3::RCC_CFGR3rs>;
#[doc = "RCC clock configuration register 3"]
pub mod rcc_cfgr3;
#[doc = "RCC_PLL1CFGR (rw) register accessor: RCC PLL1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1cfgr`]
module"]
pub type RCC_PLL1CFGR = crate::Reg<rcc_pll1cfgr::RCC_PLL1CFGRrs>;
#[doc = "RCC PLL1 configuration register"]
pub mod rcc_pll1cfgr;
#[doc = "RCC_PLL2CFGR (rw) register accessor: RCC PLL2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2cfgr`]
module"]
pub type RCC_PLL2CFGR = crate::Reg<rcc_pll2cfgr::RCC_PLL2CFGRrs>;
#[doc = "RCC PLL2 configuration register"]
pub mod rcc_pll2cfgr;
#[doc = "RCC_PLL3CFGR (rw) register accessor: RCC PLL3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3cfgr`]
module"]
pub type RCC_PLL3CFGR = crate::Reg<rcc_pll3cfgr::RCC_PLL3CFGRrs>;
#[doc = "RCC PLL3 configuration register"]
pub mod rcc_pll3cfgr;
#[doc = "RCC_PLL1DIVR (rw) register accessor: RCC PLL1 dividers register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1divr`]
module"]
pub type RCC_PLL1DIVR = crate::Reg<rcc_pll1divr::RCC_PLL1DIVRrs>;
#[doc = "RCC PLL1 dividers register"]
pub mod rcc_pll1divr;
#[doc = "RCC_PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1fracr`]
module"]
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACRrs>;
#[doc = "RCC PLL1 fractional divider register"]
pub mod rcc_pll1fracr;
#[doc = "RCC_PLL2DIVR (rw) register accessor: RCC PLL2 dividers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2divr`]
module"]
pub type RCC_PLL2DIVR = crate::Reg<rcc_pll2divr::RCC_PLL2DIVRrs>;
#[doc = "RCC PLL2 dividers configuration register"]
pub mod rcc_pll2divr;
#[doc = "RCC_PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2fracr`]
module"]
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACRrs>;
#[doc = "RCC PLL2 fractional divider register"]
pub mod rcc_pll2fracr;
#[doc = "RCC_PLL3DIVR (rw) register accessor: RCC PLL3 dividers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3divr`]
module"]
pub type RCC_PLL3DIVR = crate::Reg<rcc_pll3divr::RCC_PLL3DIVRrs>;
#[doc = "RCC PLL3 dividers configuration register"]
pub mod rcc_pll3divr;
#[doc = "RCC_PLL3FRACR (rw) register accessor: RCC PLL3 fractional divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3fracr`]
module"]
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACRrs>;
#[doc = "RCC PLL3 fractional divider register"]
pub mod rcc_pll3fracr;
#[doc = "RCC_CIER (rw) register accessor: RCC clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cier`]
module"]
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIERrs>;
#[doc = "RCC clock interrupt enable register"]
pub mod rcc_cier;
#[doc = "RCC_CIFR (r) register accessor: RCC clock interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cifr`]
module"]
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFRrs>;
#[doc = "RCC clock interrupt flag register"]
pub mod rcc_cifr;
#[doc = "RCC_CICR (w) register accessor: RCC clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cicr`]
module"]
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICRrs>;
#[doc = "RCC clock interrupt clear register"]
pub mod rcc_cicr;
#[doc = "RCC_AHB1RSTR (rw) register accessor: RCC AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1rstr`]
module"]
pub type RCC_AHB1RSTR = crate::Reg<rcc_ahb1rstr::RCC_AHB1RSTRrs>;
#[doc = "RCC AHB1 peripheral reset register"]
pub mod rcc_ahb1rstr;
#[doc = "RCC_AHB2RSTR1 (rw) register accessor: RCC AHB2 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2rstr1`]
module"]
pub type RCC_AHB2RSTR1 = crate::Reg<rcc_ahb2rstr1::RCC_AHB2RSTR1rs>;
#[doc = "RCC AHB2 peripheral reset register 1"]
pub mod rcc_ahb2rstr1;
#[doc = "RCC_AHB2RSTR2 (rw) register accessor: RCC AHB2 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2rstr2`]
module"]
pub type RCC_AHB2RSTR2 = crate::Reg<rcc_ahb2rstr2::RCC_AHB2RSTR2rs>;
#[doc = "RCC AHB2 peripheral reset register 2"]
pub mod rcc_ahb2rstr2;
#[doc = "RCC_AHB3RSTR (rw) register accessor: RCC AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3rstr`]
module"]
pub type RCC_AHB3RSTR = crate::Reg<rcc_ahb3rstr::RCC_AHB3RSTRrs>;
#[doc = "RCC AHB3 peripheral reset register"]
pub mod rcc_ahb3rstr;
#[doc = "RCC_APB1RSTR1 (rw) register accessor: RCC APB1 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1rstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1rstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstr1`]
module"]
pub type RCC_APB1RSTR1 = crate::Reg<rcc_apb1rstr1::RCC_APB1RSTR1rs>;
#[doc = "RCC APB1 peripheral reset register 1"]
pub mod rcc_apb1rstr1;
#[doc = "RCC_APB1RSTR2 (rw) register accessor: RCC APB1 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1rstr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1rstr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstr2`]
module"]
pub type RCC_APB1RSTR2 = crate::Reg<rcc_apb1rstr2::RCC_APB1RSTR2rs>;
#[doc = "RCC APB1 peripheral reset register 2"]
pub mod rcc_apb1rstr2;
#[doc = "RCC_APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2rstr`]
module"]
pub type RCC_APB2RSTR = crate::Reg<rcc_apb2rstr::RCC_APB2RSTRrs>;
#[doc = "RCC APB2 peripheral reset register"]
pub mod rcc_apb2rstr;
#[doc = "RCC_APB3RSTR (rw) register accessor: RCC APB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3rstr`]
module"]
pub type RCC_APB3RSTR = crate::Reg<rcc_apb3rstr::RCC_APB3RSTRrs>;
#[doc = "RCC APB3 peripheral reset register"]
pub mod rcc_apb3rstr;
#[doc = "RCC_AHB1ENR (rw) register accessor: RCC AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1enr`]
module"]
pub type RCC_AHB1ENR = crate::Reg<rcc_ahb1enr::RCC_AHB1ENRrs>;
#[doc = "RCC AHB1 peripheral clock enable register"]
pub mod rcc_ahb1enr;
#[doc = "RCC_AHB2ENR1 (rw) register accessor: RCC AHB2 peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2enr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2enr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2enr1`]
module"]
pub type RCC_AHB2ENR1 = crate::Reg<rcc_ahb2enr1::RCC_AHB2ENR1rs>;
#[doc = "RCC AHB2 peripheral clock enable register 1"]
pub mod rcc_ahb2enr1;
#[doc = "RCC_AHB2ENR2 (rw) register accessor: RCC AHB2 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2enr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2enr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2enr2`]
module"]
pub type RCC_AHB2ENR2 = crate::Reg<rcc_ahb2enr2::RCC_AHB2ENR2rs>;
#[doc = "RCC AHB2 peripheral clock enable register 2"]
pub mod rcc_ahb2enr2;
#[doc = "RCC_AHB3ENR (rw) register accessor: RCC AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3enr`]
module"]
pub type RCC_AHB3ENR = crate::Reg<rcc_ahb3enr::RCC_AHB3ENRrs>;
#[doc = "RCC AHB3 peripheral clock enable register"]
pub mod rcc_ahb3enr;
#[doc = "RCC_APB1ENR1 (rw) register accessor: RCC APB1 peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1enr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1enr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1enr1`]
module"]
pub type RCC_APB1ENR1 = crate::Reg<rcc_apb1enr1::RCC_APB1ENR1rs>;
#[doc = "RCC APB1 peripheral clock enable register 1"]
pub mod rcc_apb1enr1;
#[doc = "RCC_APB1ENR2 (rw) register accessor: RCC APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1enr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1enr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1enr2`]
module"]
pub type RCC_APB1ENR2 = crate::Reg<rcc_apb1enr2::RCC_APB1ENR2rs>;
#[doc = "RCC APB1 peripheral clock enable register 2"]
pub mod rcc_apb1enr2;
#[doc = "RCC_APB2ENR (rw) register accessor: RCC APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2enr`]
module"]
pub type RCC_APB2ENR = crate::Reg<rcc_apb2enr::RCC_APB2ENRrs>;
#[doc = "RCC APB2 peripheral clock enable register"]
pub mod rcc_apb2enr;
#[doc = "RCC_APB3ENR (rw) register accessor: RCC APB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3enr`]
module"]
pub type RCC_APB3ENR = crate::Reg<rcc_apb3enr::RCC_APB3ENRrs>;
#[doc = "RCC APB3 peripheral clock enable register"]
pub mod rcc_apb3enr;
#[doc = "RCC_AHB1SMENR (rw) register accessor: RCC AHB1 peripheral clock enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1smenr`]
module"]
pub type RCC_AHB1SMENR = crate::Reg<rcc_ahb1smenr::RCC_AHB1SMENRrs>;
#[doc = "RCC AHB1 peripheral clock enable in Sleep and Stop modes register"]
pub mod rcc_ahb1smenr;
#[doc = "RCC_AHB2SMENR1 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2smenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2smenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2smenr1`]
module"]
pub type RCC_AHB2SMENR1 = crate::Reg<rcc_ahb2smenr1::RCC_AHB2SMENR1rs>;
#[doc = "RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1"]
pub mod rcc_ahb2smenr1;
#[doc = "RCC_AHB2SMENR2 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2smenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2smenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2smenr2`]
module"]
pub type RCC_AHB2SMENR2 = crate::Reg<rcc_ahb2smenr2::RCC_AHB2SMENR2rs>;
#[doc = "RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2"]
pub mod rcc_ahb2smenr2;
#[doc = "RCC_AHB3SMENR (rw) register accessor: RCC AHB3 peripheral clock enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3smenr`]
module"]
pub type RCC_AHB3SMENR = crate::Reg<rcc_ahb3smenr::RCC_AHB3SMENRrs>;
#[doc = "RCC AHB3 peripheral clock enable in Sleep and Stop modes register"]
pub mod rcc_ahb3smenr;
#[doc = "RCC_APB1SMENR1 (rw) register accessor: RCC APB1 peripheral clock enable in Sleep and Stop modes register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1smenr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1smenr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1smenr1`]
module"]
pub type RCC_APB1SMENR1 = crate::Reg<rcc_apb1smenr1::RCC_APB1SMENR1rs>;
#[doc = "RCC APB1 peripheral clock enable in Sleep and Stop modes register 1"]
pub mod rcc_apb1smenr1;
#[doc = "RCC_APB1SMENR2 (rw) register accessor: RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1smenr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1smenr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1smenr2`]
module"]
pub type RCC_APB1SMENR2 = crate::Reg<rcc_apb1smenr2::RCC_APB1SMENR2rs>;
#[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod rcc_apb1smenr2;
#[doc = "RCC_APB2SMENR (rw) register accessor: RCC APB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2smenr`]
module"]
pub type RCC_APB2SMENR = crate::Reg<rcc_apb2smenr::RCC_APB2SMENRrs>;
#[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_apb2smenr;
#[doc = "RCC_APB3SMENR (rw) register accessor: RCC APB3 peripheral clock enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3smenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3smenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3smenr`]
module"]
pub type RCC_APB3SMENR = crate::Reg<rcc_apb3smenr::RCC_APB3SMENRrs>;
#[doc = "RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
pub mod rcc_apb3smenr;
#[doc = "RCC_SRDAMR (rw) register accessor: RCC SmartRun domain peripheral autonomous mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_srdamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_srdamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_srdamr`]
module"]
pub type RCC_SRDAMR = crate::Reg<rcc_srdamr::RCC_SRDAMRrs>;
#[doc = "RCC SmartRun domain peripheral autonomous mode register"]
pub mod rcc_srdamr;
#[doc = "RCC_CCIPR1 (rw) register accessor: RCC peripherals independent clock configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ccipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ccipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr1`]
module"]
pub type RCC_CCIPR1 = crate::Reg<rcc_ccipr1::RCC_CCIPR1rs>;
#[doc = "RCC peripherals independent clock configuration register 1"]
pub mod rcc_ccipr1;
#[doc = "RCC_CCIPR2 (rw) register accessor: RCC peripherals independent clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ccipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ccipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr2`]
module"]
pub type RCC_CCIPR2 = crate::Reg<rcc_ccipr2::RCC_CCIPR2rs>;
#[doc = "RCC peripherals independent clock configuration register 2"]
pub mod rcc_ccipr2;
#[doc = "RCC_CCIPR3 (rw) register accessor: RCC peripherals independent clock configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ccipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ccipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr3`]
module"]
pub type RCC_CCIPR3 = crate::Reg<rcc_ccipr3::RCC_CCIPR3rs>;
#[doc = "RCC peripherals independent clock configuration register 3"]
pub mod rcc_ccipr3;
#[doc = "RCC_BDCR (rw) register accessor: RCC backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_bdcr`]
module"]
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCRrs>;
#[doc = "RCC backup domain control register"]
pub mod rcc_bdcr;
#[doc = "RCC_CSR (rw) register accessor: RCC control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_csr`]
module"]
pub type RCC_CSR = crate::Reg<rcc_csr::RCC_CSRrs>;
#[doc = "RCC control/status register"]
pub mod rcc_csr;
#[doc = "RCC_SECCFGR (rw) register accessor: RCC secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_seccfgr`]
module"]
pub type RCC_SECCFGR = crate::Reg<rcc_seccfgr::RCC_SECCFGRrs>;
#[doc = "RCC secure configuration register"]
pub mod rcc_seccfgr;
#[doc = "RCC_PRIVCFGR (rw) register accessor: RCC privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_privcfgr`]
module"]
pub type RCC_PRIVCFGR = crate::Reg<rcc_privcfgr::RCC_PRIVCFGRrs>;
#[doc = "RCC privilege configuration register"]
pub mod rcc_privcfgr;
