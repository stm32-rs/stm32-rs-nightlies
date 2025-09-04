#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    pllcfgr: PLLCFGR,
    _reserved4: [u8; 0x08],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved7: [u8; 0x04],
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb3rstr: AHB3RSTR,
    _reserved10: [u8; 0x04],
    apb1rstr1: APB1RSTR1,
    apb1rstr2: APB1RSTR2,
    apb2rstr: APB2RSTR,
    apb3rstr: APB3RSTR,
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb3enr: AHB3ENR,
    _reserved17: [u8; 0x04],
    apb1enr1: APB1ENR1,
    apb1enr2: APB1ENR2,
    apb2enr: APB2ENR,
    apb3enr: APB3ENR,
    ahb1smenr: AHB1SMENR,
    ahb2smenr: AHB2SMENR,
    ahb3smenr: AHB3SMENR,
    _reserved24: [u8; 0x04],
    apb1smenr1: APB1SMENR1,
    apb1smenr2: APB1SMENR2,
    apb2smenr: APB2SMENR,
    apb3smenr: APB3SMENR,
    ccipr: CCIPR,
    _reserved29: [u8; 0x04],
    bdcr: BDCR,
    csr: CSR,
    _reserved31: [u8; 0x70],
    extcfgr: EXTCFGR,
    _reserved32: [u8; 0x3c],
    c2ahb1enr: C2AHB1ENR,
    c2ahb2enr: C2AHB2ENR,
    c2ahb3enr: C2AHB3ENR,
    _reserved35: [u8; 0x04],
    c2apb1enr1: C2APB1ENR1,
    c2apb1enr2: C2APB1ENR2,
    c2apb2enr: C2APB2ENR,
    c2apb3enr: C2APB3ENR,
    c2ahb1smenr: C2AHB1SMENR,
    c2ahb2smenr: C2AHB2SMENR,
    c2ahb3smenr: C2AHB3SMENR,
    _reserved42: [u8; 0x04],
    c2apb1smenr1: C2APB1SMENR1,
    c2apb1smenr2: C2APB1SMENR2,
    c2apb2smenr: C2APB2SMENR,
    c2apb3smenr: C2APB3SMENR,
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
    ///0x44 - APB3 peripheral reset register
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &APB3RSTR {
        &self.apb3rstr
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
    ///0x58 - APB1 peripheral clock enable register 1
    #[inline(always)]
    pub const fn apb1enr1(&self) -> &APB1ENR1 {
        &self.apb1enr1
    }
    ///0x5c - APB1 peripheral clock enable register 2
    #[inline(always)]
    pub const fn apb1enr2(&self) -> &APB1ENR2 {
        &self.apb1enr2
    }
    ///0x60 - APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x64 - APB3 peripheral clock enable register
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    ///0x68 - AHB1 peripheral clocks enable in Sleep modes register
    #[inline(always)]
    pub const fn ahb1smenr(&self) -> &AHB1SMENR {
        &self.ahb1smenr
    }
    ///0x6c - AHB2 peripheral clocks enable in Sleep modes register
    #[inline(always)]
    pub const fn ahb2smenr(&self) -> &AHB2SMENR {
        &self.ahb2smenr
    }
    ///0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register
    #[inline(always)]
    pub const fn ahb3smenr(&self) -> &AHB3SMENR {
        &self.ahb3smenr
    }
    ///0x78 - APB1 peripheral clocks enable in Sleep mode register 1
    #[inline(always)]
    pub const fn apb1smenr1(&self) -> &APB1SMENR1 {
        &self.apb1smenr1
    }
    ///0x7c - APB1 peripheral clocks enable in Sleep mode register 2
    #[inline(always)]
    pub const fn apb1smenr2(&self) -> &APB1SMENR2 {
        &self.apb1smenr2
    }
    ///0x80 - APB2 peripheral clocks enable in Sleep mode register
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    ///0x84 - APB3 peripheral clock enable in Sleep mode register
    #[inline(always)]
    pub const fn apb3smenr(&self) -> &APB3SMENR {
        &self.apb3smenr
    }
    ///0x88 - Peripherals independent clock configuration register
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    ///0x90 - Backup domain control register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x94 - Control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x108 - Extended clock recovery register
    #[inline(always)]
    pub const fn extcfgr(&self) -> &EXTCFGR {
        &self.extcfgr
    }
    ///0x148 - CPU2 AHB1 peripheral clock enable register
    #[inline(always)]
    pub const fn c2ahb1enr(&self) -> &C2AHB1ENR {
        &self.c2ahb1enr
    }
    ///0x14c - CPU2 AHB2 peripheral clock enable register
    #[inline(always)]
    pub const fn c2ahb2enr(&self) -> &C2AHB2ENR {
        &self.c2ahb2enr
    }
    ///0x150 - CPU2 AHB3 peripheral clock enable register \[dual core device only\]
    #[inline(always)]
    pub const fn c2ahb3enr(&self) -> &C2AHB3ENR {
        &self.c2ahb3enr
    }
    ///0x158 - CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb1enr1(&self) -> &C2APB1ENR1 {
        &self.c2apb1enr1
    }
    ///0x15c - CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb1enr2(&self) -> &C2APB1ENR2 {
        &self.c2apb1enr2
    }
    ///0x160 - CPU2 APB2 peripheral clock enable register \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb2enr(&self) -> &C2APB2ENR {
        &self.c2apb2enr
    }
    ///0x164 - CPU2 APB3 peripheral clock enable register \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb3enr(&self) -> &C2APB3ENR {
        &self.c2apb3enr
    }
    ///0x168 - CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]
    #[inline(always)]
    pub const fn c2ahb1smenr(&self) -> &C2AHB1SMENR {
        &self.c2ahb1smenr
    }
    ///0x16c - CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]
    #[inline(always)]
    pub const fn c2ahb2smenr(&self) -> &C2AHB2SMENR {
        &self.c2ahb2smenr
    }
    ///0x170 - CPU2 AHB3 peripheral clocks enable in Sleep mode register \[dual core device only\]
    #[inline(always)]
    pub const fn c2ahb3smenr(&self) -> &C2AHB3SMENR {
        &self.c2ahb3smenr
    }
    ///0x178 - CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb1smenr1(&self) -> &C2APB1SMENR1 {
        &self.c2apb1smenr1
    }
    ///0x17c - CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb1smenr2(&self) -> &C2APB1SMENR2 {
        &self.c2apb1smenr2
    }
    ///0x180 - CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb2smenr(&self) -> &C2APB2SMENR {
        &self.c2apb2smenr
    }
    ///0x184 - CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]
    #[inline(always)]
    pub const fn c2apb3smenr(&self) -> &C2APB3SMENR {
        &self.c2apb3smenr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**PLLCFGR (rw) register accessor: PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:PLLCFGR)

For information about available fields see [`mod@pllcfgr`] module*/
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGRrs>;
///PLL configuration register
pub mod pllcfgr;
/**CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///Clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///Clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///Clock interrupt clear register
pub mod cicr;
/**AHB1RSTR (rw) register accessor: AHB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///AHB1 peripheral reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///AHB2 peripheral reset register
pub mod ahb2rstr;
/**AHB3RSTR (rw) register accessor: AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///AHB3 peripheral reset register
pub mod ahb3rstr;
/**APB1RSTR1 (rw) register accessor: APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1RSTR1)

For information about available fields see [`mod@apb1rstr1`] module*/
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1rs>;
///APB1 peripheral reset register 1
pub mod apb1rstr1;
/**APB1RSTR2 (rw) register accessor: APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1RSTR2)

For information about available fields see [`mod@apb1rstr2`] module*/
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2rs>;
///APB1 peripheral reset register 2
pub mod apb1rstr2;
/**APB2RSTR (rw) register accessor: APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///APB2 peripheral reset register
pub mod apb2rstr;
/**APB3RSTR (rw) register accessor: APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB3RSTR)

For information about available fields see [`mod@apb3rstr`] module*/
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTRrs>;
///APB3 peripheral reset register
pub mod apb3rstr;
/**AHB1ENR (rw) register accessor: AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///AHB1 peripheral clock enable register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///AHB2 peripheral clock enable register
pub mod ahb2enr;
/**AHB3ENR (rw) register accessor: AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///AHB3 peripheral clock enable register
pub mod ahb3enr;
/**APB1ENR1 (rw) register accessor: APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1ENR1)

For information about available fields see [`mod@apb1enr1`] module*/
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1rs>;
///APB1 peripheral clock enable register 1
pub mod apb1enr1;
/**APB1ENR2 (rw) register accessor: APB1 peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1ENR2)

For information about available fields see [`mod@apb1enr2`] module*/
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2rs>;
///APB1 peripheral clock enable register 2
pub mod apb1enr2;
/**APB2ENR (rw) register accessor: APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///APB2 peripheral clock enable register
pub mod apb2enr;
/**APB3ENR (rw) register accessor: APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB3ENR)

For information about available fields see [`mod@apb3enr`] module*/
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
///APB3 peripheral clock enable register
pub mod apb3enr;
/**AHB1SMENR (rw) register accessor: AHB1 peripheral clocks enable in Sleep modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB1SMENR)

For information about available fields see [`mod@ahb1smenr`] module*/
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENRrs>;
///AHB1 peripheral clocks enable in Sleep modes register
pub mod ahb1smenr;
/**AHB2SMENR (rw) register accessor: AHB2 peripheral clocks enable in Sleep modes register

You can [`read`](crate::Reg::read) this register and get [`ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB2SMENR)

For information about available fields see [`mod@ahb2smenr`] module*/
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENRrs>;
///AHB2 peripheral clocks enable in Sleep modes register
pub mod ahb2smenr;
/**AHB3SMENR (rw) register accessor: AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB3SMENR)

For information about available fields see [`mod@ahb3smenr`] module*/
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENRrs>;
///AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb3smenr;
/**APB1SMENR1 (rw) register accessor: APB1 peripheral clocks enable in Sleep mode register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1SMENR1)

For information about available fields see [`mod@apb1smenr1`] module*/
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1rs>;
///APB1 peripheral clocks enable in Sleep mode register 1
pub mod apb1smenr1;
/**APB1SMENR2 (rw) register accessor: APB1 peripheral clocks enable in Sleep mode register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB1SMENR2)

For information about available fields see [`mod@apb1smenr2`] module*/
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2rs>;
///APB1 peripheral clocks enable in Sleep mode register 2
pub mod apb1smenr2;
/**APB2SMENR (rw) register accessor: APB2 peripheral clocks enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB2SMENR)

For information about available fields see [`mod@apb2smenr`] module*/
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
///APB2 peripheral clocks enable in Sleep mode register
pub mod apb2smenr;
/**APB3SMENR (rw) register accessor: APB3 peripheral clock enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:APB3SMENR)

For information about available fields see [`mod@apb3smenr`] module*/
pub type APB3SMENR = crate::Reg<apb3smenr::APB3SMENRrs>;
///APB3 peripheral clock enable in Sleep mode register
pub mod apb3smenr;
/**CCIPR (rw) register accessor: Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CCIPR)

For information about available fields see [`mod@ccipr`] module*/
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
///Peripherals independent clock configuration register
pub mod ccipr;
/**BDCR (rw) register accessor: Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///Backup domain control register
pub mod bdcr;
/**CSR (rw) register accessor: Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Control/status register
pub mod csr;
/**EXTCFGR (rw) register accessor: Extended clock recovery register

You can [`read`](crate::Reg::read) this register and get [`extcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:EXTCFGR)

For information about available fields see [`mod@extcfgr`] module*/
pub type EXTCFGR = crate::Reg<extcfgr::EXTCFGRrs>;
///Extended clock recovery register
pub mod extcfgr;
/**C2AHB1ENR (rw) register accessor: CPU2 AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`c2ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB1ENR)

For information about available fields see [`mod@c2ahb1enr`] module*/
pub type C2AHB1ENR = crate::Reg<c2ahb1enr::C2AHB1ENRrs>;
///CPU2 AHB1 peripheral clock enable register
pub mod c2ahb1enr;
/**C2AHB2ENR (rw) register accessor: CPU2 AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`c2ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB2ENR)

For information about available fields see [`mod@c2ahb2enr`] module*/
pub type C2AHB2ENR = crate::Reg<c2ahb2enr::C2AHB2ENRrs>;
///CPU2 AHB2 peripheral clock enable register
pub mod c2ahb2enr;
/**C2AHB3ENR (rw) register accessor: CPU2 AHB3 peripheral clock enable register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB3ENR)

For information about available fields see [`mod@c2ahb3enr`] module*/
pub type C2AHB3ENR = crate::Reg<c2ahb3enr::C2AHB3ENRrs>;
///CPU2 AHB3 peripheral clock enable register \[dual core device only\]
pub mod c2ahb3enr;
/**C2APB1ENR1 (rw) register accessor: CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB1ENR1)

For information about available fields see [`mod@c2apb1enr1`] module*/
pub type C2APB1ENR1 = crate::Reg<c2apb1enr1::C2APB1ENR1rs>;
///CPU2 APB1 peripheral clock enable register 1 \[dual core device only\]
pub mod c2apb1enr1;
/**C2APB1ENR2 (rw) register accessor: CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB1ENR2)

For information about available fields see [`mod@c2apb1enr2`] module*/
pub type C2APB1ENR2 = crate::Reg<c2apb1enr2::C2APB1ENR2rs>;
///CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
pub mod c2apb1enr2;
/**C2APB2ENR (rw) register accessor: CPU2 APB2 peripheral clock enable register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB2ENR)

For information about available fields see [`mod@c2apb2enr`] module*/
pub type C2APB2ENR = crate::Reg<c2apb2enr::C2APB2ENRrs>;
///CPU2 APB2 peripheral clock enable register \[dual core device only\]
pub mod c2apb2enr;
/**C2APB3ENR (rw) register accessor: CPU2 APB3 peripheral clock enable register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB3ENR)

For information about available fields see [`mod@c2apb3enr`] module*/
pub type C2APB3ENR = crate::Reg<c2apb3enr::C2APB3ENRrs>;
///CPU2 APB3 peripheral clock enable register \[dual core device only\]
pub mod c2apb3enr;
/**C2AHB1SMENR (rw) register accessor: CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB1SMENR)

For information about available fields see [`mod@c2ahb1smenr`] module*/
pub type C2AHB1SMENR = crate::Reg<c2ahb1smenr::C2AHB1SMENRrs>;
///CPU2 AHB1 peripheral clocks enable in Sleep modes register \[dual core device only\]
pub mod c2ahb1smenr;
/**C2AHB2SMENR (rw) register accessor: CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB2SMENR)

For information about available fields see [`mod@c2ahb2smenr`] module*/
pub type C2AHB2SMENR = crate::Reg<c2ahb2smenr::C2AHB2SMENRrs>;
///CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]
pub mod c2ahb2smenr;
/**C2AHB3SMENR (rw) register accessor: CPU2 AHB3 peripheral clocks enable in Sleep mode register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB3SMENR)

For information about available fields see [`mod@c2ahb3smenr`] module*/
pub type C2AHB3SMENR = crate::Reg<c2ahb3smenr::C2AHB3SMENRrs>;
///CPU2 AHB3 peripheral clocks enable in Sleep mode register \[dual core device only\]
pub mod c2ahb3smenr;
/**C2APB1SMENR1 (rw) register accessor: CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB1SMENR1)

For information about available fields see [`mod@c2apb1smenr1`] module*/
pub type C2APB1SMENR1 = crate::Reg<c2apb1smenr1::C2APB1SMENR1rs>;
///CPU2 APB1 peripheral clocks enable in Sleep mode register 1 \[dual core device only\]
pub mod c2apb1smenr1;
/**C2APB1SMENR2 (rw) register accessor: CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB1SMENR2)

For information about available fields see [`mod@c2apb1smenr2`] module*/
pub type C2APB1SMENR2 = crate::Reg<c2apb1smenr2::C2APB1SMENR2rs>;
///CPU2 APB1 peripheral clocks enable in Sleep mode register 2 \[dual core device only\]
pub mod c2apb1smenr2;
/**C2APB2SMENR (rw) register accessor: CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB2SMENR)

For information about available fields see [`mod@c2apb2smenr`] module*/
pub type C2APB2SMENR = crate::Reg<c2apb2smenr::C2APB2SMENRrs>;
///CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]
pub mod c2apb2smenr;
/**C2APB3SMENR (rw) register accessor: CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB3SMENR)

For information about available fields see [`mod@c2apb3smenr`] module*/
pub type C2APB3SMENR = crate::Reg<c2apb3smenr::C2APB3SMENRrs>;
///CPU2 APB3 peripheral clock enable in Sleep mode register \[dual core device only\]
pub mod c2apb3smenr;
