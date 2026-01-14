#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    pllcfgr: PLLCFGR,
    pllsai1cfgr: PLLSAI1CFGR,
    pllsai2cfgr: PLLSAI2CFGR,
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved9: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb3rstr: AHB3RSTR,
    _reserved12: [u8; 0x04],
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    _reserved15: [u8; 0x04],
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb3enr: AHB3ENR,
    _reserved18: [u8; 0x04],
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    _reserved21: [u8; 0x04],
    ahb1smenr: AHB1SMENR,
    ahb2smenr: AHB2SMENR,
    ahb3smenr: AHB3SMENR,
    _reserved24: [u8; 0x04],
    apb1smenr1: APB1SMENR1,
    apb1smenr2: APB1SMENR2,
    apb2smenr: APB2SMENR,
    _reserved27: [u8; 0x04],
    ccipr1: CCIPR1,
    _reserved28: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    crrcr: CRRCR,
    ccipr2: CCIPR2,
    _reserved32: [u8; 0x18],
    seccfgr: SECCFGR,
    secsr: SECSR,
    _reserved34: [u8; 0x28],
    ahb1secsr: AHB1SECSR,
    ahb2secsr: AHB2SECSR,
    ahb3secsr: AHB3SECSR,
    _reserved37: [u8; 0x04],
    apb1secsr1: APB1SECSR1,
    apb1secsr2: APB1SECSR2,
    apb2secsr: APB2SECSR,
}
impl RegisterBlock {
    ///0x00 - Clock control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - Internal clock sources calibration register
    #[inline(always)]
    pub const fn icscr(&self) -> &ICSCR {
        &self.icscr
    }
    ///0x08 - Clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x0c - PLL configuration register
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &PLLCFGR {
        &self.pllcfgr
    }
    ///0x10 - PLLSAI1 configuration register
    #[inline(always)]
    pub const fn pllsai1cfgr(&self) -> &PLLSAI1CFGR {
        &self.pllsai1cfgr
    }
    ///0x14 - PLLSAI2 configuration register
    #[inline(always)]
    pub const fn pllsai2cfgr(&self) -> &PLLSAI2CFGR {
        &self.pllsai2cfgr
    }
    ///0x18 - Clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x1c - Clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x20 - Clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x28 - AHB1 peripheral reset register
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x2c - AHB2 peripheral reset register
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    ///0x30 - AHB3 peripheral reset register
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    ///0x38 - APB1 peripheral reset register 1
    #[inline(always)]
    pub const fn apb1rstr1(&self) -> &APB1RSTR1 {
        &self.apb1rstr1
    }
    ///0x3c - APB1 peripheral reset register 2
    #[inline(always)]
    pub const fn apb1rstr2(&self) -> &APB1RSTR2 {
        &self.apb1rstr2
    }
    ///0x40 - APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x48 - AHB1 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x4c - AHB2 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x50 - AHB3 peripheral clock enable register
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    ///0x58 - APB1ENR1
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    ///0x5c - APB1 peripheral clock enable register 2
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    ///0x60 - APB2ENR
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &AHB1SMENR {
        &self.ahb1smenr
    }
    ///0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb2smenr(&self) -> &AHB2SMENR {
        &self.ahb2smenr
    }
    ///0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb3smenr(&self) -> &AHB3SMENR {
        &self.ahb3smenr
    }
    ///0x78 - APB1SMENR1
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &APB1SMENR1 {
        &self.apb1smenr1
    }
    ///0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &APB1SMENR2 {
        &self.apb1smenr2
    }
    ///0x80 - APB2SMENR
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    ///0x88 - CCIPR1
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0x90 - BDCR
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x94 - CSR
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x98 - Clock recovery RC register
    #[inline(always)]
    pub const fn crrcr(&self) -> &CRRCR {
        &self.crrcr
    }
    ///0x9c - Peripherals independent clock configuration register
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0xb8 - RCC secure configuration register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0xbc - RCC secure status register
    #[inline(always)]
    pub const fn secsr(&self) -> &SECSR {
        &self.secsr
    }
    ///0xe8 - RCC AHB1 security status register
    #[inline(always)]
    pub const fn ahb1secsr(&self) -> &AHB1SECSR {
        &self.ahb1secsr
    }
    ///0xec - RCC AHB2 security status register
    #[inline(always)]
    pub const fn ahb2secsr(&self) -> &AHB2SECSR {
        &self.ahb2secsr
    }
    ///0xf0 - RCC AHB3 security status register
    #[inline(always)]
    pub const fn ahb3secsr(&self) -> &AHB3SECSR {
        &self.ahb3secsr
    }
    ///0xf8 - RCC APB1 security status register 1
    #[inline(always)]
    pub const fn apb1secsr1(&self) -> &APB1SECSR1 {
        &self.apb1secsr1
    }
    ///0xfc - RCC APB1 security status register 2
    #[inline(always)]
    pub const fn apb1secsr2(&self) -> &APB1SECSR2 {
        &self.apb1secsr2
    }
    ///0x100 - RCC APB2 security status register
    #[inline(always)]
    pub const fn apb2secsr(&self) -> &APB2SECSR {
        &self.apb2secsr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**PLLCFGR (rw) register accessor: PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///PLL configuration register
pub mod pllcfgr;
/**PLLSAI1CFGR (rw) register accessor: PLLSAI1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:PLLSAI1CFGR)

For information about available fields see [`mod@pllsai1cfgr`] module*/
pub type PLLSAI1CFGR = crate::Reg<pllsai1cfgr::PLLSAI1CFGRrs>;
///PLLSAI1 configuration register
pub mod pllsai1cfgr;
/**PLLSAI2CFGR (rw) register accessor: PLLSAI2 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:PLLSAI2CFGR)

For information about available fields see [`mod@pllsai2cfgr`] module*/
pub type PLLSAI2CFGR = crate::Reg<pllsai2cfgr::PLLSAI2CFGRrs>;
///PLLSAI2 configuration register
pub mod pllsai2cfgr;
/**CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///Clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///Clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///Clock interrupt clear register
pub mod cicr;
/**AHB1RSTR (rw) register accessor: AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///AHB1 peripheral reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///AHB2 peripheral reset register
pub mod ahb2rstr;
/**AHB3RSTR (rw) register accessor: AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///AHB3 peripheral reset register
pub mod ahb3rstr;
/**APB1RSTR1 (rw) register accessor: APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1RSTR1)

For information about available fields see [`mod@apb1rstr1`] module*/
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
///APB1 peripheral reset register 1
pub mod apb1rstr1;
/**APB1RSTR2 (rw) register accessor: APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1RSTR2)

For information about available fields see [`mod@apb1rstr2`] module*/
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
///APB1 peripheral reset register 2
pub mod apb1rstr2;
/**APB2RSTR (rw) register accessor: APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///APB2 peripheral reset register
pub mod apb2rstr;
/**AHB1ENR (rw) register accessor: AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///AHB1 peripheral clock enable register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///AHB2 peripheral clock enable register
pub mod ahb2enr;
/**AHB3ENR (rw) register accessor: AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///AHB3 peripheral clock enable register
pub mod ahb3enr;
/**APB1ENR1 (rw) register accessor: APB1ENR1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1ENR1)

For information about available fields see [`mod@apb1enr1`] module*/
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
///APB1ENR1
pub mod apb1enr1;
/**APB1ENR2 (rw) register accessor: APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1ENR2)

For information about available fields see [`mod@apb1enr2`] module*/
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
///APB1 peripheral clock enable register 2
pub mod apb1enr2;
/**APB2ENR (rw) register accessor: APB2ENR

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///APB2ENR
pub mod apb2enr;
/**AHB1SMENR (rw) register accessor: AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB1SMENR)

For information about available fields see [`mod@ahb1smenr`] module*/
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENRrs>;
///AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb1smenr;
/**AHB2SMENR (rw) register accessor: AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB2SMENR)

For information about available fields see [`mod@ahb2smenr`] module*/
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENRrs>;
///AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb2smenr;
/**AHB3SMENR (rw) register accessor: AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB3SMENR)

For information about available fields see [`mod@ahb3smenr`] module*/
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENRrs>;
///AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb3smenr;
/**APB1SMENR1 (rw) register accessor: APB1SMENR1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SMENR1)

For information about available fields see [`mod@apb1smenr1`] module*/
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1rs>;
///APB1SMENR1
pub mod apb1smenr1;
/**APB1SMENR2 (rw) register accessor: APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SMENR2)

For information about available fields see [`mod@apb1smenr2`] module*/
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2rs>;
///APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod apb1smenr2;
/**APB2SMENR (rw) register accessor: APB2SMENR

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB2SMENR)

For information about available fields see [`mod@apb2smenr`] module*/
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
///APB2SMENR
pub mod apb2smenr;
/**CCIPR1 (rw) register accessor: CCIPR1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///CCIPR1
pub mod ccipr1;
/**BDCR (rw) register accessor: BDCR

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///BDCR
pub mod bdcr;
/**CSR (rw) register accessor: CSR

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///CSR
pub mod csr;
/**CRRCR (rw) register accessor: Clock recovery RC register

You can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CRRCR)

For information about available fields see [`mod@crrcr`] module*/
pub type CRRCR = crate::Reg<crrcr::CRRCRrs>;
///Clock recovery RC register
pub mod crrcr;
/**CCIPR2 (rw) register accessor: Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///Peripherals independent clock configuration register
pub mod ccipr2;
/**SECCFGR (rw) register accessor: RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///RCC secure configuration register
pub mod seccfgr;
/**SECSR (rw) register accessor: RCC secure status register

You can [`read`](crate::Reg::read) this register and get [`secsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:SECSR)

For information about available fields see [`mod@secsr`] module*/
pub type SECSR = crate::Reg<secsr::SECSRrs>;
///RCC secure status register
pub mod secsr;
/**AHB1SECSR (r) register accessor: RCC AHB1 security status register

You can [`read`](crate::Reg::read) this register and get [`ahb1secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB1SECSR)

For information about available fields see [`mod@ahb1secsr`] module*/
pub type AHB1SECSR = crate::Reg<ahb1secsr::AHB1SECSRrs>;
///RCC AHB1 security status register
pub mod ahb1secsr;
/**AHB2SECSR (r) register accessor: RCC AHB2 security status register

You can [`read`](crate::Reg::read) this register and get [`ahb2secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB2SECSR)

For information about available fields see [`mod@ahb2secsr`] module*/
pub type AHB2SECSR = crate::Reg<ahb2secsr::AHB2SECSRrs>;
///RCC AHB2 security status register
pub mod ahb2secsr;
/**AHB3SECSR (r) register accessor: RCC AHB3 security status register

You can [`read`](crate::Reg::read) this register and get [`ahb3secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB3SECSR)

For information about available fields see [`mod@ahb3secsr`] module*/
pub type AHB3SECSR = crate::Reg<ahb3secsr::AHB3SECSRrs>;
///RCC AHB3 security status register
pub mod ahb3secsr;
/**APB1SECSR1 (r) register accessor: RCC APB1 security status register 1

You can [`read`](crate::Reg::read) this register and get [`apb1secsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SECSR1)

For information about available fields see [`mod@apb1secsr1`] module*/
pub type APB1SECSR1 = crate::Reg<apb1secsr1::APB1SECSR1rs>;
///RCC APB1 security status register 1
pub mod apb1secsr1;
/**APB1SECSR2 (r) register accessor: RCC APB1 security status register 2

You can [`read`](crate::Reg::read) this register and get [`apb1secsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SECSR2)

For information about available fields see [`mod@apb1secsr2`] module*/
pub type APB1SECSR2 = crate::Reg<apb1secsr2::APB1SECSR2rs>;
///RCC APB1 security status register 2
pub mod apb1secsr2;
/**APB2SECSR (r) register accessor: RCC APB2 security status register

You can [`read`](crate::Reg::read) this register and get [`apb2secsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB2SECSR)

For information about available fields see [`mod@apb2secsr`] module*/
pub type APB2SECSR = crate::Reg<apb2secsr::APB2SECSRrs>;
///RCC APB2 security status register
pub mod apb2secsr;
