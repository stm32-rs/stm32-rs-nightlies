#[doc = "Register `SHSR2` reader"]
pub struct R(crate::R<SHSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHSR2` writer"]
pub struct W(crate::W<SHSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHSR2_SPEC>;
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
impl From<crate::W<SHSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAMPLE2` reader - DAC Channel 2 sample Time"]
pub struct TSAMPLE2_R(crate::FieldReader<u16, u16>);
impl TSAMPLE2_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSAMPLE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSAMPLE2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAMPLE2` writer - DAC Channel 2 sample Time"]
pub struct TSAMPLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    pub fn tsample2(&mut self) -> TSAMPLE2_W {
        TSAMPLE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample and Hold sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shsr2](index.html) module"]
pub struct SHSR2_SPEC;
impl crate::RegisterSpec for SHSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shsr2::R](R) reader structure"]
impl crate::Readable for SHSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shsr2::W](W) writer structure"]
impl crate::Writable for SHSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHSR2 to value 0"]
impl crate::Resettable for SHSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
