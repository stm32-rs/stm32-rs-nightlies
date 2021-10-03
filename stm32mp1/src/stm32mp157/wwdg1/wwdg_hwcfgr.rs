#[doc = "Register `WWDG_HWCFGR` reader"]
pub struct R(crate::R<WWDG_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREDIV` reader - PREDIV"]
pub struct PREDIV_R(crate::FieldReader<u16, u16>);
impl PREDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        PREDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - PREDIV"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "WWDG hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_hwcfgr](index.html) module"]
pub struct WWDG_HWCFGR_SPEC;
impl crate::RegisterSpec for WWDG_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_hwcfgr::R](R) reader structure"]
impl crate::Readable for WWDG_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WWDG_HWCFGR to value 0x0fff"]
impl crate::Resettable for WWDG_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
