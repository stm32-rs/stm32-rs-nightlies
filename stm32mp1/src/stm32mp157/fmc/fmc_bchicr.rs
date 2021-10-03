#[doc = "Register `FMC_BCHICR` writer"]
pub struct W(crate::W<FMC_BCHICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BCHICR_SPEC>;
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
impl From<crate::W<FMC_BCHICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BCHICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDUEF` writer - CDUEF"]
pub struct CDUEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDUEF_W<'a> {
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
#[doc = "Field `CDERF` writer - CDERF"]
pub struct CDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDERF_W<'a> {
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
#[doc = "Field `CDEFF` writer - CDEFF"]
pub struct CDEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEFF_W<'a> {
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
#[doc = "Field `CDSRF` writer - CDSRF"]
pub struct CDSRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSRF_W<'a> {
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
#[doc = "Field `CEPBRF` writer - CEPBRF"]
pub struct CEPBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPBRF_W<'a> {
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
    #[doc = "Bit 0 - CDUEF"]
    #[inline(always)]
    pub fn cduef(&mut self) -> CDUEF_W {
        CDUEF_W { w: self }
    }
    #[doc = "Bit 1 - CDERF"]
    #[inline(always)]
    pub fn cderf(&mut self) -> CDERF_W {
        CDERF_W { w: self }
    }
    #[doc = "Bit 2 - CDEFF"]
    #[inline(always)]
    pub fn cdeff(&mut self) -> CDEFF_W {
        CDEFF_W { w: self }
    }
    #[doc = "Bit 3 - CDSRF"]
    #[inline(always)]
    pub fn cdsrf(&mut self) -> CDSRF_W {
        CDSRF_W { w: self }
    }
    #[doc = "Bit 4 - CEPBRF"]
    #[inline(always)]
    pub fn cepbrf(&mut self) -> CEPBRF_W {
        CEPBRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC BCH Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchicr](index.html) module"]
pub struct FMC_BCHICR_SPEC;
impl crate::RegisterSpec for FMC_BCHICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fmc_bchicr::W](W) writer structure"]
impl crate::Writable for FMC_BCHICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_BCHICR to value 0"]
impl crate::Resettable for FMC_BCHICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
