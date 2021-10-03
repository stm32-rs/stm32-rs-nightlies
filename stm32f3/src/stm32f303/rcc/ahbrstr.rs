#[doc = "Register `AHBRSTR` reader"]
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRSTR` writer"]
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FMC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCRST` reader - FMC reset"]
pub struct FMCRST_R(crate::FieldReader<bool, FMCRST_A>);
impl FMCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FMCRST_A> {
        match self.bits {
            true => Some(FMCRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == FMCRST_A::RESET
    }
}
impl core::ops::Deref for FMCRST_R {
    type Target = crate::FieldReader<bool, FMCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCRST` writer - FMC reset"]
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::RESET)
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
#[doc = "I/O port H reset"]
pub type IOPHRST_A = FMCRST_A;
#[doc = "Field `IOPHRST` reader - I/O port H reset"]
pub type IOPHRST_R = FMCRST_R;
#[doc = "Field `IOPHRST` writer - I/O port H reset"]
pub struct IOPHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPHRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "I/O port A reset"]
pub type IOPARST_A = FMCRST_A;
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = FMCRST_R;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub struct IOPARST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPARST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "I/O port B reset"]
pub type IOPBRST_A = FMCRST_A;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub type IOPBRST_R = FMCRST_R;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub struct IOPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPBRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "I/O port C reset"]
pub type IOPCRST_A = FMCRST_A;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub type IOPCRST_R = FMCRST_R;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub struct IOPCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "I/O port D reset"]
pub type IOPDRST_A = FMCRST_A;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub type IOPDRST_R = FMCRST_R;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub struct IOPDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPDRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "I/O port E reset"]
pub type IOPERST_A = FMCRST_A;
#[doc = "Field `IOPERST` reader - I/O port E reset"]
pub type IOPERST_R = FMCRST_R;
#[doc = "Field `IOPERST` writer - I/O port E reset"]
pub struct IOPERST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPERST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPERST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "I/O port F reset"]
pub type IOPFRST_A = FMCRST_A;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub type IOPFRST_R = FMCRST_R;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub struct IOPFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPFRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Touch sensing controller reset"]
pub type IOPGRST_A = FMCRST_A;
#[doc = "Field `IOPGRST` reader - Touch sensing controller reset"]
pub type IOPGRST_R = FMCRST_R;
#[doc = "Field `IOPGRST` writer - Touch sensing controller reset"]
pub struct IOPGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IOPGRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Touch sensing controller reset"]
pub type TSCRST_A = FMCRST_A;
#[doc = "Field `TSCRST` reader - Touch sensing controller reset"]
pub type TSCRST_R = FMCRST_R;
#[doc = "Field `TSCRST` writer - Touch sensing controller reset"]
pub struct TSCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TSCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "ADC1 and ADC2 reset"]
pub type ADC12RST_A = FMCRST_A;
#[doc = "Field `ADC12RST` reader - ADC1 and ADC2 reset"]
pub type ADC12RST_R = FMCRST_R;
#[doc = "Field `ADC12RST` writer - ADC1 and ADC2 reset"]
pub struct ADC12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADC12RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "ADC3 and ADC4 reset"]
pub type ADC34RST_A = FMCRST_A;
#[doc = "Field `ADC34RST` reader - ADC3 and ADC4 reset"]
pub type ADC34RST_R = FMCRST_R;
#[doc = "Field `ADC34RST` writer - ADC3 and ADC4 reset"]
pub struct ADC34RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC34RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC34RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADC34RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&self) -> ADC34RST_R {
        ADC34RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&mut self) -> IOPHRST_W {
        IOPHRST_W { w: self }
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W {
        IOPARST_W { w: self }
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W {
        IOPBRST_W { w: self }
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W {
        IOPCRST_W { w: self }
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W {
        IOPDRST_W { w: self }
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&mut self) -> IOPERST_W {
        IOPERST_W { w: self }
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W {
        IOPFRST_W { w: self }
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&mut self) -> IOPGRST_W {
        IOPGRST_W { w: self }
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W {
        TSCRST_W { w: self }
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W {
        ADC12RST_W { w: self }
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&mut self) -> ADC34RST_W {
        ADC34RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](index.html) module"]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrstr::R](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
