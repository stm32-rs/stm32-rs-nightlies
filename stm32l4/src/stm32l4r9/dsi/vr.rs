#[doc = "Register `VR` reader"]
pub type R = crate::R<VRrs>;
#[doc = "Field `VERSION` reader - Version of the DSI Host"]
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
#[doc = "DSI Host Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VRrs;
impl crate::RegisterSpec for VRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vr::R`](R) reader structure"]
impl crate::Readable for VRrs {}
#[doc = "`reset()` method sets VR to value 0x3133_302a"]
impl crate::Resettable for VRrs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
