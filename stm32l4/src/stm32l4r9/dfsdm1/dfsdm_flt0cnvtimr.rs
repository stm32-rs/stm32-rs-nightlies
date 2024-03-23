#[doc = "Register `DFSDM_FLT0CNVTIMR` reader"]
pub type R = crate::R<DFSDM_FLT0CNVTIMRrs>;
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN"]
pub type CNVCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0CNVTIMRrs;
impl crate::RegisterSpec for DFSDM_FLT0CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0cnvtimr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0CNVTIMRrs {}
#[doc = "`reset()` method sets DFSDM_FLT0CNVTIMR to value 0"]
impl crate::Resettable for DFSDM_FLT0CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
