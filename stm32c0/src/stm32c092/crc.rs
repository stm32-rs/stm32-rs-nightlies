#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crc_dr: CRC_DR,
    crc_idr: CRC_IDR,
    crc_cr: CRC_CR,
    _reserved3: [u8; 0x04],
    crc_init: CRC_INIT,
    crc_pol: CRC_POL,
}
impl RegisterBlock {
    ///0x00 - CRC data register
    #[inline(always)]
    pub const fn crc_dr(&self) -> &CRC_DR {
        &self.crc_dr
    }
    ///0x04 - CRC independent data register
    #[inline(always)]
    pub const fn crc_idr(&self) -> &CRC_IDR {
        &self.crc_idr
    }
    ///0x08 - CRC control register
    #[inline(always)]
    pub const fn crc_cr(&self) -> &CRC_CR {
        &self.crc_cr
    }
    ///0x10 - CRC initial value
    #[inline(always)]
    pub const fn crc_init(&self) -> &CRC_INIT {
        &self.crc_init
    }
    ///0x14 - CRC polynomial
    #[inline(always)]
    pub const fn crc_pol(&self) -> &CRC_POL {
        &self.crc_pol
    }
}
/**CRC_DR (rw) register accessor: CRC data register

You can [`read`](crate::Reg::read) this register and get [`crc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_DR)

For information about available fields see [`mod@crc_dr`] module*/
pub type CRC_DR = crate::Reg<crc_dr::CRC_DRrs>;
///CRC data register
pub mod crc_dr;
/**CRC_IDR (rw) register accessor: CRC independent data register

You can [`read`](crate::Reg::read) this register and get [`crc_idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_IDR)

For information about available fields see [`mod@crc_idr`] module*/
pub type CRC_IDR = crate::Reg<crc_idr::CRC_IDRrs>;
///CRC independent data register
pub mod crc_idr;
/**CRC_CR (rw) register accessor: CRC control register

You can [`read`](crate::Reg::read) this register and get [`crc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_CR)

For information about available fields see [`mod@crc_cr`] module*/
pub type CRC_CR = crate::Reg<crc_cr::CRC_CRrs>;
///CRC control register
pub mod crc_cr;
/**CRC_INIT (rw) register accessor: CRC initial value

You can [`read`](crate::Reg::read) this register and get [`crc_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_INIT)

For information about available fields see [`mod@crc_init`] module*/
pub type CRC_INIT = crate::Reg<crc_init::CRC_INITrs>;
///CRC initial value
pub mod crc_init;
/**CRC_POL (rw) register accessor: CRC polynomial

You can [`read`](crate::Reg::read) this register and get [`crc_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_POL)

For information about available fields see [`mod@crc_pol`] module*/
pub type CRC_POL = crate::Reg<crc_pol::CRC_POLrs>;
///CRC polynomial
pub mod crc_pol;
