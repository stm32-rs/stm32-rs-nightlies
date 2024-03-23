#[doc = "Register `RCC_MP_APRSTSR` reader"]
pub type R = crate::R<RCC_MP_APRSTSRrs>;
#[doc = "Field `RSTTOV` reader - RSTTOV"]
pub type RSTTOV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 8:14 - RSTTOV"]
    #[inline(always)]
    pub fn rsttov(&self) -> RSTTOV_R {
        RSTTOV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aprstsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_APRSTSRrs;
impl crate::RegisterSpec for RCC_MP_APRSTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_aprstsr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_APRSTSRrs {}
#[doc = "`reset()` method sets RCC_MP_APRSTSR to value 0"]
impl crate::Resettable for RCC_MP_APRSTSRrs {
    const RESET_VALUE: u32 = 0;
}
