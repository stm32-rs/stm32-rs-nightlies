#[doc = "Register `ADC_DIFSEL` reader"]
pub type R = crate::R<ADC_DIFSELrs>;
#[doc = "Register `ADC_DIFSEL` writer"]
pub type W = crate::W<ADC_DIFSELrs>;
#[doc = "Field `DIFSEL` reader - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\\[i\\]
= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\\[i\\]
= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DIFSEL_R = crate::FieldReader<u32>;
#[doc = "Field `DIFSEL` writer - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\\[i\\]
= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\\[i\\]
= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type DIFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\\[i\\]
= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\\[i\\]
= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL\\[i\\]
= 0: ADC analog input channel-i is configured in single ended mode DIFSEL\\[i\\]
= 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn difsel(&mut self) -> DIFSEL_W<ADC_DIFSELrs> {
        DIFSEL_W::new(self, 0)
    }
}
#[doc = "ADC differential mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_difsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_difsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_DIFSELrs;
impl crate::RegisterSpec for ADC_DIFSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_difsel::R`](R) reader structure"]
impl crate::Readable for ADC_DIFSELrs {}
#[doc = "`write(|w| ..)` method takes [`adc_difsel::W`](W) writer structure"]
impl crate::Writable for ADC_DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_DIFSEL to value 0"]
impl crate::Resettable for ADC_DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
