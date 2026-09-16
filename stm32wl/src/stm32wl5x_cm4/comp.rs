#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    comp1_csr: COMP1_CSR,
    comp2_csr: COMP2_CSR,
}
impl RegisterBlock {
    ///0x00 - COMP1_CSR
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &COMP1_CSR {
        &self.comp1_csr
    }
    ///0x04 - COMP2_CSR
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &COMP2_CSR {
        &self.comp2_csr
    }
}
/**COMP1_CSR (rw) register accessor: COMP1_CSR

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#COMP:COMP1_CSR)

For information about available fields see [`mod@comp1_csr`] module*/
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSRrs>;
///COMP1_CSR
pub mod comp1_csr;
/**COMP2_CSR (rw) register accessor: COMP2_CSR

You can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#COMP:COMP2_CSR)

For information about available fields see [`mod@comp2_csr`] module*/
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSRrs>;
///COMP2_CSR
pub mod comp2_csr;
