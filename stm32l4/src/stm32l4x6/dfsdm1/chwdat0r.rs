#[doc = "Register `CHWDAT0R` reader"]
pub struct R(crate::R<CHWDAT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHWDAT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHWDAT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHWDAT0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHWDAT0R` writer"]
pub struct W(crate::W<CHWDAT0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHWDAT0R_SPEC>;
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
impl From<crate::W<CHWDAT0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHWDAT0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - WDATA"]
pub struct WDATA_R(crate::FieldReader<u16, u16>);
impl WDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDATA` writer - WDATA"]
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chwdat0r](index.html) module"]
pub struct CHWDAT0R_SPEC;
impl crate::RegisterSpec for CHWDAT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chwdat0r::R](R) reader structure"]
impl crate::Readable for CHWDAT0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chwdat0r::W](W) writer structure"]
impl crate::Writable for CHWDAT0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHWDAT0R to value 0"]
impl crate::Resettable for CHWDAT0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
