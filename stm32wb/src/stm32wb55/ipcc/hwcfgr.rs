#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Field `CHANNELS` reader - Number of channels per CPU supported by the IP, range 1 to 16"]
pub type CHANNELS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of channels per CPU supported by the IP, range 1 to 16"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPCC Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`reset()` method sets HWCFGR to value 0x06"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x06;
}
