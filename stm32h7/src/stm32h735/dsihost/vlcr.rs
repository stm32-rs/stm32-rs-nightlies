#[doc = "Register `VLCR` reader"]
pub struct R(crate::R<VLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLCR` writer"]
pub struct W(crate::W<VLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLCR_SPEC>;
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
impl From<crate::W<VLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HLINE` reader - Horizontal line duration"]
pub struct HLINE_R(crate::FieldReader<u16, u16>);
impl HLINE_R {
    pub(crate) fn new(bits: u16) -> Self {
        HLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLINE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLINE` writer - Horizontal line duration"]
pub struct HLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> HLINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Horizontal line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal line duration"]
    #[inline(always)]
    pub fn hline(&mut self) -> HLINE_W {
        HLINE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video line configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlcr](index.html) module"]
pub struct VLCR_SPEC;
impl crate::RegisterSpec for VLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlcr::R](R) reader structure"]
impl crate::Readable for VLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlcr::W](W) writer structure"]
impl crate::Writable for VLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLCR to value 0"]
impl crate::Resettable for VLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}