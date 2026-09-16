#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    icscr3: ICSCR3,
    _reserved2: [u8; 0x08],
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    cfgr3: CFGR3,
    pll1cfgr: PLL1CFGR,
    _reserved6: [u8; 0x08],
    pll1divr: PLL1DIVR,
    pll1fracr: PLL1FRACR,
    _reserved8: [u8; 0x14],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved11: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    _reserved13: [u8; 0x04],
    ahb4rstr: AHB4RSTR,
    ahb5rstr: AHB5RSTR,
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    apb7rstr: APB7RSTR,
    _reserved19: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    _reserved21: [u8; 0x04],
    ahb4enr: AHB4ENR,
    ahb5enr: AHB5ENR,
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    apb7enr: APB7ENR,
    _reserved27: [u8; 0x04],
    ahb1smenr: AHB1SMENR,
    ahb2smenr: AHB2SMENR,
    _reserved29: [u8; 0x04],
    ahb4smenr: AHB4SMENR,
    ahb5smenr: AHB5SMENR,
    apb1smenr1: APB1SMENR1,
    apb1smenr2: APB1SMENR2,
    apb2smenr: APB2SMENR,
    apb7smenr: APB7SMENR,
    _reserved35: [u8; 0x0c],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    _reserved38: [u8; 0x04],
    bdcr1: BDCR1,
    csr: CSR,
    _reserved40: [u8; 0x18],
    seccfgr: SECCFGR,
    privcfgr: PRIVCFGR,
    _reserved42: [u8; 0xe8],
    cfgr4: CFGR4,
    _reserved43: [u8; 0x04],
    radioenr: RADIOENR,
    _reserved44: [u8; 0x04],
    ecscr1: ECSCR1,
}
impl RegisterBlock {
    ///0x00 - RCC clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - RCC internal clock sources calibration register 3
    #[inline(always)]
    pub const fn icscr3(&self) -> &ICSCR3 {
        &self.icscr3
    }
    ///0x1c - RCC clock configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x20 - RCC clock configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x24 - RCC clock configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    ///0x28 - RCC PLL1 configuration register
    #[inline(always)]
    pub const fn pll1cfgr(&self) -> &PLL1CFGR {
        &self.pll1cfgr
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
    ///0x50 - RCC clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x54 - RCC clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x58 - RCC clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x60 - RCC AHB1 peripheral reset register
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
    ///0x70 - RCC AHB5 peripheral reset register
    #[inline(always)]
    pub const fn ahb5rstr(&self) -> &AHB5RSTR {
        &self.ahb5rstr
    }
    ///0x74 - RCC APB1 peripheral reset register 1
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &APB1RSTR1 {
        &self.apb1rstr1
    }
    ///0x78 - RCC APB1 peripheral reset register 2
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &APB1RSTR2 {
        &self.apb1rstr2
    }
    ///0x7c - RCC APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x80 - RCC APB7 peripheral reset register
    #[inline(always)]
    pub const fn apb7rstr(&self) -> &APB7RSTR {
        &self.apb7rstr
    }
    ///0x88 - RCC AHB1 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x8c - RCC AHB2 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x94 - RCC AHB4 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    ///0x98 - RCC AHB5 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb5enr(&self) -> &AHB5ENR {
        &self.ahb5enr
    }
    ///0x9c - RCC APB1 peripheral clock enable register 1
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    ///0xa0 - RCC APB1 peripheral clock enable register 2
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    ///0xa4 - RCC APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0xa8 - RCC APB7 peripheral clock enable register
    #[inline(always)]
    pub const fn apb7enr(&self) -> &APB7ENR {
        &self.apb7enr
    }
    ///0xb0 - RCC AHB1 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &AHB1SMENR {
        &self.ahb1smenr
    }
    ///0xb4 - RCC AHB2 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb2smenr(&self) -> &AHB2SMENR {
        &self.ahb2smenr
    }
    ///0xbc - RCC AHB4 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb4smenr(&self) -> &AHB4SMENR {
        &self.ahb4smenr
    }
    ///0xc0 - RCC AHB5 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb5smenr(&self) -> &AHB5SMENR {
        &self.ahb5smenr
    }
    ///0xc4 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &APB1SMENR1 {
        &self.apb1smenr1
    }
    ///0xc8 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &APB1SMENR2 {
        &self.apb1smenr2
    }
    ///0xcc - RCC APB2 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    ///0xd0 - RCC APB7 peripheral clock enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn apb7smenr(&self) -> &APB7SMENR {
        &self.apb7smenr
    }
    ///0xe0 - RCC peripherals independent clock configuration register 1
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0xe4 - RCC peripherals independent clock configuration register 2
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0xe8 - RCC peripherals independent clock configuration register 3
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    ///0xf0 - RCC backup domain control register
    #[inline(always)]
    pub const fn bdcr1(&self) -> &BDCR1 {
        &self.bdcr1
    }
    ///0xf4 - RCC control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
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
    ///0x200 - RCC clock configuration register 2
    #[inline(always)]
    pub const fn cfgr4(&self) -> &CFGR4 {
        &self.cfgr4
    }
    ///0x208 - RCC RADIO peripheral clock enable register
    #[inline(always)]
    pub const fn radioenr(&self) -> &RADIOENR {
        &self.radioenr
    }
    ///0x210 - RCC external clock sources calibration register 1
    #[inline(always)]
    pub const fn ecscr1(&self) -> &ECSCR1 {
        &self.ecscr1
    }
}
/**CR (rw) register accessor: RCC clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC clock control register
pub mod cr;
/**ICSCR3 (rw) register accessor: RCC internal clock sources calibration register 3

You can [`read`](crate::Reg::read) this register and get [`icscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:ICSCR3)

For information about available fields see [`mod@icscr3`] module*/
pub type ICSCR3 = crate::Reg<icscr3::ICSCR3rs>;
///RCC internal clock sources calibration register 3
pub mod icscr3;
/**CFGR1 (rw) register accessor: RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///RCC clock configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///RCC clock configuration register 2
pub mod cfgr2;
/**CFGR3 (rw) register accessor: RCC clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CFGR3)

For information about available fields see [`mod@cfgr3`] module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///RCC clock configuration register 3
pub mod cfgr3;
/**PLL1CFGR (rw) register accessor: RCC PLL1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:PLL1CFGR)

For information about available fields see [`mod@pll1cfgr`] module*/
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGRrs>;
///RCC PLL1 configuration register
pub mod pll1cfgr;
/**PLL1DIVR (rw) register accessor: RCC PLL1 dividers register

You can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:PLL1DIVR)

For information about available fields see [`mod@pll1divr`] module*/
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVRrs>;
///RCC PLL1 dividers register
pub mod pll1divr;
/**PLL1FRACR (rw) register accessor: RCC PLL1 fractional divider register

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///RCC PLL1 fractional divider register
pub mod pll1fracr;
/**CIER (rw) register accessor: RCC clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: RCC clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock interrupt clear register
pub mod cicr;
/**AHB1RSTR (rw) register accessor: RCC AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///RCC AHB1 peripheral reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: RCC AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///RCC AHB2 peripheral reset register
pub mod ahb2rstr;
/**AHB4RSTR (rw) register accessor: RCC AHB4 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB4RSTR)

For information about available fields see [`mod@ahb4rstr`] module*/
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
///RCC AHB4 peripheral reset register
pub mod ahb4rstr;
/**AHB5RSTR (rw) register accessor: RCC AHB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB5RSTR)

For information about available fields see [`mod@ahb5rstr`] module*/
pub type AHB5RSTR = crate::Reg<ahb5rstr::AHB5RSTRrs>;
///RCC AHB5 peripheral reset register
pub mod ahb5rstr;
/**APB1RSTR1 (rw) register accessor: RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1RSTR1)

For information about available fields see [`mod@apb1rstr1`] module*/
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
///RCC APB1 peripheral reset register 1
pub mod apb1rstr1;
/**APB1RSTR2 (rw) register accessor: RCC APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1RSTR2)

For information about available fields see [`mod@apb1rstr2`] module*/
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
///RCC APB1 peripheral reset register 2
pub mod apb1rstr2;
/**APB2RSTR (rw) register accessor: RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///RCC APB2 peripheral reset register
pub mod apb2rstr;
/**APB7RSTR (rw) register accessor: RCC APB7 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb7rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB7RSTR)

For information about available fields see [`mod@apb7rstr`] module*/
pub type APB7RSTR = crate::Reg<apb7rstr::APB7RSTRrs>;
///RCC APB7 peripheral reset register
pub mod apb7rstr;
/**AHB1ENR (rw) register accessor: RCC AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///RCC AHB1 peripheral clock enable register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: RCC AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///RCC AHB2 peripheral clock enable register
pub mod ahb2enr;
/**AHB4ENR (rw) register accessor: RCC AHB4 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB4ENR)

For information about available fields see [`mod@ahb4enr`] module*/
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
///RCC AHB4 peripheral clock enable register
pub mod ahb4enr;
/**AHB5ENR (rw) register accessor: RCC AHB5 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB5ENR)

For information about available fields see [`mod@ahb5enr`] module*/
pub type AHB5ENR = crate::Reg<ahb5enr::AHB5ENRrs>;
///RCC AHB5 peripheral clock enable register
pub mod ahb5enr;
/**APB1ENR1 (rw) register accessor: RCC APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1ENR1)

For information about available fields see [`mod@apb1enr1`] module*/
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
///RCC APB1 peripheral clock enable register 1
pub mod apb1enr1;
/**APB1ENR2 (rw) register accessor: RCC APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1ENR2)

For information about available fields see [`mod@apb1enr2`] module*/
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
///RCC APB1 peripheral clock enable register 2
pub mod apb1enr2;
/**APB2ENR (rw) register accessor: RCC APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///RCC APB2 peripheral clock enable register
pub mod apb2enr;
/**APB7ENR (rw) register accessor: RCC APB7 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb7enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB7ENR)

For information about available fields see [`mod@apb7enr`] module*/
pub type APB7ENR = crate::Reg<apb7enr::APB7ENRrs>;
///RCC APB7 peripheral clock enable register
pub mod apb7enr;
/**AHB1SMENR (rw) register accessor: RCC AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB1SMENR)

For information about available fields see [`mod@ahb1smenr`] module*/
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENRrs>;
///RCC AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb1smenr;
/**AHB2SMENR (rw) register accessor: RCC AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB2SMENR)

For information about available fields see [`mod@ahb2smenr`] module*/
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENRrs>;
///RCC AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb2smenr;
/**AHB4SMENR (rw) register accessor: RCC AHB4 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb4smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB4SMENR)

For information about available fields see [`mod@ahb4smenr`] module*/
pub type AHB4SMENR = crate::Reg<ahb4smenr::AHB4SMENRrs>;
///RCC AHB4 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb4smenr;
/**AHB5SMENR (rw) register accessor: RCC AHB5 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb5smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB5SMENR)

For information about available fields see [`mod@ahb5smenr`] module*/
pub type AHB5SMENR = crate::Reg<ahb5smenr::AHB5SMENRrs>;
///RCC AHB5 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb5smenr;
/**APB1SMENR1 (rw) register accessor: RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1SMENR1)

For information about available fields see [`mod@apb1smenr1`] module*/
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1rs>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1
pub mod apb1smenr1;
/**APB1SMENR2 (rw) register accessor: RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1SMENR2)

For information about available fields see [`mod@apb1smenr2`] module*/
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2rs>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod apb1smenr2;
/**APB2SMENR (rw) register accessor: RCC APB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB2SMENR)

For information about available fields see [`mod@apb2smenr`] module*/
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
///RCC APB2 peripheral clocks enable in Sleep and Stop modes register
pub mod apb2smenr;
/**APB7SMENR (rw) register accessor: RCC APB7 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb7smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB7SMENR)

For information about available fields see [`mod@apb7smenr`] module*/
pub type APB7SMENR = crate::Reg<apb7smenr::APB7SMENRrs>;
///RCC APB7 peripheral clock enable in Sleep and Stop modes register
pub mod apb7smenr;
/**CCIPR1 (rw) register accessor: RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///RCC peripherals independent clock configuration register 1
pub mod ccipr1;
/**CCIPR2 (rw) register accessor: RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///RCC peripherals independent clock configuration register 2
pub mod ccipr2;
/**CCIPR3 (rw) register accessor: RCC peripherals independent clock configuration register 3

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CCIPR3)

For information about available fields see [`mod@ccipr3`] module*/
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
///RCC peripherals independent clock configuration register 3
pub mod ccipr3;
/**BDCR1 (rw) register accessor: RCC backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:BDCR1)

For information about available fields see [`mod@bdcr1`] module*/
pub type BDCR1 = crate::Reg<bdcr1::BDCR1rs>;
///RCC backup domain control register
pub mod bdcr1;
/**CSR (rw) register accessor: RCC control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///RCC control/status register
pub mod csr;
/**SECCFGR (rw) register accessor: RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///RCC secure configuration register
pub mod seccfgr;
/**PRIVCFGR (rw) register accessor: RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:PRIVCFGR)

For information about available fields see [`mod@privcfgr`] module*/
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGRrs>;
///RCC privilege configuration register
pub mod privcfgr;
/**CFGR4 (rw) register accessor: RCC clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CFGR4)

For information about available fields see [`mod@cfgr4`] module*/
pub type CFGR4 = crate::Reg<cfgr4::CFGR4rs>;
///RCC clock configuration register 2
pub mod cfgr4;
/**RADIOENR (rw) register accessor: RCC RADIO peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`radioenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:RADIOENR)

For information about available fields see [`mod@radioenr`] module*/
pub type RADIOENR = crate::Reg<radioenr::RADIOENRrs>;
///RCC RADIO peripheral clock enable register
pub mod radioenr;
/**ECSCR1 (rw) register accessor: RCC external clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`ecscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:ECSCR1)

For information about available fields see [`mod@ecscr1`] module*/
pub type ECSCR1 = crate::Reg<ecscr1::ECSCR1rs>;
///RCC external clock sources calibration register 1
pub mod ecscr1;
