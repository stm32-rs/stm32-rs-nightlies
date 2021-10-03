#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS` reader - Bias selector"]
pub struct BIAS_R(crate::FieldReader<u8, u8>);
impl BIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS` writer - Bias selector"]
pub struct BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `DUTY` reader - Duty selection"]
pub struct DUTY_R(crate::FieldReader<u8, u8>);
impl DUTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY` writer - Duty selection"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `VSEL` reader - Voltage source selection"]
pub struct VSEL_R(crate::FieldReader<bool, bool>);
impl VSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSEL` writer - Voltage source selection"]
pub struct VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSEL_W<'a> {
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
#[doc = "Field `LCDEN` reader - LCD controller enable"]
pub struct LCDEN_R(crate::FieldReader<bool, bool>);
impl LCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDEN` writer - LCD controller enable"]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
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
#[doc = "Field `MUX_SEG` reader - Mux segment enable"]
pub struct MUX_SEG_R(crate::FieldReader<bool, bool>);
impl MUX_SEG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SEG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SEG` writer - Mux segment enable"]
pub struct MUX_SEG_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SEG_W<'a> {
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
#[doc = "Field `BUFEN` reader - Voltage output buffer enable"]
pub struct BUFEN_R(crate::FieldReader<bool, bool>);
impl BUFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFEN` writer - Voltage output buffer enable"]
pub struct BUFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFEN_W<'a> {
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
impl R {
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mux segment enable"]
    #[inline(always)]
    pub fn mux_seg(&self) -> MUX_SEG_R {
        MUX_SEG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Voltage output buffer enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W { w: self }
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W {
        VSEL_W { w: self }
    }
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bit 7 - Mux segment enable"]
    #[inline(always)]
    pub fn mux_seg(&mut self) -> MUX_SEG_W {
        MUX_SEG_W { w: self }
    }
    #[doc = "Bit 8 - Voltage output buffer enable"]
    #[inline(always)]
    pub fn bufen(&mut self) -> BUFEN_W {
        BUFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
