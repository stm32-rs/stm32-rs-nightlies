#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    comp2_csr: COMP2_CSR,
    _reserved1: [u8; 0x04],
    comp4_csr: COMP4_CSR,
    _reserved2: [u8; 0x04],
    comp6_csr: COMP6_CSR,
}
impl RegisterBlock {
    ///0x20 - control and status register
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &COMP2_CSR {
        &self.comp2_csr
    }
    ///0x28 - control and status register
    #[inline(always)]
    pub const fn comp4_csr(&self) -> &COMP4_CSR {
        &self.comp4_csr
    }
    ///0x30 - control and status register
    #[inline(always)]
    pub const fn comp6_csr(&self) -> &COMP6_CSR {
        &self.comp6_csr
    }
}
/**COMP2_CSR (rw) register accessor: control and status register

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#COMP:COMP2_CSR)

For information about available fields see [`mod@comp2_csr`] module*/
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSRrs>;
///control and status register
pub mod comp2_csr;
/**COMP4_CSR (rw) register accessor: control and status register

You can [`read`](crate::Reg::read) this register and get [`comp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#COMP:COMP4_CSR)

For information about available fields see [`mod@comp4_csr`] module*/
pub type COMP4_CSR = crate::Reg<comp4_csr::COMP4_CSRrs>;
///control and status register
pub mod comp4_csr;
/**COMP6_CSR (rw) register accessor: control and status register

You can [`read`](crate::Reg::read) this register and get [`comp6_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp6_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#COMP:COMP6_CSR)

For information about available fields see [`mod@comp6_csr`] module*/
pub type COMP6_CSR = crate::Reg<comp6_csr::COMP6_CSRrs>;
///control and status register
pub mod comp6_csr;
