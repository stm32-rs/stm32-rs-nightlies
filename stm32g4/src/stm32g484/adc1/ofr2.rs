#[doc = "Register `OFR2` reader"]
pub struct R(crate::R<OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR2` writer"]
pub struct W(crate::W<OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR2_SPEC>;
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
impl From<crate::W<OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Offset 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSET2_EN_A {
    #[doc = "0: Offset disabled"]
    DISABLED = 0,
    #[doc = "1: Offset enabled"]
    ENABLED = 1,
}
impl From<OFFSET2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSET2_EN` reader - Offset 2 Enable"]
pub struct OFFSET2_EN_R(crate::FieldReader<bool, OFFSET2_EN_A>);
impl OFFSET2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFSET2_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSET2_EN_A {
        match self.bits {
            false => OFFSET2_EN_A::DISABLED,
            true => OFFSET2_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OFFSET2_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OFFSET2_EN_A::ENABLED
    }
}
impl core::ops::Deref for OFFSET2_EN_R {
    type Target = crate::FieldReader<bool, OFFSET2_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET2_EN` writer - Offset 2 Enable"]
pub struct OFFSET2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFSET2_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET2_EN_A::DISABLED)
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET2_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `OFFSET2_CH` reader - Channel selection for the data offset 2"]
pub struct OFFSET2_CH_R(crate::FieldReader<u8, u8>);
impl OFFSET2_CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET2_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET2_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET2_CH` writer - Channel selection for the data offset 2"]
pub struct OFFSET2_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `SATEN` reader - Saturation enable"]
pub struct SATEN_R(crate::FieldReader<bool, bool>);
impl SATEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SATEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SATEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SATEN` writer - Saturation enable"]
pub struct SATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SATEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `OFFSETPOS` reader - Positive offset"]
pub struct OFFSETPOS_R(crate::FieldReader<bool, bool>);
impl OFFSETPOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFSETPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETPOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETPOS` writer - Positive offset"]
pub struct OFFSETPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETPOS_W<'a> {
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
#[doc = "Field `OFFSET2` reader - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
pub struct OFFSET2_R(crate::FieldReader<u16, u16>);
impl OFFSET2_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET2` writer - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
pub struct OFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Offset 2 Enable"]
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 2"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Offset 2 Enable"]
    #[inline(always)]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W {
        OFFSET2_EN_W { w: self }
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 2"]
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W {
        OFFSET2_CH_W { w: self }
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W {
        SATEN_W { w: self }
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W {
        OFFSETPOS_W { w: self }
    }
    #[doc = "Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W {
        OFFSET2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](index.html) module"]
pub struct OFR2_SPEC;
impl crate::RegisterSpec for OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr2::R](R) reader structure"]
impl crate::Readable for OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr2::W](W) writer structure"]
impl crate::Writable for OFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for OFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
