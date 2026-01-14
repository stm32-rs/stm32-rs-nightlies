#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp1_otr: OPAMP1_OTR,
    opamp1_hsotr: OPAMP1_HSOTR,
    opamp_or: OPAMP_OR,
}
impl RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    ///0x04 - OPAMP1 trimming register in normal mode
    #[inline(always)]
    pub const fn opamp1_otr(&self) -> &OPAMP1_OTR {
        &self.opamp1_otr
    }
    ///0x08 - OPAMP1 trimming register in high-speed mode
    #[inline(always)]
    pub const fn opamp1_hsotr(&self) -> &OPAMP1_HSOTR {
        &self.opamp1_hsotr
    }
    ///0x0c - OPAMP option register
    #[inline(always)]
    pub const fn opamp_or(&self) -> &OPAMP_OR {
        &self.opamp_or
    }
}
/**OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP1_CSR)

For information about available fields see [`mod@opamp1_csr`] module*/
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
///OPAMP1 control/status register
pub mod opamp1_csr;
/**OPAMP1_OTR (rw) register accessor: OPAMP1 trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP1_OTR)

For information about available fields see [`mod@opamp1_otr`] module*/
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTRrs>;
///OPAMP1 trimming register in normal mode
pub mod opamp1_otr;
/**OPAMP1_HSOTR (rw) register accessor: OPAMP1 trimming register in high-speed mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP1_HSOTR)

For information about available fields see [`mod@opamp1_hsotr`] module*/
pub type OPAMP1_HSOTR = crate::Reg<opamp1_hsotr::OPAMP1_HSOTRrs>;
///OPAMP1 trimming register in high-speed mode
pub mod opamp1_hsotr;
/**OPAMP_OR (rw) register accessor: OPAMP option register

You can [`read`](crate::Reg::read) this register and get [`opamp_or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#OPAMP1:OPAMP_OR)

For information about available fields see [`mod@opamp_or`] module*/
pub type OPAMP_OR = crate::Reg<opamp_or::OPAMP_ORrs>;
///OPAMP option register
pub mod opamp_or;
