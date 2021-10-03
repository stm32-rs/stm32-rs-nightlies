#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVMEN1` reader - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
pub struct PVMEN1_R(crate::FieldReader<bool, bool>);
impl PVMEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVMEN1` writer - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
pub struct PVMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN1_W<'a> {
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
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub struct PLS_R(crate::FieldReader<u8, u8>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub struct PVDE_R(crate::FieldReader<bool, bool>);
impl PVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
#[doc = "Field `PVMEN2` reader - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
pub struct PVMEN2_R(crate::FieldReader<bool, bool>);
impl PVMEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVMEN2` writer - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
pub struct PVMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN2_W<'a> {
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
#[doc = "Field `PVMEN3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
pub struct PVMEN3_R(crate::FieldReader<bool, bool>);
impl PVMEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVMEN3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
pub struct PVMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN3_W<'a> {
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
#[doc = "Field `PVMEN4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
pub struct PVMEN4_R(crate::FieldReader<bool, bool>);
impl PVMEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVMEN4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
pub struct PVMEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMEN4_W<'a> {
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
impl R {
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen3(&self) -> PVMEN3_R {
        PVMEN3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
    #[inline(always)]
    pub fn pvmen4(&self) -> PVMEN4_R {
        PVMEN4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
    #[inline(always)]
    pub fn pvmen1(&mut self) -> PVMEN1_W {
        PVMEN1_W { w: self }
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
    #[inline(always)]
    pub fn pvmen2(&mut self) -> PVMEN2_W {
        PVMEN2_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen3(&mut self) -> PVMEN3_W {
        PVMEN3_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
    #[inline(always)]
    pub fn pvmen4(&mut self) -> PVMEN4_W {
        PVMEN4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
