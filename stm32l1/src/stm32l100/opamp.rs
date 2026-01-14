#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    otr: OTR,
    lpotr: LPOTR,
}
impl RegisterBlock {
    ///0x00 - control/status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x04 - offset trimming register for normal mode
    #[inline(always)]
    pub const fn otr(&self) -> &OTR {
        &self.otr
    }
    ///0x08 - OPAMP offset trimming register for low power mode
    #[inline(always)]
    pub const fn lpotr(&self) -> &LPOTR {
        &self.lpotr
    }
}
/**CSR (rw) register accessor: control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#OPAMP:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///control/status register
pub mod csr;
/**OTR (rw) register accessor: offset trimming register for normal mode

You can [`read`](crate::Reg::read) this register and get [`otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#OPAMP:OTR)

For information about available fields see [`mod@otr`] module*/
pub type OTR = crate::Reg<otr::OTRrs>;
///offset trimming register for normal mode
pub mod otr;
/**LPOTR (rw) register accessor: OPAMP offset trimming register for low power mode

You can [`read`](crate::Reg::read) this register and get [`lpotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#OPAMP:LPOTR)

For information about available fields see [`mod@lpotr`] module*/
pub type LPOTR = crate::Reg<lpotr::LPOTRrs>;
///OPAMP offset trimming register for low power mode
pub mod lpotr;
