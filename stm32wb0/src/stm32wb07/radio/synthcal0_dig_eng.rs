///Register `SYNTHCAL0_DIG_ENG` reader
pub type R = crate::R<SYNTHCAL0_DIG_ENGrs>;
///Register `SYNTHCAL0_DIG_ENG` writer
pub type W = crate::W<SYNTHCAL0_DIG_ENGrs>;
///Field `SYNTHCAL_DEBUG_BUS_SEL` reader - for Debug purpose
pub type SYNTHCAL_DEBUG_BUS_SEL_R = crate::FieldReader;
///Field `SYNTHCAL_DEBUG_BUS_SEL` writer - for Debug purpose
pub type SYNTHCAL_DEBUG_BUS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SYNTH_IF_FREQ_CAL` reader - Define the frequency applied on the PLL during calibration phase
pub type SYNTH_IF_FREQ_CAL_R = crate::FieldReader;
///Field `SYNTH_IF_FREQ_CAL` writer - Define the frequency applied on the PLL during calibration phase
pub type SYNTH_IF_FREQ_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - for Debug purpose
    #[inline(always)]
    pub fn synthcal_debug_bus_sel(&self) -> SYNTHCAL_DEBUG_BUS_SEL_R {
        SYNTHCAL_DEBUG_BUS_SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:7 - Define the frequency applied on the PLL during calibration phase
    #[inline(always)]
    pub fn synth_if_freq_cal(&self) -> SYNTH_IF_FREQ_CAL_R {
        SYNTH_IF_FREQ_CAL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL0_DIG_ENG")
            .field("synthcal_debug_bus_sel", &self.synthcal_debug_bus_sel())
            .field("synth_if_freq_cal", &self.synth_if_freq_cal())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - for Debug purpose
    #[inline(always)]
    pub fn synthcal_debug_bus_sel(&mut self) -> SYNTHCAL_DEBUG_BUS_SEL_W<'_, SYNTHCAL0_DIG_ENGrs> {
        SYNTHCAL_DEBUG_BUS_SEL_W::new(self, 0)
    }
    ///Bits 6:7 - Define the frequency applied on the PLL during calibration phase
    #[inline(always)]
    pub fn synth_if_freq_cal(&mut self) -> SYNTH_IF_FREQ_CAL_W<'_, SYNTHCAL0_DIG_ENGrs> {
        SYNTH_IF_FREQ_CAL_W::new(self, 6)
    }
}
/**SYNTHCAL0_DIG_ENG register

You can [`read`](crate::Reg::read) this register and get [`synthcal0_dig_eng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synthcal0_dig_eng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:SYNTHCAL0_DIG_ENG)*/
pub struct SYNTHCAL0_DIG_ENGrs;
impl crate::RegisterSpec for SYNTHCAL0_DIG_ENGrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal0_dig_eng::R`](R) reader structure
impl crate::Readable for SYNTHCAL0_DIG_ENGrs {}
///`write(|w| ..)` method takes [`synthcal0_dig_eng::W`](W) writer structure
impl crate::Writable for SYNTHCAL0_DIG_ENGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTHCAL0_DIG_ENG to value 0
impl crate::Resettable for SYNTHCAL0_DIG_ENGrs {}
