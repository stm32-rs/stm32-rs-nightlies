///Register `RCC_ADCCKSELR` reader
pub type R = crate::R<RCC_ADCCKSELRrs>;
///Register `RCC_ADCCKSELR` writer
pub type W = crate::W<RCC_ADCCKSELRrs>;
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
        f.debug_struct("RCC_ADCCKSELR")
            .field("adcsrc", &self.adcsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - ADCSRC
    #[inline(always)]
    #[must_use]
    pub fn adcsrc(&mut self) -> ADCSRC_W<RCC_ADCCKSELRrs> {
        ADCSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the ADC block.

You can [`read`](crate::Reg::read) this register and get [`rcc_adcckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_adcckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_ADCCKSELR)*/
pub struct RCC_ADCCKSELRrs;
impl crate::RegisterSpec for RCC_ADCCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_adcckselr::R`](R) reader structure
impl crate::Readable for RCC_ADCCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_adcckselr::W`](W) writer structure
impl crate::Writable for RCC_ADCCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_ADCCKSELR to value 0
impl crate::Resettable for RCC_ADCCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
