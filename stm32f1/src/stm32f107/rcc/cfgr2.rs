#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PREDIV1 division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREDIV1_A {
    #[doc = "0: PREDIV input clock not divided"]
    DIV1 = 0,
    #[doc = "1: PREDIV input clock divided by 2"]
    DIV2 = 1,
    #[doc = "2: PREDIV input clock divided by 3"]
    DIV3 = 2,
    #[doc = "3: PREDIV input clock divided by 4"]
    DIV4 = 3,
    #[doc = "4: PREDIV input clock divided by 5"]
    DIV5 = 4,
    #[doc = "5: PREDIV input clock divided by 6"]
    DIV6 = 5,
    #[doc = "6: PREDIV input clock divided by 7"]
    DIV7 = 6,
    #[doc = "7: PREDIV input clock divided by 8"]
    DIV8 = 7,
    #[doc = "8: PREDIV input clock divided by 9"]
    DIV9 = 8,
    #[doc = "9: PREDIV input clock divided by 10"]
    DIV10 = 9,
    #[doc = "10: PREDIV input clock divided by 11"]
    DIV11 = 10,
    #[doc = "11: PREDIV input clock divided by 12"]
    DIV12 = 11,
    #[doc = "12: PREDIV input clock divided by 13"]
    DIV13 = 12,
    #[doc = "13: PREDIV input clock divided by 14"]
    DIV14 = 13,
    #[doc = "14: PREDIV input clock divided by 15"]
    DIV15 = 14,
    #[doc = "15: PREDIV input clock divided by 16"]
    DIV16 = 15,
}
impl From<PREDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREDIV1` reader - PREDIV1 division factor"]
pub struct PREDIV1_R(crate::FieldReader<u8, PREDIV1_A>);
impl PREDIV1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREDIV1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDIV1_A {
        match self.bits {
            0 => PREDIV1_A::DIV1,
            1 => PREDIV1_A::DIV2,
            2 => PREDIV1_A::DIV3,
            3 => PREDIV1_A::DIV4,
            4 => PREDIV1_A::DIV5,
            5 => PREDIV1_A::DIV6,
            6 => PREDIV1_A::DIV7,
            7 => PREDIV1_A::DIV8,
            8 => PREDIV1_A::DIV9,
            9 => PREDIV1_A::DIV10,
            10 => PREDIV1_A::DIV11,
            11 => PREDIV1_A::DIV12,
            12 => PREDIV1_A::DIV13,
            13 => PREDIV1_A::DIV14,
            14 => PREDIV1_A::DIV15,
            15 => PREDIV1_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PREDIV1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PREDIV1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PREDIV1_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PREDIV1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PREDIV1_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PREDIV1_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PREDIV1_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PREDIV1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PREDIV1_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PREDIV1_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PREDIV1_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PREDIV1_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PREDIV1_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PREDIV1_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PREDIV1_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PREDIV1_A::DIV16
    }
}
impl core::ops::Deref for PREDIV1_R {
    type Target = crate::FieldReader<u8, PREDIV1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV1` writer - PREDIV1 division factor"]
pub struct PREDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREDIV1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV1_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "PREDIV2 division factor"]
pub type PREDIV2_A = PREDIV1_A;
#[doc = "Field `PREDIV2` reader - PREDIV2 division factor"]
pub type PREDIV2_R = PREDIV1_R;
#[doc = "Field `PREDIV2` writer - PREDIV2 division factor"]
pub struct PREDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREDIV2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV2_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "PLL2 Multiplication Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL2MUL_A {
    #[doc = "6: PLL clock entry x8"]
    MUL8 = 6,
    #[doc = "7: PLL clock entry x9"]
    MUL9 = 7,
    #[doc = "8: PLL clock entry x10"]
    MUL10 = 8,
    #[doc = "9: PLL clock entry x11"]
    MUL11 = 9,
    #[doc = "10: PLL clock entry x12"]
    MUL12 = 10,
    #[doc = "11: PLL clock entry x13"]
    MUL13 = 11,
    #[doc = "12: PLL clock entry x14"]
    MUL14 = 12,
    #[doc = "14: PLL clock entry x16"]
    MUL16 = 14,
    #[doc = "15: PLL clock entry x20"]
    MUL20 = 15,
}
impl From<PLL2MUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2MUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLL2MUL` reader - PLL2 Multiplication Factor"]
pub struct PLL2MUL_R(crate::FieldReader<u8, PLL2MUL_A>);
impl PLL2MUL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL2MUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLL2MUL_A> {
        match self.bits {
            6 => Some(PLL2MUL_A::MUL8),
            7 => Some(PLL2MUL_A::MUL9),
            8 => Some(PLL2MUL_A::MUL10),
            9 => Some(PLL2MUL_A::MUL11),
            10 => Some(PLL2MUL_A::MUL12),
            11 => Some(PLL2MUL_A::MUL13),
            12 => Some(PLL2MUL_A::MUL14),
            14 => Some(PLL2MUL_A::MUL16),
            15 => Some(PLL2MUL_A::MUL20),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        **self == PLL2MUL_A::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        **self == PLL2MUL_A::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        **self == PLL2MUL_A::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        **self == PLL2MUL_A::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        **self == PLL2MUL_A::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        **self == PLL2MUL_A::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        **self == PLL2MUL_A::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        **self == PLL2MUL_A::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL20`"]
    #[inline(always)]
    pub fn is_mul20(&self) -> bool {
        **self == PLL2MUL_A::MUL20
    }
}
impl core::ops::Deref for PLL2MUL_R {
    type Target = crate::FieldReader<u8, PLL2MUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL2MUL` writer - PLL2 Multiplication Factor"]
pub struct PLL2MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2MUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2MUL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL clock entry x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL8)
    }
    #[doc = "PLL clock entry x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL9)
    }
    #[doc = "PLL clock entry x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL10)
    }
    #[doc = "PLL clock entry x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL11)
    }
    #[doc = "PLL clock entry x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL12)
    }
    #[doc = "PLL clock entry x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL13)
    }
    #[doc = "PLL clock entry x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL14)
    }
    #[doc = "PLL clock entry x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL16)
    }
    #[doc = "PLL clock entry x20"]
    #[inline(always)]
    pub fn mul20(self) -> &'a mut W {
        self.variant(PLL2MUL_A::MUL20)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PLL3 Multiplication Factor"]
pub type PLL3MUL_A = PLL2MUL_A;
#[doc = "Field `PLL3MUL` reader - PLL3 Multiplication Factor"]
pub type PLL3MUL_R = PLL2MUL_R;
#[doc = "Field `PLL3MUL` writer - PLL3 Multiplication Factor"]
pub struct PLL3MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3MUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3MUL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLL clock entry x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL8)
    }
    #[doc = "PLL clock entry x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL9)
    }
    #[doc = "PLL clock entry x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL10)
    }
    #[doc = "PLL clock entry x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL11)
    }
    #[doc = "PLL clock entry x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL12)
    }
    #[doc = "PLL clock entry x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL13)
    }
    #[doc = "PLL clock entry x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL14)
    }
    #[doc = "PLL clock entry x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL16)
    }
    #[doc = "PLL clock entry x20"]
    #[inline(always)]
    pub fn mul20(self) -> &'a mut W {
        self.variant(PLL3MUL_A::MUL20)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "PREDIV1 entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREDIV1SRC_A {
    #[doc = "0: HSE oscillator clock selected as PREDIV1 clock entry"]
    HSE = 0,
    #[doc = "1: PLL2 selected as PREDIV1 clock entry"]
    PLL2 = 1,
}
impl From<PREDIV1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PREDIV1SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREDIV1SRC` reader - PREDIV1 entry clock source"]
pub struct PREDIV1SRC_R(crate::FieldReader<bool, PREDIV1SRC_A>);
impl PREDIV1SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREDIV1SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDIV1SRC_A {
        match self.bits {
            false => PREDIV1SRC_A::HSE,
            true => PREDIV1SRC_A::PLL2,
        }
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PREDIV1SRC_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL2`"]
    #[inline(always)]
    pub fn is_pll2(&self) -> bool {
        **self == PREDIV1SRC_A::PLL2
    }
}
impl core::ops::Deref for PREDIV1SRC_R {
    type Target = crate::FieldReader<bool, PREDIV1SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV1SRC` writer - PREDIV1 entry clock source"]
pub struct PREDIV1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREDIV1SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE oscillator clock selected as PREDIV1 clock entry"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PREDIV1SRC_A::HSE)
    }
    #[doc = "PLL2 selected as PREDIV1 clock entry"]
    #[inline(always)]
    pub fn pll2(self) -> &'a mut W {
        self.variant(PREDIV1SRC_A::PLL2)
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
#[doc = "I2S2 clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S2SRC_A {
    #[doc = "0: System clock (SYSCLK) selected as I2S clock entry"]
    SYSCLK = 0,
    #[doc = "1: PLL3 VCO clock selected as I2S clock entry"]
    PLL3 = 1,
}
impl From<I2S2SRC_A> for bool {
    #[inline(always)]
    fn from(variant: I2S2SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S2SRC` reader - I2S2 clock source"]
pub struct I2S2SRC_R(crate::FieldReader<bool, I2S2SRC_A>);
impl I2S2SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S2SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S2SRC_A {
        match self.bits {
            false => I2S2SRC_A::SYSCLK,
            true => I2S2SRC_A::PLL3,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == I2S2SRC_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PLL3`"]
    #[inline(always)]
    pub fn is_pll3(&self) -> bool {
        **self == I2S2SRC_A::PLL3
    }
}
impl core::ops::Deref for I2S2SRC_R {
    type Target = crate::FieldReader<bool, I2S2SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S2SRC` writer - I2S2 clock source"]
pub struct I2S2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S2SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2S2SRC_A::SYSCLK)
    }
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    #[inline(always)]
    pub fn pll3(self) -> &'a mut W {
        self.variant(I2S2SRC_A::PLL3)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "I2S3 clock source"]
pub type I2S3SRC_A = I2S2SRC_A;
#[doc = "Field `I2S3SRC` reader - I2S3 clock source"]
pub type I2S3SRC_R = I2S2SRC_R;
#[doc = "Field `I2S3SRC` writer - I2S3 clock source"]
pub struct I2S3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S3SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System clock (SYSCLK) selected as I2S clock entry"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2S3SRC_A::SYSCLK)
    }
    #[doc = "PLL3 VCO clock selected as I2S clock entry"]
    #[inline(always)]
    pub fn pll3(self) -> &'a mut W {
        self.variant(I2S3SRC_A::PLL3)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    pub fn prediv1(&self) -> PREDIV1_R {
        PREDIV1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline(always)]
    pub fn prediv2(&self) -> PREDIV2_R {
        PREDIV2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline(always)]
    pub fn pll2mul(&self) -> PLL2MUL_R {
        PLL2MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline(always)]
    pub fn pll3mul(&self) -> PLL3MUL_R {
        PLL3MUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline(always)]
    pub fn prediv1src(&self) -> PREDIV1SRC_R {
        PREDIV1SRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline(always)]
    pub fn i2s3src(&self) -> I2S3SRC_R {
        I2S3SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV1 division factor"]
    #[inline(always)]
    pub fn prediv1(&mut self) -> PREDIV1_W {
        PREDIV1_W { w: self }
    }
    #[doc = "Bits 4:7 - PREDIV2 division factor"]
    #[inline(always)]
    pub fn prediv2(&mut self) -> PREDIV2_W {
        PREDIV2_W { w: self }
    }
    #[doc = "Bits 8:11 - PLL2 Multiplication Factor"]
    #[inline(always)]
    pub fn pll2mul(&mut self) -> PLL2MUL_W {
        PLL2MUL_W { w: self }
    }
    #[doc = "Bits 12:15 - PLL3 Multiplication Factor"]
    #[inline(always)]
    pub fn pll3mul(&mut self) -> PLL3MUL_W {
        PLL3MUL_W { w: self }
    }
    #[doc = "Bit 16 - PREDIV1 entry clock source"]
    #[inline(always)]
    pub fn prediv1src(&mut self) -> PREDIV1SRC_W {
        PREDIV1SRC_W { w: self }
    }
    #[doc = "Bit 17 - I2S2 clock source"]
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W {
        I2S2SRC_W { w: self }
    }
    #[doc = "Bit 18 - I2S3 clock source"]
    #[inline(always)]
    pub fn i2s3src(&mut self) -> I2S3SRC_W {
        I2S3SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register2 (RCC_CFGR2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
