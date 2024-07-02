///Register `ADC_CALFACT2` reader
pub type R = crate::R<ADC_CALFACT2rs>;
///Register `ADC_CALFACT2` writer
pub type W = crate::W<ADC_CALFACT2rs>;
///Field `CALFACT` reader - CALFACT
pub type CALFACT_R = crate::FieldReader<u32>;
///Field `CALFACT` writer - CALFACT
pub type CALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CALFACT
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALFACT2")
            .field("calfact", &self.calfact())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CALFACT
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CALFACT_W<ADC_CALFACT2rs> {
        CALFACT_W::new(self, 0)
    }
}
/**ADC calibration factor register

You can [`read`](crate::Reg::read) this register and get [`adc_calfact2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:ADC_CALFACT2)*/
pub struct ADC_CALFACT2rs;
impl crate::RegisterSpec for ADC_CALFACT2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_calfact2::R`](R) reader structure
impl crate::Readable for ADC_CALFACT2rs {}
///`write(|w| ..)` method takes [`adc_calfact2::W`](W) writer structure
impl crate::Writable for ADC_CALFACT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CALFACT2 to value 0
impl crate::Resettable for ADC_CALFACT2rs {
    const RESET_VALUE: u32 = 0;
}
