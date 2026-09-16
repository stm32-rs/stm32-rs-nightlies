#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp1_otr: OPAMP1_OTR,
    opamp1_lpotr: OPAMP1_LPOTR,
}
impl RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    ///0x04 - OPAMP1 offset trimming register in normal mode
    #[inline(always)]
    pub const fn opamp1_otr(&self) -> &OPAMP1_OTR {
        &self.opamp1_otr
    }
    ///0x08 - OPAMP1 offset trimming register in low-power mode
    #[inline(always)]
    pub const fn opamp1_lpotr(&self) -> &OPAMP1_LPOTR {
        &self.opamp1_lpotr
    }
}
/**OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#OPAMP:OPAMP1_CSR)

For information about available fields see [`mod@opamp1_csr`] module*/
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
///OPAMP1 control/status register
pub mod opamp1_csr;
/**OPAMP1_OTR (rw) register accessor: OPAMP1 offset trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#OPAMP:OPAMP1_OTR)

For information about available fields see [`mod@opamp1_otr`] module*/
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTRrs>;
///OPAMP1 offset trimming register in normal mode
pub mod opamp1_otr;
/**OPAMP1_LPOTR (rw) register accessor: OPAMP1 offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_lpotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_lpotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#OPAMP:OPAMP1_LPOTR)

For information about available fields see [`mod@opamp1_lpotr`] module*/
pub type OPAMP1_LPOTR = crate::Reg<opamp1_lpotr::OPAMP1_LPOTRrs>;
///OPAMP1 offset trimming register in low-power mode
pub mod opamp1_lpotr;
