#[doc = "Register `ETH_MACVR` reader"]
pub type R = crate::R<ETH_MACVRrs>;
#[doc = "Field `SNPSVER` reader - SNPSVER"]
pub type SNPSVER_R = crate::FieldReader;
#[doc = "Field `USERVER` reader - USERVER"]
pub type USERVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SNPSVER"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USERVER"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "The version register identifies the version of the Ethernet peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACVRrs;
impl crate::RegisterSpec for ETH_MACVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macvr::R`](R) reader structure"]
impl crate::Readable for ETH_MACVRrs {}
#[doc = "`reset()` method sets ETH_MACVR to value 0x4042"]
impl crate::Resettable for ETH_MACVRrs {
    const RESET_VALUE: u32 = 0x4042;
}
