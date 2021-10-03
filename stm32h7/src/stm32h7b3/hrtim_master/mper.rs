#[doc = "Register `MPER` reader"]
pub struct R(crate::R<MPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPER` writer"]
pub struct W(crate::W<MPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPER_SPEC>;
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
impl From<crate::W<MPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPER` reader - Master Timer Period value"]
pub struct MPER_R(crate::FieldReader<u16, u16>);
impl MPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPER` writer - Master Timer Period value"]
pub struct MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W {
        MPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mper](index.html) module"]
pub struct MPER_SPEC;
impl crate::RegisterSpec for MPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mper::R](R) reader structure"]
impl crate::Readable for MPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mper::W](W) writer structure"]
impl crate::Writable for MPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPER to value 0xffff"]
impl crate::Resettable for MPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
