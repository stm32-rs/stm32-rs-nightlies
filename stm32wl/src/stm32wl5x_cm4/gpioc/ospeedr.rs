#[doc = "Register `OSPEEDR` reader"]
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSPEEDR` writer"]
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR15_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR15` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR15_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR15` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR15_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR15_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR15_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR15_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR14_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR14` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR14_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR14` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR14_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR14_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR14_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR14_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR13_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR13` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR13_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR13` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR13_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR13_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR13_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR13_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR6_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR6` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR6_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR6` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR6_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR6_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR6_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR6_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR5_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR5` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR5_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR5` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR5_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR5_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR5_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR5_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR4_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR4_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR4_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR4_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR4_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR4_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR3_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR2_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR2_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR2_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR2_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR2_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR2_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)"]
pub type OSPEEDR1_A = OSPEEDR0_A;
#[doc = "Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR1_R = OSPEEDR0_R;
#[doc = "Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR1_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR1_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR1_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR1_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPEEDR0_A {
    #[doc = "0: Low speed"]
    LOWSPEED = 0,
    #[doc = "1: Medium speed"]
    MEDIUMSPEED = 1,
    #[doc = "2: High speed"]
    HIGHSPEED = 2,
    #[doc = "3: Very high speed"]
    VERYHIGHSPEED = 3,
}
impl From<OSPEEDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR0_R(crate::FieldReader<u8, OSPEEDR0_A>);
impl OSPEEDR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPEEDR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSPEEDR0_A {
        match self.bits {
            0 => OSPEEDR0_A::LOWSPEED,
            1 => OSPEEDR0_A::MEDIUMSPEED,
            2 => OSPEEDR0_A::HIGHSPEED,
            3 => OSPEEDR0_A::VERYHIGHSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == OSPEEDR0_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        **self == OSPEEDR0_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == OSPEEDR0_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `VERYHIGHSPEED`"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        **self == OSPEEDR0_A::VERYHIGHSPEED
    }
}
impl core::ops::Deref for OSPEEDR0_R {
    type Target = crate::FieldReader<u8, OSPEEDR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)"]
pub struct OSPEEDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSPEEDR0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::LOWSPEED)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::MEDIUMSPEED)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::HIGHSPEED)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEEDR0_A::VERYHIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
        OSPEEDR15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
        OSPEEDR14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
        OSPEEDR13_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
        OSPEEDR6_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
        OSPEEDR5_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
        OSPEEDR4_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
        OSPEEDR3_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
        OSPEEDR2_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
        OSPEEDR1_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
        OSPEEDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](index.html) module"]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ospeedr::R](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
