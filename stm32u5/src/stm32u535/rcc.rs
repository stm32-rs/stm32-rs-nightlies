#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RCC_CR {
        &self.rcc_cr
    }
    ///0x08 - RCC internal clock sources calibration register 1
    #[inline(always)]
    pub const fn rcc_icscr1(&self) -> &RCC_ICSCR1 {
        &self.rcc_icscr1
    }
    ///0x0c - RCC internal clock sources calibration register 2
    #[inline(always)]
    pub const fn rcc_icscr2(&self) -> &RCC_ICSCR2 {
        &self.rcc_icscr2
    }
    ///0x10 - RCC internal clock sources calibration register 3
    #[inline(always)]
    pub const fn rcc_icscr3(&self) -> &RCC_ICSCR3 {
        &self.rcc_icscr3
    }
    ///0x14 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RCC_CRRCR {
        &self.rcc_crrcr
    }
    ///0x1c - RCC clock configuration register 1
    #[inline(always)]
    pub const fn rcc_cfgr1(&self) -> &RCC_CFGR1 {
        &self.rcc_cfgr1
    }
    ///0x20 - RCC clock configuration register 2
    #[inline(always)]
    pub const fn rcc_cfgr2(&self) -> &RCC_CFGR2 {
        &self.rcc_cfgr2
    }
    ///0x24 - RCC clock configuration register 3
    #[inline(always)]
    pub const fn rcc_cfgr3(&self) -> &RCC_CFGR3 {
        &self.rcc_cfgr3
    }
    ///0x28 - RCC PLL1 configuration register
    #[inline(always)]
    pub const fn rcc_pll1cfgr(&self) -> &RCC_PLL1CFGR {
        &self.rcc_pll1cfgr
    }
    ///0x2c - RCC PLL2 configuration register
    #[inline(always)]
    pub const fn rcc_pll2cfgr(&self) -> &RCC_PLL2CFGR {
        &self.rcc_pll2cfgr
    }
    ///0x30 - RCC PLL3 configuration register
    #[inline(always)]
    pub const fn rcc_pll3cfgr(&self) -> &RCC_PLL3CFGR {
        &self.rcc_pll3cfgr
    }
    ///0x34 - RCC PLL1 dividers register
    #[inline(always)]
    pub const fn rcc_pll1divr(&self) -> &RCC_PLL1DIVR {
        &self.rcc_pll1divr
    }
    ///0x38 - RCC PLL1 fractional divider register
    #[inline(always)]
    pub const fn rcc_pll1fracr(&self) -> &RCC_PLL1FRACR {
        &self.rcc_pll1fracr
    }
    ///0x3c - RCC PLL2 dividers configuration register
    #[inline(always)]
    pub const fn rcc_pll2divr(&self) -> &RCC_PLL2DIVR {
        &self.rcc_pll2divr
    }
    ///0x40 - RCC PLL2 fractional divider register
    #[inline(always)]
    pub const fn rcc_pll2fracr(&self) -> &RCC_PLL2FRACR {
        &self.rcc_pll2fracr
    }
    ///0x44 - RCC PLL3 dividers configuration register
    #[inline(always)]
    pub const fn rcc_pll3divr(&self) -> &RCC_PLL3DIVR {
        &self.rcc_pll3divr
    }
    ///0x48 - RCC PLL3 fractional divider register
    #[inline(always)]
    pub const fn rcc_pll3fracr(&self) -> &RCC_PLL3FRACR {
        &self.rcc_pll3fracr
    }
    ///0x50 - RCC clock interrupt enable register
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RCC_CIER {
        &self.rcc_cier
    }
    ///0x54 - RCC clock interrupt flag register
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RCC_CIFR {
        &self.rcc_cifr
    }
    ///0x58 - RCC clock interrupt clear register
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RCC_CICR {
        &self.rcc_cicr
    }
    ///0x60 - RCC AHB1 peripheral reset register
    #[inline(always)]
    pub const fn rcc_ahb1rstr(&self) -> &RCC_AHB1RSTR {
        &self.rcc_ahb1rstr
    }
    ///0x64 - RCC AHB2 peripheral reset register 1
    #[inline(always)]
    pub const fn rcc_ahb2rstr1(&self) -> &RCC_AHB2RSTR1 {
        &self.rcc_ahb2rstr1
    }
    ///0x68 - RCC AHB2 peripheral reset register 2
    #[inline(always)]
    pub const fn rcc_ahb2rstr2(&self) -> &RCC_AHB2RSTR2 {
        &self.rcc_ahb2rstr2
    }
    ///0x6c - RCC AHB3 peripheral reset register
    #[inline(always)]
    pub const fn rcc_ahb3rstr(&self) -> &RCC_AHB3RSTR {
        &self.rcc_ahb3rstr
    }
    ///0x74 - RCC APB1 peripheral reset register 1
    #[inline(always)]
    pub const fn rcc_apb1rstr1(&self) -> &RCC_APB1RSTR1 {
        &self.rcc_apb1rstr1
    }
    ///0x78 - RCC APB1 peripheral reset register 2
    #[inline(always)]
    pub const fn rcc_apb1rstr2(&self) -> &RCC_APB1RSTR2 {
        &self.rcc_apb1rstr2
    }
    ///0x7c - RCC APB2 peripheral reset register
    #[inline(always)]
    pub const fn rcc_apb2rstr(&self) -> &RCC_APB2RSTR {
        &self.rcc_apb2rstr
    }
    ///0x80 - RCC APB3 peripheral reset register
    #[inline(always)]
    pub const fn rcc_apb3rstr(&self) -> &RCC_APB3RSTR {
        &self.rcc_apb3rstr
    }
    ///0x88 - RCC AHB1 peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_ahb1enr(&self) -> &RCC_AHB1ENR {
        &self.rcc_ahb1enr
    }
    ///0x8c - RCC AHB2 peripheral clock enable register 1
    #[inline(always)]
    pub const fn rcc_ahb2enr1(&self) -> &RCC_AHB2ENR1 {
        &self.rcc_ahb2enr1
    }
    ///0x90 - RCC AHB2 peripheral clock enable register 2
    #[inline(always)]
    pub const fn rcc_ahb2enr2(&self) -> &RCC_AHB2ENR2 {
        &self.rcc_ahb2enr2
    }
    ///0x94 - RCC AHB3 peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_ahb3enr(&self) -> &RCC_AHB3ENR {
        &self.rcc_ahb3enr
    }
    ///0x9c - RCC APB1 peripheral clock enable register 1
    #[inline(always)]
    pub const fn rcc_apb1enr1(&self) -> &RCC_APB1ENR1 {
        &self.rcc_apb1enr1
    }
    ///0xa0 - RCC APB1 peripheral clock enable register 2
    #[inline(always)]
    pub const fn rcc_apb1enr2(&self) -> &RCC_APB1ENR2 {
        &self.rcc_apb1enr2
    }
    ///0xa4 - RCC APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_apb2enr(&self) -> &RCC_APB2ENR {
        &self.rcc_apb2enr
    }
    ///0xa8 - RCC APB3 peripheral clock enable register
    #[inline(always)]
    pub const fn rcc_apb3enr(&self) -> &RCC_APB3ENR {
        &self.rcc_apb3enr
    }
    ///0xb0 - RCC AHB1 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn rcc_ahb1smenr(&self) -> &RCC_AHB1SMENR {
        &self.rcc_ahb1smenr
    }
    ///0xb4 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1
    #[inline(always)]
    pub const fn rcc_ahb2smenr1(&self) -> &RCC_AHB2SMENR1 {
        &self.rcc_ahb2smenr1
    }
    ///0xb8 - RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn rcc_ahb2smenr2(&self) -> &RCC_AHB2SMENR2 {
        &self.rcc_ahb2smenr2
    }
    ///0xbc - RCC AHB3 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn rcc_ahb3smenr(&self) -> &RCC_AHB3SMENR {
        &self.rcc_ahb3smenr
    }
    ///0xc4 - RCC APB1 peripheral clock enable in Sleep and Stop modes register 1
    #[inline(always)]
    pub const fn rcc_apb1smenr1(&self) -> &RCC_APB1SMENR1 {
        &self.rcc_apb1smenr1
    }
    ///0xc8 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn rcc_apb1smenr2(&self) -> &RCC_APB1SMENR2 {
        &self.rcc_apb1smenr2
    }
    ///0xcc - RCC APB2 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn rcc_apb2smenr(&self) -> &RCC_APB2SMENR {
        &self.rcc_apb2smenr
    }
    ///0xd0 - RCC APB3 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn rcc_apb3smenr(&self) -> &RCC_APB3SMENR {
        &self.rcc_apb3smenr
    }
    ///0xd8 - RCC SmartRun domain peripheral autonomous mode register
    #[inline(always)]
    pub const fn rcc_srdamr(&self) -> &RCC_SRDAMR {
        &self.rcc_srdamr
    }
    ///0xe0 - RCC peripherals independent clock configuration register 1
    #[inline(always)]
    pub const fn rcc_ccipr1(&self) -> &RCC_CCIPR1 {
        &self.rcc_ccipr1
    }
    ///0xe4 - RCC peripherals independent clock configuration register 2
    #[inline(always)]
    pub const fn rcc_ccipr2(&self) -> &RCC_CCIPR2 {
        &self.rcc_ccipr2
    }
    ///0xe8 - RCC peripherals independent clock configuration register 3
    #[inline(always)]
    pub const fn rcc_ccipr3(&self) -> &RCC_CCIPR3 {
        &self.rcc_ccipr3
    }
    ///0xf0 - RCC backup domain control register
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RCC_BDCR {
        &self.rcc_bdcr
    }
    ///0xf4 - RCC control/status register
    #[inline(always)]
    pub const fn rcc_csr(&self) -> &RCC_CSR {
        &self.rcc_csr
    }
    ///0x110 - RCC secure configuration register
    #[inline(always)]
    pub const fn rcc_seccfgr(&self) -> &RCC_SECCFGR {
        &self.rcc_seccfgr
    }
    ///0x114 - RCC privilege configuration register
    #[inline(always)]
    pub const fn rcc_privcfgr(&self) -> &RCC_PRIVCFGR {
        &self.rcc_privcfgr
    }
}
/**RCC_CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`rcc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CR)

For information about available fields see [`mod@rcc_cr`]
module*/
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CRrs>;
///RCC clock control register
pub mod rcc_cr;
/**RCC_ICSCR1 (rw) register accessor: RCC internal clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_ICSCR1)

For information about available fields see [`mod@rcc_icscr1`]
module*/
pub type RCC_ICSCR1 = crate::Reg<rcc_icscr1::RCC_ICSCR1rs>;
///RCC internal clock sources calibration register 1
pub mod rcc_icscr1;
/**RCC_ICSCR2 (rw) register accessor: RCC internal clock sources calibration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_ICSCR2)

For information about available fields see [`mod@rcc_icscr2`]
module*/
pub type RCC_ICSCR2 = crate::Reg<rcc_icscr2::RCC_ICSCR2rs>;
///RCC internal clock sources calibration register 2
pub mod rcc_icscr2;
/**RCC_ICSCR3 (rw) register accessor: RCC internal clock sources calibration register 3

You can [`read`](crate::Reg::read) this register and get [`rcc_icscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_ICSCR3)

For information about available fields see [`mod@rcc_icscr3`]
module*/
pub type RCC_ICSCR3 = crate::Reg<rcc_icscr3::RCC_ICSCR3rs>;
///RCC internal clock sources calibration register 3
pub mod rcc_icscr3;
/**RCC_CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CRRCR)

For information about available fields see [`mod@rcc_crrcr`]
module*/
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCRrs>;
///RCC clock recovery RC register
pub mod rcc_crrcr;
/**RCC_CFGR1 (rw) register accessor: RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CFGR1)

For information about available fields see [`mod@rcc_cfgr1`]
module*/
pub type RCC_CFGR1 = crate::Reg<rcc_cfgr1::RCC_CFGR1rs>;
///RCC clock configuration register 1
pub mod rcc_cfgr1;
/**RCC_CFGR2 (rw) register accessor: RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CFGR2)

For information about available fields see [`mod@rcc_cfgr2`]
module*/
pub type RCC_CFGR2 = crate::Reg<rcc_cfgr2::RCC_CFGR2rs>;
///RCC clock configuration register 2
pub mod rcc_cfgr2;
/**RCC_CFGR3 (rw) register accessor: RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CFGR3)

For information about available fields see [`mod@rcc_cfgr3`]
module*/
pub type RCC_CFGR3 = crate::Reg<rcc_cfgr3::RCC_CFGR3rs>;
///RCC clock configuration register 3
pub mod rcc_cfgr3;
/**RCC_PLL1CFGR (rw) register accessor: RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL1CFGR)

For information about available fields see [`mod@rcc_pll1cfgr`]
module*/
pub type RCC_PLL1CFGR = crate::Reg<rcc_pll1cfgr::RCC_PLL1CFGRrs>;
///RCC PLL1 configuration register
pub mod rcc_pll1cfgr;
/**RCC_PLL2CFGR (rw) register accessor: RCC PLL2 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL2CFGR)

For information about available fields see [`mod@rcc_pll2cfgr`]
module*/
pub type RCC_PLL2CFGR = crate::Reg<rcc_pll2cfgr::RCC_PLL2CFGRrs>;
///RCC PLL2 configuration register
pub mod rcc_pll2cfgr;
/**RCC_PLL3CFGR (rw) register accessor: RCC PLL3 configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL3CFGR)

For information about available fields see [`mod@rcc_pll3cfgr`]
module*/
pub type RCC_PLL3CFGR = crate::Reg<rcc_pll3cfgr::RCC_PLL3CFGRrs>;
///RCC PLL3 configuration register
pub mod rcc_pll3cfgr;
/**RCC_PLL1DIVR (rw) register accessor: RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL1DIVR)

For information about available fields see [`mod@rcc_pll1divr`]
module*/
pub type RCC_PLL1DIVR = crate::Reg<rcc_pll1divr::RCC_PLL1DIVRrs>;
///RCC PLL1 dividers register
pub mod rcc_pll1divr;
/**RCC_PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL1FRACR)

For information about available fields see [`mod@rcc_pll1fracr`]
module*/
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACRrs>;
///RCC PLL1 fractional divider register
pub mod rcc_pll1fracr;
/**RCC_PLL2DIVR (rw) register accessor: RCC PLL2 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL2DIVR)

For information about available fields see [`mod@rcc_pll2divr`]
module*/
pub type RCC_PLL2DIVR = crate::Reg<rcc_pll2divr::RCC_PLL2DIVRrs>;
///RCC PLL2 dividers configuration register
pub mod rcc_pll2divr;
/**RCC_PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL2FRACR)

For information about available fields see [`mod@rcc_pll2fracr`]
module*/
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACRrs>;
///RCC PLL2 fractional divider register
pub mod rcc_pll2fracr;
/**RCC_PLL3DIVR (rw) register accessor: RCC PLL3 dividers configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL3DIVR)

For information about available fields see [`mod@rcc_pll3divr`]
module*/
pub type RCC_PLL3DIVR = crate::Reg<rcc_pll3divr::RCC_PLL3DIVRrs>;
///RCC PLL3 dividers configuration register
pub mod rcc_pll3divr;
/**RCC_PLL3FRACR (rw) register accessor: RCC PLL3 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`rcc_pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PLL3FRACR)

For information about available fields see [`mod@rcc_pll3fracr`]
module*/
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACRrs>;
///RCC PLL3 fractional divider register
pub mod rcc_pll3fracr;
/**RCC_CIER (rw) register accessor: RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CIER)

For information about available fields see [`mod@rcc_cier`]
module*/
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIERrs>;
///RCC clock interrupt enable register
pub mod rcc_cier;
/**RCC_CIFR (r) register accessor: RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CIFR)

For information about available fields see [`mod@rcc_cifr`]
module*/
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFRrs>;
///RCC clock interrupt flag register
pub mod rcc_cifr;
/**RCC_CICR (w) register accessor: RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CICR)

For information about available fields see [`mod@rcc_cicr`]
module*/
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICRrs>;
///RCC clock interrupt clear register
pub mod rcc_cicr;
/**RCC_AHB1RSTR (rw) register accessor: RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB1RSTR)

For information about available fields see [`mod@rcc_ahb1rstr`]
module*/
pub type RCC_AHB1RSTR = crate::Reg<rcc_ahb1rstr::RCC_AHB1RSTRrs>;
///RCC AHB1 peripheral reset register
pub mod rcc_ahb1rstr;
/**RCC_AHB2RSTR1 (rw) register accessor: RCC AHB2 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2RSTR1)

For information about available fields see [`mod@rcc_ahb2rstr1`]
module*/
pub type RCC_AHB2RSTR1 = crate::Reg<rcc_ahb2rstr1::RCC_AHB2RSTR1rs>;
///RCC AHB2 peripheral reset register 1
pub mod rcc_ahb2rstr1;
/**RCC_AHB2RSTR2 (rw) register accessor: RCC AHB2 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2RSTR2)

For information about available fields see [`mod@rcc_ahb2rstr2`]
module*/
pub type RCC_AHB2RSTR2 = crate::Reg<rcc_ahb2rstr2::RCC_AHB2RSTR2rs>;
///RCC AHB2 peripheral reset register 2
pub mod rcc_ahb2rstr2;
/**RCC_AHB3RSTR (rw) register accessor: RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB3RSTR)

For information about available fields see [`mod@rcc_ahb3rstr`]
module*/
pub type RCC_AHB3RSTR = crate::Reg<rcc_ahb3rstr::RCC_AHB3RSTRrs>;
///RCC AHB3 peripheral reset register
pub mod rcc_ahb3rstr;
/**RCC_APB1RSTR1 (rw) register accessor: RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1RSTR1)

For information about available fields see [`mod@rcc_apb1rstr1`]
module*/
pub type RCC_APB1RSTR1 = crate::Reg<rcc_apb1rstr1::RCC_APB1RSTR1rs>;
///RCC APB1 peripheral reset register 1
pub mod rcc_apb1rstr1;
/**RCC_APB1RSTR2 (rw) register accessor: RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1RSTR2)

For information about available fields see [`mod@rcc_apb1rstr2`]
module*/
pub type RCC_APB1RSTR2 = crate::Reg<rcc_apb1rstr2::RCC_APB1RSTR2rs>;
///RCC APB1 peripheral reset register 2
pub mod rcc_apb1rstr2;
/**RCC_APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB2RSTR)

For information about available fields see [`mod@rcc_apb2rstr`]
module*/
pub type RCC_APB2RSTR = crate::Reg<rcc_apb2rstr::RCC_APB2RSTRrs>;
///RCC APB2 peripheral reset register
pub mod rcc_apb2rstr;
/**RCC_APB3RSTR (rw) register accessor: RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB3RSTR)

For information about available fields see [`mod@rcc_apb3rstr`]
module*/
pub type RCC_APB3RSTR = crate::Reg<rcc_apb3rstr::RCC_APB3RSTRrs>;
///RCC APB3 peripheral reset register
pub mod rcc_apb3rstr;
/**RCC_AHB1ENR (rw) register accessor: RCC AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB1ENR)

For information about available fields see [`mod@rcc_ahb1enr`]
module*/
pub type RCC_AHB1ENR = crate::Reg<rcc_ahb1enr::RCC_AHB1ENRrs>;
///RCC AHB1 peripheral clock enable register
pub mod rcc_ahb1enr;
/**RCC_AHB2ENR1 (rw) register accessor: RCC AHB2 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2ENR1)

For information about available fields see [`mod@rcc_ahb2enr1`]
module*/
pub type RCC_AHB2ENR1 = crate::Reg<rcc_ahb2enr1::RCC_AHB2ENR1rs>;
///RCC AHB2 peripheral clock enable register 1
pub mod rcc_ahb2enr1;
/**RCC_AHB2ENR2 (rw) register accessor: RCC AHB2 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2ENR2)

For information about available fields see [`mod@rcc_ahb2enr2`]
module*/
pub type RCC_AHB2ENR2 = crate::Reg<rcc_ahb2enr2::RCC_AHB2ENR2rs>;
///RCC AHB2 peripheral clock enable register 2
pub mod rcc_ahb2enr2;
/**RCC_AHB3ENR (rw) register accessor: RCC AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB3ENR)

For information about available fields see [`mod@rcc_ahb3enr`]
module*/
pub type RCC_AHB3ENR = crate::Reg<rcc_ahb3enr::RCC_AHB3ENRrs>;
///RCC AHB3 peripheral clock enable register
pub mod rcc_ahb3enr;
/**RCC_APB1ENR1 (rw) register accessor: RCC APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1ENR1)

For information about available fields see [`mod@rcc_apb1enr1`]
module*/
pub type RCC_APB1ENR1 = crate::Reg<rcc_apb1enr1::RCC_APB1ENR1rs>;
///RCC APB1 peripheral clock enable register 1
pub mod rcc_apb1enr1;
/**RCC_APB1ENR2 (rw) register accessor: RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1ENR2)

For information about available fields see [`mod@rcc_apb1enr2`]
module*/
pub type RCC_APB1ENR2 = crate::Reg<rcc_apb1enr2::RCC_APB1ENR2rs>;
///RCC APB1 peripheral clock enable register 2
pub mod rcc_apb1enr2;
/**RCC_APB2ENR (rw) register accessor: RCC APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB2ENR)

For information about available fields see [`mod@rcc_apb2enr`]
module*/
pub type RCC_APB2ENR = crate::Reg<rcc_apb2enr::RCC_APB2ENRrs>;
///RCC APB2 peripheral clock enable register
pub mod rcc_apb2enr;
/**RCC_APB3ENR (rw) register accessor: RCC APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB3ENR)

For information about available fields see [`mod@rcc_apb3enr`]
module*/
pub type RCC_APB3ENR = crate::Reg<rcc_apb3enr::RCC_APB3ENRrs>;
///RCC APB3 peripheral clock enable register
pub mod rcc_apb3enr;
/**RCC_AHB1SMENR (rw) register accessor: RCC AHB1 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB1SMENR)

For information about available fields see [`mod@rcc_ahb1smenr`]
module*/
pub type RCC_AHB1SMENR = crate::Reg<rcc_ahb1smenr::RCC_AHB1SMENRrs>;
///RCC AHB1 peripheral clock enable in Sleep and Stop modes register
pub mod rcc_ahb1smenr;
/**RCC_AHB2SMENR1 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2SMENR1)

For information about available fields see [`mod@rcc_ahb2smenr1`]
module*/
pub type RCC_AHB2SMENR1 = crate::Reg<rcc_ahb2smenr1::RCC_AHB2SMENR1rs>;
///RCC AHB2 peripheral clock enable in Sleep and Stop modes register 1
pub mod rcc_ahb2smenr1;
/**RCC_AHB2SMENR2 (rw) register accessor: RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB2SMENR2)

For information about available fields see [`mod@rcc_ahb2smenr2`]
module*/
pub type RCC_AHB2SMENR2 = crate::Reg<rcc_ahb2smenr2::RCC_AHB2SMENR2rs>;
///RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2
pub mod rcc_ahb2smenr2;
/**RCC_AHB3SMENR (rw) register accessor: RCC AHB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_AHB3SMENR)

For information about available fields see [`mod@rcc_ahb3smenr`]
module*/
pub type RCC_AHB3SMENR = crate::Reg<rcc_ahb3smenr::RCC_AHB3SMENRrs>;
///RCC AHB3 peripheral clock enable in Sleep and Stop modes register
pub mod rcc_ahb3smenr;
/**RCC_APB1SMENR1 (rw) register accessor: RCC APB1 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1SMENR1)

For information about available fields see [`mod@rcc_apb1smenr1`]
module*/
pub type RCC_APB1SMENR1 = crate::Reg<rcc_apb1smenr1::RCC_APB1SMENR1rs>;
///RCC APB1 peripheral clock enable in Sleep and Stop modes register 1
pub mod rcc_apb1smenr1;
/**RCC_APB1SMENR2 (rw) register accessor: RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB1SMENR2)

For information about available fields see [`mod@rcc_apb1smenr2`]
module*/
pub type RCC_APB1SMENR2 = crate::Reg<rcc_apb1smenr2::RCC_APB1SMENR2rs>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod rcc_apb1smenr2;
/**RCC_APB2SMENR (rw) register accessor: RCC APB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB2SMENR)

For information about available fields see [`mod@rcc_apb2smenr`]
module*/
pub type RCC_APB2SMENR = crate::Reg<rcc_apb2smenr::RCC_APB2SMENRrs>;
///RCC APB2 peripheral clocks enable in Sleep and Stop modes register
pub mod rcc_apb2smenr;
/**RCC_APB3SMENR (rw) register accessor: RCC APB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_APB3SMENR)

For information about available fields see [`mod@rcc_apb3smenr`]
module*/
pub type RCC_APB3SMENR = crate::Reg<rcc_apb3smenr::RCC_APB3SMENRrs>;
///RCC APB3 peripheral clock enable in Sleep and Stop modes register
pub mod rcc_apb3smenr;
/**RCC_SRDAMR (rw) register accessor: RCC SmartRun domain peripheral autonomous mode register

You can [`read`](crate::Reg::read) this register and get [`rcc_srdamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_srdamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_SRDAMR)

For information about available fields see [`mod@rcc_srdamr`]
module*/
pub type RCC_SRDAMR = crate::Reg<rcc_srdamr::RCC_SRDAMRrs>;
///RCC SmartRun domain peripheral autonomous mode register
pub mod rcc_srdamr;
/**RCC_CCIPR1 (rw) register accessor: RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CCIPR1)

For information about available fields see [`mod@rcc_ccipr1`]
module*/
pub type RCC_CCIPR1 = crate::Reg<rcc_ccipr1::RCC_CCIPR1rs>;
///RCC peripherals independent clock configuration register 1
pub mod rcc_ccipr1;
/**RCC_CCIPR2 (rw) register accessor: RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CCIPR2)

For information about available fields see [`mod@rcc_ccipr2`]
module*/
pub type RCC_CCIPR2 = crate::Reg<rcc_ccipr2::RCC_CCIPR2rs>;
///RCC peripherals independent clock configuration register 2
pub mod rcc_ccipr2;
/**RCC_CCIPR3 (rw) register accessor: RCC peripherals independent clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CCIPR3)

For information about available fields see [`mod@rcc_ccipr3`]
module*/
pub type RCC_CCIPR3 = crate::Reg<rcc_ccipr3::RCC_CCIPR3rs>;
///RCC peripherals independent clock configuration register 3
pub mod rcc_ccipr3;
/**RCC_BDCR (rw) register accessor: RCC backup domain control register

You can [`read`](crate::Reg::read) this register and get [`rcc_bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_BDCR)

For information about available fields see [`mod@rcc_bdcr`]
module*/
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCRrs>;
///RCC backup domain control register
pub mod rcc_bdcr;
/**RCC_CSR (rw) register accessor: RCC control/status register

You can [`read`](crate::Reg::read) this register and get [`rcc_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_CSR)

For information about available fields see [`mod@rcc_csr`]
module*/
pub type RCC_CSR = crate::Reg<rcc_csr::RCC_CSRrs>;
///RCC control/status register
pub mod rcc_csr;
/**RCC_SECCFGR (rw) register accessor: RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_SECCFGR)

For information about available fields see [`mod@rcc_seccfgr`]
module*/
pub type RCC_SECCFGR = crate::Reg<rcc_seccfgr::RCC_SECCFGRrs>;
///RCC secure configuration register
pub mod rcc_seccfgr;
/**RCC_PRIVCFGR (rw) register accessor: RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_PRIVCFGR)

For information about available fields see [`mod@rcc_privcfgr`]
module*/
pub type RCC_PRIVCFGR = crate::Reg<rcc_privcfgr::RCC_PRIVCFGRrs>;
///RCC privilege configuration register
pub mod rcc_privcfgr;
