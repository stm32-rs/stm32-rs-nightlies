#[doc = "Register `CH7DATINR` reader"]
pub struct R(crate::R<CH7DATINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7DATINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7DATINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7DATINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7DATINR` writer"]
pub struct W(crate::W<CH7DATINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7DATINR_SPEC>;
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
impl From<crate::W<CH7DATINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7DATINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDAT1` reader - INDAT1"]
pub struct INDAT1_R(crate::FieldReader<u16, u16>);
impl INDAT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        INDAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDAT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDAT1` writer - INDAT1"]
pub struct INDAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `INDAT0` reader - INDAT0"]
pub struct INDAT0_R(crate::FieldReader<u16, u16>);
impl INDAT0_R {
    pub(crate) fn new(bits: u16) -> Self {
        INDAT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INDAT0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INDAT0` writer - INDAT0"]
pub struct INDAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W {
        INDAT1_W { w: self }
    }
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W {
        INDAT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH7DATINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7datinr](index.html) module"]
pub struct CH7DATINR_SPEC;
impl crate::RegisterSpec for CH7DATINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7datinr::R](R) reader structure"]
impl crate::Readable for CH7DATINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7datinr::W](W) writer structure"]
impl crate::Writable for CH7DATINR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH7DATINR to value 0"]
impl crate::Resettable for CH7DATINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
