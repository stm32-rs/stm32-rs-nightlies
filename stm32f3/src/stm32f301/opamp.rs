#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x3c],
    opamp2_csr: OPAMP2_CSR,
}
impl RegisterBlock {
    ///0x3c - OPAMP2 control register
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
}
/**OPAMP2_CSR (rw) register accessor: OPAMP2 control register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#OPAMP:OPAMP2_CSR)

For information about available fields see [`mod@opamp2_csr`] module*/
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
///OPAMP2 control register
pub mod opamp2_csr;
