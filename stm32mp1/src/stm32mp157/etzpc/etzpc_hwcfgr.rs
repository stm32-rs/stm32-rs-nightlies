#[doc = "Register `ETZPC_HWCFGR` reader"]
pub type R = crate::R<ETZPC_HWCFGRrs>;
#[doc = "Field `NUM_TZMA` reader - NUM_TZMA"]
pub type NUM_TZMA_R = crate::FieldReader;
#[doc = "Field `NUM_PER_SEC` reader - NUM_PER_SEC"]
pub type NUM_PER_SEC_R = crate::FieldReader;
#[doc = "Field `NUM_AHB_SEC` reader - NUM_AHB_SEC"]
pub type NUM_AHB_SEC_R = crate::FieldReader;
#[doc = "Field `CHUNKS1N4` reader - CHUNKS1N4"]
pub type CHUNKS1N4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NUM_TZMA"]
    #[inline(always)]
    pub fn num_tzma(&self) -> NUM_TZMA_R {
        NUM_TZMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NUM_PER_SEC"]
    #[inline(always)]
    pub fn num_per_sec(&self) -> NUM_PER_SEC_R {
        NUM_PER_SEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NUM_AHB_SEC"]
    #[inline(always)]
    pub fn num_ahb_sec(&self) -> NUM_AHB_SEC_R {
        NUM_AHB_SEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CHUNKS1N4"]
    #[inline(always)]
    pub fn chunks1n4(&self) -> CHUNKS1N4_R {
        CHUNKS1N4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ETZPC IP HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etzpc_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETZPC_HWCFGRrs;
impl crate::RegisterSpec for ETZPC_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etzpc_hwcfgr::R`](R) reader structure"]
impl crate::Readable for ETZPC_HWCFGRrs {}
#[doc = "`reset()` method sets ETZPC_HWCFGR to value 0x6002"]
impl crate::Resettable for ETZPC_HWCFGRrs {
    const RESET_VALUE: u32 = 0x6002;
}
