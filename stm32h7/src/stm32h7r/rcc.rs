#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    hsicfgr: HSICFGR,
    crrcr: CRRCR,
    csicfgr: CSICFGR,
    cfgr: CFGR,
    _reserved5: [u8; 0x04],
    cdcfgr: CDCFGR,
    bmcfgr: BMCFGR,
    apbcfgr: APBCFGR,
    _reserved8: [u8; 0x04],
    pllckselr: PLLCKSELR,
    pllcfgr: PLLCFGR,
    pll1divr1: PLL1DIVR1,
    pll1fracr: PLL1FRACR,
    pll2divr1: PLL2DIVR1,
    pll2fracr: PLL2FRACR,
    pll3divr1: PLL3DIVR1,
    pll3fracr: PLL3FRACR,
    _reserved16: [u8; 0x04],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    ccipr4: CCIPR4,
    _reserved20: [u8; 0x04],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved23: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    _reserved25: [u8; 0x04],
    ahb5rstr: AHB5RSTR,
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb4rstr: AHB4RSTR,
    apb5rstr: APB5RSTR,
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    apb4rstr: APB4RSTR,
    _reserved34: [u8; 0x04],
    ahb3rstr: AHB3RSTR,
    _reserved35: [u8; 0x08],
    ckgdisr: CKGDISR,
    _reserved36: [u8; 0x0c],
    pll1divr2: PLL1DIVR2,
    pll2divr2: PLL2DIVR2,
    pll3divr2: PLL3DIVR2,
    pll1sscgr: PLL1SSCGR,
    pll2sscgr: PLL2SSCGR,
    pll3sscgr: PLL3SSCGR,
    _reserved42: [u8; 0x28],
    ckprotr: CKPROTR,
    _reserved43: [u8; 0x2c],
    rsr: RSR,
    ahb5enr: AHB5ENR,
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb4enr: AHB4ENR,
    apb5enr: APB5ENR,
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    apb4enr: APB4ENR,
    ahb3enr: AHB3ENR,
    ahb5lpenr: AHB5LPENR,
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    ahb4lpenr: AHB4LPENR,
    ahb3lpenr: AHB3LPENR,
    apb1lpenr1: APB1LPENR1,
    apb1lpenr2: APB1LPENR2,
    apb2lpenr: APB2LPENR,
    apb4lpenr: APB4LPENR,
    apb5lpenr: APB5LPENR,
}
impl RegisterBlock {
    ///0x00 - RCC source control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RCC HSI calibration register
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    ///0x08 - RCC clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    ///0x0c - RCC CSI calibration register
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    ///0x10 - RCC clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x18 - RCC CPU domain clock configuration register
    #[inline(always)]
    pub const fn cdcfgr(&self) -> &CDCFGR {
        &self.cdcfgr
    }
    ///0x1c - RCC AHB clock configuration register
    #[inline(always)]
    pub const fn bmcfgr(&self) -> &BMCFGR {
        &self.bmcfgr
    }
    ///0x20 - RCC APB clocks configuration register
    #[inline(always)]
    pub const fn apbcfgr(&self) -> &APBCFGR {
        &self.apbcfgr
    }
    ///0x28 - RCC PLLs clock source selection register
    #[inline(always)]
    pub const fn pllckselr(&self) -> &PLLCKSELR {
        &self.pllckselr
    }
    ///0x2c - RCC PLLs configuration register
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    ///0x30 - RCC PLL1 dividers configuration register 1
    #[inline(always)]
    pub const fn pll1divr1(&self) -> &PLL1DIVR1 {
        &self.pll1divr1
    }
    ///0x34 - RCC PLL1 fractional divider register
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    ///0x38 - RCC PLL2 dividers configuration register 1
    #[inline(always)]
    pub const fn pll2divr1(&self) -> &PLL2DIVR1 {
        &self.pll2divr1
    }
    ///0x3c - RCC PLL2 fractional divider register
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    ///0x40 - RCC PLL3 dividers configuration register 1
    #[inline(always)]
    pub const fn pll3divr1(&self) -> &PLL3DIVR1 {
        &self.pll3divr1
    }
    ///0x44 - RCC PLL3 fractional divider register
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    ///0x4c - RCC AHB peripheral kernel clock selection register
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0x50 - RCC APB1 peripherals kernel clock selection register
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0x54 - RCC APB2 peripherals kernel clock selection register
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    ///0x58 - RCC APB4,5 peripherals kernel clock selection register
    #[inline(always)]
    pub const fn ccipr4(&self) -> &CCIPR4 {
        &self.ccipr4
    }
    ///0x60 - RCC clock source interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x64 - RCC clock source interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x68 - RCC clock source interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x70 - RCC Backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x74 - RCC clock control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x7c - RCC AHB5 peripheral reset register
    #[inline(always)]
    pub const fn ahb5rstr(&self) -> &AHB5RSTR {
        &self.ahb5rstr
    }
    ///0x80 - RCC AHB1 peripheral reset register
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x84 - RCC AHB2 peripheral reset register
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    ///0x88 - RCC AHB4 peripheral reset register
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    ///0x8c - RCC APB5 peripheral reset register
    #[inline(always)]
    pub const fn apb5rstr(&self) -> &APB5RSTR {
        &self.apb5rstr
    }
    ///0x90 - RCC APB1 peripheral reset register 1
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &APB1RSTR1 {
        &self.apb1rstr1
    }
    ///0x94 - RCC APB1 peripheral reset register 2
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &APB1RSTR2 {
        &self.apb1rstr2
    }
    ///0x98 - RCC APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x9c - RCC APB4 peripheral reset register
    #[inline(always)]
    pub const fn apb4rstr(&self) -> &APB4RSTR {
        &self.apb4rstr
    }
    ///0xa4 - RCC AHB3 peripheral reset register
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    ///0xb0 - RCC AXI clocks gating disable register
    #[inline(always)]
    pub const fn ckgdisr(&self) -> &CKGDISR {
        &self.ckgdisr
    }
    ///0xc0 - RCC PLL1 dividers configuration register 2
    #[inline(always)]
    pub const fn pll1divr2(&self) -> &PLL1DIVR2 {
        &self.pll1divr2
    }
    ///0xc4 - RCC PLL2 dividers configuration register 2
    #[inline(always)]
    pub const fn pll2divr2(&self) -> &PLL2DIVR2 {
        &self.pll2divr2
    }
    ///0xc8 - RCC PLL3 dividers configuration register 2
    #[inline(always)]
    pub const fn pll3divr2(&self) -> &PLL3DIVR2 {
        &self.pll3divr2
    }
    ///0xcc - RCC PLL1 Spread Spectrum Clock Generator register
    #[inline(always)]
    pub const fn pll1sscgr(&self) -> &PLL1SSCGR {
        &self.pll1sscgr
    }
    ///0xd0 - RCC PLL2 Spread Spectrum Clock Generator register
    #[inline(always)]
    pub const fn pll2sscgr(&self) -> &PLL2SSCGR {
        &self.pll2sscgr
    }
    ///0xd4 - RCC PLL3 Spread Spectrum Clock Generator register
    #[inline(always)]
    pub const fn pll3sscgr(&self) -> &PLL3SSCGR {
        &self.pll3sscgr
    }
    ///0x100 - RCC clock protection register
    #[inline(always)]
    pub const fn ckprotr(&self) -> &CKPROTR {
        &self.ckprotr
    }
    ///0x130 - RCC Reset status register
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    ///0x134 - RCC AHB5 clock enable register
    #[inline(always)]
    pub const fn ahb5enr(&self) -> &AHB5ENR {
        &self.ahb5enr
    }
    ///0x138 - RCC AHB1 clock enable register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x13c - RCC AHB2 clock enable register
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x140 - RCC AHB4 clock enable register
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    ///0x144 - RCC APB5 clock enable register
    #[inline(always)]
    pub const fn apb5enr(&self) -> &APB5ENR {
        &self.apb5enr
    }
    ///0x148 - RCC APB1 clock enable register 1
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    ///0x14c - RCC APB1 clock enable register 2
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    ///0x150 - RCC APB2 clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x154 - RCC APB4 clock enable register
    #[inline(always)]
    pub const fn apb4enr(&self) -> &APB4ENR {
        &self.apb4enr
    }
    ///0x158 - RCC AHB3 clock enable register
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    ///0x15c - RCC AHB5 low-power clock enable register
    #[inline(always)]
    pub const fn ahb5lpenr(&self) -> &AHB5LPENR {
        &self.ahb5lpenr
    }
    ///0x160 - RCC AHB1 low-power clock enable register
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    ///0x164 - RCC AHB2 low-power clock enable register
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    ///0x168 - RCC AHB4 low-power clock enable register
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    ///0x16c - RCC AHB3 low-power clock enable register
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    ///0x170 - RCC APB1 low-power clock enable register 1
    #[inline(always)]
    pub const fn apb1lpenr1(&self) -> &APB1LPENR1 {
        &self.apb1lpenr1
    }
    ///0x174 - RCC APB1 low-power clock enable register 2
    #[inline(always)]
    pub const fn apb1lpenr2(&self) -> &APB1LPENR2 {
        &self.apb1lpenr2
    }
    ///0x178 - RCC APB2 low-power clock enable register
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    ///0x17c - RCC APB4 low-power clock enable register
    #[inline(always)]
    pub const fn apb4lpenr(&self) -> &APB4LPENR {
        &self.apb4lpenr
    }
    ///0x180 - RCC APB5 low-power clock enable register
    #[inline(always)]
    pub const fn apb5lpenr(&self) -> &APB5LPENR {
        &self.apb5lpenr
    }
}
/**CR (rw) register accessor: RCC source control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC source control register
pub mod cr;
/**HSICFGR (rw) register accessor: RCC HSI calibration register

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:HSICFGR)

For information about available fields see [`mod@hsicfgr`] module*/
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
///RCC HSI calibration register
pub mod hsicfgr;
/**CRRCR (r) register accessor: RCC clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///RCC clock recovery RC register
pub mod crrcr;
/**CSICFGR (rw) register accessor: RCC CSI calibration register

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CSICFGR)

For information about available fields see [`mod@csicfgr`] module*/
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
///RCC CSI calibration register
pub mod csicfgr;
/**CFGR (rw) register accessor: RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///RCC clock configuration register
pub mod cfgr;
/**CDCFGR (rw) register accessor: RCC CPU domain clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cdcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CDCFGR)

For information about available fields see [`mod@cdcfgr`] module*/
pub type CDCFGR = crate::Reg<cdcfgr::CDCFGRrs>;
///RCC CPU domain clock configuration register
pub mod cdcfgr;
/**BMCFGR (rw) register accessor: RCC AHB clock configuration register

You can [`read`](crate::Reg::read) this register and get [`bmcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:BMCFGR)

For information about available fields see [`mod@bmcfgr`] module*/
pub type BMCFGR = crate::Reg<bmcfgr::BMCFGRrs>;
///RCC AHB clock configuration register
pub mod bmcfgr;
/**APBCFGR (rw) register accessor: RCC APB clocks configuration register

You can [`read`](crate::Reg::read) this register and get [`apbcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APBCFGR)

For information about available fields see [`mod@apbcfgr`] module*/
pub type APBCFGR = crate::Reg<apbcfgr::APBCFGRrs>;
///RCC APB clocks configuration register
pub mod apbcfgr;
/**PLLCKSELR (rw) register accessor: RCC PLLs clock source selection register

You can [`read`](crate::Reg::read) this register and get [`pllckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLLCKSELR)

For information about available fields see [`mod@pllckselr`] module*/
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELRrs>;
///RCC PLLs clock source selection register
pub mod pllckselr;
/**PLLCFGR (rw) register accessor: RCC PLLs configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///RCC PLLs configuration register
pub mod pllcfgr;
/**PLL1DIVR1 (rw) register accessor: RCC PLL1 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll1divr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1DIVR1)

For information about available fields see [`mod@pll1divr1`] module*/
pub type PLL1DIVR1 = crate::Reg<pll1divr1::PLL1DIVR1rs>;
///RCC PLL1 dividers configuration register 1
pub mod pll1divr1;
/**PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///RCC PLL1 fractional divider register
pub mod pll1fracr;
/**PLL2DIVR1 (rw) register accessor: RCC PLL2 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll2divr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2DIVR1)

For information about available fields see [`mod@pll2divr1`] module*/
pub type PLL2DIVR1 = crate::Reg<pll2divr1::PLL2DIVR1rs>;
///RCC PLL2 dividers configuration register 1
pub mod pll2divr1;
/**PLL2FRACR (rw) register accessor: RCC PLL2 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2FRACR)

For information about available fields see [`mod@pll2fracr`] module*/
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
///RCC PLL2 fractional divider register
pub mod pll2fracr;
/**PLL3DIVR1 (rw) register accessor: RCC PLL3 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll3divr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL3DIVR1)

For information about available fields see [`mod@pll3divr1`] module*/
pub type PLL3DIVR1 = crate::Reg<pll3divr1::PLL3DIVR1rs>;
///RCC PLL3 dividers configuration register 1
pub mod pll3divr1;
/**PLL3FRACR (rw) register accessor: RCC PLL3 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL3FRACR)

For information about available fields see [`mod@pll3fracr`] module*/
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
///RCC PLL3 fractional divider register
pub mod pll3fracr;
/**CCIPR1 (rw) register accessor: RCC AHB peripheral kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///RCC AHB peripheral kernel clock selection register
pub mod ccipr1;
/**CCIPR2 (rw) register accessor: RCC APB1 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///RCC APB1 peripherals kernel clock selection register
pub mod ccipr2;
/**CCIPR3 (rw) register accessor: RCC APB2 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR3)

For information about available fields see [`mod@ccipr3`] module*/
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
///RCC APB2 peripherals kernel clock selection register
pub mod ccipr3;
/**CCIPR4 (rw) register accessor: RCC APB4,5 peripherals kernel clock selection register

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CCIPR4)

For information about available fields see [`mod@ccipr4`] module*/
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4rs>;
///RCC APB4,5 peripherals kernel clock selection register
pub mod ccipr4;
/**CIER (rw) register accessor: RCC clock source interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock source interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock source interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock source interrupt flag register
pub mod cifr;
/**CICR (rw) register accessor: RCC clock source interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock source interrupt clear register
pub mod cicr;
/**BDCR (rw) register accessor: RCC Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RCC Backup domain control register
pub mod bdcr;
/**CSR (rw) register accessor: RCC clock control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///RCC clock control and status register
pub mod csr;
/**AHB5RSTR (rw) register accessor: RCC AHB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5RSTR)

For information about available fields see [`mod@ahb5rstr`] module*/
pub type AHB5RSTR = crate::Reg<ahb5rstr::AHB5RSTRrs>;
///RCC AHB5 peripheral reset register
pub mod ahb5rstr;
/**AHB1RSTR (rw) register accessor: RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///RCC AHB1 peripheral reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: RCC AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///RCC AHB2 peripheral reset register
pub mod ahb2rstr;
/**AHB4RSTR (rw) register accessor: RCC AHB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB4RSTR)

For information about available fields see [`mod@ahb4rstr`] module*/
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
///RCC AHB4 peripheral reset register
pub mod ahb4rstr;
/**APB5RSTR (rw) register accessor: RCC APB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb5rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB5RSTR)

For information about available fields see [`mod@apb5rstr`] module*/
pub type APB5RSTR = crate::Reg<apb5rstr::APB5RSTRrs>;
///RCC APB5 peripheral reset register
pub mod apb5rstr;
/**APB1RSTR1 (rw) register accessor: RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1RSTR1)

For information about available fields see [`mod@apb1rstr1`] module*/
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
///RCC APB1 peripheral reset register 1
pub mod apb1rstr1;
/**APB1RSTR2 (rw) register accessor: RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1RSTR2)

For information about available fields see [`mod@apb1rstr2`] module*/
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
///RCC APB1 peripheral reset register 2
pub mod apb1rstr2;
/**APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///RCC APB2 peripheral reset register
pub mod apb2rstr;
/**APB4RSTR (rw) register accessor: RCC APB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB4RSTR)

For information about available fields see [`mod@apb4rstr`] module*/
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTRrs>;
///RCC APB4 peripheral reset register
pub mod apb4rstr;
/**AHB3RSTR (rw) register accessor: RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///RCC AHB3 peripheral reset register
pub mod ahb3rstr;
/**CKGDISR (rw) register accessor: RCC AXI clocks gating disable register

You can [`read`](crate::Reg::read) this register and get [`ckgdisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgdisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CKGDISR)

For information about available fields see [`mod@ckgdisr`] module*/
pub type CKGDISR = crate::Reg<ckgdisr::CKGDISRrs>;
///RCC AXI clocks gating disable register
pub mod ckgdisr;
/**PLL1DIVR2 (rw) register accessor: RCC PLL1 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll1divr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1DIVR2)

For information about available fields see [`mod@pll1divr2`] module*/
pub type PLL1DIVR2 = crate::Reg<pll1divr2::PLL1DIVR2rs>;
///RCC PLL1 dividers configuration register 2
pub mod pll1divr2;
/**PLL2DIVR2 (rw) register accessor: RCC PLL2 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll2divr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2DIVR2)

For information about available fields see [`mod@pll2divr2`] module*/
pub type PLL2DIVR2 = crate::Reg<pll2divr2::PLL2DIVR2rs>;
///RCC PLL2 dividers configuration register 2
pub mod pll2divr2;
/**PLL3DIVR2 (rw) register accessor: RCC PLL3 dividers configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll3divr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL3DIVR2)

For information about available fields see [`mod@pll3divr2`] module*/
pub type PLL3DIVR2 = crate::Reg<pll3divr2::PLL3DIVR2rs>;
///RCC PLL3 dividers configuration register 2
pub mod pll3divr2;
/**PLL1SSCGR (rw) register accessor: RCC PLL1 Spread Spectrum Clock Generator register

You can [`read`](crate::Reg::read) this register and get [`pll1sscgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1sscgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL1SSCGR)

For information about available fields see [`mod@pll1sscgr`] module*/
pub type PLL1SSCGR = crate::Reg<pll1sscgr::PLL1SSCGRrs>;
///RCC PLL1 Spread Spectrum Clock Generator register
pub mod pll1sscgr;
/**PLL2SSCGR (rw) register accessor: RCC PLL2 Spread Spectrum Clock Generator register

You can [`read`](crate::Reg::read) this register and get [`pll2sscgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2sscgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2SSCGR)

For information about available fields see [`mod@pll2sscgr`] module*/
pub type PLL2SSCGR = crate::Reg<pll2sscgr::PLL2SSCGRrs>;
///RCC PLL2 Spread Spectrum Clock Generator register
pub mod pll2sscgr;
/**PLL3SSCGR (rw) register accessor: RCC PLL3 Spread Spectrum Clock Generator register

You can [`read`](crate::Reg::read) this register and get [`pll3sscgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3sscgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL3SSCGR)

For information about available fields see [`mod@pll3sscgr`] module*/
pub type PLL3SSCGR = crate::Reg<pll3sscgr::PLL3SSCGRrs>;
///RCC PLL3 Spread Spectrum Clock Generator register
pub mod pll3sscgr;
/**CKPROTR (rw) register accessor: RCC clock protection register

You can [`read`](crate::Reg::read) this register and get [`ckprotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckprotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:CKPROTR)

For information about available fields see [`mod@ckprotr`] module*/
pub type CKPROTR = crate::Reg<ckprotr::CKPROTRrs>;
///RCC clock protection register
pub mod ckprotr;
/**RSR (rw) register accessor: RCC Reset status register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:RSR)

For information about available fields see [`mod@rsr`] module*/
pub type RSR = crate::Reg<rsr::RSRrs>;
///RCC Reset status register
pub mod rsr;
/**AHB5ENR (rw) register accessor: RCC AHB5 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5ENR)

For information about available fields see [`mod@ahb5enr`] module*/
pub type AHB5ENR = crate::Reg<ahb5enr::AHB5ENRrs>;
///RCC AHB5 clock enable register
pub mod ahb5enr;
/**AHB1ENR (rw) register accessor: RCC AHB1 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///RCC AHB1 clock enable register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: RCC AHB2 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///RCC AHB2 clock enable register
pub mod ahb2enr;
/**AHB4ENR (rw) register accessor: RCC AHB4 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB4ENR)

For information about available fields see [`mod@ahb4enr`] module*/
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
///RCC AHB4 clock enable register
pub mod ahb4enr;
/**APB5ENR (rw) register accessor: RCC APB5 clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb5enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB5ENR)

For information about available fields see [`mod@apb5enr`] module*/
pub type APB5ENR = crate::Reg<apb5enr::APB5ENRrs>;
///RCC APB5 clock enable register
pub mod apb5enr;
/**APB1ENR1 (rw) register accessor: RCC APB1 clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1ENR1)

For information about available fields see [`mod@apb1enr1`] module*/
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
///RCC APB1 clock enable register 1
pub mod apb1enr1;
/**APB1ENR2 (rw) register accessor: RCC APB1 clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1ENR2)

For information about available fields see [`mod@apb1enr2`] module*/
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
///RCC APB1 clock enable register 2
pub mod apb1enr2;
/**APB2ENR (rw) register accessor: RCC APB2 clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///RCC APB2 clock enable register
pub mod apb2enr;
/**APB4ENR (rw) register accessor: RCC APB4 clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB4ENR)

For information about available fields see [`mod@apb4enr`] module*/
pub type APB4ENR = crate::Reg<apb4enr::APB4ENRrs>;
///RCC APB4 clock enable register
pub mod apb4enr;
/**AHB3ENR (rw) register accessor: RCC AHB3 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///RCC AHB3 clock enable register
pub mod ahb3enr;
/**AHB5LPENR (rw) register accessor: RCC AHB5 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB5LPENR)

For information about available fields see [`mod@ahb5lpenr`] module*/
pub type AHB5LPENR = crate::Reg<ahb5lpenr::AHB5LPENRrs>;
///RCC AHB5 low-power clock enable register
pub mod ahb5lpenr;
/**AHB1LPENR (rw) register accessor: RCC AHB1 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB1LPENR)

For information about available fields see [`mod@ahb1lpenr`] module*/
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
///RCC AHB1 low-power clock enable register
pub mod ahb1lpenr;
/**AHB2LPENR (rw) register accessor: RCC AHB2 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB2LPENR)

For information about available fields see [`mod@ahb2lpenr`] module*/
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
///RCC AHB2 low-power clock enable register
pub mod ahb2lpenr;
/**AHB4LPENR (rw) register accessor: RCC AHB4 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB4LPENR)

For information about available fields see [`mod@ahb4lpenr`] module*/
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
///RCC AHB4 low-power clock enable register
pub mod ahb4lpenr;
/**AHB3LPENR (rw) register accessor: RCC AHB3 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB3LPENR)

For information about available fields see [`mod@ahb3lpenr`] module*/
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
///RCC AHB3 low-power clock enable register
pub mod ahb3lpenr;
/**APB1LPENR1 (rw) register accessor: RCC APB1 low-power clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1LPENR1)

For information about available fields see [`mod@apb1lpenr1`] module*/
pub type APB1LPENR1 = crate::Reg<apb1lpenr1::APB1LPENR1rs>;
///RCC APB1 low-power clock enable register 1
pub mod apb1lpenr1;
/**APB1LPENR2 (rw) register accessor: RCC APB1 low-power clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB1LPENR2)

For information about available fields see [`mod@apb1lpenr2`] module*/
pub type APB1LPENR2 = crate::Reg<apb1lpenr2::APB1LPENR2rs>;
///RCC APB1 low-power clock enable register 2
pub mod apb1lpenr2;
/**APB2LPENR (rw) register accessor: RCC APB2 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB2LPENR)

For information about available fields see [`mod@apb2lpenr`] module*/
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
///RCC APB2 low-power clock enable register
pub mod apb2lpenr;
/**APB4LPENR (rw) register accessor: RCC APB4 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB4LPENR)

For information about available fields see [`mod@apb4lpenr`] module*/
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENRrs>;
///RCC APB4 low-power clock enable register
pub mod apb4lpenr;
/**APB5LPENR (rw) register accessor: RCC APB5 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb5lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APB5LPENR)

For information about available fields see [`mod@apb5lpenr`] module*/
pub type APB5LPENR = crate::Reg<apb5lpenr::APB5LPENRrs>;
///RCC APB5 low-power clock enable register
pub mod apb5lpenr;
