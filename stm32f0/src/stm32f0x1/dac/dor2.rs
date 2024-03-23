#[doc = "Register `DOR2` reader"]
pub type R = crate::R<DOR2rs>;
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output"]
pub type DACC2DOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC channel2 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOR2rs;
impl crate::RegisterSpec for DOR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor2::R`](R) reader structure"]
impl crate::Readable for DOR2rs {}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for DOR2rs {
    const RESET_VALUE: u32 = 0;
}
