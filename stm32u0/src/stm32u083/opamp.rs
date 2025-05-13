#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    opamp_csr: OPAMP_CSR,
    opamp_otr: OPAMP_OTR,
    opamp_lpotr: OPAMP_LPOTR,
}
impl RegisterBlock {
    ///0x00 - OPAMP control/status register
    #[inline(always)]
    pub const fn opamp_csr(&self) -> &OPAMP_CSR {
        &self.opamp_csr
    }
    ///0x04 - OPAMP offset trimming register in normal mode
    #[inline(always)]
    pub const fn opamp_otr(&self) -> &OPAMP_OTR {
        &self.opamp_otr
    }
    ///0x08 - OPAMP offset trimming register in low-power mode
    #[inline(always)]
    pub const fn opamp_lpotr(&self) -> &OPAMP_LPOTR {
        &self.opamp_lpotr
    }
}
/**OPAMP_CSR (rw) register accessor: OPAMP control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#OPAMP:OPAMP_CSR)

For information about available fields see [`mod@opamp_csr`] module*/
pub type OPAMP_CSR = crate::Reg<opamp_csr::OPAMP_CSRrs>;
///OPAMP control/status register
pub mod opamp_csr;
/**OPAMP_OTR (rw) register accessor: OPAMP offset trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#OPAMP:OPAMP_OTR)

For information about available fields see [`mod@opamp_otr`] module*/
pub type OPAMP_OTR = crate::Reg<opamp_otr::OPAMP_OTRrs>;
///OPAMP offset trimming register in normal mode
pub mod opamp_otr;
/**OPAMP_LPOTR (rw) register accessor: OPAMP offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp_lpotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_lpotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#OPAMP:OPAMP_LPOTR)

For information about available fields see [`mod@opamp_lpotr`] module*/
pub type OPAMP_LPOTR = crate::Reg<opamp_lpotr::OPAMP_LPOTRrs>;
///OPAMP offset trimming register in low-power mode
pub mod opamp_lpotr;
