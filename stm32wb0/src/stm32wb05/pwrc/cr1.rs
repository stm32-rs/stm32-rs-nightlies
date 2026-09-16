///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPMS` reader - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
pub type LPMS_R = crate::BitReader;
///Field `LPMS` writer - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENSDNBOR` reader - ENSDNBOR: Enable BOR supply monitoring during shutdown mode.
pub type ENSDNBOR_R = crate::BitReader;
///Field `ENSDNBOR` writer - ENSDNBOR: Enable BOR supply monitoring during shutdown mode.
pub type ENSDNBOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBIAS_RUN_AUTO` reader - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default).
pub type IBIAS_RUN_AUTO_R = crate::BitReader;
///Field `IBIAS_RUN_AUTO` writer - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default).
pub type IBIAS_RUN_AUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IBIAS_RUN_STATE` reader - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled.
pub type IBIAS_RUN_STATE_R = crate::BitReader;
///Field `IBIAS_RUN_STATE` writer - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled.
pub type IBIAS_RUN_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APC` reader - APC Apply Pull-up and pull-down configuration from CPU
pub type APC_R = crate::BitReader;
///Field `APC` writer - APC Apply Pull-up and pull-down configuration from CPU
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENBORH` reader - ENBORH: enable BORH configuration
pub type ENBORH_R = crate::BitReader;
///Field `ENBORH` writer - ENBORH: enable BORH configuration
pub type ENBORH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SELBORH` reader - SELBORH\[1:0\]: BORH selection of Vbor threshold
pub type SELBORH_R = crate::FieldReader;
///Field `SELBORH` writer - SELBORH\[1:0\]: BORH selection of Vbor threshold
pub type SELBORH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENBORL` reader - ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN.
pub type ENBORL_R = crate::BitReader;
///Field `ENBORL` writer - ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN.
pub type ENBORL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ENSDNBOR: Enable BOR supply monitoring during shutdown mode.
    #[inline(always)]
    pub fn ensdnbor(&self) -> ENSDNBOR_R {
        ENSDNBOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default).
    #[inline(always)]
    pub fn ibias_run_auto(&self) -> IBIAS_RUN_AUTO_R {
        IBIAS_RUN_AUTO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled.
    #[inline(always)]
    pub fn ibias_run_state(&self) -> IBIAS_RUN_STATE_R {
        IBIAS_RUN_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - APC Apply Pull-up and pull-down configuration from CPU
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ENBORH: enable BORH configuration
    #[inline(always)]
    pub fn enborh(&self) -> ENBORH_R {
        ENBORH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - SELBORH\[1:0\]: BORH selection of Vbor threshold
    #[inline(always)]
    pub fn selborh(&self) -> SELBORH_R {
        SELBORH_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN.
    #[inline(always)]
    pub fn enborl(&self) -> ENBORL_R {
        ENBORL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("ensdnbor", &self.ensdnbor())
            .field("ibias_run_auto", &self.ibias_run_auto())
            .field("ibias_run_state", &self.ibias_run_state())
            .field("apc", &self.apc())
            .field("enborh", &self.enborh())
            .field("selborh", &self.selborh())
            .field("enborl", &self.enborl())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 1 - ENSDNBOR: Enable BOR supply monitoring during shutdown mode.
    #[inline(always)]
    pub fn ensdnbor(&mut self) -> ENSDNBOR_W<'_, CR1rs> {
        ENSDNBOR_W::new(self, 1)
    }
    ///Bit 2 - IBIAS_RUN_AUTO: Enable automatic IBIAS control during RUN/DEEPSTOP mode. 0: IBIAS control is manual (and controlled by IBIAS_RUN_STATE register) 1: IBIAS control is automatic (default).
    #[inline(always)]
    pub fn ibias_run_auto(&mut self) -> IBIAS_RUN_AUTO_W<'_, CR1rs> {
        IBIAS_RUN_AUTO_W::new(self, 2)
    }
    ///Bit 3 - IBIAS_RUN_STATE: Enable/Disable IBIAS during RUN mode when automatic mode is disabled. 0: IBIAS control is disabled (default). 1: IBIAS control is enabled.
    #[inline(always)]
    pub fn ibias_run_state(&mut self) -> IBIAS_RUN_STATE_W<'_, CR1rs> {
        IBIAS_RUN_STATE_W::new(self, 3)
    }
    ///Bit 4 - APC Apply Pull-up and pull-down configuration from CPU
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, CR1rs> {
        APC_W::new(self, 4)
    }
    ///Bit 5 - ENBORH: enable BORH configuration
    #[inline(always)]
    pub fn enborh(&mut self) -> ENBORH_W<'_, CR1rs> {
        ENBORH_W::new(self, 5)
    }
    ///Bits 6:7 - SELBORH\[1:0\]: BORH selection of Vbor threshold
    #[inline(always)]
    pub fn selborh(&mut self) -> SELBORH_W<'_, CR1rs> {
        SELBORH_W::new(self, 6)
    }
    ///Bit 8 - ENBORL: Enable BORL reset supervising during RUN mode. 0: No BORL is monitored during RUN mode. 1: BORL is monitored during RUN mode (a POR reset will happen if VDDIO goes below 1.6V during RUN mode) (default). Note: Enabling this feature prevents blocking the device if VDDIO goes below supported voltages during RUN.
    #[inline(always)]
    pub fn enborl(&mut self) -> ENBORL_W<'_, CR1rs> {
        ENBORL_W::new(self, 8)
    }
}
/**CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x0114
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0114;
}
