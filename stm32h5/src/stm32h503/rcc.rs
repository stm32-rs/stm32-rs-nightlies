#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    hsicfgr: HSICFGR,
    crrcr: CRRCR,
    csicfgr: CSICFGR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    _reserved6: [u8; 0x04],
    pll1cfgr: PLL1CFGR,
    pll2cfgr: PLL2CFGR,
    _reserved8: [u8; 0x04],
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    pll2divr: PLL2DIVR,
    pll2fracr: PLL2FRACR,
    _reserved12: [u8; 0x0c],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved15: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    _reserved17: [u8; 0x0c],
    apb1lrstr: APB1LRSTR,
    apb1hrstr: APB1HRSTR,
    apb2rstr: APB2RSTR,
    apb3rstr: APB3RSTR,
    _reserved21: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    _reserved23: [u8; 0x0c],
    apb1lenr: APB1LENR,
    apb1henr: APB1HENR,
    apb2enr: APB2ENR,
    apb3enr: APB3ENR,
    _reserved27: [u8; 0x04],
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    _reserved29: [u8; 0x0c],
    apb1llpenr: APB1LLPENR,
    apb1hlpenr: APB1HLPENR,
    apb2lpenr: APB2LPENR,
    apb3lpenr: APB3LPENR,
    _reserved33: [u8; 0x04],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    ccipr4: CCIPR4,
    ccipr5: CCIPR5,
    _reserved38: [u8; 0x04],
    bdcr: BDCR,
    rsr: RSR,
    _reserved40: [u8; 0x1c],
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    #[doc = "0x00 - RCC clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - RCC HSI calibration register"]
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    #[doc = "0x14 - RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    #[doc = "0x18 - RCC CSI calibration register"]
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    #[doc = "0x1c - RCC clock configuration register"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x20 - RCC CPU domain clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x28 - RCC PLL clock source selection register"]
    #[inline(always)]
    pub const fn pll1cfgr(&self) -> &PLL1CFGR {
        &self.pll1cfgr
    }
    #[doc = "0x2c - RCC PLL clock source selection register"]
    #[inline(always)]
    pub const fn pll2cfgr(&self) -> &PLL2CFGR {
        &self.pll2cfgr
    }
    #[doc = "0x34 - RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    #[doc = "0x38 - RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    #[doc = "0x3c - RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    #[doc = "0x40 - RCC PLL2 fractional divider register"]
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    #[doc = "0x50 - RCC clock source interrupt enable register"]
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    #[doc = "0x54 - RCC clock source interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    #[doc = "0x58 - RCC clock source interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    #[doc = "0x60 - RCC AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    #[doc = "0x64 - RCC AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    #[doc = "0x74 - RCC APB1 peripheral low reset register"]
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    #[doc = "0x78 - RCC APB1 peripheral high reset register"]
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    #[doc = "0x7c - RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    #[doc = "0x80 - RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    #[doc = "0x88 - RCC AHB1 peripherals clock register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    #[doc = "0x8c - RCC AHB2 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    #[doc = "0x9c - RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    #[doc = "0xa0 - RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    #[doc = "0xa4 - RCC APB2 peripheral clock register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    #[doc = "0xa8 - RCC APB3 peripheral clock register"]
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    #[doc = "0xb0 - RCC AHB1 sleep clock register"]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    #[doc = "0xb4 - RCC AHB2 sleep clock register"]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    #[doc = "0xc4 - RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    #[doc = "0xc8 - RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    #[doc = "0xcc - RCC APB2 sleep clock register"]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    #[doc = "0xd0 - RCC APB3 sleep clock register"]
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    #[doc = "0xd8 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    #[doc = "0xdc - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    #[doc = "0xe0 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    #[doc = "0xe4 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr4(&self) -> &CCIPR4 {
        &self.ccipr4
    }
    #[doc = "0xe8 - RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr5(&self) -> &CCIPR5 {
        &self.ccipr5
    }
    #[doc = "0xf0 - RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    #[doc = "0xf4 - RCC reset status register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    #[doc = "0x114 - RCC privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
#[doc = "CR (rw) register accessor: RCC clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "RCC clock control register"]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: RCC HSI calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicfgr`]
module"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
#[doc = "RCC HSI calibration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: RCC clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`]
module"]
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
#[doc = "RCC clock recovery RC register"]
pub mod crrcr;
#[doc = "CSICFGR (rw) register accessor: RCC CSI calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csicfgr`]
module"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
#[doc = "RCC CSI calibration register"]
pub mod csicfgr;
#[doc = "CFGR1 (rw) register accessor: RCC clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "RCC clock configuration register"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: RCC CPU domain clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "RCC CPU domain clock configuration register 2"]
pub mod cfgr2;
#[doc = "PLL1CFGR (rw) register accessor: RCC PLL clock source selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1cfgr`]
module"]
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGRrs>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll1cfgr;
#[doc = "PLL2CFGR (rw) register accessor: RCC PLL clock source selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2cfgr`]
module"]
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGRrs>;
#[doc = "RCC PLL clock source selection register"]
pub mod pll2cfgr;
#[doc = "PLL1DIVR (rw) register accessor: RCC PLL1 dividers register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1divr`]
module"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1fracr`]
module"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
#[doc = "RCC PLL1 fractional divider register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: RCC PLL1 dividers register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2divr`]
module"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
#[doc = "RCC PLL1 dividers register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2fracr`]
module"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
#[doc = "RCC PLL2 fractional divider register"]
pub mod pll2fracr;
#[doc = "CIER (rw) register accessor: RCC clock source interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`]
module"]
pub type CIER = crate::Reg<cier::CIERrs>;
#[doc = "RCC clock source interrupt enable register"]
pub mod cier;
#[doc = "CIFR (r) register accessor: RCC clock source interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cifr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`]
module"]
pub type CIFR = crate::Reg<cifr::CIFRrs>;
#[doc = "RCC clock source interrupt flag register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: RCC clock source interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`]
module"]
pub type CICR = crate::Reg<cicr::CICRrs>;
#[doc = "RCC clock source interrupt clear register"]
pub mod cicr;
#[doc = "AHB1RSTR (rw) register accessor: RCC AHB1 reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`]
module"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
#[doc = "RCC AHB1 reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: RCC AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`]
module"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
#[doc = "RCC AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "APB1LRSTR (rw) register accessor: RCC APB1 peripheral low reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lrstr`]
module"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
#[doc = "RCC APB1 peripheral low reset register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: RCC APB1 peripheral high reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hrstr`]
module"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
#[doc = "RCC APB1 peripheral high reset register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`]
module"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
#[doc = "RCC APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3RSTR (rw) register accessor: RCC APB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3rstr`]
module"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
#[doc = "RCC APB3 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1ENR (rw) register accessor: RCC AHB1 peripherals clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`]
module"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
#[doc = "RCC AHB1 peripherals clock register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: RCC AHB2 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`]
module"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
#[doc = "RCC AHB2 peripheral clock register"]
pub mod ahb2enr;
#[doc = "APB1LENR (rw) register accessor: RCC APB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lenr`]
module"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1lenr;
#[doc = "APB1HENR (rw) register accessor: RCC APB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1henr`]
module"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
#[doc = "RCC APB1 peripheral clock register"]
pub mod apb1henr;
#[doc = "APB2ENR (rw) register accessor: RCC APB2 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`]
module"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
#[doc = "RCC APB2 peripheral clock register"]
pub mod apb2enr;
#[doc = "APB3ENR (rw) register accessor: RCC APB3 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3enr`]
module"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
#[doc = "RCC APB3 peripheral clock register"]
pub mod apb3enr;
#[doc = "AHB1LPENR (rw) register accessor: RCC AHB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`]
module"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
#[doc = "RCC AHB1 sleep clock register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: RCC AHB2 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`]
module"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
#[doc = "RCC AHB2 sleep clock register"]
pub mod ahb2lpenr;
#[doc = "APB1LLPENR (rw) register accessor: RCC APB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1llpenr`]
module"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1llpenr;
#[doc = "APB1HLPENR (rw) register accessor: RCC APB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hlpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hlpenr`]
module"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
#[doc = "RCC APB1 sleep clock register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: RCC APB2 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`]
module"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
#[doc = "RCC APB2 sleep clock register"]
pub mod apb2lpenr;
#[doc = "APB3LPENR (rw) register accessor: RCC APB3 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3lpenr`]
module"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
#[doc = "RCC APB3 sleep clock register"]
pub mod apb3lpenr;
#[doc = "CCIPR1 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr1`]
module"]
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr1;
#[doc = "CCIPR2 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr2`]
module"]
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr2;
#[doc = "CCIPR3 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr3`]
module"]
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr3;
#[doc = "CCIPR4 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr4`]
module"]
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4rs>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr4;
#[doc = "CCIPR5 (rw) register accessor: RCC kernel clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccipr5`]
module"]
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5rs>;
#[doc = "RCC kernel clock configuration register"]
pub mod ccipr5;
#[doc = "BDCR (rw) register accessor: RCC Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`]
module"]
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
#[doc = "RCC Backup domain control register"]
pub mod bdcr;
#[doc = "RSR (rw) register accessor: RCC reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSRrs>;
#[doc = "RCC reset status register"]
pub mod rsr;
#[doc = "PRIVCFGR (rw) register accessor: RCC privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@privcfgr`]
module"]
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
#[doc = "RCC privilege configuration register"]
pub mod privcfgr;
