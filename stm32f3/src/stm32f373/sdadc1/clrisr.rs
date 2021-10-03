#[doc = "Register `CLRISR` reader"]
pub struct R(crate::R<CLRISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRISR` writer"]
pub struct W(crate::W<CLRISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRISR_SPEC>;
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
impl From<crate::W<CLRISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub struct CLRROVRF_R(crate::FieldReader<bool, bool>);
impl CLRROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRROVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRROVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub struct CLRROVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRROVRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub struct CLRJOVRF_R(crate::FieldReader<bool, bool>);
impl CLRJOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLRJOVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRJOVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub struct CLRJOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRJOVRF_W<'a> {
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
#[doc = "Field `CLREOCALF` reader - Clear the end of calibration flag"]
pub struct CLREOCALF_R(crate::FieldReader<bool, bool>);
impl CLREOCALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLREOCALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLREOCALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLREOCALF` writer - Clear the end of calibration flag"]
pub struct CLREOCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLREOCALF_W<'a> {
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
impl R {
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    pub fn clreocalf(&self) -> CLREOCALF_R {
        CLREOCALF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W {
        CLRROVRF_W { w: self }
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W {
        CLRJOVRF_W { w: self }
    }
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    pub fn clreocalf(&mut self) -> CLREOCALF_W {
        CLREOCALF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt and status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrisr](index.html) module"]
pub struct CLRISR_SPEC;
impl crate::RegisterSpec for CLRISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrisr::R](R) reader structure"]
impl crate::Readable for CLRISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrisr::W](W) writer structure"]
impl crate::Writable for CLRISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRISR to value 0"]
impl crate::Resettable for CLRISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
