#[doc = "Register `PKA_CLRFR` writer"]
pub struct W(crate::W<PKA_CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKA_CLRFR_SPEC>;
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
impl From<crate::W<PKA_CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKA_CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCENDFC` writer - clear PKA end of operation flag"]
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
#[doc = "Field `RAMERRFC` writer - CLEAR PKA RAM ERROR FLAG"]
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
#[doc = "Field `ADDRERRFC` writer - clear address error flag"]
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
impl W {
    #[doc = "Bit 17 - clear PKA end of operation flag"]
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W {
        PROCENDFC_W { w: self }
    }
    #[doc = "Bit 19 - CLEAR PKA RAM ERROR FLAG"]
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W {
        RAMERRFC_W { w: self }
    }
    #[doc = "Bit 20 - clear address error flag"]
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W {
        ADDRERRFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_clrfr](index.html) module"]
pub struct PKA_CLRFR_SPEC;
impl crate::RegisterSpec for PKA_CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pka_clrfr::W](W) writer structure"]
impl crate::Writable for PKA_CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKA_CLRFR to value 0"]
impl crate::Resettable for PKA_CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
