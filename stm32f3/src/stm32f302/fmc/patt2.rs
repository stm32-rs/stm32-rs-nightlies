#[doc = "Register `PATT2` reader"]
pub struct R(crate::R<PATT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT2` writer"]
pub struct W(crate::W<PATT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT2_SPEC>;
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
impl From<crate::W<PATT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTHIZx` reader - ATTHIZx"]
pub struct ATTHIZX_R(crate::FieldReader<u8, u8>);
impl ATTHIZX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHIZX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHIZX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHIZx` writer - ATTHIZx"]
pub struct ATTHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ATTHOLDx` reader - ATTHOLDx"]
pub struct ATTHOLDX_R(crate::FieldReader<u8, u8>);
impl ATTHOLDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTHOLDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTHOLDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTHOLDx` writer - ATTHOLDx"]
pub struct ATTHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ATTWAITx` reader - ATTWAITx"]
pub struct ATTWAITX_R(crate::FieldReader<u8, u8>);
impl ATTWAITX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTWAITX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTWAITX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTWAITx` writer - ATTWAITx"]
pub struct ATTWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ATTSETx` reader - ATTSETx"]
pub struct ATTSETX_R(crate::FieldReader<u8, u8>);
impl ATTSETX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ATTSETX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTSETX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTSETx` writer - ATTSETx"]
pub struct ATTSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W {
        ATTHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W {
        ATTHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W {
        ATTWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W {
        ATTSETX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Attribute memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt2](index.html) module"]
pub struct PATT2_SPEC;
impl crate::RegisterSpec for PATT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt2::R](R) reader structure"]
impl crate::Readable for PATT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt2::W](W) writer structure"]
impl crate::Writable for PATT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
