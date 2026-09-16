///Register `DBG_STATUS_REG1` reader
pub type R = crate::R<DBG_STATUS_REG1rs>;
///Field `SMPS_FSM_STATE` reader - SMPS_FSM_STATE\[2:0\]: Indicates the current state of the SMPS FSM inside the PWRC.: - 000: STARTUP - 001: SMPS_REQ - 010: SMPS_RUN - 011: STOP - 100: NOSMPS - 101: PRECHARGE - 110: NOSMPS_BOF
pub type SMPS_FSM_STATE_R = crate::FieldReader;
///Field `FLASH_FSM_STATE` reader - FLASH_FSM_STATE\[2:0\]: Indicates the current state of the FLASH FSM inside the PWRC: - 000: STATE1: FLASH POR - 001: STATE2: FLASH PWRUP - 010: STATE3: FLASH READY - 101: STATE4: FLASH SWITCH OFF - 110: STATE5: FLASH PWR DOWN
pub type FLASH_FSM_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - SMPS_FSM_STATE\[2:0\]: Indicates the current state of the SMPS FSM inside the PWRC.: - 000: STARTUP - 001: SMPS_REQ - 010: SMPS_RUN - 011: STOP - 100: NOSMPS - 101: PRECHARGE - 110: NOSMPS_BOF
    #[inline(always)]
    pub fn smps_fsm_state(&self) -> SMPS_FSM_STATE_R {
        SMPS_FSM_STATE_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - FLASH_FSM_STATE\[2:0\]: Indicates the current state of the FLASH FSM inside the PWRC: - 000: STATE1: FLASH POR - 001: STATE2: FLASH PWRUP - 010: STATE3: FLASH READY - 101: STATE4: FLASH SWITCH OFF - 110: STATE5: FLASH PWR DOWN
    #[inline(always)]
    pub fn flash_fsm_state(&self) -> FLASH_FSM_STATE_R {
        FLASH_FSM_STATE_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_STATUS_REG1")
            .field("smps_fsm_state", &self.smps_fsm_state())
            .field("flash_fsm_state", &self.flash_fsm_state())
            .finish()
    }
}
/**DBG_STATUS_REG1 register

You can [`read`](crate::Reg::read) this register and get [`dbg_status_reg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:DBG_STATUS_REG1)*/
pub struct DBG_STATUS_REG1rs;
impl crate::RegisterSpec for DBG_STATUS_REG1rs {
    type Ux = u32;
}
///`read()` method returns [`dbg_status_reg1::R`](R) reader structure
impl crate::Readable for DBG_STATUS_REG1rs {}
///`reset()` method sets DBG_STATUS_REG1 to value 0x0202
impl crate::Resettable for DBG_STATUS_REG1rs {
    const RESET_VALUE: u32 = 0x0202;
}
