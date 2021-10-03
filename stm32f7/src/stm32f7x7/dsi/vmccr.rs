#[doc = "Register `VMCCR` reader"]
pub struct R(crate::R<VMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VMT` reader - Video mode Type"]
pub struct VMT_R(crate::FieldReader<u8, u8>);
impl VMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync time Enable"]
pub struct LPVSAE_R(crate::FieldReader<bool, bool>);
impl LPVSAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVSAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVSAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub struct LPVBPE_R(crate::FieldReader<bool, bool>);
impl LPVBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-Porch Enable"]
pub struct LPVFPE_R(crate::FieldReader<bool, bool>);
impl LPVFPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVFPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVFPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPVAE` reader - Low-Power Vertical Active Enable"]
pub struct LPVAE_R(crate::FieldReader<bool, bool>);
impl LPVAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPVAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPVAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPHBPE` reader - Low-power Horizontal Back-Porch Enable"]
pub struct LPHBPE_R(crate::FieldReader<bool, bool>);
impl LPHBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPHBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPHBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPHFE` reader - Low-Power Horizontal Front-Porch Enable"]
pub struct LPHFE_R(crate::FieldReader<bool, bool>);
impl LPHFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPHFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPHFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBTAAE` reader - Frame BTA Acknowledge Enable"]
pub struct FBTAAE_R(crate::FieldReader<bool, bool>);
impl FBTAAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBTAAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBTAAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub struct LPCE_R(crate::FieldReader<bool, bool>);
impl LPCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "DSI Host Video mode Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
pub struct VMCCR_SPEC;
impl crate::RegisterSpec for VMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmccr::R](R) reader structure"]
impl crate::Readable for VMCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
