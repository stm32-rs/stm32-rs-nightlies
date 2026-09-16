///Register `FSM_USR` reader
pub type R = crate::R<FSM_USRrs>;
///Register `FSM_USR` writer
pub type W = crate::W<FSM_USRrs>;
///Field `EN_CALIB_CBP` reader - CBP calibration enable bit.
pub type EN_CALIB_CBP_R = crate::BitReader;
///Field `EN_CALIB_CBP` writer - CBP calibration enable bit.
pub type EN_CALIB_CBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_CALIB_SYNTH` reader - SYNTH calibration enable bit.
pub type EN_CALIB_SYNTH_R = crate::BitReader;
///Field `EN_CALIB_SYNTH` writer - SYNTH calibration enable bit.
pub type EN_CALIB_SYNTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA_POWER` reader - PA Power coefficient.
pub type PA_POWER_R = crate::FieldReader;
///Field `PA_POWER` writer - PA Power coefficient.
pub type PA_POWER_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 1 - CBP calibration enable bit.
    #[inline(always)]
    pub fn en_calib_cbp(&self) -> EN_CALIB_CBP_R {
        EN_CALIB_CBP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SYNTH calibration enable bit.
    #[inline(always)]
    pub fn en_calib_synth(&self) -> EN_CALIB_SYNTH_R {
        EN_CALIB_SYNTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:7 - PA Power coefficient.
    #[inline(always)]
    pub fn pa_power(&self) -> PA_POWER_R {
        PA_POWER_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_USR")
            .field("en_calib_cbp", &self.en_calib_cbp())
            .field("en_calib_synth", &self.en_calib_synth())
            .field("pa_power", &self.pa_power())
            .finish()
    }
}
impl W {
    ///Bit 1 - CBP calibration enable bit.
    #[inline(always)]
    pub fn en_calib_cbp(&mut self) -> EN_CALIB_CBP_W<'_, FSM_USRrs> {
        EN_CALIB_CBP_W::new(self, 1)
    }
    ///Bit 2 - SYNTH calibration enable bit.
    #[inline(always)]
    pub fn en_calib_synth(&mut self) -> EN_CALIB_SYNTH_W<'_, FSM_USRrs> {
        EN_CALIB_SYNTH_W::new(self, 2)
    }
    ///Bits 3:7 - PA Power coefficient.
    #[inline(always)]
    pub fn pa_power(&mut self) -> PA_POWER_W<'_, FSM_USRrs> {
        PA_POWER_W::new(self, 3)
    }
}
/**RADIO_FSM_USR register

You can [`read`](crate::Reg::read) this register and get [`fsm_usr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_usr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:FSM_USR)*/
pub struct FSM_USRrs;
impl crate::RegisterSpec for FSM_USRrs {
    type Ux = u32;
}
///`read()` method returns [`fsm_usr::R`](R) reader structure
impl crate::Readable for FSM_USRrs {}
///`write(|w| ..)` method takes [`fsm_usr::W`](W) writer structure
impl crate::Writable for FSM_USRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FSM_USR to value 0x04
impl crate::Resettable for FSM_USRrs {
    const RESET_VALUE: u32 = 0x04;
}
