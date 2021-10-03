#[doc = "Register `JOFR3` reader"]
pub struct R(crate::R<JOFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JOFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JOFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JOFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JOFR3` writer"]
pub struct W(crate::W<JOFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JOFR3_SPEC>;
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
impl From<crate::W<JOFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JOFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JOFFSET3` reader - Data offset for injected channel x"]
pub struct JOFFSET3_R(crate::FieldReader<u16, u16>);
impl JOFFSET3_R {
    pub(crate) fn new(bits: u16) -> Self {
        JOFFSET3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOFFSET3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOFFSET3` writer - Data offset for injected channel x"]
pub struct JOFFSET3_W<'a> {
    w: &'a mut W,
}
impl<'a> JOFFSET3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset3(&self) -> JOFFSET3_R {
        JOFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset3(&mut self) -> JOFFSET3_W {
        JOFFSET3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr3](index.html) module"]
pub struct JOFR3_SPEC;
impl crate::RegisterSpec for JOFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jofr3::R](R) reader structure"]
impl crate::Readable for JOFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jofr3::W](W) writer structure"]
impl crate::Writable for JOFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JOFR3 to value 0"]
impl crate::Resettable for JOFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
