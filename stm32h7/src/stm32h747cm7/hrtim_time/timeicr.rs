#[doc = "Register `TIMEICR` writer"]
pub struct W(crate::W<TIMEICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEICR_SPEC>;
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
impl From<crate::W<TIMEICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYPRTC` writer - Delayed Protection Flag Clear"]
pub struct DLYPRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RSTC` writer - Reset Interrupt flag Clear"]
pub struct RSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RSTx2C` writer - Output 2 Reset flag Clear"]
pub struct RSTX2C_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SET2xC` writer - Output 2 Set flag Clear"]
pub struct SET2XC_W<'a> {
    w: &'a mut W,
}
impl<'a> SET2XC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RSTx1C` writer - Output 1 Reset flag Clear"]
pub struct RSTX1C_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SET1xC` writer - Output 1 Set flag Clear"]
pub struct SET1XC_W<'a> {
    w: &'a mut W,
}
impl<'a> SET1XC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CPT2C` writer - Capture2 Interrupt flag Clear"]
pub struct CPT2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CPT1C` writer - Capture1 Interrupt flag Clear"]
pub struct CPT1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `UPDC` writer - Update Interrupt flag Clear"]
pub struct UPDC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDC_W<'a> {
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
#[doc = "Field `REPC` writer - Repetition Interrupt flag Clear"]
pub struct REPC_W<'a> {
    w: &'a mut W,
}
impl<'a> REPC_W<'a> {
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
#[doc = "Field `CMP4C` writer - Compare 4 Interrupt flag Clear"]
pub struct CMP4C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4C_W<'a> {
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
#[doc = "Field `CMP3C` writer - Compare 3 Interrupt flag Clear"]
pub struct CMP3C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3C_W<'a> {
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
#[doc = "Field `CMP2C` writer - Compare 2 Interrupt flag Clear"]
pub struct CMP2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2C_W<'a> {
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
#[doc = "Field `CMP1C` writer - Compare 1 Interrupt flag Clear"]
pub struct CMP1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1C_W<'a> {
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
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W {
        DLYPRTC_W { w: self }
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RSTC_W {
        RSTC_W { w: self }
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    pub fn rstx2c(&mut self) -> RSTX2C_W {
        RSTX2C_W { w: self }
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    pub fn set2x_c(&mut self) -> SET2XC_W {
        SET2XC_W { w: self }
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    pub fn rstx1c(&mut self) -> RSTX1C_W {
        RSTX1C_W { w: self }
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    pub fn set1x_c(&mut self) -> SET1XC_W {
        SET1XC_W { w: self }
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cpt2c(&mut self) -> CPT2C_W {
        CPT2C_W { w: self }
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cpt1c(&mut self) -> CPT1C_W {
        CPT1C_W { w: self }
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    pub fn updc(&mut self) -> UPDC_W {
        UPDC_W { w: self }
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    pub fn repc(&mut self) -> REPC_W {
        REPC_W { w: self }
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp4c(&mut self) -> CMP4C_W {
        CMP4C_W { w: self }
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp3c(&mut self) -> CMP3C_W {
        CMP3C_W { w: self }
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp2c(&mut self) -> CMP2C_W {
        CMP2C_W { w: self }
    }
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp1c(&mut self) -> CMP1C_W {
        CMP1C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeicr](index.html) module"]
pub struct TIMEICR_SPEC;
impl crate::RegisterSpec for TIMEICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [timeicr::W](W) writer structure"]
impl crate::Writable for TIMEICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEICR to value 0"]
impl crate::Resettable for TIMEICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
