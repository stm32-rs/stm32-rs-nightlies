#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csr: CSR,
    wdata: WDATA,
    rdata: RDATA,
}
impl RegisterBlock {
    ///0x00 - CORDIC Control Status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x04 - CORDIC argument register
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    ///0x08 - CORDIC result register
    #[inline(always)]
    pub const fn rdata(&self) -> &RDATA {
        &self.rdata
    }
}
/**CSR (rw) register accessor: CORDIC Control Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#CORDIC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///CORDIC Control Status register
pub mod csr;
/**WDATA (rw) register accessor: CORDIC argument register

You can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#CORDIC:WDATA)

For information about available fields see [`mod@wdata`] module*/
pub type WDATA = crate::Reg<wdata::WDATArs>;
///CORDIC argument register
pub mod wdata;
/**RDATA (r) register accessor: CORDIC result register

You can [`read`](crate::Reg::read) this register and get [`rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#CORDIC:RDATA)

For information about available fields see [`mod@rdata`] module*/
pub type RDATA = crate::Reg<rdata::RDATArs>;
///CORDIC result register
pub mod rdata;
