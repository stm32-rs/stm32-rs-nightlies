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
#[doc = "Field `VMT` reader - VMT"]
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
#[doc = "Field `LPVSAE` reader - LPVSAE"]
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
#[doc = "Field `LPVBPE` reader - LPVBPE"]
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
#[doc = "Field `LPVFPE` reader - LPVFPE"]
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
#[doc = "Field `LPVAE` reader - LPVAE"]
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
#[doc = "Field `LPHBPE` reader - LPHBPE"]
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
#[doc = "Field `LPHFE` reader - LPHFE"]
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
#[doc = "Field `FBTAAE` reader - FBTAAE"]
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
#[doc = "Field `LPCE` reader - LPCE"]
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
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPHFE"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
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
