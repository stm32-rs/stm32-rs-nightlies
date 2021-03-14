#[doc = "Reader of register ADC2R"]
pub type R = crate::R<u32, super::ADC2R>;
#[doc = "Writer for register ADC2R"]
pub type W = crate::W<u32, super::ADC2R>;
#[doc = "Register ADC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC trigger 2 on Timer E Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2TERST_A {
    #[doc = "0: No generation of ADC trigger on timer reset and roll-over"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer reset and roll-over"]
    ENABLED = 1,
}
impl From<AD2TERST_A> for bool {
    #[inline(always)]
    fn from(variant: AD2TERST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AD2TERST`"]
pub type AD2TERST_R = crate::R<bool, AD2TERST_A>;
impl AD2TERST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2TERST_A {
        match self.bits {
            false => AD2TERST_A::DISABLED,
            true => AD2TERST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TERST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TERST_A::ENABLED
    }
}
#[doc = "Write proxy for field `AD2TERST`"]
pub struct AD2TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TERST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::ENABLED)
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
#[doc = "ADC trigger 2 on Timer E compare 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2TEC4_A {
    #[doc = "0: No generation of ADC trigger on timer compare event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer compare event"]
    ENABLED = 1,
}
impl From<AD2TEC4_A> for bool {
    #[inline(always)]
    fn from(variant: AD2TEC4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AD2TEC4`"]
pub type AD2TEC4_R = crate::R<bool, AD2TEC4_A>;
impl AD2TEC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2TEC4_A {
        match self.bits {
            false => AD2TEC4_A::DISABLED,
            true => AD2TEC4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TEC4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TEC4_A::ENABLED
    }
}
#[doc = "Write proxy for field `AD2TEC4`"]
pub struct AD2TEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TEC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer E compare 3"]
pub type AD2TEC3_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TEC3`"]
pub type AD2TEC3_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TEC3`"]
pub struct AD2TEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TEC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer E compare 2"]
pub type AD2TEC2_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TEC2`"]
pub type AD2TEC2_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TEC2`"]
pub struct AD2TEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TEC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TEC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer D Reset"]
pub type AD2TDRST_A = AD2TERST_A;
#[doc = "Reader of field `AD2TDRST`"]
pub type AD2TDRST_R = crate::R<bool, AD2TERST_A>;
#[doc = "Write proxy for field `AD2TDRST`"]
pub struct AD2TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TDRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer D Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2TDPER_A {
    #[doc = "0: No generation of ADC trigger on timer period event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on timer period event"]
    ENABLED = 1,
}
impl From<AD2TDPER_A> for bool {
    #[inline(always)]
    fn from(variant: AD2TDPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AD2TDPER`"]
pub type AD2TDPER_R = crate::R<bool, AD2TDPER_A>;
impl AD2TDPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2TDPER_A {
        match self.bits {
            false => AD2TDPER_A::DISABLED,
            true => AD2TDPER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TDPER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TDPER_A::ENABLED
    }
}
#[doc = "Write proxy for field `AD2TDPER`"]
pub struct AD2TDPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TDPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer D compare 4"]
pub type AD2TDC4_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TDC4`"]
pub type AD2TDC4_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TDC4`"]
pub struct AD2TDC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TDC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer D compare 3"]
pub type AD2TDC3_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TDC3`"]
pub type AD2TDC3_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TDC3`"]
pub struct AD2TDC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TDC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer D compare 2"]
pub type AD2TDC2_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TDC2`"]
pub type AD2TDC2_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TDC2`"]
pub struct AD2TDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TDC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TDC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer C Reset"]
pub type AD2TCRST_A = AD2TERST_A;
#[doc = "Reader of field `AD2TCRST`"]
pub type AD2TCRST_R = crate::R<bool, AD2TERST_A>;
#[doc = "Write proxy for field `AD2TCRST`"]
pub struct AD2TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TERST_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer C Period"]
pub type AD2TCPER_A = AD2TDPER_A;
#[doc = "Reader of field `AD2TCPER`"]
pub type AD2TCPER_R = crate::R<bool, AD2TDPER_A>;
#[doc = "Write proxy for field `AD2TCPER`"]
pub struct AD2TCPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TCPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer C compare 4"]
pub type AD2TCC4_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TCC4`"]
pub type AD2TCC4_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TCC4`"]
pub struct AD2TCC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TCC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer C compare 3"]
pub type AD2TCC3_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TCC3`"]
pub type AD2TCC3_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TCC3`"]
pub struct AD2TCC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TCC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer C compare 2"]
pub type AD2TCC2_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TCC2`"]
pub type AD2TCC2_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TCC2`"]
pub struct AD2TCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TCC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TCC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer B Period"]
pub type AD2TBPER_A = AD2TDPER_A;
#[doc = "Reader of field `AD2TBPER`"]
pub type AD2TBPER_R = crate::R<bool, AD2TDPER_A>;
#[doc = "Write proxy for field `AD2TBPER`"]
pub struct AD2TBPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TBPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer B compare 4"]
pub type AD2TBC4_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TBC4`"]
pub type AD2TBC4_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TBC4`"]
pub struct AD2TBC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TBC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer B compare 3"]
pub type AD2TBC3_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TBC3`"]
pub type AD2TBC3_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TBC3`"]
pub struct AD2TBC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TBC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer B compare 2"]
pub type AD2TBC2_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TBC2`"]
pub type AD2TBC2_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TBC2`"]
pub struct AD2TBC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TBC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TBC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer A Period"]
pub type AD2TAPER_A = AD2TDPER_A;
#[doc = "Reader of field `AD2TAPER`"]
pub type AD2TAPER_R = crate::R<bool, AD2TDPER_A>;
#[doc = "Write proxy for field `AD2TAPER`"]
pub struct AD2TAPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TAPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer A compare 4"]
pub type AD2TAC4_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TAC4`"]
pub type AD2TAC4_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TAC4`"]
pub struct AD2TAC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TAC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer A compare 3"]
pub type AD2TAC3_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TAC3`"]
pub type AD2TAC3_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TAC3`"]
pub struct AD2TAC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TAC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "ADC trigger 2 on Timer A compare 2"]
pub type AD2TAC2_A = AD2TEC4_A;
#[doc = "Reader of field `AD2TAC2`"]
pub type AD2TAC2_R = crate::R<bool, AD2TEC4_A>;
#[doc = "Write proxy for field `AD2TAC2`"]
pub struct AD2TAC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2TAC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2TAC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TEC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "ADC trigger 2 on External Event 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2EEV10_A {
    #[doc = "0: No generation of ADC trigger on external event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on external event"]
    ENABLED = 1,
}
impl From<AD2EEV10_A> for bool {
    #[inline(always)]
    fn from(variant: AD2EEV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AD2EEV10`"]
pub type AD2EEV10_R = crate::R<bool, AD2EEV10_A>;
impl AD2EEV10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2EEV10_A {
        match self.bits {
            false => AD2EEV10_A::DISABLED,
            true => AD2EEV10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2EEV10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2EEV10_A::ENABLED
    }
}
#[doc = "Write proxy for field `AD2EEV10`"]
pub struct AD2EEV10_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2EEV10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "ADC trigger 2 on External Event 9"]
pub type AD2EEV9_A = AD2EEV10_A;
#[doc = "Reader of field `AD2EEV9`"]
pub type AD2EEV9_R = crate::R<bool, AD2EEV10_A>;
#[doc = "Write proxy for field `AD2EEV9`"]
pub struct AD2EEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2EEV9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "ADC trigger 2 on External Event 8"]
pub type AD2EEV8_A = AD2EEV10_A;
#[doc = "Reader of field `AD2EEV8`"]
pub type AD2EEV8_R = crate::R<bool, AD2EEV10_A>;
#[doc = "Write proxy for field `AD2EEV8`"]
pub struct AD2EEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2EEV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "ADC trigger 2 on External Event 7"]
pub type AD2EEV7_A = AD2EEV10_A;
#[doc = "Reader of field `AD2EEV7`"]
pub type AD2EEV7_R = crate::R<bool, AD2EEV10_A>;
#[doc = "Write proxy for field `AD2EEV7`"]
pub struct AD2EEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2EEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "ADC trigger 2 on External Event 6"]
pub type AD2EEV6_A = AD2EEV10_A;
#[doc = "Reader of field `AD2EEV6`"]
pub type AD2EEV6_R = crate::R<bool, AD2EEV10_A>;
#[doc = "Write proxy for field `AD2EEV6`"]
pub struct AD2EEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2EEV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2EEV6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV10_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "ADC trigger 2 on Master Period"]
pub type AD2MPER_A = AD2TDPER_A;
#[doc = "Reader of field `AD2MPER`"]
pub type AD2MPER_R = crate::R<bool, AD2TDPER_A>;
#[doc = "Write proxy for field `AD2MPER`"]
pub struct AD2MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2MPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TDPER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC trigger 2 on Master Compare 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2MC4_A {
    #[doc = "0: No generation of ADC trigger on master compare event"]
    DISABLED = 0,
    #[doc = "1: Generation of ADC trigger on master compare event"]
    ENABLED = 1,
}
impl From<AD2MC4_A> for bool {
    #[inline(always)]
    fn from(variant: AD2MC4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AD2MC4`"]
pub type AD2MC4_R = crate::R<bool, AD2MC4_A>;
impl AD2MC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2MC4_A {
        match self.bits {
            false => AD2MC4_A::DISABLED,
            true => AD2MC4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2MC4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2MC4_A::ENABLED
    }
}
#[doc = "Write proxy for field `AD2MC4`"]
pub struct AD2MC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2MC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC trigger 2 on Master Compare 3"]
pub type AD2MC3_A = AD2MC4_A;
#[doc = "Reader of field `AD2MC3`"]
pub type AD2MC3_R = crate::R<bool, AD2MC4_A>;
#[doc = "Write proxy for field `AD2MC3`"]
pub struct AD2MC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2MC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "ADC trigger 2 on Master Compare 2"]
pub type AD2MC2_A = AD2MC4_A;
#[doc = "Reader of field `AD2MC2`"]
pub type AD2MC2_R = crate::R<bool, AD2MC4_A>;
#[doc = "Write proxy for field `AD2MC2`"]
pub struct AD2MC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2MC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC trigger 2 on Master Compare 1"]
pub type AD2MC1_A = AD2MC4_A;
#[doc = "Reader of field `AD2MC1`"]
pub type AD2MC1_R = crate::R<bool, AD2MC4_A>;
#[doc = "Write proxy for field `AD2MC1`"]
pub struct AD2MC1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2MC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2MC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::DISABLED)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MC4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - ADC trigger 2 on Timer E Reset"]
    #[inline(always)]
    pub fn ad2terst(&self) -> AD2TERST_R {
        AD2TERST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ADC trigger 2 on Timer E compare 4"]
    #[inline(always)]
    pub fn ad2tec4(&self) -> AD2TEC4_R {
        AD2TEC4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC trigger 2 on Timer E compare 3"]
    #[inline(always)]
    pub fn ad2tec3(&self) -> AD2TEC3_R {
        AD2TEC3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC trigger 2 on Timer E compare 2"]
    #[inline(always)]
    pub fn ad2tec2(&self) -> AD2TEC2_R {
        AD2TEC2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC trigger 2 on Timer D Reset"]
    #[inline(always)]
    pub fn ad2tdrst(&self) -> AD2TDRST_R {
        AD2TDRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC trigger 2 on Timer D Period"]
    #[inline(always)]
    pub fn ad2tdper(&self) -> AD2TDPER_R {
        AD2TDPER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC trigger 2 on Timer D compare 4"]
    #[inline(always)]
    pub fn ad2tdc4(&self) -> AD2TDC4_R {
        AD2TDC4_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC trigger 2 on Timer D compare 3"]
    #[inline(always)]
    pub fn ad2tdc3(&self) -> AD2TDC3_R {
        AD2TDC3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC trigger 2 on Timer D compare 2"]
    #[inline(always)]
    pub fn ad2tdc2(&self) -> AD2TDC2_R {
        AD2TDC2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC trigger 2 on Timer C Reset"]
    #[inline(always)]
    pub fn ad2tcrst(&self) -> AD2TCRST_R {
        AD2TCRST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC trigger 2 on Timer C Period"]
    #[inline(always)]
    pub fn ad2tcper(&self) -> AD2TCPER_R {
        AD2TCPER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC trigger 2 on Timer C compare 4"]
    #[inline(always)]
    pub fn ad2tcc4(&self) -> AD2TCC4_R {
        AD2TCC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC trigger 2 on Timer C compare 3"]
    #[inline(always)]
    pub fn ad2tcc3(&self) -> AD2TCC3_R {
        AD2TCC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC trigger 2 on Timer C compare 2"]
    #[inline(always)]
    pub fn ad2tcc2(&self) -> AD2TCC2_R {
        AD2TCC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC trigger 2 on Timer B Period"]
    #[inline(always)]
    pub fn ad2tbper(&self) -> AD2TBPER_R {
        AD2TBPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC trigger 2 on Timer B compare 4"]
    #[inline(always)]
    pub fn ad2tbc4(&self) -> AD2TBC4_R {
        AD2TBC4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC trigger 2 on Timer B compare 3"]
    #[inline(always)]
    pub fn ad2tbc3(&self) -> AD2TBC3_R {
        AD2TBC3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC trigger 2 on Timer B compare 2"]
    #[inline(always)]
    pub fn ad2tbc2(&self) -> AD2TBC2_R {
        AD2TBC2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC trigger 2 on Timer A Period"]
    #[inline(always)]
    pub fn ad2taper(&self) -> AD2TAPER_R {
        AD2TAPER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC trigger 2 on Timer A compare 4"]
    #[inline(always)]
    pub fn ad2tac4(&self) -> AD2TAC4_R {
        AD2TAC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC trigger 2 on Timer A compare 3"]
    #[inline(always)]
    pub fn ad2tac3(&self) -> AD2TAC3_R {
        AD2TAC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC trigger 2 on Timer A compare 2"]
    #[inline(always)]
    pub fn ad2tac2(&self) -> AD2TAC2_R {
        AD2TAC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC trigger 2 on External Event 10"]
    #[inline(always)]
    pub fn ad2eev10(&self) -> AD2EEV10_R {
        AD2EEV10_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC trigger 2 on External Event 9"]
    #[inline(always)]
    pub fn ad2eev9(&self) -> AD2EEV9_R {
        AD2EEV9_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC trigger 2 on External Event 8"]
    #[inline(always)]
    pub fn ad2eev8(&self) -> AD2EEV8_R {
        AD2EEV8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC trigger 2 on External Event 7"]
    #[inline(always)]
    pub fn ad2eev7(&self) -> AD2EEV7_R {
        AD2EEV7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC trigger 2 on External Event 6"]
    #[inline(always)]
    pub fn ad2eev6(&self) -> AD2EEV6_R {
        AD2EEV6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC trigger 2 on Master Period"]
    #[inline(always)]
    pub fn ad2mper(&self) -> AD2MPER_R {
        AD2MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC trigger 2 on Master Compare 4"]
    #[inline(always)]
    pub fn ad2mc4(&self) -> AD2MC4_R {
        AD2MC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC trigger 2 on Master Compare 3"]
    #[inline(always)]
    pub fn ad2mc3(&self) -> AD2MC3_R {
        AD2MC3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC trigger 2 on Master Compare 2"]
    #[inline(always)]
    pub fn ad2mc2(&self) -> AD2MC2_R {
        AD2MC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC trigger 2 on Master Compare 1"]
    #[inline(always)]
    pub fn ad2mc1(&self) -> AD2MC1_R {
        AD2MC1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ADC trigger 2 on Timer E Reset"]
    #[inline(always)]
    pub fn ad2terst(&mut self) -> AD2TERST_W {
        AD2TERST_W { w: self }
    }
    #[doc = "Bit 30 - ADC trigger 2 on Timer E compare 4"]
    #[inline(always)]
    pub fn ad2tec4(&mut self) -> AD2TEC4_W {
        AD2TEC4_W { w: self }
    }
    #[doc = "Bit 29 - ADC trigger 2 on Timer E compare 3"]
    #[inline(always)]
    pub fn ad2tec3(&mut self) -> AD2TEC3_W {
        AD2TEC3_W { w: self }
    }
    #[doc = "Bit 28 - ADC trigger 2 on Timer E compare 2"]
    #[inline(always)]
    pub fn ad2tec2(&mut self) -> AD2TEC2_W {
        AD2TEC2_W { w: self }
    }
    #[doc = "Bit 27 - ADC trigger 2 on Timer D Reset"]
    #[inline(always)]
    pub fn ad2tdrst(&mut self) -> AD2TDRST_W {
        AD2TDRST_W { w: self }
    }
    #[doc = "Bit 26 - ADC trigger 2 on Timer D Period"]
    #[inline(always)]
    pub fn ad2tdper(&mut self) -> AD2TDPER_W {
        AD2TDPER_W { w: self }
    }
    #[doc = "Bit 25 - ADC trigger 2 on Timer D compare 4"]
    #[inline(always)]
    pub fn ad2tdc4(&mut self) -> AD2TDC4_W {
        AD2TDC4_W { w: self }
    }
    #[doc = "Bit 24 - ADC trigger 2 on Timer D compare 3"]
    #[inline(always)]
    pub fn ad2tdc3(&mut self) -> AD2TDC3_W {
        AD2TDC3_W { w: self }
    }
    #[doc = "Bit 23 - ADC trigger 2 on Timer D compare 2"]
    #[inline(always)]
    pub fn ad2tdc2(&mut self) -> AD2TDC2_W {
        AD2TDC2_W { w: self }
    }
    #[doc = "Bit 22 - ADC trigger 2 on Timer C Reset"]
    #[inline(always)]
    pub fn ad2tcrst(&mut self) -> AD2TCRST_W {
        AD2TCRST_W { w: self }
    }
    #[doc = "Bit 21 - ADC trigger 2 on Timer C Period"]
    #[inline(always)]
    pub fn ad2tcper(&mut self) -> AD2TCPER_W {
        AD2TCPER_W { w: self }
    }
    #[doc = "Bit 20 - ADC trigger 2 on Timer C compare 4"]
    #[inline(always)]
    pub fn ad2tcc4(&mut self) -> AD2TCC4_W {
        AD2TCC4_W { w: self }
    }
    #[doc = "Bit 19 - ADC trigger 2 on Timer C compare 3"]
    #[inline(always)]
    pub fn ad2tcc3(&mut self) -> AD2TCC3_W {
        AD2TCC3_W { w: self }
    }
    #[doc = "Bit 18 - ADC trigger 2 on Timer C compare 2"]
    #[inline(always)]
    pub fn ad2tcc2(&mut self) -> AD2TCC2_W {
        AD2TCC2_W { w: self }
    }
    #[doc = "Bit 17 - ADC trigger 2 on Timer B Period"]
    #[inline(always)]
    pub fn ad2tbper(&mut self) -> AD2TBPER_W {
        AD2TBPER_W { w: self }
    }
    #[doc = "Bit 16 - ADC trigger 2 on Timer B compare 4"]
    #[inline(always)]
    pub fn ad2tbc4(&mut self) -> AD2TBC4_W {
        AD2TBC4_W { w: self }
    }
    #[doc = "Bit 15 - ADC trigger 2 on Timer B compare 3"]
    #[inline(always)]
    pub fn ad2tbc3(&mut self) -> AD2TBC3_W {
        AD2TBC3_W { w: self }
    }
    #[doc = "Bit 14 - ADC trigger 2 on Timer B compare 2"]
    #[inline(always)]
    pub fn ad2tbc2(&mut self) -> AD2TBC2_W {
        AD2TBC2_W { w: self }
    }
    #[doc = "Bit 13 - ADC trigger 2 on Timer A Period"]
    #[inline(always)]
    pub fn ad2taper(&mut self) -> AD2TAPER_W {
        AD2TAPER_W { w: self }
    }
    #[doc = "Bit 12 - ADC trigger 2 on Timer A compare 4"]
    #[inline(always)]
    pub fn ad2tac4(&mut self) -> AD2TAC4_W {
        AD2TAC4_W { w: self }
    }
    #[doc = "Bit 11 - ADC trigger 2 on Timer A compare 3"]
    #[inline(always)]
    pub fn ad2tac3(&mut self) -> AD2TAC3_W {
        AD2TAC3_W { w: self }
    }
    #[doc = "Bit 10 - ADC trigger 2 on Timer A compare 2"]
    #[inline(always)]
    pub fn ad2tac2(&mut self) -> AD2TAC2_W {
        AD2TAC2_W { w: self }
    }
    #[doc = "Bit 9 - ADC trigger 2 on External Event 10"]
    #[inline(always)]
    pub fn ad2eev10(&mut self) -> AD2EEV10_W {
        AD2EEV10_W { w: self }
    }
    #[doc = "Bit 8 - ADC trigger 2 on External Event 9"]
    #[inline(always)]
    pub fn ad2eev9(&mut self) -> AD2EEV9_W {
        AD2EEV9_W { w: self }
    }
    #[doc = "Bit 7 - ADC trigger 2 on External Event 8"]
    #[inline(always)]
    pub fn ad2eev8(&mut self) -> AD2EEV8_W {
        AD2EEV8_W { w: self }
    }
    #[doc = "Bit 6 - ADC trigger 2 on External Event 7"]
    #[inline(always)]
    pub fn ad2eev7(&mut self) -> AD2EEV7_W {
        AD2EEV7_W { w: self }
    }
    #[doc = "Bit 5 - ADC trigger 2 on External Event 6"]
    #[inline(always)]
    pub fn ad2eev6(&mut self) -> AD2EEV6_W {
        AD2EEV6_W { w: self }
    }
    #[doc = "Bit 4 - ADC trigger 2 on Master Period"]
    #[inline(always)]
    pub fn ad2mper(&mut self) -> AD2MPER_W {
        AD2MPER_W { w: self }
    }
    #[doc = "Bit 3 - ADC trigger 2 on Master Compare 4"]
    #[inline(always)]
    pub fn ad2mc4(&mut self) -> AD2MC4_W {
        AD2MC4_W { w: self }
    }
    #[doc = "Bit 2 - ADC trigger 2 on Master Compare 3"]
    #[inline(always)]
    pub fn ad2mc3(&mut self) -> AD2MC3_W {
        AD2MC3_W { w: self }
    }
    #[doc = "Bit 1 - ADC trigger 2 on Master Compare 2"]
    #[inline(always)]
    pub fn ad2mc2(&mut self) -> AD2MC2_W {
        AD2MC2_W { w: self }
    }
    #[doc = "Bit 0 - ADC trigger 2 on Master Compare 1"]
    #[inline(always)]
    pub fn ad2mc1(&mut self) -> AD2MC1_W {
        AD2MC1_W { w: self }
    }
}
