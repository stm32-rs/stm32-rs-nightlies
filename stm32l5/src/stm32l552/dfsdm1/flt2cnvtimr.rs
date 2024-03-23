#[doc = "Register `FLT2CNVTIMR` reader"]
pub type R = crate::R<FLT2CNVTIMRrs>;
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
#[doc = "conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt2cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT2CNVTIMRrs;
impl crate::RegisterSpec for FLT2CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt2cnvtimr::R`](R) reader structure"]
impl crate::Readable for FLT2CNVTIMRrs {}
#[doc = "`reset()` method sets FLT2CNVTIMR to value 0"]
impl crate::Resettable for FLT2CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
