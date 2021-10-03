#[doc = "Register `EECR3` reader"]
pub struct R(crate::R<EECR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR3` writer"]
pub struct W(crate::W<EECR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR3_SPEC>;
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
impl From<crate::W<EECR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6F` reader - EE6F"]
pub struct EE6F_R(crate::FieldReader<u8, u8>);
impl EE6F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6F` writer - EE6F"]
pub struct EE6F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `EE7F` reader - EE7F"]
pub struct EE7F_R(crate::FieldReader<u8, u8>);
impl EE7F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE7F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7F` writer - EE7F"]
pub struct EE7F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `EE8F` reader - EE8F"]
pub struct EE8F_R(crate::FieldReader<u8, u8>);
impl EE8F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE8F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8F` writer - EE8F"]
pub struct EE8F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `EE9F` reader - EE9F"]
pub struct EE9F_R(crate::FieldReader<u8, u8>);
impl EE9F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE9F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9F` writer - EE9F"]
pub struct EE9F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `EE10F` reader - EE10F"]
pub struct EE10F_R(crate::FieldReader<u8, u8>);
impl EE10F_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE10F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10F` writer - EE10F"]
pub struct EE10F_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `EEVSD` reader - EEVSD"]
pub struct EEVSD_R(crate::FieldReader<u8, u8>);
impl EEVSD_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EEVSD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVSD` writer - EEVSD"]
pub struct EEVSD_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&self) -> EE6F_R {
        EE6F_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&self) -> EE7F_R {
        EE7F_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&self) -> EE8F_R {
        EE8F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&self) -> EE9F_R {
        EE9F_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&self) -> EE10F_R {
        EE10F_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&self) -> EEVSD_R {
        EEVSD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EE6F"]
    #[inline(always)]
    pub fn ee6f(&mut self) -> EE6F_W {
        EE6F_W { w: self }
    }
    #[doc = "Bits 6:9 - EE7F"]
    #[inline(always)]
    pub fn ee7f(&mut self) -> EE7F_W {
        EE7F_W { w: self }
    }
    #[doc = "Bits 12:15 - EE8F"]
    #[inline(always)]
    pub fn ee8f(&mut self) -> EE8F_W {
        EE8F_W { w: self }
    }
    #[doc = "Bits 18:21 - EE9F"]
    #[inline(always)]
    pub fn ee9f(&mut self) -> EE9F_W {
        EE9F_W { w: self }
    }
    #[doc = "Bits 24:27 - EE10F"]
    #[inline(always)]
    pub fn ee10f(&mut self) -> EE10F_W {
        EE10F_W { w: self }
    }
    #[doc = "Bits 30:31 - EEVSD"]
    #[inline(always)]
    pub fn eevsd(&mut self) -> EEVSD_W {
        EEVSD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr3](index.html) module"]
pub struct EECR3_SPEC;
impl crate::RegisterSpec for EECR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr3::R](R) reader structure"]
impl crate::Readable for EECR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr3::W](W) writer structure"]
impl crate::Writable for EECR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR3 to value 0"]
impl crate::Resettable for EECR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
