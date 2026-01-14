#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    die_id: DIE_ID,
    jtag_id: JTAG_ID,
    i2c_fmp_ctrl: I2C_FMP_CTRL,
    io_dtr: IO_DTR,
    io_iber: IO_IBER,
    io_ievr: IO_IEVR,
    io_ier: IO_IER,
    io_iscr: IO_ISCR,
    pwrc_ier: PWRC_IER,
    pwrc_iscr: PWRC_ISCR,
    gpio_swa_ctrl: GPIO_SWA_CTRL,
    intai_dtr: INTAI_DTR,
    intai_iber: INTAI_IBER,
    intai_ievr: INTAI_IEVR,
    intai_ier: INTAI_IER,
    intai_iscr: INTAI_ISCR,
    syscfg_sr1: SYSCFG_SR1,
    rf_dtb_config: RF_DTB_CONFIG,
}
impl RegisterBlock {
    ///0x00 - DIE_ID register
    #[inline(always)]
    pub const fn die_id(&self) -> &DIE_ID {
        &self.die_id
    }
    ///0x04 - JTAG_ID register
    #[inline(always)]
    pub const fn jtag_id(&self) -> &JTAG_ID {
        &self.jtag_id
    }
    ///0x08 - I2C_FMP_CTRL register
    #[inline(always)]
    pub const fn i2c_fmp_ctrl(&self) -> &I2C_FMP_CTRL {
        &self.i2c_fmp_ctrl
    }
    ///0x0c - IO_DTR register
    #[inline(always)]
    pub const fn io_dtr(&self) -> &IO_DTR {
        &self.io_dtr
    }
    ///0x10 - IO_IBER register
    #[inline(always)]
    pub const fn io_iber(&self) -> &IO_IBER {
        &self.io_iber
    }
    ///0x14 - IO_IEVR register
    #[inline(always)]
    pub const fn io_ievr(&self) -> &IO_IEVR {
        &self.io_ievr
    }
    ///0x18 - IO_IER register
    #[inline(always)]
    pub const fn io_ier(&self) -> &IO_IER {
        &self.io_ier
    }
    ///0x1c - IO_ISCR register
    #[inline(always)]
    pub const fn io_iscr(&self) -> &IO_ISCR {
        &self.io_iscr
    }
    ///0x20 - PWRC_IER register
    #[inline(always)]
    pub const fn pwrc_ier(&self) -> &PWRC_IER {
        &self.pwrc_ier
    }
    ///0x24 - PWRC_ISCR register
    #[inline(always)]
    pub const fn pwrc_iscr(&self) -> &PWRC_ISCR {
        &self.pwrc_iscr
    }
    ///0x28 - GPIO_SWA_CTRL register
    #[inline(always)]
    pub const fn gpio_swa_ctrl(&self) -> &GPIO_SWA_CTRL {
        &self.gpio_swa_ctrl
    }
    ///0x2c - INTAI_DTR register
    #[inline(always)]
    pub const fn intai_dtr(&self) -> &INTAI_DTR {
        &self.intai_dtr
    }
    ///0x30 - INTAI_IBER register
    #[inline(always)]
    pub const fn intai_iber(&self) -> &INTAI_IBER {
        &self.intai_iber
    }
    ///0x34 - INTAI_IEVR register
    #[inline(always)]
    pub const fn intai_ievr(&self) -> &INTAI_IEVR {
        &self.intai_ievr
    }
    ///0x38 - INTAI_IER register
    #[inline(always)]
    pub const fn intai_ier(&self) -> &INTAI_IER {
        &self.intai_ier
    }
    ///0x3c - INTAI_ISCR register
    #[inline(always)]
    pub const fn intai_iscr(&self) -> &INTAI_ISCR {
        &self.intai_iscr
    }
    ///0x40 - SYSCFG_SR1 register
    #[inline(always)]
    pub const fn syscfg_sr1(&self) -> &SYSCFG_SR1 {
        &self.syscfg_sr1
    }
    ///0x44 - RF_DTB_CONFIG register
    #[inline(always)]
    pub const fn rf_dtb_config(&self) -> &RF_DTB_CONFIG {
        &self.rf_dtb_config
    }
}
/**DIE_ID (r) register accessor: DIE_ID register

You can [`read`](crate::Reg::read) this register and get [`die_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:DIE_ID)

For information about available fields see [`mod@die_id`] module*/
pub type DIE_ID = crate::Reg<die_id::DIE_IDrs>;
///DIE_ID register
pub mod die_id;
/**JTAG_ID (r) register accessor: JTAG_ID register

You can [`read`](crate::Reg::read) this register and get [`jtag_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:JTAG_ID)

For information about available fields see [`mod@jtag_id`] module*/
pub type JTAG_ID = crate::Reg<jtag_id::JTAG_IDrs>;
///JTAG_ID register
pub mod jtag_id;
/**I2C_FMP_CTRL (rw) register accessor: I2C_FMP_CTRL register

You can [`read`](crate::Reg::read) this register and get [`i2c_fmp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fmp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:I2C_FMP_CTRL)

For information about available fields see [`mod@i2c_fmp_ctrl`] module*/
pub type I2C_FMP_CTRL = crate::Reg<i2c_fmp_ctrl::I2C_FMP_CTRLrs>;
///I2C_FMP_CTRL register
pub mod i2c_fmp_ctrl;
/**IO_DTR (rw) register accessor: IO_DTR register

You can [`read`](crate::Reg::read) this register and get [`io_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:IO_DTR)

For information about available fields see [`mod@io_dtr`] module*/
pub type IO_DTR = crate::Reg<io_dtr::IO_DTRrs>;
///IO_DTR register
pub mod io_dtr;
/**IO_IBER (rw) register accessor: IO_IBER register

You can [`read`](crate::Reg::read) this register and get [`io_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:IO_IBER)

For information about available fields see [`mod@io_iber`] module*/
pub type IO_IBER = crate::Reg<io_iber::IO_IBERrs>;
///IO_IBER register
pub mod io_iber;
/**IO_IEVR (rw) register accessor: IO_IEVR register

You can [`read`](crate::Reg::read) this register and get [`io_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:IO_IEVR)

For information about available fields see [`mod@io_ievr`] module*/
pub type IO_IEVR = crate::Reg<io_ievr::IO_IEVRrs>;
///IO_IEVR register
pub mod io_ievr;
/**IO_IER (rw) register accessor: IO_IER register

You can [`read`](crate::Reg::read) this register and get [`io_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:IO_IER)

For information about available fields see [`mod@io_ier`] module*/
pub type IO_IER = crate::Reg<io_ier::IO_IERrs>;
///IO_IER register
pub mod io_ier;
/**IO_ISCR (rw) register accessor: IO_ISCR register

You can [`read`](crate::Reg::read) this register and get [`io_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:IO_ISCR)

For information about available fields see [`mod@io_iscr`] module*/
pub type IO_ISCR = crate::Reg<io_iscr::IO_ISCRrs>;
///IO_ISCR register
pub mod io_iscr;
/**PWRC_IER (rw) register accessor: PWRC_IER register

You can [`read`](crate::Reg::read) this register and get [`pwrc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:PWRC_IER)

For information about available fields see [`mod@pwrc_ier`] module*/
pub type PWRC_IER = crate::Reg<pwrc_ier::PWRC_IERrs>;
///PWRC_IER register
pub mod pwrc_ier;
/**PWRC_ISCR (rw) register accessor: PWRC_ISCR register

You can [`read`](crate::Reg::read) this register and get [`pwrc_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:PWRC_ISCR)

For information about available fields see [`mod@pwrc_iscr`] module*/
pub type PWRC_ISCR = crate::Reg<pwrc_iscr::PWRC_ISCRrs>;
///PWRC_ISCR register
pub mod pwrc_iscr;
/**GPIO_SWA_CTRL (rw) register accessor: GPIO_SWA_CTRL register

You can [`read`](crate::Reg::read) this register and get [`gpio_swa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_swa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:GPIO_SWA_CTRL)

For information about available fields see [`mod@gpio_swa_ctrl`] module*/
pub type GPIO_SWA_CTRL = crate::Reg<gpio_swa_ctrl::GPIO_SWA_CTRLrs>;
///GPIO_SWA_CTRL register
pub mod gpio_swa_ctrl;
/**INTAI_DTR (rw) register accessor: INTAI_DTR register

You can [`read`](crate::Reg::read) this register and get [`intai_dtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_dtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_DTR)

For information about available fields see [`mod@intai_dtr`] module*/
pub type INTAI_DTR = crate::Reg<intai_dtr::INTAI_DTRrs>;
///INTAI_DTR register
pub mod intai_dtr;
/**INTAI_IBER (rw) register accessor: INTAI_IBER register

You can [`read`](crate::Reg::read) this register and get [`intai_iber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IBER)

For information about available fields see [`mod@intai_iber`] module*/
pub type INTAI_IBER = crate::Reg<intai_iber::INTAI_IBERrs>;
///INTAI_IBER register
pub mod intai_iber;
/**INTAI_IEVR (rw) register accessor: INTAI_IEVR register

You can [`read`](crate::Reg::read) this register and get [`intai_ievr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ievr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IEVR)

For information about available fields see [`mod@intai_ievr`] module*/
pub type INTAI_IEVR = crate::Reg<intai_ievr::INTAI_IEVRrs>;
///INTAI_IEVR register
pub mod intai_ievr;
/**INTAI_IER (rw) register accessor: INTAI_IER register

You can [`read`](crate::Reg::read) this register and get [`intai_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IER)

For information about available fields see [`mod@intai_ier`] module*/
pub type INTAI_IER = crate::Reg<intai_ier::INTAI_IERrs>;
///INTAI_IER register
pub mod intai_ier;
/**INTAI_ISCR (rw) register accessor: INTAI_ISCR register

You can [`read`](crate::Reg::read) this register and get [`intai_iscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_ISCR)

For information about available fields see [`mod@intai_iscr`] module*/
pub type INTAI_ISCR = crate::Reg<intai_iscr::INTAI_ISCRrs>;
///INTAI_ISCR register
pub mod intai_iscr;
/**SYSCFG_SR1 (r) register accessor: SYSCFG_SR1 register

You can [`read`](crate::Reg::read) this register and get [`syscfg_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:SYSCFG_SR1)

For information about available fields see [`mod@syscfg_sr1`] module*/
pub type SYSCFG_SR1 = crate::Reg<syscfg_sr1::SYSCFG_SR1rs>;
///SYSCFG_SR1 register
pub mod syscfg_sr1;
/**RF_DTB_CONFIG (rw) register accessor: RF_DTB_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rf_dtb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_dtb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:RF_DTB_CONFIG)

For information about available fields see [`mod@rf_dtb_config`] module*/
pub type RF_DTB_CONFIG = crate::Reg<rf_dtb_config::RF_DTB_CONFIGrs>;
///RF_DTB_CONFIG register
pub mod rf_dtb_config;
