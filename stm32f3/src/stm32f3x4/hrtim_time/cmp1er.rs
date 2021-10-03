#[doc = "Register `CMP1ER` reader"]
pub struct R(crate::R<CMP1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP1ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP1ER` writer"]
pub struct W(crate::W<CMP1ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP1ER_SPEC>;
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
impl From<crate::W<CMP1ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP1ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1x` reader - Timerx Compare 1 value"]
pub struct CMP1X_R(crate::FieldReader<u16, u16>);
impl CMP1X_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMP1X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1x` writer - Timerx Compare 1 value"]
pub struct CMP1X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&self) -> CMP1X_R {
        CMP1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 1 value"]
    #[inline(always)]
    pub fn cmp1x(&mut self) -> CMP1X_W {
        CMP1X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1er](index.html) module"]
pub struct CMP1ER_SPEC;
impl crate::RegisterSpec for CMP1ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp1er::R](R) reader structure"]
impl crate::Readable for CMP1ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp1er::W](W) writer structure"]
impl crate::Writable for CMP1ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP1ER to value 0"]
impl crate::Resettable for CMP1ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
