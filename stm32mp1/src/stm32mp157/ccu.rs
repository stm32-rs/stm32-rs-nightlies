#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fccan_ccu_crel: FCCAN_CCU_CREL,
    fccan_ccu_ccfg: FCCAN_CCU_CCFG,
    fccan_ccu_cstat: FCCAN_CCU_CSTAT,
    fccan_ccu_cwd: FCCAN_CCU_CWD,
    fccan_ccu_ir: FCCAN_CCU_IR,
    fccan_ccu_ie: FCCAN_CCU_IE,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock calibration unit core release register"]
    #[inline(always)]
    pub const fn fccan_ccu_crel(&self) -> &FCCAN_CCU_CREL {
        &self.fccan_ccu_crel
    }
    #[doc = "0x04 - Calibration configuration register"]
    #[inline(always)]
    pub const fn fccan_ccu_ccfg(&self) -> &FCCAN_CCU_CCFG {
        &self.fccan_ccu_ccfg
    }
    #[doc = "0x08 - Calibration status register"]
    #[inline(always)]
    pub const fn fccan_ccu_cstat(&self) -> &FCCAN_CCU_CSTAT {
        &self.fccan_ccu_cstat
    }
    #[doc = "0x0c - The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
    #[inline(always)]
    pub const fn fccan_ccu_cwd(&self) -> &FCCAN_CCU_CWD {
        &self.fccan_ccu_cwd
    }
    #[doc = "0x10 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
    #[inline(always)]
    pub const fn fccan_ccu_ir(&self) -> &FCCAN_CCU_IR {
        &self.fccan_ccu_ir
    }
    #[doc = "0x14 - The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
    #[inline(always)]
    pub const fn fccan_ccu_ie(&self) -> &FCCAN_CCU_IE {
        &self.fccan_ccu_ie
    }
}
#[doc = "FCCAN_CCU_CREL (r) register accessor: Clock calibration unit core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_crel`]
module"]
pub type FCCAN_CCU_CREL = crate::Reg<fccan_ccu_crel::FCCAN_CCU_CRELrs>;
#[doc = "Clock calibration unit core release register"]
pub mod fccan_ccu_crel;
#[doc = "FCCAN_CCU_CCFG (rw) register accessor: Calibration configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_ccfg`]
module"]
pub type FCCAN_CCU_CCFG = crate::Reg<fccan_ccu_ccfg::FCCAN_CCU_CCFGrs>;
#[doc = "Calibration configuration register"]
pub mod fccan_ccu_ccfg;
#[doc = "FCCAN_CCU_CSTAT (r) register accessor: Calibration status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_cstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_cstat`]
module"]
pub type FCCAN_CCU_CSTAT = crate::Reg<fccan_ccu_cstat::FCCAN_CCU_CSTATrs>;
#[doc = "Calibration status register"]
pub mod fccan_ccu_cstat;
#[doc = "FCCAN_CCU_CWD (rw) register accessor: The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_cwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_cwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_cwd`]
module"]
pub type FCCAN_CCU_CWD = crate::Reg<fccan_ccu_cwd::FCCAN_CCU_CWDrs>;
#[doc = "The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
pub mod fccan_ccu_cwd;
#[doc = "FCCAN_CCU_IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_ir`]
module"]
pub type FCCAN_CCU_IR = crate::Reg<fccan_ccu_ir::FCCAN_CCU_IRrs>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
pub mod fccan_ccu_ir;
#[doc = "FCCAN_CCU_IE (rw) register accessor: The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccan_ccu_ie`]
module"]
pub type FCCAN_CCU_IE = crate::Reg<fccan_ccu_ie::FCCAN_CCU_IErs>;
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
pub mod fccan_ccu_ie;
