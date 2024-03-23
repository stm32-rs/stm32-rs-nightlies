#[doc = "Register `ADC_JDR3` reader"]
pub type R = crate::R<ADC_JDR3rs>;
#[doc = "Field `JDATA` reader - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in ."]
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in ."]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(self.bits)
    }
}
#[doc = "ADC injected data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_jdr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_JDR3rs;
impl crate::RegisterSpec for ADC_JDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_jdr3::R`](R) reader structure"]
impl crate::Readable for ADC_JDR3rs {}
#[doc = "`reset()` method sets ADC_JDR3 to value 0"]
impl crate::Resettable for ADC_JDR3rs {
    const RESET_VALUE: u32 = 0;
}
