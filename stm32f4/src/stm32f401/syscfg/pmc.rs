///Register `PMC` reader
pub type R = crate::R<PMCrs>;
///Register `PMC` writer
pub type W = crate::W<PMCrs>;
///Field `ADC1DC2` reader - ADC1DC2
pub type ADC1DC2_R = crate::BitReader;
///Field `ADC1DC2` writer - ADC1DC2
pub type ADC1DC2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - ADC1DC2
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMC")
            .field("adc1dc2", &self.adc1dc2())
            .finish()
    }
}
impl W {
    ///Bit 16 - ADC1DC2
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W<'_, PMCrs> {
        ADC1DC2_W::new(self, 16)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F401.html#SYSCFG:PMC)*/
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
///`read()` method returns [`pmc::R`](R) reader structure
impl crate::Readable for PMCrs {}
///`write(|w| ..)` method takes [`pmc::W`](W) writer structure
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMC to value 0
impl crate::Resettable for PMCrs {}
