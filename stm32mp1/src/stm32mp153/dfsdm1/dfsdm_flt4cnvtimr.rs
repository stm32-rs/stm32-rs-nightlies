#[doc = "Register `DFSDM_FLT4CNVTIMR` reader"]
pub type R = crate::R<DFSDM_FLT4CNVTIMRrs>;
#[doc = "Field `CNVCNT` reader - CNVCNT"]
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - CNVCNT"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "DFSDM filter 4 conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt4cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT4CNVTIMRrs;
impl crate::RegisterSpec for DFSDM_FLT4CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt4cnvtimr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT4CNVTIMRrs {}
#[doc = "`reset()` method sets DFSDM_FLT4CNVTIMR to value 0"]
impl crate::Resettable for DFSDM_FLT4CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
