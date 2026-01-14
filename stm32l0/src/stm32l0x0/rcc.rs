#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    _reserved2: [u8; 0x04],
    cfgr: CFGR,
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    ioprstr: IOPRSTR,
    ahbrstr: AHBRSTR,
    apb2rstr: APB2RSTR,
    apb1rstr: APB1RSTR,
    iopenr: IOPENR,
    ahbenr: AHBENR,
    apb2enr: APB2ENR,
    apb1enr: APB1ENR,
    iopsmen: IOPSMEN,
    ahbsmenr: AHBSMENR,
    apb2smenr: APB2SMENR,
    apb1smenr: APB1SMENR,
    ccipr: CCIPR,
    csr: CSR,
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
    ///0x0c - Clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - Clock interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x14 - Clock interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x18 - Clock interrupt clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x1c - GPIO reset register
    #[inline(always)]
    pub const fn ioprstr(&self) -> &IOPRSTR {
        &self.ioprstr
    }
    ///0x20 - AHB peripheral reset register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x24 - APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x28 - APB1 peripheral reset register
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    ///0x2c - GPIO clock enable register
    #[inline(always)]
    pub const fn iopenr(&self) -> &IOPENR {
        &self.iopenr
    }
    ///0x30 - AHB peripheral clock enable register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x34 - APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x38 - APB1 peripheral clock enable register
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    ///0x3c - GPIO clock enable in sleep mode register
    #[inline(always)]
    pub const fn iopsmen(&self) -> &IOPSMEN {
        &self.iopsmen
    }
    ///0x40 - AHB peripheral clock enable in sleep mode register
    #[inline(always)]
    pub const fn ahbsmenr(&self) -> &AHBSMENR {
        &self.ahbsmenr
    }
    ///0x44 - APB2 peripheral clock enable in sleep mode register
    #[inline(always)]
    pub const fn apb2smenr(&self) -> &APB2SMENR {
        &self.apb2smenr
    }
    ///0x48 - APB1 peripheral clock enable in sleep mode register
    #[inline(always)]
    pub const fn apb1smenr(&self) -> &APB1SMENR {
        &self.apb1smenr
    }
    ///0x4c - Clock configuration register
    #[inline(always)]
    pub const fn ccipr(&self) -> &CCIPR {
        &self.ccipr
    }
    ///0x50 - Control and status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**CIER (rw) register accessor: Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///Clock interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///Clock interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: Clock interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///Clock interrupt clear register
pub mod cicr;
/**IOPRSTR (rw) register accessor: GPIO reset register

You can [`read`](crate::Reg::read) this register and get [`ioprstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:IOPRSTR)

For information about available fields see [`mod@ioprstr`] module*/
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTRrs>;
///GPIO reset register
pub mod ioprstr;
/**AHBRSTR (rw) register accessor: AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`] module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///AHB peripheral reset register
pub mod ahbrstr;
/**APB2RSTR (rw) register accessor: APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///APB2 peripheral reset register
pub mod apb2rstr;
/**APB1RSTR (rw) register accessor: APB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB1RSTR)

For information about available fields see [`mod@apb1rstr`] module*/
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
///APB1 peripheral reset register
pub mod apb1rstr;
/**IOPENR (rw) register accessor: GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:IOPENR)

For information about available fields see [`mod@iopenr`] module*/
pub type IOPENR = crate::Reg<iopenr::IOPENRrs>;
///GPIO clock enable register
pub mod iopenr;
/**AHBENR (rw) register accessor: AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`] module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///AHB peripheral clock enable register
pub mod ahbenr;
/**APB2ENR (rw) register accessor: APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///APB2 peripheral clock enable register
pub mod apb2enr;
/**APB1ENR (rw) register accessor: APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB1ENR)

For information about available fields see [`mod@apb1enr`] module*/
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
///APB1 peripheral clock enable register
pub mod apb1enr;
/**IOPSMEN (rw) register accessor: GPIO clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`iopsmen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:IOPSMEN)

For information about available fields see [`mod@iopsmen`] module*/
pub type IOPSMEN = crate::Reg<iopsmen::IOPSMENrs>;
///GPIO clock enable in sleep mode register
pub mod iopsmen;
/**AHBSMENR (rw) register accessor: AHB peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:AHBSMENR)

For information about available fields see [`mod@ahbsmenr`] module*/
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENRrs>;
///AHB peripheral clock enable in sleep mode register
pub mod ahbsmenr;
/**APB2SMENR (rw) register accessor: APB2 peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB2SMENR)

For information about available fields see [`mod@apb2smenr`] module*/
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENRrs>;
///APB2 peripheral clock enable in sleep mode register
pub mod apb2smenr;
/**APB1SMENR (rw) register accessor: APB1 peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB1SMENR)

For information about available fields see [`mod@apb1smenr`] module*/
pub type APB1SMENR = crate::Reg<apb1smenr::APB1SMENRrs>;
///APB1 peripheral clock enable in sleep mode register
pub mod apb1smenr;
/**CCIPR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CCIPR)

For information about available fields see [`mod@ccipr`] module*/
pub type CCIPR = crate::Reg<ccipr::CCIPRrs>;
///Clock configuration register
pub mod ccipr;
/**CSR (rw) register accessor: Control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Control and status register
pub mod csr;
