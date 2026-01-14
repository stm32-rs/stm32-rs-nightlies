#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dr: DR,
    idr: IDR,
    cr: CR,
    _reserved3: [u8; 0x04],
    init: INIT,
    pol: POL,
}
impl RegisterBlock {
    ///0x00 - CRC_DR register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x04 - CRC_IDR register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x08 - CRC_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - CRC_INIT register
    #[inline(always)]
    pub const fn init(&self) -> &INIT {
        &self.init
    }
    ///0x14 - CRC_POL register
    #[inline(always)]
    pub const fn pol(&self) -> &POL {
        &self.pol
    }
}
/**DR (rw) register accessor: CRC_DR register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///CRC_DR register
pub mod dr;
/**IDR (rw) register accessor: CRC_IDR register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///CRC_IDR register
pub mod idr;
/**CR (rw) register accessor: CRC_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CRC_CR register
pub mod cr;
/**INIT (rw) register accessor: CRC_INIT register

You can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:INIT)

For information about available fields see [`mod@init`] module*/
pub type INIT = crate::Reg<init::INITrs>;
///CRC_INIT register
pub mod init;
/**POL (rw) register accessor: CRC_POL register

You can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#CRC:POL)

For information about available fields see [`mod@pol`] module*/
pub type POL = crate::Reg<pol::POLrs>;
///CRC_POL register
pub mod pol;
