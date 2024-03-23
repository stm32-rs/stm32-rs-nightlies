#[doc = "Register `DOUTR` reader"]
pub type R = crate::R<DOUTRrs>;
#[doc = "Field `DOUTR` reader - Data output"]
pub type DOUTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn doutr(&self) -> DOUTR_R {
        DOUTR_R::new(self.bits)
    }
}
#[doc = "Data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUTRrs;
impl crate::RegisterSpec for DOUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr::R`](R) reader structure"]
impl crate::Readable for DOUTRrs {}
#[doc = "`reset()` method sets DOUTR to value 0"]
impl crate::Resettable for DOUTRrs {
    const RESET_VALUE: u32 = 0;
}
