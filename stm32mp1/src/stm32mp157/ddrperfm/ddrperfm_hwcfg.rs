#[doc = "Register `DDRPERFM_HWCFG` reader"]
pub type R = crate::R<DDRPERFM_HWCFGrs>;
#[doc = "Field `NCNT` reader - NCNT"]
pub type NCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - NCNT"]
    #[inline(always)]
    pub fn ncnt(&self) -> NCNT_R {
        NCNT_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "DDRPERFM hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_hwcfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_HWCFGrs;
impl crate::RegisterSpec for DDRPERFM_HWCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_hwcfg::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_HWCFGrs {}
#[doc = "`reset()` method sets DDRPERFM_HWCFG to value 0x04"]
impl crate::Resettable for DDRPERFM_HWCFGrs {
    const RESET_VALUE: u32 = 0x04;
}
