///Register `DBG_STATUS_REG2` reader
pub type R = crate::R<DBG_STATUS_REG2rs>;
///Field `PMU_FSM_STATE` reader - PMU_FSM_STATE\[3:0\]: Indicates the current state of the PMU FSM inside the PWRC. - 0000: POR - 0001: RUN - 0010: DS ENTRY - 0011: WAIT1 - 0100: WAIT2 - 0101: WAIT - 0110: WAIT3 - 0111: WAIT4 - 1000: ISOLATION - 1001: DEEPSTOP - 1010: SHUTDOWN - 1011: DEEPSTOP EXIT
pub type PMU_FSM_STATE_R = crate::FieldReader;
///Field `RAM_FSM_STATE` reader - RAM_FSM_STATE\[1:0\]: Indicates the current state of the RAM FSM inside the PWRC: - 00: POR - 01: POWER UP - 10: READY - 11: OFF
pub type RAM_FSM_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - PMU_FSM_STATE\[3:0\]: Indicates the current state of the PMU FSM inside the PWRC. - 0000: POR - 0001: RUN - 0010: DS ENTRY - 0011: WAIT1 - 0100: WAIT2 - 0101: WAIT - 0110: WAIT3 - 0111: WAIT4 - 1000: ISOLATION - 1001: DEEPSTOP - 1010: SHUTDOWN - 1011: DEEPSTOP EXIT
    #[inline(always)]
    pub fn pmu_fsm_state(&self) -> PMU_FSM_STATE_R {
        PMU_FSM_STATE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - RAM_FSM_STATE\[1:0\]: Indicates the current state of the RAM FSM inside the PWRC: - 00: POR - 01: POWER UP - 10: READY - 11: OFF
    #[inline(always)]
    pub fn ram_fsm_state(&self) -> RAM_FSM_STATE_R {
        RAM_FSM_STATE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_STATUS_REG2")
            .field("pmu_fsm_state", &self.pmu_fsm_state())
            .field("ram_fsm_state", &self.ram_fsm_state())
            .finish()
    }
}
/**DBG_STATUS_REG2 register

You can [`read`](crate::Reg::read) this register and get [`dbg_status_reg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBG_STATUS_REG2)*/
pub struct DBG_STATUS_REG2rs;
impl crate::RegisterSpec for DBG_STATUS_REG2rs {
    type Ux = u32;
}
///`read()` method returns [`dbg_status_reg2::R`](R) reader structure
impl crate::Readable for DBG_STATUS_REG2rs {}
///`reset()` method sets DBG_STATUS_REG2 to value 0x0201
impl crate::Resettable for DBG_STATUS_REG2rs {
    const RESET_VALUE: u32 = 0x0201;
}
