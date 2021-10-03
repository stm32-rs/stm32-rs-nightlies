#[doc = "Register `CLRFR` reader"]
pub struct R(crate::R<CLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRERRFC` reader - Clear Address error flag"]
pub struct ADDRERRFC_R(crate::FieldReader<bool, bool>);
impl ADDRERRFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRERRFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRERRFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRERRFC` writer - Clear Address error flag"]
pub struct ADDRERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRERRFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RAMERRFC` reader - Clear RAM error flag"]
pub struct RAMERRFC_R(crate::FieldReader<bool, bool>);
impl RAMERRFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMERRFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMERRFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMERRFC` writer - Clear RAM error flag"]
pub struct RAMERRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMERRFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PROCENDFC` reader - Clear PKA End of Operation flag"]
pub struct PROCENDFC_R(crate::FieldReader<bool, bool>);
impl PROCENDFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROCENDFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCENDFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCENDFC` writer - Clear PKA End of Operation flag"]
pub struct PROCENDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCENDFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&self) -> ADDRERRFC_R {
        ADDRERRFC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    pub fn ramerrfc(&self) -> RAMERRFC_R {
        RAMERRFC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendfc(&self) -> PROCENDFC_R {
        PROCENDFC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W {
        ADDRERRFC_W { w: self }
    }
    #[doc = "Bit 19 - Clear RAM error flag"]
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W {
        RAMERRFC_W { w: self }
    }
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W {
        PROCENDFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrfr::R](R) reader structure"]
impl crate::Readable for CLRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
