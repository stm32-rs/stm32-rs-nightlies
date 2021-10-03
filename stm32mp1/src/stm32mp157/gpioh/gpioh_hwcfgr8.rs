#[doc = "Register `GPIOH_HWCFGR8` reader"]
pub struct R(crate::R<GPIOH_HWCFGR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOH_HWCFGR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOH_HWCFGR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOH_HWCFGR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AF_PRIO8` reader - AF_PRIO8"]
pub struct AF_PRIO8_R(crate::FieldReader<u8, u8>);
impl AF_PRIO8_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO9` reader - AF_PRIO9"]
pub struct AF_PRIO9_R(crate::FieldReader<u8, u8>);
impl AF_PRIO9_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO10` reader - AF_PRIO10"]
pub struct AF_PRIO10_R(crate::FieldReader<u8, u8>);
impl AF_PRIO10_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO11` reader - AF_PRIO11"]
pub struct AF_PRIO11_R(crate::FieldReader<u8, u8>);
impl AF_PRIO11_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO12` reader - AF_PRIO12"]
pub struct AF_PRIO12_R(crate::FieldReader<u8, u8>);
impl AF_PRIO12_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO13` reader - AF_PRIO13"]
pub struct AF_PRIO13_R(crate::FieldReader<u8, u8>);
impl AF_PRIO13_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO14` reader - AF_PRIO14"]
pub struct AF_PRIO14_R(crate::FieldReader<u8, u8>);
impl AF_PRIO14_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AF_PRIO15` reader - AF_PRIO15"]
pub struct AF_PRIO15_R(crate::FieldReader<u8, u8>);
impl AF_PRIO15_R {
    pub(crate) fn new(bits: u8) -> Self {
        AF_PRIO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AF_PRIO15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AF_PRIO8"]
    #[inline(always)]
    pub fn af_prio8(&self) -> AF_PRIO8_R {
        AF_PRIO8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO9"]
    #[inline(always)]
    pub fn af_prio9(&self) -> AF_PRIO9_R {
        AF_PRIO9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO10"]
    #[inline(always)]
    pub fn af_prio10(&self) -> AF_PRIO10_R {
        AF_PRIO10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO11"]
    #[inline(always)]
    pub fn af_prio11(&self) -> AF_PRIO11_R {
        AF_PRIO11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO12"]
    #[inline(always)]
    pub fn af_prio12(&self) -> AF_PRIO12_R {
        AF_PRIO12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO13"]
    #[inline(always)]
    pub fn af_prio13(&self) -> AF_PRIO13_R {
        AF_PRIO13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO14"]
    #[inline(always)]
    pub fn af_prio14(&self) -> AF_PRIO14_R {
        AF_PRIO14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO15"]
    #[inline(always)]
    pub fn af_prio15(&self) -> AF_PRIO15_R {
        AF_PRIO15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr8](index.html) module"]
pub struct GPIOH_HWCFGR8_SPEC;
impl crate::RegisterSpec for GPIOH_HWCFGR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioh_hwcfgr8::R](R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOH_HWCFGR8 to value 0"]
impl crate::Resettable for GPIOH_HWCFGR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
