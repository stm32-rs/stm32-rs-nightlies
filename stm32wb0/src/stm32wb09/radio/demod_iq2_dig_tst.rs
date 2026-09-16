///Register `DEMOD_IQ2_DIG_TST` reader
pub type R = crate::R<DEMOD_IQ2_DIG_TSTrs>;
///Register `DEMOD_IQ2_DIG_TST` writer
pub type W = crate::W<DEMOD_IQ2_DIG_TSTrs>;
///Field `EXTCFG_SAMPLING_TIME` reader - Defines the sampling time, when extended configuration is enabled:
pub type EXTCFG_SAMPLING_TIME_R = crate::FieldReader;
///Field `EXTCFG_SAMPLING_TIME` writer - Defines the sampling time, when extended configuration is enabled:
pub type EXTCFG_SAMPLING_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EXTCFG_TRIG_SELECTION` reader - Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:
pub type EXTCFG_TRIG_SELECTION_R = crate::FieldReader;
///Field `EXTCFG_TRIG_SELECTION` writer - Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:
pub type EXTCFG_TRIG_SELECTION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Defines the sampling time, when extended configuration is enabled:
    #[inline(always)]
    pub fn extcfg_sampling_time(&self) -> EXTCFG_SAMPLING_TIME_R {
        EXTCFG_SAMPLING_TIME_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:
    #[inline(always)]
    pub fn extcfg_trig_selection(&self) -> EXTCFG_TRIG_SELECTION_R {
        EXTCFG_TRIG_SELECTION_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMOD_IQ2_DIG_TST")
            .field("extcfg_sampling_time", &self.extcfg_sampling_time())
            .field("extcfg_trig_selection", &self.extcfg_trig_selection())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Defines the sampling time, when extended configuration is enabled:
    #[inline(always)]
    pub fn extcfg_sampling_time(&mut self) -> EXTCFG_SAMPLING_TIME_W<'_, DEMOD_IQ2_DIG_TSTrs> {
        EXTCFG_SAMPLING_TIME_W::new(self, 0)
    }
    ///Bits 2:3 - Defines the trigger/anchor point of the IQ sampling, when extended configuration is enabled:
    #[inline(always)]
    pub fn extcfg_trig_selection(&mut self) -> EXTCFG_TRIG_SELECTION_W<'_, DEMOD_IQ2_DIG_TSTrs> {
        EXTCFG_TRIG_SELECTION_W::new(self, 2)
    }
}
/**DEMOD_IQ2_DIG_TST register

You can [`read`](crate::Reg::read) this register and get [`demod_iq2_dig_tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demod_iq2_dig_tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:DEMOD_IQ2_DIG_TST)*/
pub struct DEMOD_IQ2_DIG_TSTrs;
impl crate::RegisterSpec for DEMOD_IQ2_DIG_TSTrs {
    type Ux = u32;
}
///`read()` method returns [`demod_iq2_dig_tst::R`](R) reader structure
impl crate::Readable for DEMOD_IQ2_DIG_TSTrs {}
///`write(|w| ..)` method takes [`demod_iq2_dig_tst::W`](W) writer structure
impl crate::Writable for DEMOD_IQ2_DIG_TSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEMOD_IQ2_DIG_TST to value 0
impl crate::Resettable for DEMOD_IQ2_DIG_TSTrs {}
