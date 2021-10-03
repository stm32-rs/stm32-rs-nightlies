#[doc = "Register `MDIOS_CLRFR` reader"]
pub struct R(crate::R<MDIOS_CLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_CLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_CLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_CLRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIOS_CLRFR` writer"]
pub struct W(crate::W<MDIOS_CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_CLRFR_SPEC>;
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
impl From<crate::W<MDIOS_CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPERF` reader - Clear the preamble error flag"]
pub struct CPERF_R(crate::FieldReader<bool, bool>);
impl CPERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPERF` writer - Clear the preamble error flag"]
pub struct CPERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPERF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CSERF` reader - Clear the start error flag"]
pub struct CSERF_R(crate::FieldReader<bool, bool>);
impl CSERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSERF` writer - Clear the start error flag"]
pub struct CSERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSERF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CTERF` reader - Clear the turnaround error flag"]
pub struct CTERF_R(crate::FieldReader<bool, bool>);
impl CTERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTERF` writer - Clear the turnaround error flag"]
pub struct CTERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTERF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&mut self) -> CPERF_W {
        CPERF_W { w: self }
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&mut self) -> CSERF_W {
        CSERF_W { w: self }
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&mut self) -> CTERF_W {
        CTERF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_clrfr](index.html) module"]
pub struct MDIOS_CLRFR_SPEC;
impl crate::RegisterSpec for MDIOS_CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_clrfr::R](R) reader structure"]
impl crate::Readable for MDIOS_CLRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdios_clrfr::W](W) writer structure"]
impl crate::Writable for MDIOS_CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIOS_CLRFR to value 0"]
impl crate::Resettable for MDIOS_CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
