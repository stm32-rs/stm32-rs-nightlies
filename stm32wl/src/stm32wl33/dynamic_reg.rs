#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pcktlen_config: PCKTLEN_CONFIG,
    mod0_config: MOD0_CONFIG,
    mod1_config: MOD1_CONFIG,
    snyth_freq: SNYTH_FREQ,
    vco_cal_config: VCO_CAL_CONFIG,
    rx_timer: RX_TIMER,
    databuffer_thr: DATABUFFER_THR,
    rfseq_irq_enable: RFSEQ_IRQ_ENABLE,
    additional_ctrl: ADDITIONAL_CTRL,
    fast_rx_timer: FAST_RX_TIMER,
    command: COMMAND,
}
impl RegisterBlock {
    ///0x00 - PCKTLEN_CONFIG register
    #[inline(always)]
    pub const fn pcktlen_config(&self) -> &PCKTLEN_CONFIG {
        &self.pcktlen_config
    }
    ///0x04 - MOD0_CONFIG register
    #[inline(always)]
    pub const fn mod0_config(&self) -> &MOD0_CONFIG {
        &self.mod0_config
    }
    ///0x08 - MOD1_CONFIG register
    #[inline(always)]
    pub const fn mod1_config(&self) -> &MOD1_CONFIG {
        &self.mod1_config
    }
    ///0x0c - SNYTH_FREQ register
    #[inline(always)]
    pub const fn snyth_freq(&self) -> &SNYTH_FREQ {
        &self.snyth_freq
    }
    ///0x10 - VCO_CAL_CONFIG register
    #[inline(always)]
    pub const fn vco_cal_config(&self) -> &VCO_CAL_CONFIG {
        &self.vco_cal_config
    }
    ///0x14 - RX_TIMER register
    #[inline(always)]
    pub const fn rx_timer(&self) -> &RX_TIMER {
        &self.rx_timer
    }
    ///0x18 - DATABUFFER_THR register
    #[inline(always)]
    pub const fn databuffer_thr(&self) -> &DATABUFFER_THR {
        &self.databuffer_thr
    }
    ///0x1c - RFSEQ_IRQ_ENABLE register
    #[inline(always)]
    pub const fn rfseq_irq_enable(&self) -> &RFSEQ_IRQ_ENABLE {
        &self.rfseq_irq_enable
    }
    ///0x20 - ADDITIONAL_CTRL register
    #[inline(always)]
    pub const fn additional_ctrl(&self) -> &ADDITIONAL_CTRL {
        &self.additional_ctrl
    }
    ///0x24 - FAST_RX_TIMER register
    #[inline(always)]
    pub const fn fast_rx_timer(&self) -> &FAST_RX_TIMER {
        &self.fast_rx_timer
    }
    ///0x28 - COMMAND register
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
}
/**PCKTLEN_CONFIG (rw) register accessor: PCKTLEN_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`pcktlen_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcktlen_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:PCKTLEN_CONFIG)

For information about available fields see [`mod@pcktlen_config`] module*/
pub type PCKTLEN_CONFIG = crate::Reg<pcktlen_config::PCKTLEN_CONFIGrs>;
///PCKTLEN_CONFIG register
pub mod pcktlen_config;
/**MOD0_CONFIG (rw) register accessor: MOD0_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`mod0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:MOD0_CONFIG)

For information about available fields see [`mod@mod0_config`] module*/
pub type MOD0_CONFIG = crate::Reg<mod0_config::MOD0_CONFIGrs>;
///MOD0_CONFIG register
pub mod mod0_config;
/**MOD1_CONFIG (rw) register accessor: MOD1_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`mod1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:MOD1_CONFIG)

For information about available fields see [`mod@mod1_config`] module*/
pub type MOD1_CONFIG = crate::Reg<mod1_config::MOD1_CONFIGrs>;
///MOD1_CONFIG register
pub mod mod1_config;
/**SNYTH_FREQ (rw) register accessor: SNYTH_FREQ register

You can [`read`](crate::Reg::read) this register and get [`snyth_freq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snyth_freq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:SNYTH_FREQ)

For information about available fields see [`mod@snyth_freq`] module*/
pub type SNYTH_FREQ = crate::Reg<snyth_freq::SNYTH_FREQrs>;
///SNYTH_FREQ register
pub mod snyth_freq;
/**VCO_CAL_CONFIG (rw) register accessor: VCO_CAL_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`vco_cal_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco_cal_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:VCO_CAL_CONFIG)

For information about available fields see [`mod@vco_cal_config`] module*/
pub type VCO_CAL_CONFIG = crate::Reg<vco_cal_config::VCO_CAL_CONFIGrs>;
///VCO_CAL_CONFIG register
pub mod vco_cal_config;
/**RX_TIMER (rw) register accessor: RX_TIMER register

You can [`read`](crate::Reg::read) this register and get [`rx_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:RX_TIMER)

For information about available fields see [`mod@rx_timer`] module*/
pub type RX_TIMER = crate::Reg<rx_timer::RX_TIMERrs>;
///RX_TIMER register
pub mod rx_timer;
/**DATABUFFER_THR (rw) register accessor: DATABUFFER_THR register

You can [`read`](crate::Reg::read) this register and get [`databuffer_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:DATABUFFER_THR)

For information about available fields see [`mod@databuffer_thr`] module*/
pub type DATABUFFER_THR = crate::Reg<databuffer_thr::DATABUFFER_THRrs>;
///DATABUFFER_THR register
pub mod databuffer_thr;
/**RFSEQ_IRQ_ENABLE (rw) register accessor: RFSEQ_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`rfseq_irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:RFSEQ_IRQ_ENABLE)

For information about available fields see [`mod@rfseq_irq_enable`] module*/
pub type RFSEQ_IRQ_ENABLE = crate::Reg<rfseq_irq_enable::RFSEQ_IRQ_ENABLErs>;
///RFSEQ_IRQ_ENABLE register
pub mod rfseq_irq_enable;
/**ADDITIONAL_CTRL (rw) register accessor: ADDITIONAL_CTRL register

You can [`read`](crate::Reg::read) this register and get [`additional_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`additional_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:ADDITIONAL_CTRL)

For information about available fields see [`mod@additional_ctrl`] module*/
pub type ADDITIONAL_CTRL = crate::Reg<additional_ctrl::ADDITIONAL_CTRLrs>;
///ADDITIONAL_CTRL register
pub mod additional_ctrl;
/**FAST_RX_TIMER (rw) register accessor: FAST_RX_TIMER register

You can [`read`](crate::Reg::read) this register and get [`fast_rx_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_rx_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:FAST_RX_TIMER)

For information about available fields see [`mod@fast_rx_timer`] module*/
pub type FAST_RX_TIMER = crate::Reg<fast_rx_timer::FAST_RX_TIMERrs>;
///FAST_RX_TIMER register
pub mod fast_rx_timer;
/**COMMAND (rw) register accessor: COMMAND register

You can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:COMMAND)

For information about available fields see [`mod@command`] module*/
pub type COMMAND = crate::Reg<command::COMMANDrs>;
///COMMAND register
pub mod command;
