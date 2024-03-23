#[doc = "Register `ADC_DR` reader"]
pub type R = crate::R<ADC_DRrs>;
#[doc = "Field `RDATA` reader - RDATA"]
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "ADC regular Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
