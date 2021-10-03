#[doc = "Register `ITLINE0` reader"]
pub struct R(crate::R<ITLINE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WWDG` reader - Window watchdog interrupt pending flag"]
pub struct WWDG_R(crate::FieldReader<bool, bool>);
impl WWDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Window watchdog interrupt pending flag"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt line 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline0](index.html) module"]
pub struct ITLINE0_SPEC;
impl crate::RegisterSpec for ITLINE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline0::R](R) reader structure"]
impl crate::Readable for ITLINE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE0 to value 0"]
impl crate::Resettable for ITLINE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
