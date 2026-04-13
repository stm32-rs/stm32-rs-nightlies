#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp1_otr: OPAMP1_OTR,
    opamp1_hsotr: OPAMP1_HSOTR,
    _reserved3: [u8; 0x04],
    opamp2_csr: OPAMP2_CSR,
    opamp2_otr: OPAMP2_OTR,
    opamp2_hsotr: OPAMP2_HSOTR,
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
    pub const fn opamp1_hsotr(&self) -> &OPAMP1_HSOTR {
        &self.opamp1_hsotr
    }
    ///0x10 - OPAMP2 control/status register
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
    ///0x14 - OPAMP2 offset trimming register in normal mode
    #[inline(always)]
    pub const fn opamp2_otr(&self) -> &OPAMP2_OTR {
        &self.opamp2_otr
    }
    ///0x18 - OPAMP2 offset trimming register in low-power mode
    #[inline(always)]
    pub const fn opamp2_hsotr(&self) -> &OPAMP2_HSOTR {
        &self.opamp2_hsotr
    }
}
/**OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP1_CSR)

For information about available fields see [`mod@opamp1_csr`] module*/
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
///OPAMP1 control/status register
pub mod opamp1_csr;
/**OPAMP1_OTR (rw) register accessor: OPAMP1 offset trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP1_OTR)

For information about available fields see [`mod@opamp1_otr`] module*/
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTRrs>;
///OPAMP1 offset trimming register in normal mode
pub mod opamp1_otr;
/**OPAMP1_HSOTR (rw) register accessor: OPAMP1 offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP1_HSOTR)

For information about available fields see [`mod@opamp1_hsotr`] module*/
pub type OPAMP1_HSOTR = crate::Reg<opamp1_hsotr::OPAMP1_HSOTRrs>;
///OPAMP1 offset trimming register in low-power mode
pub mod opamp1_hsotr;
/**OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP2_CSR)

For information about available fields see [`mod@opamp2_csr`] module*/
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
///OPAMP2 control/status register
pub mod opamp2_csr;
/**OPAMP2_OTR (rw) register accessor: OPAMP2 offset trimming register in normal mode

You can [`read`](crate::Reg::read) this register and get [`opamp2_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP2_OTR)

For information about available fields see [`mod@opamp2_otr`] module*/
pub type OPAMP2_OTR = crate::Reg<opamp2_otr::OPAMP2_OTRrs>;
///OPAMP2 offset trimming register in normal mode
pub mod opamp2_otr;
/**OPAMP2_HSOTR (rw) register accessor: OPAMP2 offset trimming register in low-power mode

You can [`read`](crate::Reg::read) this register and get [`opamp2_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OPAMP:OPAMP2_HSOTR)

For information about available fields see [`mod@opamp2_hsotr`] module*/
pub type OPAMP2_HSOTR = crate::Reg<opamp2_hsotr::OPAMP2_HSOTRrs>;
///OPAMP2 offset trimming register in low-power mode
pub mod opamp2_hsotr;
