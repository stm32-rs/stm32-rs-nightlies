#[doc = "Register `ADF_SADSDLVR` reader"]
pub type R = crate::R<ADF_SADSDLVRrs>;
#[doc = "Field `SDLVL` reader - SDLVL"]
pub type SDLVL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - SDLVL"]
    #[inline(always)]
    pub fn sdlvl(&self) -> SDLVL_R {
        SDLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "ADF SAD sound level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_sadsdlvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_SADSDLVRrs;
impl crate::RegisterSpec for ADF_SADSDLVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_sadsdlvr::R`](R) reader structure"]
impl crate::Readable for ADF_SADSDLVRrs {}
#[doc = "`reset()` method sets ADF_SADSDLVR to value 0"]
impl crate::Resettable for ADF_SADSDLVRrs {
    const RESET_VALUE: u32 = 0;
}
