#[doc = "Register `IPCC_HWCFGR` reader"]
pub type R = crate::R<IPCC_HWCFGRrs>;
#[doc = "Field `CHANNELS` reader - CHANNELS"]
pub type CHANNELS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CHANNELS"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPCC Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_HWCFGRrs;
impl crate::RegisterSpec for IPCC_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_hwcfgr::R`](R) reader structure"]
impl crate::Readable for IPCC_HWCFGRrs {}
#[doc = "`reset()` method sets IPCC_HWCFGR to value 0x02"]
impl crate::Resettable for IPCC_HWCFGRrs {
    const RESET_VALUE: u32 = 0x02;
}
