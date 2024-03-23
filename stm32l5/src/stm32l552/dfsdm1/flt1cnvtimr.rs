#[doc = "Register `FLT1CNVTIMR` reader"]
pub type R = crate::R<FLT1CNVTIMRrs>;
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
#[doc = "conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt1cnvtimr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT1CNVTIMRrs;
impl crate::RegisterSpec for FLT1CNVTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt1cnvtimr::R`](R) reader structure"]
impl crate::Readable for FLT1CNVTIMRrs {}
#[doc = "`reset()` method sets FLT1CNVTIMR to value 0"]
impl crate::Resettable for FLT1CNVTIMRrs {
    const RESET_VALUE: u32 = 0;
}
