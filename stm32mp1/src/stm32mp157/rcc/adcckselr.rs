///Register `ADCCKSELR` reader
pub type R = crate::R<ADCCKSELRrs>;
///Register `ADCCKSELR` writer
pub type W = crate::W<ADCCKSELRrs>;
///Field `ADCSRC` reader - ADCSRC
pub type ADCSRC_R = crate::FieldReader;
///Field `ADCSRC` writer - ADCSRC
pub type ADCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - ADCSRC
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCKSELR")
            .field("adcsrc", &self.adcsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - ADCSRC
    #[inline(always)]
    pub fn adcsrc(&mut self) -> ADCSRC_W<'_, ADCCKSELRrs> {
        ADCSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the ADC block.

You can [`read`](crate::Reg::read) this register and get [`adcckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:ADCCKSELR)*/
pub struct ADCCKSELRrs;
impl crate::RegisterSpec for ADCCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`adcckselr::R`](R) reader structure
impl crate::Readable for ADCCKSELRrs {}
///`write(|w| ..)` method takes [`adcckselr::W`](W) writer structure
impl crate::Writable for ADCCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCCKSELR to value 0
impl crate::Resettable for ADCCKSELRrs {}
