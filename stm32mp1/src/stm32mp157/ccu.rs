#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crel: CREL,
    ccfg: CCFG,
    cstat: CSTAT,
    cwd: CWD,
    ir: IR,
    ie: IE,
}
impl RegisterBlock {
    ///0x00 - Clock calibration unit core release register
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    ///0x04 - Calibration configuration register
    #[inline(always)]
    pub const fn ccfg(&self) -> &CCFG {
        &self.ccfg
    }
    ///0x08 - Calibration status register
    #[inline(always)]
    pub const fn cstat(&self) -> &CSTAT {
        &self.cstat
    }
    ///0x0c - The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.
    #[inline(always)]
    pub const fn cwd(&self) -> &CWD {
        &self.cwd
    }
    ///0x10 - The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    ///0x14 - The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
}
/**CREL (r) register accessor: Clock calibration unit core release register

You can [`read`](crate::Reg::read) this register and get [`crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:CREL)

For information about available fields see [`mod@crel`] module*/
pub type CREL = crate::Reg<crel::CRELrs>;
///Clock calibration unit core release register
pub mod crel;
/**CCFG (rw) register accessor: Calibration configuration register

You can [`read`](crate::Reg::read) this register and get [`ccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:CCFG)

For information about available fields see [`mod@ccfg`] module*/
pub type CCFG = crate::Reg<ccfg::CCFGrs>;
///Calibration configuration register
pub mod ccfg;
/**CSTAT (r) register accessor: Calibration status register

You can [`read`](crate::Reg::read) this register and get [`cstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:CSTAT)

For information about available fields see [`mod@cstat`] module*/
pub type CSTAT = crate::Reg<cstat::CSTATrs>;
///Calibration status register
pub mod cstat;
/**CWD (rw) register accessor: The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.

You can [`read`](crate::Reg::read) this register and get [`cwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:CWD)

For information about available fields see [`mod@cwd`] module*/
pub type CWD = crate::Reg<cwd::CWDrs>;
///The calibration watchdog is started after the first falling edge when the calibration FSM is in state Not_Calibrated (CCU_CSTAT.CALS = 00). In this state the calibration watchdog monitors the message received. In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM stays in state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When in state Basic_Calibrated (CCU_CSTAT.CALS = 01), the calibration watchdog is restarted with each received message . In case no message was received until the calibration watchdog has counted down to 0, the calibration FSM returns to state Not_Calibrated (CCU_CSTAT.CALS = 00), the counter is reloaded with FDCAN_RWD.WDC and basic calibration is restarted after the next falling edge. When a quartz message is received, state Precision_Calibrated (CCU_CSTAT.CALS = 10) is entered and the calibration watchdog is restarted. In this state the calibration watchdog monitors the quartz message received input. In case no message from a quartz controlled node is received by the attached TTCAN until the calibration watchdog has counted down to 0, the calibration FSM transits back to state Basic_Calibrated (CCU_CSTAT.CALS = 01). The signal is active when the CAN protocol engine on the attached TTCAN is started i.e. when the INIT bit is reset. A calibration watchdog event also sets interrupt flag CCU_IR.CWE. If enabled by CCU_IE.CWEE, interrupt line is activated (set to high). Interrupt line remains active until interrupt flag CCU_IR.CWE is reset.
pub mod cwd;
/**IR (rw) register accessor: The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.

You can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:IR)

For information about available fields see [`mod@ir`] module*/
pub type IR = crate::Reg<ir::IRrs>;
///The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.
pub mod ir;
/**IE (rw) register accessor: The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.

You can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CCU:IE)

For information about available fields see [`mod@ie`] module*/
pub type IE = crate::Reg<ie::IErs>;
///The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.
pub mod ie;
