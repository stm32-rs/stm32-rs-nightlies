#[doc = "Register `DOR` reader"]
pub type R = crate::R<DORrs>;
#[doc = "Field `DATAOUT` reader - Data Output FIFO Output FIFO data register."]
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output FIFO Output FIFO data register."]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
#[doc = "JPEG data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DORrs;
impl crate::RegisterSpec for DORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DORrs {}
#[doc = "`reset()` method sets DOR to value 0"]
impl crate::Resettable for DORrs {
    const RESET_VALUE: u32 = 0;
}
