#[doc = "Register `MACVR` reader"]
pub type R = crate::R<MACVRrs>;
#[doc = "Field `SNPSVER` reader - IP version"]
pub type SNPSVER_R = crate::FieldReader;
#[doc = "Field `USERVER` reader - ST-defined version"]
pub type USERVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IP version"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST-defined version"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVRrs;
impl crate::RegisterSpec for MACVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvr::R`](R) reader structure"]
impl crate::Readable for MACVRrs {}
#[doc = "`reset()` method sets MACVR to value 0x3242"]
impl crate::Resettable for MACVRrs {
    const RESET_VALUE: u32 = 0x3242;
}
