#[doc = "Register `DOUT` reader"]
pub type R = crate::R<DOUTrs>;
#[doc = "Field `DATAOUT` reader - Data output"]
pub type DATAOUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new(self.bits)
    }
}
#[doc = "data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTrs;
impl crate::RegisterSpec for DOUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout::R`](R) reader structure"]
impl crate::Readable for DOUTrs {}
#[doc = "`reset()` method sets DOUT to value 0"]
impl crate::Resettable for DOUTrs {
    const RESET_VALUE: u32 = 0;
}
