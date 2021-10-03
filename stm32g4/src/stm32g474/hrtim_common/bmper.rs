#[doc = "Register `BMPER` reader"]
pub struct R(crate::R<BMPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMPER` writer"]
pub struct W(crate::W<BMPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMPER_SPEC>;
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
impl From<crate::W<BMPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMPER` reader - Burst mode Period"]
pub struct BMPER_R(crate::FieldReader<u16, u16>);
impl BMPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        BMPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMPER` writer - Burst mode Period"]
pub struct BMPER_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&mut self) -> BMPER_W {
        BMPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst Mode Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmper](index.html) module"]
pub struct BMPER_SPEC;
impl crate::RegisterSpec for BMPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmper::R](R) reader structure"]
impl crate::Readable for BMPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmper::W](W) writer structure"]
impl crate::Writable for BMPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMPER to value 0"]
impl crate::Resettable for BMPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
