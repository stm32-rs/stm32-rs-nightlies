#[doc = "Register `IWDG_HWCFGR` reader"]
pub type R = crate::R<IWDG_HWCFGRrs>;
#[doc = "Field `WINDOW` reader - WINDOW"]
pub type WINDOW_R = crate::FieldReader;
#[doc = "Field `PR_DEFAULT` reader - PR_DEFAULT"]
pub type PR_DEFAULT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - WINDOW"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PR_DEFAULT"]
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "IWDG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_HWCFGRrs;
impl crate::RegisterSpec for IWDG_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_hwcfgr::R`](R) reader structure"]
impl crate::Readable for IWDG_HWCFGRrs {}
#[doc = "`reset()` method sets IWDG_HWCFGR to value 0x71"]
impl crate::Resettable for IWDG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x71;
}
