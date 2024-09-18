#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cordic_csr: CORDIC_CSR,
    cordic_wdata: CORDIC_WDATA,
    cordic_rdata: CORDIC_RDATA,
}
impl RegisterBlock {
    ///0x00 - CORDIC control/status register
    #[inline(always)]
    pub const fn cordic_csr(&self) -> &CORDIC_CSR {
        &self.cordic_csr
    }
    ///0x04 - CORDIC argument register
    #[inline(always)]
    pub const fn cordic_wdata(&self) -> &CORDIC_WDATA {
        &self.cordic_wdata
    }
    ///0x08 - CORDIC result register
    #[inline(always)]
    pub const fn cordic_rdata(&self) -> &CORDIC_RDATA {
        &self.cordic_rdata
    }
}
/**CORDIC_CSR (rw) register accessor: CORDIC control/status register

You can [`read`](crate::Reg::read) this register and get [`cordic_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#CORDIC:CORDIC_CSR)

For information about available fields see [`mod@cordic_csr`]
module*/
pub type CORDIC_CSR = crate::Reg<cordic_csr::CORDIC_CSRrs>;
///CORDIC control/status register
pub mod cordic_csr;
/**CORDIC_WDATA (w) register accessor: CORDIC argument register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#CORDIC:CORDIC_WDATA)

For information about available fields see [`mod@cordic_wdata`]
module*/
pub type CORDIC_WDATA = crate::Reg<cordic_wdata::CORDIC_WDATArs>;
///CORDIC argument register
pub mod cordic_wdata;
/**CORDIC_RDATA (r) register accessor: CORDIC result register

You can [`read`](crate::Reg::read) this register and get [`cordic_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#CORDIC:CORDIC_RDATA)

For information about available fields see [`mod@cordic_rdata`]
module*/
pub type CORDIC_RDATA = crate::Reg<cordic_rdata::CORDIC_RDATArs>;
///CORDIC result register
pub mod cordic_rdata;
