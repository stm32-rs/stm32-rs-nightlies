#[doc = "Register `RGCFR` writer"]
pub struct W(crate::W<RGCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCFR_SPEC>;
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
impl From<crate::W<RGCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear trigger overrun event flag"]
pub type COF3_AW = COF0_AW;
#[doc = "Field `COF3` writer - Clear trigger overrun event flag"]
pub struct COF3_W<'a> {
    w: &'a mut W,
}
impl<'a> COF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COF3_AW::CLEAR)
    }
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
#[doc = "COF2"]
pub type COF2_AW = COF0_AW;
#[doc = "Field `COF2` writer - COF2"]
pub struct COF2_W<'a> {
    w: &'a mut W,
}
impl<'a> COF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COF2_AW::CLEAR)
    }
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
#[doc = "COF1"]
pub type COF1_AW = COF0_AW;
#[doc = "Field `COF1` writer - COF1"]
pub struct COF1_W<'a> {
    w: &'a mut W,
}
impl<'a> COF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COF1_AW::CLEAR)
    }
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
#[doc = "COF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COF0_AW {
    #[doc = "1: Clear overrun flag"]
    CLEAR = 1,
}
impl From<COF0_AW> for bool {
    #[inline(always)]
    fn from(variant: COF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COF0` writer - COF0"]
pub struct COF0_W<'a> {
    w: &'a mut W,
}
impl<'a> COF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear overrun flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COF0_AW::CLEAR)
    }
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
impl W {
    #[doc = "Bit 3 - Clear trigger overrun event flag"]
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W {
        COF3_W { w: self }
    }
    #[doc = "Bit 2 - COF2"]
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W {
        COF2_W { w: self }
    }
    #[doc = "Bit 1 - COF1"]
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W {
        COF1_W { w: self }
    }
    #[doc = "Bit 0 - COF0"]
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W {
        COF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "request generator interrupt clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](index.html) module"]
pub struct RGCFR_SPEC;
impl crate::RegisterSpec for RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rgcfr::W](W) writer structure"]
impl crate::Writable for RGCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
