#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    clidr: CLIDR,
    ctr: CTR,
    ccsidr: CCSIDR,
}
impl RegisterBlock {
    ///0x00 - Cache Level ID register
    #[inline(always)]
    pub const fn clidr(&self) -> &CLIDR {
        &self.clidr
    }
    ///0x04 - Cache Type register
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    ///0x08 - Cache Size ID register
    #[inline(always)]
    pub const fn ccsidr(&self) -> &CCSIDR {
        &self.ccsidr
    }
}
/**CLIDR (r) register accessor: Cache Level ID register

You can [`read`](crate::Reg::read) this register and get [`clidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#PF:CLIDR)

For information about available fields see [`mod@clidr`]
module*/
pub type CLIDR = crate::Reg<clidr::CLIDRrs>;
///Cache Level ID register
pub mod clidr;
/**CTR (r) register accessor: Cache Type register

You can [`read`](crate::Reg::read) this register and get [`ctr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#PF:CTR)

For information about available fields see [`mod@ctr`]
module*/
pub type CTR = crate::Reg<ctr::CTRrs>;
///Cache Type register
pub mod ctr;
/**CCSIDR (r) register accessor: Cache Size ID register

You can [`read`](crate::Reg::read) this register and get [`ccsidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#PF:CCSIDR)

For information about available fields see [`mod@ccsidr`]
module*/
pub type CCSIDR = crate::Reg<ccsidr::CCSIDRrs>;
///Cache Size ID register
pub mod ccsidr;
