#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main PLL division factor for PLLRCLK"]
pub type PLLR_A = PLLQ_A;
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLRCLK"]
pub type PLLR_R = PLLQ_R;
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLRCLK"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLR_A::DIV2)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLR_A::DIV3)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLR_A::DIV4)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLR_A::DIV5)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLR_A::DIV6)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLR_A::DIV7)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLR_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Main PLL PLLRCLK output enable"]
pub type PLLREN_A = PLLPEN_A;
#[doc = "Field `PLLREN` reader - Main PLL PLLRCLK output enable"]
pub type PLLREN_R = PLLPEN_R;
#[doc = "Field `PLLREN` writer - Main PLL PLLRCLK output enable"]
pub struct PLLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLLCLK output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLREN_A::DISABLED)
    }
    #[doc = "PLLCLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLREN_A::ENABLED)
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
#[doc = "Main PLL division factor for PLLQCLK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "1: PLL = VCO/(N+1)"]
    DIV2 = 1,
    #[doc = "2: PLL = VCO/(N+1)"]
    DIV3 = 2,
    #[doc = "3: PLL = VCO/(N+1)"]
    DIV4 = 3,
    #[doc = "4: PLL = VCO/(N+1)"]
    DIV5 = 4,
    #[doc = "5: PLL = VCO/(N+1)"]
    DIV6 = 5,
    #[doc = "6: PLL = VCO/(N+1)"]
    DIV7 = 6,
    #[doc = "7: PLL = VCO/(N+1)"]
    DIV8 = 7,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLQCLK"]
pub struct PLLQ_R(crate::FieldReader<u8, PLLQ_A>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLQ_A> {
        match self.bits {
            1 => Some(PLLQ_A::DIV2),
            2 => Some(PLLQ_A::DIV3),
            3 => Some(PLLQ_A::DIV4),
            4 => Some(PLLQ_A::DIV5),
            5 => Some(PLLQ_A::DIV6),
            6 => Some(PLLQ_A::DIV7),
            7 => Some(PLLQ_A::DIV8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PLLQ_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PLLQ_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLQ_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLQ_A::DIV8
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, PLLQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLQCLK"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV2)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV3)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV4)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV5)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV6)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV7)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Main PLL PLLQCLK output enable"]
pub type PLLQEN_A = PLLPEN_A;
#[doc = "Field `PLLQEN` reader - Main PLL PLLQCLK output enable"]
pub type PLLQEN_R = PLLPEN_R;
#[doc = "Field `PLLQEN` writer - Main PLL PLLQCLK output enable"]
pub struct PLLQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLLCLK output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::DISABLED)
    }
    #[doc = "PLLCLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::ENABLED)
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
#[doc = "Main PLL division factor for PLLPCLK.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLP_A {
    #[doc = "1: PLL = VCO/(N+1)"]
    DIV2 = 1,
    #[doc = "2: PLL = VCO/(N+1)"]
    DIV3 = 2,
    #[doc = "3: PLL = VCO/(N+1)"]
    DIV4 = 3,
    #[doc = "4: PLL = VCO/(N+1)"]
    DIV5 = 4,
    #[doc = "5: PLL = VCO/(N+1)"]
    DIV6 = 5,
    #[doc = "6: PLL = VCO/(N+1)"]
    DIV7 = 6,
    #[doc = "7: PLL = VCO/(N+1)"]
    DIV8 = 7,
    #[doc = "8: PLL = VCO/(N+1)"]
    DIV9 = 8,
    #[doc = "9: PLL = VCO/(N+1)"]
    DIV10 = 9,
    #[doc = "10: PLL = VCO/(N+1)"]
    DIV11 = 10,
    #[doc = "11: PLL = VCO/(N+1)"]
    DIV12 = 11,
    #[doc = "12: PLL = VCO/(N+1)"]
    DIV13 = 12,
    #[doc = "13: PLL = VCO/(N+1)"]
    DIV14 = 13,
    #[doc = "14: PLL = VCO/(N+1)"]
    DIV15 = 14,
    #[doc = "15: PLL = VCO/(N+1)"]
    DIV16 = 15,
    #[doc = "16: PLL = VCO/(N+1)"]
    DIV17 = 16,
    #[doc = "17: PLL = VCO/(N+1)"]
    DIV18 = 17,
    #[doc = "18: PLL = VCO/(N+1)"]
    DIV19 = 18,
    #[doc = "19: PLL = VCO/(N+1)"]
    DIV20 = 19,
    #[doc = "20: PLL = VCO/(N+1)"]
    DIV21 = 20,
    #[doc = "21: PLL = VCO/(N+1)"]
    DIV22 = 21,
    #[doc = "22: PLL = VCO/(N+1)"]
    DIV23 = 22,
    #[doc = "23: PLL = VCO/(N+1)"]
    DIV24 = 23,
    #[doc = "24: PLL = VCO/(N+1)"]
    DIV25 = 24,
    #[doc = "25: PLL = VCO/(N+1)"]
    DIV26 = 25,
    #[doc = "26: PLL = VCO/(N+1)"]
    DIV27 = 26,
    #[doc = "27: PLL = VCO/(N+1)"]
    DIV28 = 27,
    #[doc = "28: PLL = VCO/(N+1)"]
    DIV29 = 28,
    #[doc = "29: PLL = VCO/(N+1)"]
    DIV30 = 29,
    #[doc = "30: PLL = VCO/(N+1)"]
    DIV31 = 30,
    #[doc = "31: PLL = VCO/(N+1)"]
    DIV32 = 31,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLPCLK."]
pub struct PLLP_R(crate::FieldReader<u8, PLLP_A>);
impl PLLP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLP_A> {
        match self.bits {
            1 => Some(PLLP_A::DIV2),
            2 => Some(PLLP_A::DIV3),
            3 => Some(PLLP_A::DIV4),
            4 => Some(PLLP_A::DIV5),
            5 => Some(PLLP_A::DIV6),
            6 => Some(PLLP_A::DIV7),
            7 => Some(PLLP_A::DIV8),
            8 => Some(PLLP_A::DIV9),
            9 => Some(PLLP_A::DIV10),
            10 => Some(PLLP_A::DIV11),
            11 => Some(PLLP_A::DIV12),
            12 => Some(PLLP_A::DIV13),
            13 => Some(PLLP_A::DIV14),
            14 => Some(PLLP_A::DIV15),
            15 => Some(PLLP_A::DIV16),
            16 => Some(PLLP_A::DIV17),
            17 => Some(PLLP_A::DIV18),
            18 => Some(PLLP_A::DIV19),
            19 => Some(PLLP_A::DIV20),
            20 => Some(PLLP_A::DIV21),
            21 => Some(PLLP_A::DIV22),
            22 => Some(PLLP_A::DIV23),
            23 => Some(PLLP_A::DIV24),
            24 => Some(PLLP_A::DIV25),
            25 => Some(PLLP_A::DIV26),
            26 => Some(PLLP_A::DIV27),
            27 => Some(PLLP_A::DIV28),
            28 => Some(PLLP_A::DIV29),
            29 => Some(PLLP_A::DIV30),
            30 => Some(PLLP_A::DIV31),
            31 => Some(PLLP_A::DIV32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLP_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PLLP_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLP_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PLLP_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLP_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLP_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLP_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PLLP_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PLLP_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PLLP_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PLLP_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PLLP_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PLLP_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PLLP_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PLLP_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        **self == PLLP_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        **self == PLLP_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        **self == PLLP_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        **self == PLLP_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        **self == PLLP_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        **self == PLLP_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        **self == PLLP_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        **self == PLLP_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        **self == PLLP_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        **self == PLLP_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        **self == PLLP_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        **self == PLLP_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        **self == PLLP_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        **self == PLLP_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        **self == PLLP_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PLLP_A::DIV32
    }
}
impl core::ops::Deref for PLLP_R {
    type Target = crate::FieldReader<u8, PLLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLPCLK."]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLP_A::DIV2)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLP_A::DIV3)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLP_A::DIV4)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLP_A::DIV5)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLP_A::DIV6)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::DIV7)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLP_A::DIV8)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLP_A::DIV9)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLP_A::DIV10)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLP_A::DIV11)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLP_A::DIV12)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLP_A::DIV13)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLP_A::DIV14)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLP_A::DIV15)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLP_A::DIV16)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::DIV17)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLP_A::DIV18)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLP_A::DIV19)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLP_A::DIV20)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLP_A::DIV21)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLP_A::DIV22)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLP_A::DIV23)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLP_A::DIV24)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLP_A::DIV25)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLP_A::DIV26)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLP_A::DIV27)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLP_A::DIV28)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLP_A::DIV29)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLP_A::DIV30)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLP_A::DIV31)
    }
    #[doc = "PLL = VCO/(N+1)"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLP_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Main PLL PLLPCLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPEN_A {
    #[doc = "0: PLLCLK output disabled"]
    DISABLED = 0,
    #[doc = "1: PLLCLK output enabled"]
    ENABLED = 1,
}
impl From<PLLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPEN` reader - Main PLL PLLPCLK output enable"]
pub struct PLLPEN_R(crate::FieldReader<bool, PLLPEN_A>);
impl PLLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLPEN_A {
        match self.bits {
            false => PLLPEN_A::DISABLED,
            true => PLLPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PLLPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PLLPEN_A::ENABLED
    }
}
impl core::ops::Deref for PLLPEN_R {
    type Target = crate::FieldReader<bool, PLLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPEN` writer - Main PLL PLLPCLK output enable"]
pub struct PLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLLCLK output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::DISABLED)
    }
    #[doc = "PLLCLK output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::ENABLED)
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
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub struct PLLN_R(crate::FieldReader<u8, u8>);
impl PLLN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO"]
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Division factor for the main PLL input clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLM_A {
    #[doc = "0: VCO input = PLL input / PLLM"]
    DIV1 = 0,
    #[doc = "1: VCO input = PLL input / PLLM"]
    DIV2 = 1,
    #[doc = "2: VCO input = PLL input / PLLM"]
    DIV3 = 2,
    #[doc = "3: VCO input = PLL input / PLLM"]
    DIV4 = 3,
    #[doc = "4: VCO input = PLL input / PLLM"]
    DIV5 = 4,
    #[doc = "5: VCO input = PLL input / PLLM"]
    DIV6 = 5,
    #[doc = "6: VCO input = PLL input / PLLM"]
    DIV7 = 6,
    #[doc = "7: VCO input = PLL input / PLLM"]
    DIV8 = 7,
}
impl From<PLLM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL input clock"]
pub struct PLLM_R(crate::FieldReader<u8, PLLM_A>);
impl PLLM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLM_A {
        match self.bits {
            0 => PLLM_A::DIV1,
            1 => PLLM_A::DIV2,
            2 => PLLM_A::DIV3,
            3 => PLLM_A::DIV4,
            4 => PLLM_A::DIV5,
            5 => PLLM_A::DIV6,
            6 => PLLM_A::DIV7,
            7 => PLLM_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PLLM_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLM_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PLLM_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLM_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PLLM_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLM_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLM_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLM_A::DIV8
    }
}
impl core::ops::Deref for PLLM_R {
    type Target = crate::FieldReader<u8, PLLM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL input clock"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLM_A::DIV1)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLM_A::DIV2)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLM_A::DIV3)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLM_A::DIV4)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLM_A::DIV5)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLM_A::DIV6)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLM_A::DIV7)
    }
    #[doc = "VCO input = PLL input / PLLM"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLM_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Main PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: No clock sent to PLL"]
    NOCLOCK = 0,
    #[doc = "1: MSI clock selected as PLL clock entry"]
    MSI = 1,
    #[doc = "2: HSI16 clock selected as PLL clock entry"]
    HSI16 = 2,
    #[doc = "3: HSE32 clock selected as PLL clock entry"]
    HSE32 = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL entry clock source"]
pub struct PLLSRC_R(crate::FieldReader<u8, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::NOCLOCK,
            1 => PLLSRC_A::MSI,
            2 => PLLSRC_A::HSI16,
            3 => PLLSRC_A::HSE32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == PLLSRC_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `MSI`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == PLLSRC_A::MSI
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == PLLSRC_A::HSI16
    }
    #[doc = "Checks if the value of the field is `HSE32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        **self == PLLSRC_A::HSE32
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL entry clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PLLSRC_A::NOCLOCK)
    }
    #[doc = "MSI clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(PLLSRC_A::MSI)
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI16)
    }
    #[doc = "HSE32 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - Main PLL division factor for PLLRCLK"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Main PLL PLLRCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Main PLL division factor for PLLQCLK"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLQCLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - Main PLL division factor for PLLPCLK."]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Main PLL PLLPCLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - Main PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - Main PLL division factor for PLLRCLK"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Bit 28 - Main PLL PLLRCLK output enable"]
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W {
        PLLREN_W { w: self }
    }
    #[doc = "Bits 25:27 - Main PLL division factor for PLLQCLK"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL PLLQCLK output enable"]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W {
        PLLQEN_W { w: self }
    }
    #[doc = "Bits 17:21 - Main PLL division factor for PLLPCLK."]
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    #[doc = "Bit 16 - Main PLL PLLPCLK output enable"]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W {
        PLLPEN_W { w: self }
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    #[doc = "Bits 4:6 - Division factor for the main PLL input clock"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    #[doc = "Bits 0:1 - Main PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2204_0100"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2204_0100
    }
}
