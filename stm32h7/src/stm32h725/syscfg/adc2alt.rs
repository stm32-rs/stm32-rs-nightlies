///Register `ADC2ALT` reader
pub type R = crate::R<ADC2ALTrs>;
///Register `ADC2ALT` writer
pub type W = crate::W<ADC2ALTrs>;
///Field `ADC2_ROUT0` reader - ADC2 V_INP16 alternate connection
pub type ADC2_ROUT0_R = crate::BitReader;
///Field `ADC2_ROUT0` writer - ADC2 V_INP16 alternate connection
pub type ADC2_ROUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2_ROUT1` reader - ADC2 V_INP17 alternate connection
pub type ADC2_ROUT1_R = crate::BitReader;
///Field `ADC2_ROUT1` writer - ADC2 V_INP17 alternate connection
pub type ADC2_ROUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC2 V_INP16 alternate connection
    #[inline(always)]
    pub fn adc2_rout0(&self) -> ADC2_ROUT0_R {
        ADC2_ROUT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC2 V_INP17 alternate connection
    #[inline(always)]
    pub fn adc2_rout1(&self) -> ADC2_ROUT1_R {
        ADC2_ROUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC2ALT")
            .field("adc2_rout1", &self.adc2_rout1())
            .field("adc2_rout0", &self.adc2_rout0())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC2 V_INP16 alternate connection
    #[inline(always)]
    pub fn adc2_rout0(&mut self) -> ADC2_ROUT0_W<'_, ADC2ALTrs> {
        ADC2_ROUT0_W::new(self, 0)
    }
    ///Bit 1 - ADC2 V_INP17 alternate connection
    #[inline(always)]
    pub fn adc2_rout1(&mut self) -> ADC2_ROUT1_W<'_, ADC2ALTrs> {
        ADC2_ROUT1_W::new(self, 1)
    }
}
/**ADC2 internal input alternate connection

You can [`read`](crate::Reg::read) this register and get [`adc2alt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2alt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SYSCFG:ADC2ALT)*/
pub struct ADC2ALTrs;
impl crate::RegisterSpec for ADC2ALTrs {
    type Ux = u32;
}
///`read()` method returns [`adc2alt::R`](R) reader structure
impl crate::Readable for ADC2ALTrs {}
///`write(|w| ..)` method takes [`adc2alt::W`](W) writer structure
impl crate::Writable for ADC2ALTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC2ALT to value 0
impl crate::Resettable for ADC2ALTrs {}
