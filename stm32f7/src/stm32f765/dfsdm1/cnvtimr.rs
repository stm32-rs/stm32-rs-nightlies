#[doc = "Register `CNVTIMR` reader"]
pub type R = crate::R<CNVTIMRrs>;
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time"]
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNVTIMRrs;
impl crate::RegisterSpec for CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnvtimr::R`](R) reader structure"]
impl crate::Readable for CNVTIMRrs {}
#[doc = "`reset()` method sets CNVTIMR to value 0"]
impl crate::Resettable for CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
