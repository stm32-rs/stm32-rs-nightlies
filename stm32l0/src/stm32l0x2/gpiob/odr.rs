#[doc = "Reader of register ODR"]
pub type R = crate::R<u32, super::ODR>;
#[doc = "Writer for register ODR"]
pub type W = crate::W<u32, super::ODR>;
#[doc = "Register ODR `reset()`'s with value 0"]
impl crate::ResetValue for super::ODR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port output data bit (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD15_A {
    #[doc = "1: Set output to logic high"]
    HIGH = 1,
    #[doc = "0: Set output to logic low"]
    LOW = 0,
}
impl From<OD15_A> for bool {
    #[inline(always)]
    fn from(variant: OD15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OD15`"]
pub type OD15_R = crate::R<bool, OD15_A>;
impl OD15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD15_A {
        match self.bits {
            true => OD15_A::HIGH,
            false => OD15_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OD15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OD15_A::LOW
    }
}
#[doc = "Write proxy for field `OD15`"]
pub struct OD15_W<'a> {
    w: &'a mut W,
}
impl<'a> OD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD14_A = OD15_A;
#[doc = "Reader of field `OD14`"]
pub type OD14_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD14`"]
pub struct OD14_W<'a> {
    w: &'a mut W,
}
impl<'a> OD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD13_A = OD15_A;
#[doc = "Reader of field `OD13`"]
pub type OD13_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD13`"]
pub struct OD13_W<'a> {
    w: &'a mut W,
}
impl<'a> OD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD12_A = OD15_A;
#[doc = "Reader of field `OD12`"]
pub type OD12_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD12`"]
pub struct OD12_W<'a> {
    w: &'a mut W,
}
impl<'a> OD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD11_A = OD15_A;
#[doc = "Reader of field `OD11`"]
pub type OD11_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD11`"]
pub struct OD11_W<'a> {
    w: &'a mut W,
}
impl<'a> OD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD10_A = OD15_A;
#[doc = "Reader of field `OD10`"]
pub type OD10_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD10`"]
pub struct OD10_W<'a> {
    w: &'a mut W,
}
impl<'a> OD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD9_A = OD15_A;
#[doc = "Reader of field `OD9`"]
pub type OD9_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD9`"]
pub struct OD9_W<'a> {
    w: &'a mut W,
}
impl<'a> OD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD8_A = OD15_A;
#[doc = "Reader of field `OD8`"]
pub type OD8_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD8`"]
pub struct OD8_W<'a> {
    w: &'a mut W,
}
impl<'a> OD8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD7_A = OD15_A;
#[doc = "Reader of field `OD7`"]
pub type OD7_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD7`"]
pub struct OD7_W<'a> {
    w: &'a mut W,
}
impl<'a> OD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD6_A = OD15_A;
#[doc = "Reader of field `OD6`"]
pub type OD6_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD6`"]
pub struct OD6_W<'a> {
    w: &'a mut W,
}
impl<'a> OD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD5_A = OD15_A;
#[doc = "Reader of field `OD5`"]
pub type OD5_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD5`"]
pub struct OD5_W<'a> {
    w: &'a mut W,
}
impl<'a> OD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD4_A = OD15_A;
#[doc = "Reader of field `OD4`"]
pub type OD4_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD4`"]
pub struct OD4_W<'a> {
    w: &'a mut W,
}
impl<'a> OD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD3_A = OD15_A;
#[doc = "Reader of field `OD3`"]
pub type OD3_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD3`"]
pub struct OD3_W<'a> {
    w: &'a mut W,
}
impl<'a> OD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD2_A = OD15_A;
#[doc = "Reader of field `OD2`"]
pub type OD2_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD2`"]
pub struct OD2_W<'a> {
    w: &'a mut W,
}
impl<'a> OD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD1_A = OD15_A;
#[doc = "Reader of field `OD1`"]
pub type OD1_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD1`"]
pub struct OD1_W<'a> {
    w: &'a mut W,
}
impl<'a> OD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
#[doc = "Port output data bit (y = 0..15)"]
pub type OD0_A = OD15_A;
#[doc = "Reader of field `OD0`"]
pub type OD0_R = crate::R<bool, OD15_A>;
#[doc = "Write proxy for field `OD0`"]
pub struct OD0_W<'a> {
    w: &'a mut W,
}
impl<'a> OD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
    #[doc = "Bit 15 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od15(&mut self) -> OD15_W {
        OD15_W { w: self }
    }
    #[doc = "Bit 14 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od14(&mut self) -> OD14_W {
        OD14_W { w: self }
    }
    #[doc = "Bit 13 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od13(&mut self) -> OD13_W {
        OD13_W { w: self }
    }
    #[doc = "Bit 12 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od12(&mut self) -> OD12_W {
        OD12_W { w: self }
    }
    #[doc = "Bit 11 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od11(&mut self) -> OD11_W {
        OD11_W { w: self }
    }
    #[doc = "Bit 10 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od10(&mut self) -> OD10_W {
        OD10_W { w: self }
    }
    #[doc = "Bit 9 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od9(&mut self) -> OD9_W {
        OD9_W { w: self }
    }
    #[doc = "Bit 8 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&mut self) -> OD8_W {
        OD8_W { w: self }
    }
    #[doc = "Bit 7 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&mut self) -> OD7_W {
        OD7_W { w: self }
    }
    #[doc = "Bit 6 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&mut self) -> OD6_W {
        OD6_W { w: self }
    }
    #[doc = "Bit 5 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&mut self) -> OD5_W {
        OD5_W { w: self }
    }
    #[doc = "Bit 4 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&mut self) -> OD4_W {
        OD4_W { w: self }
    }
    #[doc = "Bit 3 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W {
        OD3_W { w: self }
    }
    #[doc = "Bit 2 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&mut self) -> OD2_W {
        OD2_W { w: self }
    }
    #[doc = "Bit 1 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&mut self) -> OD1_W {
        OD1_W { w: self }
    }
    #[doc = "Bit 0 - Port output data bit (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&mut self) -> OD0_W {
        OD0_W { w: self }
    }
}
