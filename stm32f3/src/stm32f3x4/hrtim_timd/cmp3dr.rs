#[doc = "Register `CMP3DR` reader"]
pub struct R(crate::R<CMP3DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP3DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP3DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP3DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP3DR` writer"]
pub struct W(crate::W<CMP3DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP3DR_SPEC>;
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
impl From<crate::W<CMP3DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP3DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub struct CMP3X_R(crate::FieldReader<u16, u16>);
impl CMP3X_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMP3X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub struct CMP3X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&mut self) -> CMP3X_W {
        CMP3X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3dr](index.html) module"]
pub struct CMP3DR_SPEC;
impl crate::RegisterSpec for CMP3DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp3dr::R](R) reader structure"]
impl crate::Readable for CMP3DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp3dr::W](W) writer structure"]
impl crate::Writable for CMP3DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP3DR to value 0"]
impl crate::Resettable for CMP3DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
