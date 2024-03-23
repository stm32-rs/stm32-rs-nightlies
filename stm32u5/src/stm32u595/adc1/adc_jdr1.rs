#[doc = "Register `ADC_JDR1` reader"]
pub type R = crate::R<ADC_JDR1rs>;
#[doc = "Field `JDATA` reader - JDATA"]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
#[doc = "ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_JDR1rs;
impl crate::RegisterSpec for ADC_JDR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_jdr1::R`](R) reader structure"]
impl crate::Readable for ADC_JDR1rs {}
#[doc = "`reset()` method sets ADC_JDR1 to value 0"]
impl crate::Resettable for ADC_JDR1rs {
    const RESET_VALUE: u32 = 0;
}
