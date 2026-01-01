#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pckt_config: PCKT_CONFIG,
    sync: SYNC,
    sec_sync: SEC_SYNC,
    crc_init: CRC_INIT,
    pckt_ctrl: PCKT_CTRL,
    databuffer0_ptr: DATABUFFER0_PTR,
    databuffer1_ptr: DATABUFFER1_PTR,
    databuffer_size: DATABUFFER_SIZE,
    pa_level_3_0: PA_LEVEL_3_0,
    pa_level_7_4: PA_LEVEL_7_4,
    pa_config: PA_CONFIG,
    if_ctrl: IF_CTRL,
    as_qi_ctrl: AS_QI_CTRL,
    iqc_config: IQC_CONFIG,
    dsss_ctrl: DSSS_CTRL,
}
impl RegisterBlock {
    ///0x00 - PCKT_CONFIG register
    #[inline(always)]
    pub const fn pckt_config(&self) -> &PCKT_CONFIG {
        &self.pckt_config
    }
    ///0x04 - SYNC register
    #[inline(always)]
    pub const fn sync(&self) -> &SYNC {
        &self.sync
    }
    ///0x08 - SEC_SYNC register
    #[inline(always)]
    pub const fn sec_sync(&self) -> &SEC_SYNC {
        &self.sec_sync
    }
    ///0x0c - CRC_INIT register
    #[inline(always)]
    pub const fn crc_init(&self) -> &CRC_INIT {
        &self.crc_init
    }
    ///0x10 - PCKT_CTRL register
    #[inline(always)]
    pub const fn pckt_ctrl(&self) -> &PCKT_CTRL {
        &self.pckt_ctrl
    }
    ///0x14 - DATABUFFER0_PTR register
    #[inline(always)]
    pub const fn databuffer0_ptr(&self) -> &DATABUFFER0_PTR {
        &self.databuffer0_ptr
    }
    ///0x18 - DATABUFFER1_PTR register
    #[inline(always)]
    pub const fn databuffer1_ptr(&self) -> &DATABUFFER1_PTR {
        &self.databuffer1_ptr
    }
    ///0x1c - DATABUFFER_SIZE register
    #[inline(always)]
    pub const fn databuffer_size(&self) -> &DATABUFFER_SIZE {
        &self.databuffer_size
    }
    ///0x20 - PA_LEVEL_3_0 register
    #[inline(always)]
    pub const fn pa_level_3_0(&self) -> &PA_LEVEL_3_0 {
        &self.pa_level_3_0
    }
    ///0x24 - PA_LEVEL_7_4 register
    #[inline(always)]
    pub const fn pa_level_7_4(&self) -> &PA_LEVEL_7_4 {
        &self.pa_level_7_4
    }
    ///0x28 - PA_CONFIG register
    #[inline(always)]
    pub const fn pa_config(&self) -> &PA_CONFIG {
        &self.pa_config
    }
    ///0x2c - IF_CTRL register
    #[inline(always)]
    pub const fn if_ctrl(&self) -> &IF_CTRL {
        &self.if_ctrl
    }
    ///0x30 - AS_QI_CTRL register
    #[inline(always)]
    pub const fn as_qi_ctrl(&self) -> &AS_QI_CTRL {
        &self.as_qi_ctrl
    }
    ///0x34 - IQC_CONFIG register
    #[inline(always)]
    pub const fn iqc_config(&self) -> &IQC_CONFIG {
        &self.iqc_config
    }
    ///0x38 - DSSS_CTRL register
    #[inline(always)]
    pub const fn dsss_ctrl(&self) -> &DSSS_CTRL {
        &self.dsss_ctrl
    }
}
/**PCKT_CONFIG (rw) register accessor: PCKT_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pckt_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PCKT_CONFIG)

For information about available fields see [`mod@pckt_config`] module*/
pub type PCKT_CONFIG = crate::Reg<pckt_config::PCKT_CONFIGrs>;
///PCKT_CONFIG register
pub mod pckt_config;
/**SYNC (rw) register accessor: SYNC register

You can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:SYNC)

For information about available fields see [`mod@sync`] module*/
pub type SYNC = crate::Reg<sync::SYNCrs>;
///SYNC register
pub mod sync;
/**SEC_SYNC (rw) register accessor: SEC_SYNC register

You can [`read`](crate::Reg::read) this register and get [`sec_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:SEC_SYNC)

For information about available fields see [`mod@sec_sync`] module*/
pub type SEC_SYNC = crate::Reg<sec_sync::SEC_SYNCrs>;
///SEC_SYNC register
pub mod sec_sync;
/**CRC_INIT (rw) register accessor: CRC_INIT register

You can [`read`](crate::Reg::read) this register and get [`crc_init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:CRC_INIT)

For information about available fields see [`mod@crc_init`] module*/
pub type CRC_INIT = crate::Reg<crc_init::CRC_INITrs>;
///CRC_INIT register
pub mod crc_init;
/**PCKT_CTRL (rw) register accessor: PCKT_CTRL register

You can [`read`](crate::Reg::read) this register and get [`pckt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PCKT_CTRL)

For information about available fields see [`mod@pckt_ctrl`] module*/
pub type PCKT_CTRL = crate::Reg<pckt_ctrl::PCKT_CTRLrs>;
///PCKT_CTRL register
pub mod pckt_ctrl;
/**DATABUFFER0_PTR (rw) register accessor: DATABUFFER0_PTR register

You can [`read`](crate::Reg::read) this register and get [`databuffer0_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer0_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER0_PTR)

For information about available fields see [`mod@databuffer0_ptr`] module*/
pub type DATABUFFER0_PTR = crate::Reg<databuffer0_ptr::DATABUFFER0_PTRrs>;
///DATABUFFER0_PTR register
pub mod databuffer0_ptr;
/**DATABUFFER1_PTR (rw) register accessor: DATABUFFER1_PTR register

You can [`read`](crate::Reg::read) this register and get [`databuffer1_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer1_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER1_PTR)

For information about available fields see [`mod@databuffer1_ptr`] module*/
pub type DATABUFFER1_PTR = crate::Reg<databuffer1_ptr::DATABUFFER1_PTRrs>;
///DATABUFFER1_PTR register
pub mod databuffer1_ptr;
/**DATABUFFER_SIZE (rw) register accessor: DATABUFFER_SIZE register

You can [`read`](crate::Reg::read) this register and get [`databuffer_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DATABUFFER_SIZE)

For information about available fields see [`mod@databuffer_size`] module*/
pub type DATABUFFER_SIZE = crate::Reg<databuffer_size::DATABUFFER_SIZErs>;
///DATABUFFER_SIZE register
pub mod databuffer_size;
/**PA_LEVEL_3_0 (rw) register accessor: PA_LEVEL_3_0 register

You can [`read`](crate::Reg::read) this register and get [`pa_level_3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_LEVEL_3_0)

For information about available fields see [`mod@pa_level_3_0`] module*/
pub type PA_LEVEL_3_0 = crate::Reg<pa_level_3_0::PA_LEVEL_3_0rs>;
///PA_LEVEL_3_0 register
pub mod pa_level_3_0;
/**PA_LEVEL_7_4 (rw) register accessor: PA_LEVEL_7_4 register

You can [`read`](crate::Reg::read) this register and get [`pa_level_7_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_level_7_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_LEVEL_7_4)

For information about available fields see [`mod@pa_level_7_4`] module*/
pub type PA_LEVEL_7_4 = crate::Reg<pa_level_7_4::PA_LEVEL_7_4rs>;
///PA_LEVEL_7_4 register
pub mod pa_level_7_4;
/**PA_CONFIG (rw) register accessor: PA_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pa_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PA_CONFIG)

For information about available fields see [`mod@pa_config`] module*/
pub type PA_CONFIG = crate::Reg<pa_config::PA_CONFIGrs>;
///PA_CONFIG register
pub mod pa_config;
/**IF_CTRL (rw) register accessor: IF_CTRL register

You can [`read`](crate::Reg::read) this register and get [`if_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:IF_CTRL)

For information about available fields see [`mod@if_ctrl`] module*/
pub type IF_CTRL = crate::Reg<if_ctrl::IF_CTRLrs>;
///IF_CTRL register
pub mod if_ctrl;
/**AS_QI_CTRL (rw) register accessor: AS_QI_CTRL register

You can [`read`](crate::Reg::read) this register and get [`as_qi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`as_qi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:AS_QI_CTRL)

For information about available fields see [`mod@as_qi_ctrl`] module*/
pub type AS_QI_CTRL = crate::Reg<as_qi_ctrl::AS_QI_CTRLrs>;
///AS_QI_CTRL register
pub mod as_qi_ctrl;
/**IQC_CONFIG (rw) register accessor: IQC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`iqc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:IQC_CONFIG)

For information about available fields see [`mod@iqc_config`] module*/
pub type IQC_CONFIG = crate::Reg<iqc_config::IQC_CONFIGrs>;
///IQC_CONFIG register
pub mod iqc_config;
/**DSSS_CTRL (rw) register accessor: DSSS_CTRL register

You can [`read`](crate::Reg::read) this register and get [`dsss_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsss_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DSSS_CTRL)

For information about available fields see [`mod@dsss_ctrl`] module*/
pub type DSSS_CTRL = crate::Reg<dsss_ctrl::DSSS_CTRLrs>;
///DSSS_CTRL register
pub mod dsss_ctrl;
