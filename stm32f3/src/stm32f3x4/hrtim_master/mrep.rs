#[doc = "Register `MREP` reader"]
pub struct R(crate::R<MREP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MREP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MREP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MREP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MREP` writer"]
pub struct W(crate::W<MREP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MREP_SPEC>;
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
impl From<crate::W<MREP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MREP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MREP` reader - Master Timer Repetition counter value"]
pub struct MREP_R(crate::FieldReader<u8, u8>);
impl MREP_R {
    pub(crate) fn new(bits: u8) -> Self {
        MREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREP` writer - Master Timer Repetition counter value"]
pub struct MREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master Timer Repetition counter value"]
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W {
        MREP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrep](index.html) module"]
pub struct MREP_SPEC;
impl crate::RegisterSpec for MREP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrep::R](R) reader structure"]
impl crate::Readable for MREP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrep::W](W) writer structure"]
impl crate::Writable for MREP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MREP to value 0"]
impl crate::Resettable for MREP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
