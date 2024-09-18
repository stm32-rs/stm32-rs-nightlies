///Register `ADC_DIFSEL` reader
pub type R = crate::R<ADC_DIFSELrs>;
///Register `ADC_DIFSEL` writer
pub type W = crate::W<ADC_DIFSELrs>;
///Field `DIFSEL` reader - DIFSEL
pub type DIFSEL_R = crate::FieldReader<u32>;
///Field `DIFSEL` writer - DIFSEL
pub type DIFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DIFSEL")
            .field("difsel", &self.difsel())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    #[must_use]
    pub fn difsel(&mut self) -> DIFSEL_W<ADC_DIFSELrs> {
        DIFSEL_W::new(self, 0)
    }
}
/**ADC differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`adc_difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC2:ADC_DIFSEL)*/
pub struct ADC_DIFSELrs;
impl crate::RegisterSpec for ADC_DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`adc_difsel::R`](R) reader structure
impl crate::Readable for ADC_DIFSELrs {}
///`write(|w| ..)` method takes [`adc_difsel::W`](W) writer structure
impl crate::Writable for ADC_DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_DIFSEL to value 0
impl crate::Resettable for ADC_DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
