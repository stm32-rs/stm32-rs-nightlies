#[doc = "Register `HWCFGR` reader"]
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TECHNO` reader - TECHNO"]
pub struct TECHNO_R(crate::FieldReader<u8, u8>);
impl TECHNO_R {
    pub(crate) fn new(bits: u8) -> Self {
        TECHNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECHNO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOSIZE` reader - FIFOSIZE"]
pub struct FIFOSIZE_R(crate::FieldReader<u16, u16>);
impl FIFOSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        FIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - TECHNO"]
    #[inline(always)]
    pub fn techno(&self) -> TECHNO_R {
        TECHNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[doc = "DSI Host hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](index.html) module"]
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr::R](R) reader structure"]
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR to value 0x5a01"]
impl crate::Resettable for HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5a01
    }
}
