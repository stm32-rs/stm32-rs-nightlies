#[doc = "Register `DSI_VR` reader"]
pub type R = crate::R<DSI_VRrs>;
#[doc = "Field `VERSION` reader - Version of the DSI Host This read-only register contains the version of the DSI Host"]
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version of the DSI Host This read-only register contains the version of the DSI Host"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
#[doc = "DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VRrs;
impl crate::RegisterSpec for DSI_VRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vr::R`](R) reader structure"]
impl crate::Readable for DSI_VRrs {}
#[doc = "`reset()` method sets DSI_VR to value 0x3134_312a"]
impl crate::Resettable for DSI_VRrs {
    const RESET_VALUE: u32 = 0x3134_312a;
}
