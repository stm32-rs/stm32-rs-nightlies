#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rfip_version: RFIP_VERSION,
    rrm_udra_ctrl: RRM_UDRA_CTRL,
    sequencer_ctrl: SEQUENCER_CTRL,
    absolute_time: ABSOLUTE_TIME,
    scm_counter_val: SCM_COUNTER_VAL,
    scm_min_max: SCM_MIN_MAX,
    wakeup_irq_status: WAKEUP_IRQ_STATUS,
}
impl RegisterBlock {
    ///0x00 - RFIP_VERSION register
    #[inline(always)]
    pub const fn rfip_version(&self) -> &RFIP_VERSION {
        &self.rfip_version
    }
    ///0x04 - RRM_UDRA_CTRL register
    #[inline(always)]
    pub const fn rrm_udra_ctrl(&self) -> &RRM_UDRA_CTRL {
        &self.rrm_udra_ctrl
    }
    ///0x08 - SEQUENCER_CTRL register
    #[inline(always)]
    pub const fn sequencer_ctrl(&self) -> &SEQUENCER_CTRL {
        &self.sequencer_ctrl
    }
    ///0x0c - ABSOLUTE_TIME register
    #[inline(always)]
    pub const fn absolute_time(&self) -> &ABSOLUTE_TIME {
        &self.absolute_time
    }
    ///0x10 - SCM_COUNTER_VAL register
    #[inline(always)]
    pub const fn scm_counter_val(&self) -> &SCM_COUNTER_VAL {
        &self.scm_counter_val
    }
    ///0x14 - SCM_MIN_MAX register
    #[inline(always)]
    pub const fn scm_min_max(&self) -> &SCM_MIN_MAX {
        &self.scm_min_max
    }
    ///0x18 - WAKEUP_IRQ_STATUS register
    #[inline(always)]
    pub const fn wakeup_irq_status(&self) -> &WAKEUP_IRQ_STATUS {
        &self.wakeup_irq_status
    }
}
/**RFIP_VERSION (r) register accessor: RFIP_VERSION register

You can [`read`](crate::Reg::read) this register and get [`rfip_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:RFIP_VERSION)

For information about available fields see [`mod@rfip_version`] module*/
pub type RFIP_VERSION = crate::Reg<rfip_version::RFIP_VERSIONrs>;
///RFIP_VERSION register
pub mod rfip_version;
/**RRM_UDRA_CTRL (w) register accessor: RRM_UDRA_CTRL register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_udra_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:RRM_UDRA_CTRL)

For information about available fields see [`mod@rrm_udra_ctrl`] module*/
pub type RRM_UDRA_CTRL = crate::Reg<rrm_udra_ctrl::RRM_UDRA_CTRLrs>;
///RRM_UDRA_CTRL register
pub mod rrm_udra_ctrl;
/**SEQUENCER_CTRL (rw) register accessor: SEQUENCER_CTRL register

You can [`read`](crate::Reg::read) this register and get [`sequencer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sequencer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SEQUENCER_CTRL)

For information about available fields see [`mod@sequencer_ctrl`] module*/
pub type SEQUENCER_CTRL = crate::Reg<sequencer_ctrl::SEQUENCER_CTRLrs>;
///SEQUENCER_CTRL register
pub mod sequencer_ctrl;
/**ABSOLUTE_TIME (r) register accessor: ABSOLUTE_TIME register

You can [`read`](crate::Reg::read) this register and get [`absolute_time::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:ABSOLUTE_TIME)

For information about available fields see [`mod@absolute_time`] module*/
pub type ABSOLUTE_TIME = crate::Reg<absolute_time::ABSOLUTE_TIMErs>;
///ABSOLUTE_TIME register
pub mod absolute_time;
/**SCM_COUNTER_VAL (r) register accessor: SCM_COUNTER_VAL register

You can [`read`](crate::Reg::read) this register and get [`scm_counter_val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SCM_COUNTER_VAL)

For information about available fields see [`mod@scm_counter_val`] module*/
pub type SCM_COUNTER_VAL = crate::Reg<scm_counter_val::SCM_COUNTER_VALrs>;
///SCM_COUNTER_VAL register
pub mod scm_counter_val;
/**SCM_MIN_MAX (rw) register accessor: SCM_MIN_MAX register

You can [`read`](crate::Reg::read) this register and get [`scm_min_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm_min_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SCM_MIN_MAX)

For information about available fields see [`mod@scm_min_max`] module*/
pub type SCM_MIN_MAX = crate::Reg<scm_min_max::SCM_MIN_MAXrs>;
///SCM_MIN_MAX register
pub mod scm_min_max;
/**WAKEUP_IRQ_STATUS (rw) register accessor: WAKEUP_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`wakeup_irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:WAKEUP_IRQ_STATUS)

For information about available fields see [`mod@wakeup_irq_status`] module*/
pub type WAKEUP_IRQ_STATUS = crate::Reg<wakeup_irq_status::WAKEUP_IRQ_STATUSrs>;
///WAKEUP_IRQ_STATUS register
pub mod wakeup_irq_status;
