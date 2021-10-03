#[doc = "Register `MICR` writer"]
pub struct W(crate::W<MICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MICR_SPEC>;
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
impl From<crate::W<MICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master update Interrupt flag clear"]
pub type MUPDC_AW = MCMP1C_AW;
#[doc = "Field `MUPDC` writer - Master update Interrupt flag clear"]
pub struct MUPDC_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUPDC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MUPDC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Sync Input Interrupt flag clear"]
pub type SYNCC_AW = MCMP1C_AW;
#[doc = "Field `SYNCC` writer - Sync Input Interrupt flag clear"]
pub struct SYNCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SYNCC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Repetition Interrupt flag clear"]
pub type MREPC_AW = MCMP1C_AW;
#[doc = "Field `MREPC` writer - Repetition Interrupt flag clear"]
pub struct MREPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MREPC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Master Compare 4 Interrupt flag clear"]
pub type MCMP4C_AW = MCMP1C_AW;
#[doc = "Field `MCMP4C` writer - Master Compare 4 Interrupt flag clear"]
pub struct MCMP4C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MCMP4C_AW::CLEAR)
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
#[doc = "Master Compare 3 Interrupt flag clear"]
pub type MCMP3C_AW = MCMP1C_AW;
#[doc = "Field `MCMP3C` writer - Master Compare 3 Interrupt flag clear"]
pub struct MCMP3C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MCMP3C_AW::CLEAR)
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
#[doc = "Master Compare 2 Interrupt flag clear"]
pub type MCMP2C_AW = MCMP1C_AW;
#[doc = "Field `MCMP2C` writer - Master Compare 2 Interrupt flag clear"]
pub struct MCMP2C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MCMP2C_AW::CLEAR)
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
#[doc = "Master Compare 1 Interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMP1C_AW {
    #[doc = "1: Clears flag in MISR register"]
    CLEAR = 1,
}
impl From<MCMP1C_AW> for bool {
    #[inline(always)]
    fn from(variant: MCMP1C_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCMP1C` writer - Master Compare 1 Interrupt flag clear"]
pub struct MCMP1C_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1C_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears flag in MISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MCMP1C_AW::CLEAR)
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
    #[doc = "Bit 6 - Master update Interrupt flag clear"]
    #[inline(always)]
    pub fn mupdc(&mut self) -> MUPDC_W {
        MUPDC_W { w: self }
    }
    #[doc = "Bit 5 - Sync Input Interrupt flag clear"]
    #[inline(always)]
    pub fn syncc(&mut self) -> SYNCC_W {
        SYNCC_W { w: self }
    }
    #[doc = "Bit 4 - Repetition Interrupt flag clear"]
    #[inline(always)]
    pub fn mrepc(&mut self) -> MREPC_W {
        MREPC_W { w: self }
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp4c(&mut self) -> MCMP4C_W {
        MCMP4C_W { w: self }
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp3c(&mut self) -> MCMP3C_W {
        MCMP3C_W { w: self }
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp2c(&mut self) -> MCMP2C_W {
        MCMP2C_W { w: self }
    }
    #[doc = "Bit 0 - Master Compare 1 Interrupt flag clear"]
    #[inline(always)]
    pub fn mcmp1c(&mut self) -> MCMP1C_W {
        MCMP1C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](index.html) module"]
pub struct MICR_SPEC;
impl crate::RegisterSpec for MICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [micr::W](W) writer structure"]
impl crate::Writable for MICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
