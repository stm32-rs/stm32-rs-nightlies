#[doc = "Reader of register OFR4"]
pub type R = crate::R<u32, super::OFR4>;
#[doc = "Writer for register OFR4"]
pub type W = crate::W<u32, super::OFR4>;
#[doc = "Register OFR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::OFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SSATE`"]
pub type SSATE_R = crate::R<bool, SSATE_A>;
impl SSATE_R {
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
        *self == SSATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSATE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSATE`"]
pub struct SSATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `OFFSET4_CH`"]
pub type OFFSET4_CH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSET4_CH`"]
pub struct OFFSET4_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `OFFSET4`"]
pub type OFFSET4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OFFSET4`"]
pub struct OFFSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
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
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x03ff_ffff) as u32)
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
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W {
        OFFSET4_CH_W { w: self }
    }
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W {
        OFFSET4_W { w: self }
    }
}
