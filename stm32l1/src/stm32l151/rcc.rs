#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    icscr: ICSCR,
    cfgr: CFGR,
    cir: CIR,
    ahbrstr: AHBRSTR,
    apb2rstr: APB2RSTR,
    apb1rstr: APB1RSTR,
    ahbenr: AHBENR,
    apb2enr: APB2ENR,
    apb1enr: APB1ENR,
    ahblpenr: AHBLPENR,
    apb2lpenr: APB2LPENR,
    apb1lpenr: APB1LPENR,
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
    ///0x08 - Clock configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x0c - Clock interrupt register
    #[inline(always)]
    pub const fn cir(&self) -> &CIR {
        &self.cir
    }
    ///0x10 - AHB peripheral reset register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x14 - APB2 peripheral reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x18 - APB1 peripheral reset register
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    ///0x1c - AHB peripheral clock enable register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x20 - APB2 peripheral clock enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x24 - APB1 peripheral clock enable register
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    ///0x28 - AHB peripheral clock enable in low power mode register
    #[inline(always)]
    pub const fn ahblpenr(&self) -> &AHBLPENR {
        &self.ahblpenr
    }
    ///0x2c - APB2 peripheral clock enable in low power mode register
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    ///0x30 - APB1 peripheral clock enable in low power mode register
    #[inline(always)]
    pub const fn apb1lpenr(&self) -> &APB1LPENR {
        &self.apb1lpenr
    }
    ///0x34 - Control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
}
/**CR (rw) register accessor: Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Clock control register
pub mod cr;
/**ICSCR (rw) register accessor: Internal clock sources calibration register

You can [`read`](crate::Reg::read) this register and get [`icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:ICSCR)

For information about available fields see [`mod@icscr`] module*/
pub type ICSCR = crate::Reg<icscr::ICSCRrs>;
///Internal clock sources calibration register
pub mod icscr;
/**CFGR (rw) register accessor: Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///Clock configuration register
pub mod cfgr;
/**CIR (rw) register accessor: Clock interrupt register

You can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:CIR)

For information about available fields see [`mod@cir`] module*/
pub type CIR = crate::Reg<cir::CIRrs>;
///Clock interrupt register
pub mod cir;
/**AHBRSTR (rw) register accessor: AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`] module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///AHB peripheral reset register
pub mod ahbrstr;
/**APB2RSTR (rw) register accessor: APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///APB2 peripheral reset register
pub mod apb2rstr;
/**APB1RSTR (rw) register accessor: APB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB1RSTR)

For information about available fields see [`mod@apb1rstr`] module*/
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
///APB1 peripheral reset register
pub mod apb1rstr;
/**AHBENR (rw) register accessor: AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`] module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///AHB peripheral clock enable register
pub mod ahbenr;
/**APB2ENR (rw) register accessor: APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///APB2 peripheral clock enable register
pub mod apb2enr;
/**APB1ENR (rw) register accessor: APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB1ENR)

For information about available fields see [`mod@apb1enr`] module*/
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
///APB1 peripheral clock enable register
pub mod apb1enr;
/**AHBLPENR (rw) register accessor: AHB peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahblpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:AHBLPENR)

For information about available fields see [`mod@ahblpenr`] module*/
pub type AHBLPENR = crate::Reg<ahblpenr::AHBLPENRrs>;
///AHB peripheral clock enable in low power mode register
pub mod ahblpenr;
/**APB2LPENR (rw) register accessor: APB2 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB2LPENR)

For information about available fields see [`mod@apb2lpenr`] module*/
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
///APB2 peripheral clock enable in low power mode register
pub mod apb2lpenr;
/**APB1LPENR (rw) register accessor: APB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB1LPENR)

For information about available fields see [`mod@apb1lpenr`] module*/
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENRrs>;
///APB1 peripheral clock enable in low power mode register
pub mod apb1lpenr;
/**CSR (rw) register accessor: Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///Control/status register
pub mod csr;
