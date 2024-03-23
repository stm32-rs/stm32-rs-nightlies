#[doc = "Register `WWDG_HWCFGR` reader"]
pub type R = crate::R<WWDG_HWCFGRrs>;
#[doc = "Field `PREDIV` reader - PREDIV"]
pub type PREDIV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PREDIV"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "WWDG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_HWCFGRrs;
impl crate::RegisterSpec for WWDG_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_hwcfgr::R`](R) reader structure"]
impl crate::Readable for WWDG_HWCFGRrs {}
#[doc = "`reset()` method sets WWDG_HWCFGR to value 0x0fff"]
impl crate::Resettable for WWDG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
