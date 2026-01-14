#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    frame_config0: FRAME_CONFIG0,
    frame_config1: FRAME_CONFIG1,
    frame_sync_config: FRAME_SYNC_CONFIG,
    rfip_config: RFIP_CONFIG,
    rf_config: RF_CONFIG,
    agc_config: AGC_CONFIG,
    _reserved6: [u8; 0x04],
    payload_0: PAYLOAD_0,
    payload_1: PAYLOAD_1,
}
impl RegisterBlock {
    ///0x00 - FRAME_CONFIG0 register
    #[inline(always)]
    pub const fn frame_config0(&self) -> &FRAME_CONFIG0 {
        &self.frame_config0
    }
    ///0x04 - FRAME_CONFIG1 register
    #[inline(always)]
    pub const fn frame_config1(&self) -> &FRAME_CONFIG1 {
        &self.frame_config1
    }
    ///0x08 - FRAME_SYNC_CONFIG register
    #[inline(always)]
    pub const fn frame_sync_config(&self) -> &FRAME_SYNC_CONFIG {
        &self.frame_sync_config
    }
    ///0x0c - RFIP_CONFIG register
    #[inline(always)]
    pub const fn rfip_config(&self) -> &RFIP_CONFIG {
        &self.rfip_config
    }
    ///0x10 - RF_CONFIG register
    #[inline(always)]
    pub const fn rf_config(&self) -> &RF_CONFIG {
        &self.rf_config
    }
    ///0x14 - AGC_CONFIG register
    #[inline(always)]
    pub const fn agc_config(&self) -> &AGC_CONFIG {
        &self.agc_config
    }
    ///0x1c - PAYLOAD_0 register
    #[inline(always)]
    pub const fn payload_0(&self) -> &PAYLOAD_0 {
        &self.payload_0
    }
    ///0x20 - PAYLOAD_1 register
    #[inline(always)]
    pub const fn payload_1(&self) -> &PAYLOAD_1 {
        &self.payload_1
    }
}
/**FRAME_CONFIG0 (rw) register accessor: FRAME_CONFIG0 register

You can [`read`](crate::Reg::read) this register and get [`frame_config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_CONFIG0)

For information about available fields see [`mod@frame_config0`] module*/
pub type FRAME_CONFIG0 = crate::Reg<frame_config0::FRAME_CONFIG0rs>;
///FRAME_CONFIG0 register
pub mod frame_config0;
/**FRAME_CONFIG1 (rw) register accessor: FRAME_CONFIG1 register

You can [`read`](crate::Reg::read) this register and get [`frame_config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_CONFIG1)

For information about available fields see [`mod@frame_config1`] module*/
pub type FRAME_CONFIG1 = crate::Reg<frame_config1::FRAME_CONFIG1rs>;
///FRAME_CONFIG1 register
pub mod frame_config1;
/**FRAME_SYNC_CONFIG (rw) register accessor: FRAME_SYNC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`frame_sync_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_sync_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_SYNC_CONFIG)

For information about available fields see [`mod@frame_sync_config`] module*/
pub type FRAME_SYNC_CONFIG = crate::Reg<frame_sync_config::FRAME_SYNC_CONFIGrs>;
///FRAME_SYNC_CONFIG register
pub mod frame_sync_config;
/**RFIP_CONFIG (rw) register accessor: RFIP_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rfip_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfip_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:RFIP_CONFIG)

For information about available fields see [`mod@rfip_config`] module*/
pub type RFIP_CONFIG = crate::Reg<rfip_config::RFIP_CONFIGrs>;
///RFIP_CONFIG register
pub mod rfip_config;
/**RF_CONFIG (rw) register accessor: RF_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rf_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:RF_CONFIG)

For information about available fields see [`mod@rf_config`] module*/
pub type RF_CONFIG = crate::Reg<rf_config::RF_CONFIGrs>;
///RF_CONFIG register
pub mod rf_config;
/**AGC_CONFIG (rw) register accessor: AGC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`agc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:AGC_CONFIG)

For information about available fields see [`mod@agc_config`] module*/
pub type AGC_CONFIG = crate::Reg<agc_config::AGC_CONFIGrs>;
///AGC_CONFIG register
pub mod agc_config;
/**PAYLOAD_0 (r) register accessor: PAYLOAD_0 register

You can [`read`](crate::Reg::read) this register and get [`payload_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:PAYLOAD_0)

For information about available fields see [`mod@payload_0`] module*/
pub type PAYLOAD_0 = crate::Reg<payload_0::PAYLOAD_0rs>;
///PAYLOAD_0 register
pub mod payload_0;
/**PAYLOAD_1 (r) register accessor: PAYLOAD_1 register

You can [`read`](crate::Reg::read) this register and get [`payload_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:PAYLOAD_1)

For information about available fields see [`mod@payload_1`] module*/
pub type PAYLOAD_1 = crate::Reg<payload_1::PAYLOAD_1rs>;
///PAYLOAD_1 register
pub mod payload_1;
