#[doc = "Register `LPTIM_SIDR` reader"]
pub type R = crate::R<LPTIM_SIDRrs>;
#[doc = "Field `S_ID` reader - S_ID"]
pub type S_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - S_ID"]
    #[inline(always)]
    pub fn s_id(&self) -> S_ID_R {
        S_ID_R::new(self.bits)
    }
}
#[doc = "LPTIM registers map size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_SIDRrs;
impl crate::RegisterSpec for LPTIM_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_sidr::R`](R) reader structure"]
impl crate::Readable for LPTIM_SIDRrs {}
#[doc = "`reset()` method sets LPTIM_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for LPTIM_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
