#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock calibration unit core release register"]
    pub fccan_ccu_crel: crate::Reg<fccan_ccu_crel::FCCAN_CCU_CREL_SPEC>,
    #[doc = "0x04 - Calibration configuration register"]
    pub fccan_ccu_ccfg: crate::Reg<fccan_ccu_ccfg::FCCAN_CCU_CCFG_SPEC>,
    #[doc = "0x08 - Calibration status register"]
    pub fccan_ccu_cstat: crate::Reg<fccan_ccu_cstat::FCCAN_CCU_CSTAT_SPEC>,
    #[doc = "0x0c - The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
    pub fccan_ccu_cwd: crate::Reg<fccan_ccu_cwd::FCCAN_CCU_CWD_SPEC>,
    #[doc = "0x10 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
    pub fccan_ccu_ir: crate::Reg<fccan_ccu_ir::FCCAN_CCU_IR_SPEC>,
    #[doc = "0x14 - The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
    pub fccan_ccu_ie: crate::Reg<fccan_ccu_ie::FCCAN_CCU_IE_SPEC>,
}
#[doc = "FCCAN_CCU_CREL register accessor: an alias for `Reg<FCCAN_CCU_CREL_SPEC>`"]
pub type FCCAN_CCU_CREL = crate::Reg<fccan_ccu_crel::FCCAN_CCU_CREL_SPEC>;
#[doc = "Clock calibration unit core release register"]
pub mod fccan_ccu_crel;
#[doc = "FCCAN_CCU_CCFG register accessor: an alias for `Reg<FCCAN_CCU_CCFG_SPEC>`"]
pub type FCCAN_CCU_CCFG = crate::Reg<fccan_ccu_ccfg::FCCAN_CCU_CCFG_SPEC>;
#[doc = "Calibration configuration register"]
pub mod fccan_ccu_ccfg;
#[doc = "FCCAN_CCU_CSTAT register accessor: an alias for `Reg<FCCAN_CCU_CSTAT_SPEC>`"]
pub type FCCAN_CCU_CSTAT = crate::Reg<fccan_ccu_cstat::FCCAN_CCU_CSTAT_SPEC>;
#[doc = "Calibration status register"]
pub mod fccan_ccu_cstat;
#[doc = "FCCAN_CCU_CWD register accessor: an alias for `Reg<FCCAN_CCU_CWD_SPEC>`"]
pub type FCCAN_CCU_CWD = crate::Reg<fccan_ccu_cwd::FCCAN_CCU_CWD_SPEC>;
#[doc = "The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
pub mod fccan_ccu_cwd;
#[doc = "FCCAN_CCU_IR register accessor: an alias for `Reg<FCCAN_CCU_IR_SPEC>`"]
pub type FCCAN_CCU_IR = crate::Reg<fccan_ccu_ir::FCCAN_CCU_IR_SPEC>;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
pub mod fccan_ccu_ir;
#[doc = "FCCAN_CCU_IE register accessor: an alias for `Reg<FCCAN_CCU_IE_SPEC>`"]
pub type FCCAN_CCU_IE = crate::Reg<fccan_ccu_ie::FCCAN_CCU_IE_SPEC>;
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
pub mod fccan_ccu_ie;
