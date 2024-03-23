#[doc = "Register `ITLINE12` reader"]
pub type R = crate::R<ITLINE12rs>;
#[doc = "Field `ADC` reader - ADC"]
pub type ADC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 12 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE12rs;
impl crate::RegisterSpec for ITLINE12rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline12::R`](R) reader structure"]
impl crate::Readable for ITLINE12rs {}
#[doc = "`reset()` method sets ITLINE12 to value 0"]
impl crate::Resettable for ITLINE12rs {
    const RESET_VALUE: u32 = 0;
}
