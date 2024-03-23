#[doc = "Register `ADC_DR` reader"]
pub type R = crate::R<ADC_DRrs>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_DRrs;
impl crate::RegisterSpec for ADC_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_dr::R`](R) reader structure"]
impl crate::Readable for ADC_DRrs {}
#[doc = "`reset()` method sets ADC_DR to value 0"]
impl crate::Resettable for ADC_DRrs {
    const RESET_VALUE: u32 = 0;
}
