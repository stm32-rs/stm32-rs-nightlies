#[doc = "Register `CSELR` reader"]
pub struct R(crate::R<CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSELR` writer"]
pub struct W(crate::W<CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSELR_SPEC>;
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
impl From<crate::W<CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C7S` reader - DMA channel 7 selection"]
pub struct C7S_R(crate::FieldReader<u8, u8>);
impl C7S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C7S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C7S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C7S` writer - DMA channel 7 selection"]
pub struct C7S_W<'a> {
    w: &'a mut W,
}
impl<'a> C7S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `C6S` reader - DMA channel 6 selection"]
pub struct C6S_R(crate::FieldReader<u8, u8>);
impl C6S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C6S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C6S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C6S` writer - DMA channel 6 selection"]
pub struct C6S_W<'a> {
    w: &'a mut W,
}
impl<'a> C6S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `C5S` reader - DMA channel 5 selection"]
pub struct C5S_R(crate::FieldReader<u8, u8>);
impl C5S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C5S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C5S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C5S` writer - DMA channel 5 selection"]
pub struct C5S_W<'a> {
    w: &'a mut W,
}
impl<'a> C5S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `C4S` reader - DMA channel 4 selection"]
pub struct C4S_R(crate::FieldReader<u8, u8>);
impl C4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C4S` writer - DMA channel 4 selection"]
pub struct C4S_W<'a> {
    w: &'a mut W,
}
impl<'a> C4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `C3S` reader - DMA channel 3 selection"]
pub struct C3S_R(crate::FieldReader<u8, u8>);
impl C3S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C3S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C3S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C3S` writer - DMA channel 3 selection"]
pub struct C3S_W<'a> {
    w: &'a mut W,
}
impl<'a> C3S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `C2S` reader - DMA channel 2 selection"]
pub struct C2S_R(crate::FieldReader<u8, u8>);
impl C2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2S` writer - DMA channel 2 selection"]
pub struct C2S_W<'a> {
    w: &'a mut W,
}
impl<'a> C2S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `C1S` reader - DMA channel 1 selection"]
pub struct C1S_R(crate::FieldReader<u8, u8>);
impl C1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        C1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1S` writer - DMA channel 1 selection"]
pub struct C1S_W<'a> {
    w: &'a mut W,
}
impl<'a> C1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&mut self) -> C7S_W {
        C7S_W { w: self }
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&mut self) -> C6S_W {
        C6S_W { w: self }
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&mut self) -> C5S_W {
        C5S_W { w: self }
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&mut self) -> C4S_W {
        C4S_W { w: self }
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&mut self) -> C3S_W {
        C3S_W { w: self }
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&mut self) -> C2S_W {
        C2S_W { w: self }
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&mut self) -> C1S_W {
        C1S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cselr](index.html) module"]
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cselr::R](R) reader structure"]
impl crate::Readable for CSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cselr::W](W) writer structure"]
impl crate::Writable for CSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
