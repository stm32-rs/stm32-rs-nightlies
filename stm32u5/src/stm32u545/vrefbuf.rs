#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    vrefbuf_csr: VREFBUF_CSR,
    vrefbuf_ccr: VREFBUF_CCR,
}
impl RegisterBlock {
    ///0x00 - VREFBUF control and status register
    #[inline(always)]
    pub const fn vrefbuf_csr(&self) -> &VREFBUF_CSR {
        &self.vrefbuf_csr
    }
    ///0x04 - VREFBUF calibration control register
    #[inline(always)]
    pub const fn vrefbuf_ccr(&self) -> &VREFBUF_CCR {
        &self.vrefbuf_ccr
    }
}
/**VREFBUF_CSR (rw) register accessor: VREFBUF control and status register

You can [`read`](crate::Reg::read) this register and get [`vrefbuf_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#VREFBUF:VREFBUF_CSR)

For information about available fields see [`mod@vrefbuf_csr`]
module*/
pub type VREFBUF_CSR = crate::Reg<vrefbuf_csr::VREFBUF_CSRrs>;
///VREFBUF control and status register
pub mod vrefbuf_csr;
/**VREFBUF_CCR (rw) register accessor: VREFBUF calibration control register

You can [`read`](crate::Reg::read) this register and get [`vrefbuf_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#VREFBUF:VREFBUF_CCR)

For information about available fields see [`mod@vrefbuf_ccr`]
module*/
pub type VREFBUF_CCR = crate::Reg<vrefbuf_ccr::VREFBUF_CCRrs>;
///VREFBUF calibration control register
pub mod vrefbuf_ccr;
