#[doc = "Register `R3CFGR` reader"]
pub struct R(crate::R<R3CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R3CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R3CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R3CFGR` writer"]
pub struct W(crate::W<R3CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R3CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R3CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_EN` reader - region on-the-fly decryption enable"]
pub struct REG_EN_R(crate::FieldReader<bool, bool>);
impl REG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_EN` writer - region on-the-fly decryption enable"]
pub struct REG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CONFIGLOCK` reader - region config lock"]
pub struct CONFIGLOCK_R(crate::FieldReader<bool, bool>);
impl CONFIGLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONFIGLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFIGLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFIGLOCK` writer - region config lock"]
pub struct CONFIGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIGLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `KEYLOCK` reader - region key lock"]
pub struct KEYLOCK_R(crate::FieldReader<bool, bool>);
impl KEYLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEYLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYLOCK` writer - region key lock"]
pub struct KEYLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MODE` reader - operating mode"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - operating mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `KEYCRC` reader - region key 8-bit CRC"]
pub struct KEYCRC_R(crate::FieldReader<u8, u8>);
impl KEYCRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGx_VERSION` reader - region firmware version"]
pub struct REGX_VERSION_R(crate::FieldReader<u16, u16>);
impl REGX_VERSION_R {
    pub(crate) fn new(bits: u16) -> Self {
        REGX_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGX_VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGx_VERSION` writer - region firmware version"]
pub struct REGX_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - region key 8-bit CRC"]
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - region on-the-fly decryption enable"]
    #[inline(always)]
    pub fn reg_en(&mut self) -> REG_EN_W {
        REG_EN_W { w: self }
    }
    #[doc = "Bit 1 - region config lock"]
    #[inline(always)]
    pub fn configlock(&mut self) -> CONFIGLOCK_W {
        CONFIGLOCK_W { w: self }
    }
    #[doc = "Bit 2 - region key lock"]
    #[inline(always)]
    pub fn keylock(&mut self) -> KEYLOCK_W {
        KEYLOCK_W { w: self }
    }
    #[doc = "Bits 4:5 - operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 16:31 - region firmware version"]
    #[inline(always)]
    pub fn regx_version(&mut self) -> REGX_VERSION_W {
        REGX_VERSION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3cfgr](index.html) module"]
pub struct R3CFGR_SPEC;
impl crate::RegisterSpec for R3CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r3cfgr::R](R) reader structure"]
impl crate::Readable for R3CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r3cfgr::W](W) writer structure"]
impl crate::Writable for R3CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R3CFGR to value 0"]
impl crate::Resettable for R3CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
