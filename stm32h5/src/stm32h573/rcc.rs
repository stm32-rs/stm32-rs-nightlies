#[repr(C)]
#[derive(Debug)]
///Register block
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
    pll3cfgr: PLL3CFGR,
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    pll2divr: PLL2DIVR,
    pll2fracr: PLL2FRACR,
    pll3divr: PLL3DIVR,
    pll3fracr: PLL3FRACR,
    _reserved15: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved18: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    _reserved20: [u8; 0x04],
    ahb4rstr: AHB4RSTR,
    _reserved21: [u8; 0x04],
    apb1lrstr: APB1LRSTR,
    apb1hrstr: APB1HRSTR,
    apb2rstr: APB2RSTR,
    apb3rstr: APB3RSTR,
    _reserved25: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    _reserved27: [u8; 0x04],
    ahb4enr: AHB4ENR,
    _reserved28: [u8; 0x04],
    apb1lenr: APB1LENR,
    apb1henr: APB1HENR,
    apb2enr: APB2ENR,
    apb3enr: APB3ENR,
    _reserved32: [u8; 0x04],
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    _reserved34: [u8; 0x04],
    ahb4lpenr: AHB4LPENR,
    _reserved35: [u8; 0x04],
    apb1llpenr: APB1LLPENR,
    apb1hlpenr: APB1HLPENR,
    apb2lpenr: APB2LPENR,
    apb3lpenr: APB3LPENR,
    _reserved39: [u8; 0x04],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    ccipr4: CCIPR4,
    ccipr5: CCIPR5,
    _reserved44: [u8; 0x04],
    bdcr: BDCR,
    rsr: RSR,
    _reserved46: [u8; 0x18],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - RCC HSI calibration register
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    ///0x14 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    ///0x18 - RCC CSI calibration register
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    ///0x1c - RCC clock configuration register
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x20 - RCC CPU domain clock configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x28 - RCC PLL clock source selection register
    #[inline(always)]
    pub const fn pll1cfgr(&self) -> &PLL1CFGR {
        &self.pll1cfgr
    }
    ///0x2c - RCC PLL clock source selection register
    #[inline(always)]
    pub const fn pll2cfgr(&self) -> &PLL2CFGR {
        &self.pll2cfgr
    }
    ///0x30 - RCC PLL clock source selection register
    #[inline(always)]
    pub const fn pll3cfgr(&self) -> &PLL3CFGR {
        &self.pll3cfgr
    }
    ///0x34 - RCC PLL1 dividers register
    #[inline(always)]
    pub const fn pll1divr(&self) -> &PLL1DIVR {
        &self.pll1divr
    }
    ///0x38 - RCC PLL1 fractional divider register
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    ///0x3c - RCC PLL1 dividers register
    #[inline(always)]
    pub const fn pll2divr(&self) -> &PLL2DIVR {
        &self.pll2divr
    }
    ///0x40 - RCC PLL2 fractional divider register
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    ///0x44 - RCC PLL3 dividers register
    #[inline(always)]
    pub const fn pll3divr(&self) -> &PLL3DIVR {
        &self.pll3divr
    }
    ///0x48 - RCC PLL3 fractional divider register
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    ///0x50 - RCC clock source interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x54 - RCC clock source interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x58 - RCC clock source interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x60 - RCC AHB1 reset register
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x64 - RCC AHB2 peripheral reset register
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    ///0x6c - RCC AHB4 peripheral reset register
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    ///0x74 - RCC APB1 peripheral low reset register
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    ///0x78 - RCC APB1 peripheral high reset register
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    ///0x7c - RCC APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x80 - RCC APB4 peripheral reset register
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
    }
    ///0x88 - RCC AHB1 peripherals clock register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x8c - RCC AHB2 peripheral clock register
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x94 - RCC AHB4 peripheral clock register
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    ///0x9c - RCC APB1 peripheral clock register
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    ///0xa0 - RCC APB1 peripheral clock register
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    ///0xa4 - RCC APB2 peripheral clock register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0xa8 - RCC APB4 peripheral clock register
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    ///0xb0 - RCC AHB1 sleep clock register
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    ///0xb4 - RCC AHB2 sleep clock register
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    ///0xbc - RCC AHB4 sleep clock register
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    ///0xc4 - RCC APB1 sleep clock register
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    ///0xc8 - RCC APB1 sleep clock register
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    ///0xcc - RCC APB2 sleep clock register
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    ///0xd0 - RCC APB4 sleep clock register
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    ///0xd8 - RCC kernel clock configuration register
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0xdc - RCC kernel clock configuration register
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0xe0 - RCC kernel clock configuration register
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    ///0xe4 - RCC kernel clock configuration register
    #[inline(always)]
    pub const fn ccipr4(&self) -> &CCIPR4 {
        &self.ccipr4
    }
    ///0xe8 - RCC kernel clock configuration register
    #[inline(always)]
    pub const fn ccipr5(&self) -> &CCIPR5 {
        &self.ccipr5
    }
    ///0xf0 - RCC Backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0xf4 - RCC reset status register
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    ///0x110 - RCC secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x114 - RCC privilege configuration register
    #[inline(always)]
    pub const fn privcfgr(&self) -> &PRIVCFGR {
        &self.privcfgr
    }
}
/**CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC clock control register
pub mod cr;
/**HSICFGR (rw) register accessor: RCC HSI calibration register

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:HSICFGR)

For information about available fields see [`mod@hsicfgr`] module*/
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
///RCC HSI calibration register
pub mod hsicfgr;
/**CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///RCC clock recovery RC register
pub mod crrcr;
/**CSICFGR (rw) register accessor: RCC CSI calibration register

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CSICFGR)

For information about available fields see [`mod@csicfgr`] module*/
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
///RCC CSI calibration register
pub mod csicfgr;
/**CFGR1 (rw) register accessor: RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///RCC clock configuration register
pub mod cfgr1;
/**CFGR2 (rw) register accessor: RCC CPU domain clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///RCC CPU domain clock configuration register 2
pub mod cfgr2;
/**PLL1CFGR (rw) register accessor: RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL1CFGR)

For information about available fields see [`mod@pll1cfgr`] module*/
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGRrs>;
///RCC PLL clock source selection register
pub mod pll1cfgr;
/**PLL2CFGR (rw) register accessor: RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL2CFGR)

For information about available fields see [`mod@pll2cfgr`] module*/
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGRrs>;
///RCC PLL clock source selection register
pub mod pll2cfgr;
/**PLL3CFGR (rw) register accessor: RCC PLL clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL3CFGR)

For information about available fields see [`mod@pll3cfgr`] module*/
pub type PLL3CFGR = crate::Reg<pll3cfgr::PLL3CFGRrs>;
///RCC PLL clock source selection register
pub mod pll3cfgr;
/**PLL1DIVR (rw) register accessor: RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL1DIVR)

For information about available fields see [`mod@pll1divr`] module*/
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
///RCC PLL1 dividers register
pub mod pll1divr;
/**PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///RCC PLL1 fractional divider register
pub mod pll1fracr;
/**PLL2DIVR (rw) register accessor: RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL2DIVR)

For information about available fields see [`mod@pll2divr`] module*/
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVRrs>;
///RCC PLL1 dividers register
pub mod pll2divr;
/**PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL2FRACR)

For information about available fields see [`mod@pll2fracr`] module*/
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
///RCC PLL2 fractional divider register
pub mod pll2fracr;
/**PLL3DIVR (rw) register accessor: RCC PLL3 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL3DIVR)

For information about available fields see [`mod@pll3divr`] module*/
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVRrs>;
///RCC PLL3 dividers register
pub mod pll3divr;
/**PLL3FRACR (rw) register accessor: RCC PLL3 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PLL3FRACR)

For information about available fields see [`mod@pll3fracr`] module*/
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
///RCC PLL3 fractional divider register
pub mod pll3fracr;
/**CIER (rw) register accessor: RCC clock source interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock source interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock source interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock source interrupt flag register
pub mod cifr;
/**CICR (rw) register accessor: RCC clock source interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock source interrupt clear register
pub mod cicr;
/**AHB1RSTR (rw) register accessor: RCC AHB1 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///RCC AHB1 reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: RCC AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///RCC AHB2 peripheral reset register
pub mod ahb2rstr;
/**AHB4RSTR (rw) register accessor: RCC AHB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB4RSTR)

For information about available fields see [`mod@ahb4rstr`] module*/
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
///RCC AHB4 peripheral reset register
pub mod ahb4rstr;
/**APB1LRSTR (rw) register accessor: RCC APB1 peripheral low reset register

You can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1LRSTR)

For information about available fields see [`mod@apb1lrstr`] module*/
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
///RCC APB1 peripheral low reset register
pub mod apb1lrstr;
/**APB1HRSTR (rw) register accessor: RCC APB1 peripheral high reset register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1HRSTR)

For information about available fields see [`mod@apb1hrstr`] module*/
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
///RCC APB1 peripheral high reset register
pub mod apb1hrstr;
/**APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///RCC APB2 peripheral reset register
pub mod apb2rstr;
/**APB3RSTR (rw) register accessor: RCC APB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB3RSTR)

For information about available fields see [`mod@apb3rstr`] module*/
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
///RCC APB4 peripheral reset register
pub mod apb3rstr;
/**AHB1ENR (rw) register accessor: RCC AHB1 peripherals clock register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///RCC AHB1 peripherals clock register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: RCC AHB2 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///RCC AHB2 peripheral clock register
pub mod ahb2enr;
/**AHB4ENR (rw) register accessor: RCC AHB4 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB4ENR)

For information about available fields see [`mod@ahb4enr`] module*/
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
///RCC AHB4 peripheral clock register
pub mod ahb4enr;
/**APB1LENR (rw) register accessor: RCC APB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1LENR)

For information about available fields see [`mod@apb1lenr`] module*/
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
///RCC APB1 peripheral clock register
pub mod apb1lenr;
/**APB1HENR (rw) register accessor: RCC APB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1HENR)

For information about available fields see [`mod@apb1henr`] module*/
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
///RCC APB1 peripheral clock register
pub mod apb1henr;
/**APB2ENR (rw) register accessor: RCC APB2 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///RCC APB2 peripheral clock register
pub mod apb2enr;
/**APB3ENR (rw) register accessor: RCC APB4 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB3ENR)

For information about available fields see [`mod@apb3enr`] module*/
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
///RCC APB4 peripheral clock register
pub mod apb3enr;
/**AHB1LPENR (rw) register accessor: RCC AHB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB1LPENR)

For information about available fields see [`mod@ahb1lpenr`] module*/
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
///RCC AHB1 sleep clock register
pub mod ahb1lpenr;
/**AHB2LPENR (rw) register accessor: RCC AHB2 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB2LPENR)

For information about available fields see [`mod@ahb2lpenr`] module*/
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
///RCC AHB2 sleep clock register
pub mod ahb2lpenr;
/**AHB4LPENR (rw) register accessor: RCC AHB4 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:AHB4LPENR)

For information about available fields see [`mod@ahb4lpenr`] module*/
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
///RCC AHB4 sleep clock register
pub mod ahb4lpenr;
/**APB1LLPENR (rw) register accessor: RCC APB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1LLPENR)

For information about available fields see [`mod@apb1llpenr`] module*/
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
///RCC APB1 sleep clock register
pub mod apb1llpenr;
/**APB1HLPENR (rw) register accessor: RCC APB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB1HLPENR)

For information about available fields see [`mod@apb1hlpenr`] module*/
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
///RCC APB1 sleep clock register
pub mod apb1hlpenr;
/**APB2LPENR (rw) register accessor: RCC APB2 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB2LPENR)

For information about available fields see [`mod@apb2lpenr`] module*/
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
///RCC APB2 sleep clock register
pub mod apb2lpenr;
/**APB3LPENR (rw) register accessor: RCC APB4 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB3LPENR)

For information about available fields see [`mod@apb3lpenr`] module*/
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
///RCC APB4 sleep clock register
pub mod apb3lpenr;
/**CCIPR1 (rw) register accessor: RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///RCC kernel clock configuration register
pub mod ccipr1;
/**CCIPR2 (rw) register accessor: RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///RCC kernel clock configuration register
pub mod ccipr2;
/**CCIPR3 (rw) register accessor: RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR3)

For information about available fields see [`mod@ccipr3`] module*/
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
///RCC kernel clock configuration register
pub mod ccipr3;
/**CCIPR4 (rw) register accessor: RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR4)

For information about available fields see [`mod@ccipr4`] module*/
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4rs>;
///RCC kernel clock configuration register
pub mod ccipr4;
/**CCIPR5 (rw) register accessor: RCC kernel clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CCIPR5)

For information about available fields see [`mod@ccipr5`] module*/
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5rs>;
///RCC kernel clock configuration register
pub mod ccipr5;
/**BDCR (rw) register accessor: RCC Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RCC Backup domain control register
pub mod bdcr;
/**RSR (rw) register accessor: RCC reset status register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:RSR)

For information about available fields see [`mod@rsr`] module*/
pub type RSR = crate::Reg<rsr::RSRrs>;
///RCC reset status register
pub mod rsr;
/**SECCFGR (rw) register accessor: RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///RCC secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///RCC privilege configuration register
pub mod privcfgr;
