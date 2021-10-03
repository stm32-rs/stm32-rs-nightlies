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
#[doc = "Signed saturation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSATE_A {
    #[doc = "0: Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    DISABLED = 0,
    #[doc = "1: Offset is subtracted and result is saturated to maintain result size"]
    ENABLED = 1,
}
impl From<SSATE_A> for bool {
    #[inline(always)]
    fn from(variant: SSATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSATE` reader - Signed saturation enable"]
pub struct SSATE_R(crate::FieldReader<bool, SSATE_A>);
impl SSATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSATE_A {
        match self.bits {
            false => SSATE_A::DISABLED,
            true => SSATE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSATE_A::ENABLED
    }
}
impl core::ops::Deref for SSATE_R {
    type Target = crate::FieldReader<bool, SSATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSATE` writer - Signed saturation enable"]
pub struct SSATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Offset is subtracted maintaining data integrity and extending result size (9-bit and 17-bit signed format)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSATE_A::DISABLED)
    }
    #[doc = "Offset is subtracted and result is saturated to maintain result size"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSATE_A::ENABLED)
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
#[doc = "Field `OFFSET2_CH` reader - ADC offset number 1 channel selection"]
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
#[doc = "Field `OFFSET2_CH` writer - ADC offset number 1 channel selection"]
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
#[doc = "Field `OFFSET2` reader - ADC offset number 1 offset level"]
pub struct OFFSET2_R(crate::FieldReader<u32, u32>);
impl OFFSET2_R {
    pub(crate) fn new(bits: u32) -> Self {
        OFFSET2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET2` writer - ADC offset number 1 offset level"]
pub struct OFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - Signed saturation enable"]
    #[inline(always)]
    pub fn ssate(&mut self) -> SSATE_W {
        SSATE_W { w: self }
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W {
        OFFSET2_CH_W { w: self }
    }
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
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
#[doc = "ADC offset number 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](index.html) module"]
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
