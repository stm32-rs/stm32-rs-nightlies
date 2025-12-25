#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x38],
    opamp1_csr: OPAMP1_CSR,
    opamp2_csr: OPAMP2_CSR,
    opamp3_csr: OPAMP3_CSR,
    opamp4_csr: OPAMP4_CSR,
}
impl RegisterBlock {
    ///0x38 - OPAMP1 control register
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    ///0x3c - OPAMP2 control register
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
    ///0x40 - OPAMP3 control register
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &OPAMP3_CSR {
        &self.opamp3_csr
    }
    ///0x44 - OPAMP4 control register
    #[inline(always)]
    pub const fn opamp4_csr(&self) -> &OPAMP4_CSR {
        &self.opamp4_csr
    }
}
/**OPAMP1_CSR (rw) register accessor: OPAMP1 control register

You can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#OPAMP:OPAMP1_CSR)

For information about available fields see [`mod@opamp1_csr`] module*/
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
///OPAMP1 control register
pub mod opamp1_csr;
/**OPAMP2_CSR (rw) register accessor: OPAMP2 control register

You can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#OPAMP:OPAMP2_CSR)

For information about available fields see [`mod@opamp2_csr`] module*/
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
///OPAMP2 control register
pub mod opamp2_csr;
/**OPAMP3_CSR (rw) register accessor: OPAMP3 control register

You can [`read`](crate::Reg::read) this register and get [`opamp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#OPAMP:OPAMP3_CSR)

For information about available fields see [`mod@opamp3_csr`] module*/
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSRrs>;
///OPAMP3 control register
pub mod opamp3_csr;
/**OPAMP4_CSR (rw) register accessor: OPAMP4 control register

You can [`read`](crate::Reg::read) this register and get [`opamp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#OPAMP:OPAMP4_CSR)

For information about available fields see [`mod@opamp4_csr`] module*/
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSRrs>;
///OPAMP4 control register
pub mod opamp4_csr;
