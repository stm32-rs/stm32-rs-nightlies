#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    rfip_wakeuptime: RFIP_WAKEUPTIME,
    cpu_wakeuptime: CPU_WAKEUPTIME,
    wakeup_ctrl: WAKEUP_CTRL,
    rrm_cmdlist_ptr: RRM_CMDLIST_PTR,
    seq_globaltable_ptr: SEQ_GLOBALTABLE_PTR,
}
impl RegisterBlock {
    ///0x00 - RFIP_WAKEUPTIME register
    #[inline(always)]
    pub const fn rfip_wakeuptime(&self) -> &RFIP_WAKEUPTIME {
        &self.rfip_wakeuptime
    }
    ///0x04 - CPU_WAKEUPTIME register
    #[inline(always)]
    pub const fn cpu_wakeuptime(&self) -> &CPU_WAKEUPTIME {
        &self.cpu_wakeuptime
    }
    ///0x08 - WAKEUP_CTRL register
    #[inline(always)]
    pub const fn wakeup_ctrl(&self) -> &WAKEUP_CTRL {
        &self.wakeup_ctrl
    }
    ///0x0c - RRM_CMDLIST_PTR register
    #[inline(always)]
    pub const fn rrm_cmdlist_ptr(&self) -> &RRM_CMDLIST_PTR {
        &self.rrm_cmdlist_ptr
    }
    ///0x10 - SEQ_GLOBALTABLE_PTR register
    #[inline(always)]
    pub const fn seq_globaltable_ptr(&self) -> &SEQ_GLOBALTABLE_PTR {
        &self.seq_globaltable_ptr
    }
}
/**RFIP_WAKEUPTIME (r) register accessor: RFIP_WAKEUPTIME register

You can [`read`](crate::Reg::read) this register and get [`rfip_wakeuptime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:RFIP_WAKEUPTIME)

For information about available fields see [`mod@rfip_wakeuptime`] module*/
pub type RFIP_WAKEUPTIME = crate::Reg<rfip_wakeuptime::RFIP_WAKEUPTIMErs>;
///RFIP_WAKEUPTIME register
pub mod rfip_wakeuptime;
/**CPU_WAKEUPTIME (rw) register accessor: CPU_WAKEUPTIME register

You can [`read`](crate::Reg::read) this register and get [`cpu_wakeuptime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeuptime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:CPU_WAKEUPTIME)

For information about available fields see [`mod@cpu_wakeuptime`] module*/
pub type CPU_WAKEUPTIME = crate::Reg<cpu_wakeuptime::CPU_WAKEUPTIMErs>;
///CPU_WAKEUPTIME register
pub mod cpu_wakeuptime;
/**WAKEUP_CTRL (rw) register accessor: WAKEUP_CTRL register

You can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:WAKEUP_CTRL)

For information about available fields see [`mod@wakeup_ctrl`] module*/
pub type WAKEUP_CTRL = crate::Reg<wakeup_ctrl::WAKEUP_CTRLrs>;
///WAKEUP_CTRL register
pub mod wakeup_ctrl;
/**RRM_CMDLIST_PTR (rw) register accessor: RRM_CMDLIST_PTR register

You can [`read`](crate::Reg::read) this register and get [`rrm_cmdlist_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_cmdlist_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:RRM_CMDLIST_PTR)

For information about available fields see [`mod@rrm_cmdlist_ptr`] module*/
pub type RRM_CMDLIST_PTR = crate::Reg<rrm_cmdlist_ptr::RRM_CMDLIST_PTRrs>;
///RRM_CMDLIST_PTR register
pub mod rrm_cmdlist_ptr;
/**SEQ_GLOBALTABLE_PTR (rw) register accessor: SEQ_GLOBALTABLE_PTR register

You can [`read`](crate::Reg::read) this register and get [`seq_globaltable_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_globaltable_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RETAINED:SEQ_GLOBALTABLE_PTR)

For information about available fields see [`mod@seq_globaltable_ptr`] module*/
pub type SEQ_GLOBALTABLE_PTR = crate::Reg<seq_globaltable_ptr::SEQ_GLOBALTABLE_PTRrs>;
///SEQ_GLOBALTABLE_PTR register
pub mod seq_globaltable_ptr;
