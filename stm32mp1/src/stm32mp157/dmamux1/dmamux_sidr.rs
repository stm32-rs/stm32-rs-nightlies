#[doc = "Register `DMAMUX_SIDR` reader"]
pub type R = crate::R<DMAMUX_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "DMAMUX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_SIDRrs;
impl crate::RegisterSpec for DMAMUX_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_sidr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_SIDRrs {}
#[doc = "`reset()` method sets DMAMUX_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for DMAMUX_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
