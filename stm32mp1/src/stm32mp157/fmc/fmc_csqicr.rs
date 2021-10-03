#[doc = "Register `FMC_CSQICR` writer"]
pub struct W(crate::W<FMC_CSQICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQICR_SPEC>;
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
impl From<crate::W<FMC_CSQICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTCF` writer - CTCF"]
pub struct CTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCF_W<'a> {
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
#[doc = "Field `CSCF` writer - CSCF"]
pub struct CSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCF_W<'a> {
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
#[doc = "Field `CSEF` writer - CSEF"]
pub struct CSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEF_W<'a> {
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
#[doc = "Field `CSUEF` writer - CSUEF"]
pub struct CSUEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSUEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CCMDTCF` writer - CCMDTCF"]
pub struct CCMDTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMDTCF_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CTCF"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W {
        CTCF_W { w: self }
    }
    #[doc = "Bit 1 - CSCF"]
    #[inline(always)]
    pub fn cscf(&mut self) -> CSCF_W {
        CSCF_W { w: self }
    }
    #[doc = "Bit 2 - CSEF"]
    #[inline(always)]
    pub fn csef(&mut self) -> CSEF_W {
        CSEF_W { w: self }
    }
    #[doc = "Bit 3 - CSUEF"]
    #[inline(always)]
    pub fn csuef(&mut self) -> CSUEF_W {
        CSUEF_W { w: self }
    }
    #[doc = "Bit 4 - CCMDTCF"]
    #[inline(always)]
    pub fn ccmdtcf(&mut self) -> CCMDTCF_W {
        CCMDTCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqicr](index.html) module"]
pub struct FMC_CSQICR_SPEC;
impl crate::RegisterSpec for FMC_CSQICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqicr::W](W) writer structure"]
impl crate::Writable for FMC_CSQICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_CSQICR to value 0"]
impl crate::Resettable for FMC_CSQICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
