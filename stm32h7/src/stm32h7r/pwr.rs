#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr1: CR1,
    sr1: SR1,
    csr1: CSR1,
    csr2: CSR2,
    csr3: CSR3,
    csr4: CSR4,
    _reserved6: [u8; 0x08],
    wkupcr: WKUPCR,
    wkupfr: WKUPFR,
    wkupepr: WKUPEPR,
    ucpdr: UCPDR,
    apcr: APCR,
    pucrn: PUCRN,
    pdcrn: PDCRN,
    pucro: PUCRO,
    pdcro: PDCRO,
    pdcrp: PDCRP,
    _reserved16: [u8; 0x08],
    pdr1: PDR1,
}
impl RegisterBlock {
    ///0x00 - PWR control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - PWR control status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x08 - PWR control status register 1
    #[inline(always)]
    pub const fn csr1(&self) -> &CSR1 {
        &self.csr1
    }
    ///0x0c - PWR control register 2
    #[inline(always)]
    pub const fn csr2(&self) -> &CSR2 {
        &self.csr2
    }
    ///0x10 - PWR CPU control register 3
    #[inline(always)]
    pub const fn csr3(&self) -> &CSR3 {
        &self.csr3
    }
    ///0x14 - PWR control status register 4
    #[inline(always)]
    pub const fn csr4(&self) -> &CSR4 {
        &self.csr4
    }
    ///0x20 - PWR wakeup clear register
    #[inline(always)]
    pub const fn wkupcr(&self) -> &WKUPCR {
        &self.wkupcr
    }
    ///0x24 - PWR wakeup flag register
    #[inline(always)]
    pub const fn wkupfr(&self) -> &WKUPFR {
        &self.wkupfr
    }
    ///0x28 - PWR wakeup enable and polarity register
    #[inline(always)]
    pub const fn wkupepr(&self) -> &WKUPEPR {
        &self.wkupepr
    }
    ///0x2c - PWR USB Type-C and Power Delivery register
    #[inline(always)]
    pub const fn ucpdr(&self) -> &UCPDR {
        &self.ucpdr
    }
    ///0x30 - PWR apply pull configuration register
    #[inline(always)]
    pub const fn apcr(&self) -> &APCR {
        &self.apcr
    }
    ///0x34 - PWR port N pull-up control register
    #[inline(always)]
    pub const fn pucrn(&self) -> &PUCRN {
        &self.pucrn
    }
    ///0x38 - PWR port N pull-down control register
    #[inline(always)]
    pub const fn pdcrn(&self) -> &PDCRN {
        &self.pdcrn
    }
    ///0x3c - PWR port O pull-up control register
    #[inline(always)]
    pub const fn pucro(&self) -> &PUCRO {
        &self.pucro
    }
    ///0x40 - PWR port O pull-down control register
    #[inline(always)]
    pub const fn pdcro(&self) -> &PDCRO {
        &self.pdcro
    }
    ///0x44 - PWR port P pull-down control register
    #[inline(always)]
    pub const fn pdcrp(&self) -> &PDCRP {
        &self.pdcrp
    }
    ///0x50 - PWR debug register 1
    #[inline(always)]
    pub const fn pdr1(&self) -> &PDR1 {
        &self.pdr1
    }
}
/**CR1 (rw) register accessor: PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///PWR control register 1
pub mod cr1;
/**SR1 (r) register accessor: PWR control status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///PWR control status register 1
pub mod sr1;
/**CSR1 (rw) register accessor: PWR control status register 1

You can [`read`](crate::Reg::read) this register and get [`csr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR1)

For information about available fields see [`mod@csr1`] module*/
pub type CSR1 = crate::Reg<csr1::CSR1rs>;
///PWR control status register 1
pub mod csr1;
/**CSR2 (rw) register accessor: PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`csr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR2)

For information about available fields see [`mod@csr2`] module*/
pub type CSR2 = crate::Reg<csr2::CSR2rs>;
///PWR control register 2
pub mod csr2;
/**CSR3 (rw) register accessor: PWR CPU control register 3

You can [`read`](crate::Reg::read) this register and get [`csr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR3)

For information about available fields see [`mod@csr3`] module*/
pub type CSR3 = crate::Reg<csr3::CSR3rs>;
///PWR CPU control register 3
pub mod csr3;
/**CSR4 (rw) register accessor: PWR control status register 4

You can [`read`](crate::Reg::read) this register and get [`csr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR4)

For information about available fields see [`mod@csr4`] module*/
pub type CSR4 = crate::Reg<csr4::CSR4rs>;
///PWR control status register 4
pub mod csr4;
/**WKUPCR (rw) register accessor: PWR wakeup clear register

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:WKUPCR)

For information about available fields see [`mod@wkupcr`] module*/
pub type WKUPCR = crate::Reg<wkupcr::WKUPCRrs>;
///PWR wakeup clear register
pub mod wkupcr;
/**WKUPFR (r) register accessor: PWR wakeup flag register

You can [`read`](crate::Reg::read) this register and get [`wkupfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:WKUPFR)

For information about available fields see [`mod@wkupfr`] module*/
pub type WKUPFR = crate::Reg<wkupfr::WKUPFRrs>;
///PWR wakeup flag register
pub mod wkupfr;
/**WKUPEPR (rw) register accessor: PWR wakeup enable and polarity register

You can [`read`](crate::Reg::read) this register and get [`wkupepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:WKUPEPR)

For information about available fields see [`mod@wkupepr`] module*/
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPRrs>;
///PWR wakeup enable and polarity register
pub mod wkupepr;
/**UCPDR (rw) register accessor: PWR USB Type-C and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:UCPDR)

For information about available fields see [`mod@ucpdr`] module*/
pub type UCPDR = crate::Reg<ucpdr::UCPDRrs>;
///PWR USB Type-C and Power Delivery register
pub mod ucpdr;
/**APCR (rw) register accessor: PWR apply pull configuration register

You can [`read`](crate::Reg::read) this register and get [`apcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:APCR)

For information about available fields see [`mod@apcr`] module*/
pub type APCR = crate::Reg<apcr::APCRrs>;
///PWR apply pull configuration register
pub mod apcr;
/**PUCRN (rw) register accessor: PWR port N pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PUCRN)

For information about available fields see [`mod@pucrn`] module*/
pub type PUCRN = crate::Reg<pucrn::PUCRNrs>;
///PWR port N pull-up control register
pub mod pucrn;
/**PDCRN (rw) register accessor: PWR port N pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRN)

For information about available fields see [`mod@pdcrn`] module*/
pub type PDCRN = crate::Reg<pdcrn::PDCRNrs>;
///PWR port N pull-down control register
pub mod pdcrn;
/**PUCRO (rw) register accessor: PWR port O pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PUCRO)

For information about available fields see [`mod@pucro`] module*/
pub type PUCRO = crate::Reg<pucro::PUCROrs>;
///PWR port O pull-up control register
pub mod pucro;
/**PDCRO (rw) register accessor: PWR port O pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRO)

For information about available fields see [`mod@pdcro`] module*/
pub type PDCRO = crate::Reg<pdcro::PDCROrs>;
///PWR port O pull-down control register
pub mod pdcro;
/**PDCRP (rw) register accessor: PWR port P pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRP)

For information about available fields see [`mod@pdcrp`] module*/
pub type PDCRP = crate::Reg<pdcrp::PDCRPrs>;
///PWR port P pull-down control register
pub mod pdcrp;
/**PDR1 (rw) register accessor: PWR debug register 1

You can [`read`](crate::Reg::read) this register and get [`pdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDR1)

For information about available fields see [`mod@pdr1`] module*/
pub type PDR1 = crate::Reg<pdr1::PDR1rs>;
///PWR debug register 1
pub mod pdr1;
