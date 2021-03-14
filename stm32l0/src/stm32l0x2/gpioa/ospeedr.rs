#[doc = "Reader of register OSPEEDR"]
pub type R = crate::R<u32, super::OSPEEDR>;
#[doc = "Writer for register OSPEEDR"]
pub type W = crate::W<u32, super::OSPEEDR>;
#[doc = "Register OSPEEDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSPEEDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPEED15_A {
    #[doc = "0: Low speed"]
    LOWSPEED = 0,
    #[doc = "1: Medium speed"]
    MEDIUMSPEED = 1,
    #[doc = "2: High speed"]
    HIGHSPEED = 2,
    #[doc = "3: Very high speed"]
    VERYHIGHSPEED = 3,
}
impl From<OSPEED15_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSPEED15`"]
pub type OSPEED15_R = crate::R<u8, OSPEED15_A>;
impl OSPEED15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPEED15_A {
        match self.bits {
            0 => OSPEED15_A::LOWSPEED,
            1 => OSPEED15_A::MEDIUMSPEED,
            2 => OSPEED15_A::HIGHSPEED,
            3 => OSPEED15_A::VERYHIGHSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEED15_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEED15_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEED15_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `VERYHIGHSPEED`"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEED15_A::VERYHIGHSPEED
    }
}
#[doc = "Write proxy for field `OSPEED15`"]
pub struct OSPEED15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED14_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED14`"]
pub type OSPEED14_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED14`"]
pub struct OSPEED14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED13_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED13`"]
pub type OSPEED13_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED13`"]
pub struct OSPEED13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED12_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED12`"]
pub type OSPEED12_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED12`"]
pub struct OSPEED12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED11_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED11`"]
pub type OSPEED11_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED11`"]
pub struct OSPEED11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED10_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED10`"]
pub type OSPEED10_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED10`"]
pub struct OSPEED10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED9_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED9`"]
pub type OSPEED9_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED9`"]
pub struct OSPEED9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED8_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED8`"]
pub type OSPEED8_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED8`"]
pub struct OSPEED8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED7_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED7`"]
pub type OSPEED7_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED7`"]
pub struct OSPEED7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED6_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED6`"]
pub type OSPEED6_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED6`"]
pub struct OSPEED6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED5_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED5`"]
pub type OSPEED5_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED5`"]
pub struct OSPEED5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED4_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED4`"]
pub type OSPEED4_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED4`"]
pub struct OSPEED4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED3_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED3`"]
pub type OSPEED3_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED3`"]
pub struct OSPEED3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED2_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED2`"]
pub type OSPEED2_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED2`"]
pub struct OSPEED2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED1_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED1`"]
pub type OSPEED1_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED1`"]
pub struct OSPEED1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEED0_A = OSPEED15_A;
#[doc = "Reader of field `OSPEED0`"]
pub type OSPEED0_R = crate::R<u8, OSPEED15_A>;
#[doc = "Write proxy for field `OSPEED0`"]
pub struct OSPEED0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEED0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W {
        OSPEED15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W {
        OSPEED14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W {
        OSPEED13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W {
        OSPEED12_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W {
        OSPEED11_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W {
        OSPEED10_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W {
        OSPEED9_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W {
        OSPEED8_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W {
        OSPEED7_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W {
        OSPEED6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W {
        OSPEED5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W {
        OSPEED4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W {
        OSPEED3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W {
        OSPEED2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W {
        OSPEED1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W {
        OSPEED0_W { w: self }
    }
}
