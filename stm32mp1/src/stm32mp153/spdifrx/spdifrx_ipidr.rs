#[doc = "Register `SPDIFRX_IPIDR` reader"]
pub type R = crate::R<SPDIFRX_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "SPDIFRX identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_IPIDRrs;
impl crate::RegisterSpec for SPDIFRX_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_ipidr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_IPIDRrs {}
#[doc = "`reset()` method sets SPDIFRX_IPIDR to value 0x0013_0041"]
impl crate::Resettable for SPDIFRX_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0041;
}
