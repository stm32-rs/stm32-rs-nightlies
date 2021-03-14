#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock calibration unit core release register"]
    pub fccan_ccu_crel: FCCAN_CCU_CREL,
    #[doc = "0x04 - Calibration configuration register"]
    pub fccan_ccu_ccfg: FCCAN_CCU_CCFG,
    #[doc = "0x08 - Calibration status register"]
    pub fccan_ccu_cstat: FCCAN_CCU_CSTAT,
    #[doc = "0x0c - The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
    pub fccan_ccu_cwd: FCCAN_CCU_CWD,
    #[doc = "0x10 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
    pub fccan_ccu_ir: FCCAN_CCU_IR,
    #[doc = "0x14 - The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
    pub fccan_ccu_ie: FCCAN_CCU_IE,
}
#[doc = "Clock calibration unit core release register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_crel](fccan_ccu_crel) module"]
pub type FCCAN_CCU_CREL = crate::Reg<u32, _FCCAN_CCU_CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_CREL;
#[doc = "`read()` method returns [fccan_ccu_crel::R](fccan_ccu_crel::R) reader structure"]
impl crate::Readable for FCCAN_CCU_CREL {}
#[doc = "Clock calibration unit core release register"]
pub mod fccan_ccu_crel;
#[doc = "Calibration configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ccfg](fccan_ccu_ccfg) module"]
pub type FCCAN_CCU_CCFG = crate::Reg<u32, _FCCAN_CCU_CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_CCFG;
#[doc = "`read()` method returns [fccan_ccu_ccfg::R](fccan_ccu_ccfg::R) reader structure"]
impl crate::Readable for FCCAN_CCU_CCFG {}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ccfg::W](fccan_ccu_ccfg::W) writer structure"]
impl crate::Writable for FCCAN_CCU_CCFG {}
#[doc = "Calibration configuration register"]
pub mod fccan_ccu_ccfg;
#[doc = "Calibration status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_cstat](fccan_ccu_cstat) module"]
pub type FCCAN_CCU_CSTAT = crate::Reg<u32, _FCCAN_CCU_CSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_CSTAT;
#[doc = "`read()` method returns [fccan_ccu_cstat::R](fccan_ccu_cstat::R) reader structure"]
impl crate::Readable for FCCAN_CCU_CSTAT {}
#[doc = "Calibration status register"]
pub mod fccan_ccu_cstat;
#[doc = "The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_cwd](fccan_ccu_cwd) module"]
pub type FCCAN_CCU_CWD = crate::Reg<u32, _FCCAN_CCU_CWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_CWD;
#[doc = "`read()` method returns [fccan_ccu_cwd::R](fccan_ccu_cwd::R) reader structure"]
impl crate::Readable for FCCAN_CCU_CWD {}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_cwd::W](fccan_ccu_cwd::W) writer structure"]
impl crate::Writable for FCCAN_CCU_CWD {}
#[doc = "The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset."]
pub mod fccan_ccu_cwd;
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ir](fccan_ccu_ir) module"]
pub type FCCAN_CCU_IR = crate::Reg<u32, _FCCAN_CCU_IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_IR;
#[doc = "`read()` method returns [fccan_ccu_ir::R](fccan_ccu_ir::R) reader structure"]
impl crate::Readable for FCCAN_CCU_IR {}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ir::W](fccan_ccu_ir::W) writer structure"]
impl crate::Writable for FCCAN_CCU_IR {}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not."]
pub mod fccan_ccu_ir;
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ie](fccan_ccu_ie) module"]
pub type FCCAN_CCU_IE = crate::Reg<u32, _FCCAN_CCU_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCAN_CCU_IE;
#[doc = "`read()` method returns [fccan_ccu_ie::R](fccan_ccu_ie::R) reader structure"]
impl crate::Readable for FCCAN_CCU_IE {}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ie::W](fccan_ccu_ie::W) writer structure"]
impl crate::Writable for FCCAN_CCU_IE {}
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line."]
pub mod fccan_ccu_ie;
