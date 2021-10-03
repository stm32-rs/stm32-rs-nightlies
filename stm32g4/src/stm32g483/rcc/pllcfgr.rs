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
#[doc = "Main PLL division factor for PLLSAI2CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLPDIV_A {
    #[doc = "0: pll_p_ck is controlled by PLLP"]
    PLLP = 0,
    #[doc = "2: pll_p_ck = vco_ck / 2"]
    DIV2 = 2,
    #[doc = "3: pll_p_ck = vco_ck / 3"]
    DIV3 = 3,
    #[doc = "4: pll_p_ck = vco_ck / 4"]
    DIV4 = 4,
    #[doc = "5: pll_p_ck = vco_ck / 5"]
    DIV5 = 5,
    #[doc = "6: pll_p_ck = vco_ck / 6"]
    DIV6 = 6,
    #[doc = "7: pll_p_ck = vco_ck / 7"]
    DIV7 = 7,
    #[doc = "8: pll_p_ck = vco_ck / 8"]
    DIV8 = 8,
    #[doc = "9: pll_p_ck = vco_ck / 9"]
    DIV9 = 9,
    #[doc = "10: pll_p_ck = vco_ck / 10"]
    DIV10 = 10,
    #[doc = "11: pll_p_ck = vco_ck / 11"]
    DIV11 = 11,
    #[doc = "12: pll_p_ck = vco_ck / 12"]
    DIV12 = 12,
    #[doc = "13: pll_p_ck = vco_ck / 13"]
    DIV13 = 13,
    #[doc = "14: pll_p_ck = vco_ck / 14"]
    DIV14 = 14,
    #[doc = "15: pll_p_ck = vco_ck / 15"]
    DIV15 = 15,
    #[doc = "16: pll_p_ck = vco_ck / 16"]
    DIV16 = 16,
    #[doc = "17: pll_p_ck = vco_ck / 17"]
    DIV17 = 17,
    #[doc = "18: pll_p_ck = vco_ck / 18"]
    DIV18 = 18,
    #[doc = "19: pll_p_ck = vco_ck / 19"]
    DIV19 = 19,
    #[doc = "20: pll_p_ck = vco_ck / 20"]
    DIV20 = 20,
    #[doc = "21: pll_p_ck = vco_ck / 21"]
    DIV21 = 21,
    #[doc = "22: pll_p_ck = vco_ck / 22"]
    DIV22 = 22,
    #[doc = "23: pll_p_ck = vco_ck / 23"]
    DIV23 = 23,
    #[doc = "24: pll_p_ck = vco_ck / 24"]
    DIV24 = 24,
    #[doc = "25: pll_p_ck = vco_ck / 25"]
    DIV25 = 25,
    #[doc = "26: pll_p_ck = vco_ck / 26"]
    DIV26 = 26,
    #[doc = "27: pll_p_ck = vco_ck / 27"]
    DIV27 = 27,
    #[doc = "28: pll_p_ck = vco_ck / 28"]
    DIV28 = 28,
    #[doc = "29: pll_p_ck = vco_ck / 29"]
    DIV29 = 29,
    #[doc = "30: pll_p_ck = vco_ck / 30"]
    DIV30 = 30,
    #[doc = "31: pll_p_ck = vco_ck / 31"]
    DIV31 = 31,
}
impl From<PLLPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK"]
pub struct PLLPDIV_R(crate::FieldReader<u8, PLLPDIV_A>);
impl PLLPDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLPDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLPDIV_A> {
        match self.bits {
            0 => Some(PLLPDIV_A::PLLP),
            2 => Some(PLLPDIV_A::DIV2),
            3 => Some(PLLPDIV_A::DIV3),
            4 => Some(PLLPDIV_A::DIV4),
            5 => Some(PLLPDIV_A::DIV5),
            6 => Some(PLLPDIV_A::DIV6),
            7 => Some(PLLPDIV_A::DIV7),
            8 => Some(PLLPDIV_A::DIV8),
            9 => Some(PLLPDIV_A::DIV9),
            10 => Some(PLLPDIV_A::DIV10),
            11 => Some(PLLPDIV_A::DIV11),
            12 => Some(PLLPDIV_A::DIV12),
            13 => Some(PLLPDIV_A::DIV13),
            14 => Some(PLLPDIV_A::DIV14),
            15 => Some(PLLPDIV_A::DIV15),
            16 => Some(PLLPDIV_A::DIV16),
            17 => Some(PLLPDIV_A::DIV17),
            18 => Some(PLLPDIV_A::DIV18),
            19 => Some(PLLPDIV_A::DIV19),
            20 => Some(PLLPDIV_A::DIV20),
            21 => Some(PLLPDIV_A::DIV21),
            22 => Some(PLLPDIV_A::DIV22),
            23 => Some(PLLPDIV_A::DIV23),
            24 => Some(PLLPDIV_A::DIV24),
            25 => Some(PLLPDIV_A::DIV25),
            26 => Some(PLLPDIV_A::DIV26),
            27 => Some(PLLPDIV_A::DIV27),
            28 => Some(PLLPDIV_A::DIV28),
            29 => Some(PLLPDIV_A::DIV29),
            30 => Some(PLLPDIV_A::DIV30),
            31 => Some(PLLPDIV_A::DIV31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLLP`"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        **self == PLLPDIV_A::PLLP
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLPDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PLLPDIV_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLPDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        **self == PLLPDIV_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLPDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLPDIV_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLPDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PLLPDIV_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PLLPDIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PLLPDIV_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PLLPDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PLLPDIV_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PLLPDIV_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PLLPDIV_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PLLPDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        **self == PLLPDIV_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        **self == PLLPDIV_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        **self == PLLPDIV_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        **self == PLLPDIV_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        **self == PLLPDIV_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        **self == PLLPDIV_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        **self == PLLPDIV_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        **self == PLLPDIV_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        **self == PLLPDIV_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        **self == PLLPDIV_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        **self == PLLPDIV_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        **self == PLLPDIV_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        **self == PLLPDIV_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        **self == PLLPDIV_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        **self == PLLPDIV_A::DIV31
    }
}
impl core::ops::Deref for PLLPDIV_R {
    type Target = crate::FieldReader<u8, PLLPDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK"]
pub struct PLLPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll_p_ck is controlled by PLLP"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(PLLPDIV_A::PLLP)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV2)
    }
    #[doc = "pll_p_ck = vco_ck / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV3)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV4)
    }
    #[doc = "pll_p_ck = vco_ck / 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV5)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV6)
    }
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV7)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV8)
    }
    #[doc = "pll_p_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV9)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV10)
    }
    #[doc = "pll_p_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV11)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV12)
    }
    #[doc = "pll_p_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV13)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV14)
    }
    #[doc = "pll_p_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV15)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV16)
    }
    #[doc = "pll_p_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV17)
    }
    #[doc = "pll_p_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV18)
    }
    #[doc = "pll_p_ck = vco_ck / 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV19)
    }
    #[doc = "pll_p_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV20)
    }
    #[doc = "pll_p_ck = vco_ck / 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV21)
    }
    #[doc = "pll_p_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV22)
    }
    #[doc = "pll_p_ck = vco_ck / 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV23)
    }
    #[doc = "pll_p_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV24)
    }
    #[doc = "pll_p_ck = vco_ck / 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV25)
    }
    #[doc = "pll_p_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV26)
    }
    #[doc = "pll_p_ck = vco_ck / 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV27)
    }
    #[doc = "pll_p_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV28)
    }
    #[doc = "pll_p_ck = vco_ck / 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV29)
    }
    #[doc = "pll_p_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV30)
    }
    #[doc = "pll_p_ck = vco_ck / 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLPDIV_A::DIV31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Main PLL division factor for PLLCLK (system clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLR_A {
    #[doc = "0: pll_r_ck = vco_ck / 2"]
    DIV2 = 0,
    #[doc = "1: pll_r_ck = vco_ck / 4"]
    DIV4 = 1,
    #[doc = "2: pll_r_ck = vco_ck / 6"]
    DIV6 = 2,
    #[doc = "3: pll_r_ck = vco_ck / 8"]
    DIV8 = 3,
}
impl From<PLLR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)"]
pub struct PLLR_R(crate::FieldReader<u8, PLLR_A>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLR_A {
        match self.bits {
            0 => PLLR_A::DIV2,
            1 => PLLR_A::DIV4,
            2 => PLLR_A::DIV6,
            3 => PLLR_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLR_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLR_A::DIV8
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, PLLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll_r_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLR_A::DIV2)
    }
    #[doc = "pll_r_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLR_A::DIV4)
    }
    #[doc = "pll_r_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLR_A::DIV6)
    }
    #[doc = "pll_r_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLR_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `PLLREN` reader - Main PLL PLLCLK output enable"]
pub struct PLLREN_R(crate::FieldReader<bool, bool>);
impl PLLREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLREN` writer - Main PLL PLLCLK output enable"]
pub struct PLLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLREN_W<'a> {
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
#[doc = "Main PLL division factor for PLLUSB1CLK(48 MHz clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "0: pll_q_ck = vco_ck / 2"]
    DIV2 = 0,
    #[doc = "1: pll_q_ck = vco_ck / 4"]
    DIV4 = 1,
    #[doc = "2: pll_q_ck = vco_ck / 6"]
    DIV6 = 2,
    #[doc = "3: pll_q_ck = vco_ck / 8"]
    DIV8 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub struct PLLQ_R(crate::FieldReader<u8, PLLQ_A>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLQ_A {
        match self.bits {
            0 => PLLQ_A::DIV2,
            1 => PLLQ_A::DIV4,
            2 => PLLQ_A::DIV6,
            3 => PLLQ_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLQ_A::DIV6
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
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll_q_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV2)
    }
    #[doc = "pll_q_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV4)
    }
    #[doc = "pll_q_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV6)
    }
    #[doc = "pll_q_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable"]
pub struct PLLQEN_R(crate::FieldReader<bool, bool>);
impl PLLQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable"]
pub struct PLLQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLP_A {
    #[doc = "0: pll_p_ck = vco_ck / 7"]
    DIV7 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 17"]
    DIV17 = 1,
}
impl From<PLLP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub struct PLLP_R(crate::FieldReader<bool, PLLP_A>);
impl PLLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            false => PLLP_A::DIV7,
            true => PLLP_A::DIV17,
        }
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLP_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        **self == PLLP_A::DIV17
    }
}
impl core::ops::Deref for PLLP_R {
    type Target = crate::FieldReader<bool, PLLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::DIV7)
    }
    #[doc = "pll_p_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::DIV17)
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
#[doc = "Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable"]
pub struct PLLPEN_R(crate::FieldReader<bool, bool>);
impl PLLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable"]
pub struct PLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPEN_W<'a> {
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
#[doc = "Main PLL multiplication factor for VCO\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLN_A {
    #[doc = "8: pll_n_ck = vco_ck / 8"]
    DIV8 = 8,
    #[doc = "9: pll_n_ck = vco_ck / 9"]
    DIV9 = 9,
    #[doc = "10: pll_n_ck = vco_ck / 10"]
    DIV10 = 10,
    #[doc = "11: pll_n_ck = vco_ck / 11"]
    DIV11 = 11,
    #[doc = "12: pll_n_ck = vco_ck / 12"]
    DIV12 = 12,
    #[doc = "13: pll_n_ck = vco_ck / 13"]
    DIV13 = 13,
    #[doc = "14: pll_n_ck = vco_ck / 14"]
    DIV14 = 14,
    #[doc = "15: pll_n_ck = vco_ck / 15"]
    DIV15 = 15,
    #[doc = "16: pll_n_ck = vco_ck / 16"]
    DIV16 = 16,
    #[doc = "17: pll_n_ck = vco_ck / 17"]
    DIV17 = 17,
    #[doc = "18: pll_n_ck = vco_ck / 18"]
    DIV18 = 18,
    #[doc = "19: pll_n_ck = vco_ck / 19"]
    DIV19 = 19,
    #[doc = "20: pll_n_ck = vco_ck / 20"]
    DIV20 = 20,
    #[doc = "21: pll_n_ck = vco_ck / 21"]
    DIV21 = 21,
    #[doc = "22: pll_n_ck = vco_ck / 22"]
    DIV22 = 22,
    #[doc = "23: pll_n_ck = vco_ck / 23"]
    DIV23 = 23,
    #[doc = "24: pll_n_ck = vco_ck / 24"]
    DIV24 = 24,
    #[doc = "25: pll_n_ck = vco_ck / 25"]
    DIV25 = 25,
    #[doc = "26: pll_n_ck = vco_ck / 26"]
    DIV26 = 26,
    #[doc = "27: pll_n_ck = vco_ck / 27"]
    DIV27 = 27,
    #[doc = "28: pll_n_ck = vco_ck / 28"]
    DIV28 = 28,
    #[doc = "29: pll_n_ck = vco_ck / 29"]
    DIV29 = 29,
    #[doc = "30: pll_n_ck = vco_ck / 30"]
    DIV30 = 30,
    #[doc = "31: pll_n_ck = vco_ck / 31"]
    DIV31 = 31,
    #[doc = "32: pll_n_ck = vco_ck / 32"]
    DIV32 = 32,
    #[doc = "33: pll_n_ck = vco_ck / 33"]
    DIV33 = 33,
    #[doc = "34: pll_n_ck = vco_ck / 34"]
    DIV34 = 34,
    #[doc = "35: pll_n_ck = vco_ck / 35"]
    DIV35 = 35,
    #[doc = "36: pll_n_ck = vco_ck / 36"]
    DIV36 = 36,
    #[doc = "37: pll_n_ck = vco_ck / 37"]
    DIV37 = 37,
    #[doc = "38: pll_n_ck = vco_ck / 38"]
    DIV38 = 38,
    #[doc = "39: pll_n_ck = vco_ck / 39"]
    DIV39 = 39,
    #[doc = "40: pll_n_ck = vco_ck / 40"]
    DIV40 = 40,
    #[doc = "41: pll_n_ck = vco_ck / 41"]
    DIV41 = 41,
    #[doc = "42: pll_n_ck = vco_ck / 42"]
    DIV42 = 42,
    #[doc = "43: pll_n_ck = vco_ck / 43"]
    DIV43 = 43,
    #[doc = "44: pll_n_ck = vco_ck / 44"]
    DIV44 = 44,
    #[doc = "45: pll_n_ck = vco_ck / 45"]
    DIV45 = 45,
    #[doc = "46: pll_n_ck = vco_ck / 46"]
    DIV46 = 46,
    #[doc = "47: pll_n_ck = vco_ck / 47"]
    DIV47 = 47,
    #[doc = "48: pll_n_ck = vco_ck / 48"]
    DIV48 = 48,
    #[doc = "49: pll_n_ck = vco_ck / 49"]
    DIV49 = 49,
    #[doc = "50: pll_n_ck = vco_ck / 50"]
    DIV50 = 50,
    #[doc = "51: pll_n_ck = vco_ck / 51"]
    DIV51 = 51,
    #[doc = "52: pll_n_ck = vco_ck / 52"]
    DIV52 = 52,
    #[doc = "53: pll_n_ck = vco_ck / 53"]
    DIV53 = 53,
    #[doc = "54: pll_n_ck = vco_ck / 54"]
    DIV54 = 54,
    #[doc = "55: pll_n_ck = vco_ck / 55"]
    DIV55 = 55,
    #[doc = "56: pll_n_ck = vco_ck / 56"]
    DIV56 = 56,
    #[doc = "57: pll_n_ck = vco_ck / 57"]
    DIV57 = 57,
    #[doc = "58: pll_n_ck = vco_ck / 58"]
    DIV58 = 58,
    #[doc = "59: pll_n_ck = vco_ck / 59"]
    DIV59 = 59,
    #[doc = "60: pll_n_ck = vco_ck / 60"]
    DIV60 = 60,
    #[doc = "61: pll_n_ck = vco_ck / 61"]
    DIV61 = 61,
    #[doc = "62: pll_n_ck = vco_ck / 62"]
    DIV62 = 62,
    #[doc = "63: pll_n_ck = vco_ck / 63"]
    DIV63 = 63,
    #[doc = "64: pll_n_ck = vco_ck / 64"]
    DIV64 = 64,
    #[doc = "65: pll_n_ck = vco_ck / 65"]
    DIV65 = 65,
    #[doc = "66: pll_n_ck = vco_ck / 66"]
    DIV66 = 66,
    #[doc = "67: pll_n_ck = vco_ck / 67"]
    DIV67 = 67,
    #[doc = "68: pll_n_ck = vco_ck / 68"]
    DIV68 = 68,
    #[doc = "69: pll_n_ck = vco_ck / 69"]
    DIV69 = 69,
    #[doc = "70: pll_n_ck = vco_ck / 70"]
    DIV70 = 70,
    #[doc = "71: pll_n_ck = vco_ck / 71"]
    DIV71 = 71,
    #[doc = "72: pll_n_ck = vco_ck / 72"]
    DIV72 = 72,
    #[doc = "73: pll_n_ck = vco_ck / 73"]
    DIV73 = 73,
    #[doc = "74: pll_n_ck = vco_ck / 74"]
    DIV74 = 74,
    #[doc = "75: pll_n_ck = vco_ck / 75"]
    DIV75 = 75,
    #[doc = "76: pll_n_ck = vco_ck / 76"]
    DIV76 = 76,
    #[doc = "77: pll_n_ck = vco_ck / 77"]
    DIV77 = 77,
    #[doc = "78: pll_n_ck = vco_ck / 78"]
    DIV78 = 78,
    #[doc = "79: pll_n_ck = vco_ck / 79"]
    DIV79 = 79,
    #[doc = "80: pll_n_ck = vco_ck / 80"]
    DIV80 = 80,
    #[doc = "81: pll_n_ck = vco_ck / 81"]
    DIV81 = 81,
    #[doc = "82: pll_n_ck = vco_ck / 82"]
    DIV82 = 82,
    #[doc = "83: pll_n_ck = vco_ck / 83"]
    DIV83 = 83,
    #[doc = "84: pll_n_ck = vco_ck / 84"]
    DIV84 = 84,
    #[doc = "85: pll_n_ck = vco_ck / 85"]
    DIV85 = 85,
    #[doc = "86: pll_n_ck = vco_ck / 86"]
    DIV86 = 86,
    #[doc = "87: pll_n_ck = vco_ck / 87"]
    DIV87 = 87,
    #[doc = "88: pll_n_ck = vco_ck / 88"]
    DIV88 = 88,
    #[doc = "89: pll_n_ck = vco_ck / 89"]
    DIV89 = 89,
    #[doc = "90: pll_n_ck = vco_ck / 90"]
    DIV90 = 90,
    #[doc = "91: pll_n_ck = vco_ck / 91"]
    DIV91 = 91,
    #[doc = "92: pll_n_ck = vco_ck / 92"]
    DIV92 = 92,
    #[doc = "93: pll_n_ck = vco_ck / 93"]
    DIV93 = 93,
    #[doc = "94: pll_n_ck = vco_ck / 94"]
    DIV94 = 94,
    #[doc = "95: pll_n_ck = vco_ck / 95"]
    DIV95 = 95,
    #[doc = "96: pll_n_ck = vco_ck / 96"]
    DIV96 = 96,
    #[doc = "97: pll_n_ck = vco_ck / 97"]
    DIV97 = 97,
    #[doc = "98: pll_n_ck = vco_ck / 98"]
    DIV98 = 98,
    #[doc = "99: pll_n_ck = vco_ck / 99"]
    DIV99 = 99,
    #[doc = "100: pll_n_ck = vco_ck / 100"]
    DIV100 = 100,
    #[doc = "101: pll_n_ck = vco_ck / 101"]
    DIV101 = 101,
    #[doc = "102: pll_n_ck = vco_ck / 102"]
    DIV102 = 102,
    #[doc = "103: pll_n_ck = vco_ck / 103"]
    DIV103 = 103,
    #[doc = "104: pll_n_ck = vco_ck / 104"]
    DIV104 = 104,
    #[doc = "105: pll_n_ck = vco_ck / 105"]
    DIV105 = 105,
    #[doc = "106: pll_n_ck = vco_ck / 106"]
    DIV106 = 106,
    #[doc = "107: pll_n_ck = vco_ck / 107"]
    DIV107 = 107,
    #[doc = "108: pll_n_ck = vco_ck / 108"]
    DIV108 = 108,
    #[doc = "109: pll_n_ck = vco_ck / 109"]
    DIV109 = 109,
    #[doc = "110: pll_n_ck = vco_ck / 110"]
    DIV110 = 110,
    #[doc = "111: pll_n_ck = vco_ck / 111"]
    DIV111 = 111,
    #[doc = "112: pll_n_ck = vco_ck / 112"]
    DIV112 = 112,
    #[doc = "113: pll_n_ck = vco_ck / 113"]
    DIV113 = 113,
    #[doc = "114: pll_n_ck = vco_ck / 114"]
    DIV114 = 114,
    #[doc = "115: pll_n_ck = vco_ck / 115"]
    DIV115 = 115,
    #[doc = "116: pll_n_ck = vco_ck / 116"]
    DIV116 = 116,
    #[doc = "117: pll_n_ck = vco_ck / 117"]
    DIV117 = 117,
    #[doc = "118: pll_n_ck = vco_ck / 118"]
    DIV118 = 118,
    #[doc = "119: pll_n_ck = vco_ck / 119"]
    DIV119 = 119,
    #[doc = "120: pll_n_ck = vco_ck / 120"]
    DIV120 = 120,
    #[doc = "121: pll_n_ck = vco_ck / 121"]
    DIV121 = 121,
    #[doc = "122: pll_n_ck = vco_ck / 122"]
    DIV122 = 122,
    #[doc = "123: pll_n_ck = vco_ck / 123"]
    DIV123 = 123,
    #[doc = "124: pll_n_ck = vco_ck / 124"]
    DIV124 = 124,
    #[doc = "125: pll_n_ck = vco_ck / 125"]
    DIV125 = 125,
    #[doc = "126: pll_n_ck = vco_ck / 126"]
    DIV126 = 126,
    #[doc = "127: pll_n_ck = vco_ck / 127"]
    DIV127 = 127,
}
impl From<PLLN_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub struct PLLN_R(crate::FieldReader<u8, PLLN_A>);
impl PLLN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLN_A> {
        match self.bits {
            8 => Some(PLLN_A::DIV8),
            9 => Some(PLLN_A::DIV9),
            10 => Some(PLLN_A::DIV10),
            11 => Some(PLLN_A::DIV11),
            12 => Some(PLLN_A::DIV12),
            13 => Some(PLLN_A::DIV13),
            14 => Some(PLLN_A::DIV14),
            15 => Some(PLLN_A::DIV15),
            16 => Some(PLLN_A::DIV16),
            17 => Some(PLLN_A::DIV17),
            18 => Some(PLLN_A::DIV18),
            19 => Some(PLLN_A::DIV19),
            20 => Some(PLLN_A::DIV20),
            21 => Some(PLLN_A::DIV21),
            22 => Some(PLLN_A::DIV22),
            23 => Some(PLLN_A::DIV23),
            24 => Some(PLLN_A::DIV24),
            25 => Some(PLLN_A::DIV25),
            26 => Some(PLLN_A::DIV26),
            27 => Some(PLLN_A::DIV27),
            28 => Some(PLLN_A::DIV28),
            29 => Some(PLLN_A::DIV29),
            30 => Some(PLLN_A::DIV30),
            31 => Some(PLLN_A::DIV31),
            32 => Some(PLLN_A::DIV32),
            33 => Some(PLLN_A::DIV33),
            34 => Some(PLLN_A::DIV34),
            35 => Some(PLLN_A::DIV35),
            36 => Some(PLLN_A::DIV36),
            37 => Some(PLLN_A::DIV37),
            38 => Some(PLLN_A::DIV38),
            39 => Some(PLLN_A::DIV39),
            40 => Some(PLLN_A::DIV40),
            41 => Some(PLLN_A::DIV41),
            42 => Some(PLLN_A::DIV42),
            43 => Some(PLLN_A::DIV43),
            44 => Some(PLLN_A::DIV44),
            45 => Some(PLLN_A::DIV45),
            46 => Some(PLLN_A::DIV46),
            47 => Some(PLLN_A::DIV47),
            48 => Some(PLLN_A::DIV48),
            49 => Some(PLLN_A::DIV49),
            50 => Some(PLLN_A::DIV50),
            51 => Some(PLLN_A::DIV51),
            52 => Some(PLLN_A::DIV52),
            53 => Some(PLLN_A::DIV53),
            54 => Some(PLLN_A::DIV54),
            55 => Some(PLLN_A::DIV55),
            56 => Some(PLLN_A::DIV56),
            57 => Some(PLLN_A::DIV57),
            58 => Some(PLLN_A::DIV58),
            59 => Some(PLLN_A::DIV59),
            60 => Some(PLLN_A::DIV60),
            61 => Some(PLLN_A::DIV61),
            62 => Some(PLLN_A::DIV62),
            63 => Some(PLLN_A::DIV63),
            64 => Some(PLLN_A::DIV64),
            65 => Some(PLLN_A::DIV65),
            66 => Some(PLLN_A::DIV66),
            67 => Some(PLLN_A::DIV67),
            68 => Some(PLLN_A::DIV68),
            69 => Some(PLLN_A::DIV69),
            70 => Some(PLLN_A::DIV70),
            71 => Some(PLLN_A::DIV71),
            72 => Some(PLLN_A::DIV72),
            73 => Some(PLLN_A::DIV73),
            74 => Some(PLLN_A::DIV74),
            75 => Some(PLLN_A::DIV75),
            76 => Some(PLLN_A::DIV76),
            77 => Some(PLLN_A::DIV77),
            78 => Some(PLLN_A::DIV78),
            79 => Some(PLLN_A::DIV79),
            80 => Some(PLLN_A::DIV80),
            81 => Some(PLLN_A::DIV81),
            82 => Some(PLLN_A::DIV82),
            83 => Some(PLLN_A::DIV83),
            84 => Some(PLLN_A::DIV84),
            85 => Some(PLLN_A::DIV85),
            86 => Some(PLLN_A::DIV86),
            87 => Some(PLLN_A::DIV87),
            88 => Some(PLLN_A::DIV88),
            89 => Some(PLLN_A::DIV89),
            90 => Some(PLLN_A::DIV90),
            91 => Some(PLLN_A::DIV91),
            92 => Some(PLLN_A::DIV92),
            93 => Some(PLLN_A::DIV93),
            94 => Some(PLLN_A::DIV94),
            95 => Some(PLLN_A::DIV95),
            96 => Some(PLLN_A::DIV96),
            97 => Some(PLLN_A::DIV97),
            98 => Some(PLLN_A::DIV98),
            99 => Some(PLLN_A::DIV99),
            100 => Some(PLLN_A::DIV100),
            101 => Some(PLLN_A::DIV101),
            102 => Some(PLLN_A::DIV102),
            103 => Some(PLLN_A::DIV103),
            104 => Some(PLLN_A::DIV104),
            105 => Some(PLLN_A::DIV105),
            106 => Some(PLLN_A::DIV106),
            107 => Some(PLLN_A::DIV107),
            108 => Some(PLLN_A::DIV108),
            109 => Some(PLLN_A::DIV109),
            110 => Some(PLLN_A::DIV110),
            111 => Some(PLLN_A::DIV111),
            112 => Some(PLLN_A::DIV112),
            113 => Some(PLLN_A::DIV113),
            114 => Some(PLLN_A::DIV114),
            115 => Some(PLLN_A::DIV115),
            116 => Some(PLLN_A::DIV116),
            117 => Some(PLLN_A::DIV117),
            118 => Some(PLLN_A::DIV118),
            119 => Some(PLLN_A::DIV119),
            120 => Some(PLLN_A::DIV120),
            121 => Some(PLLN_A::DIV121),
            122 => Some(PLLN_A::DIV122),
            123 => Some(PLLN_A::DIV123),
            124 => Some(PLLN_A::DIV124),
            125 => Some(PLLN_A::DIV125),
            126 => Some(PLLN_A::DIV126),
            127 => Some(PLLN_A::DIV127),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLN_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PLLN_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PLLN_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PLLN_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PLLN_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PLLN_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PLLN_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PLLN_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PLLN_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        **self == PLLN_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        **self == PLLN_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        **self == PLLN_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        **self == PLLN_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        **self == PLLN_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        **self == PLLN_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        **self == PLLN_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        **self == PLLN_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        **self == PLLN_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        **self == PLLN_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        **self == PLLN_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        **self == PLLN_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        **self == PLLN_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        **self == PLLN_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        **self == PLLN_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PLLN_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV33`"]
    #[inline(always)]
    pub fn is_div33(&self) -> bool {
        **self == PLLN_A::DIV33
    }
    #[doc = "Checks if the value of the field is `DIV34`"]
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        **self == PLLN_A::DIV34
    }
    #[doc = "Checks if the value of the field is `DIV35`"]
    #[inline(always)]
    pub fn is_div35(&self) -> bool {
        **self == PLLN_A::DIV35
    }
    #[doc = "Checks if the value of the field is `DIV36`"]
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        **self == PLLN_A::DIV36
    }
    #[doc = "Checks if the value of the field is `DIV37`"]
    #[inline(always)]
    pub fn is_div37(&self) -> bool {
        **self == PLLN_A::DIV37
    }
    #[doc = "Checks if the value of the field is `DIV38`"]
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        **self == PLLN_A::DIV38
    }
    #[doc = "Checks if the value of the field is `DIV39`"]
    #[inline(always)]
    pub fn is_div39(&self) -> bool {
        **self == PLLN_A::DIV39
    }
    #[doc = "Checks if the value of the field is `DIV40`"]
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        **self == PLLN_A::DIV40
    }
    #[doc = "Checks if the value of the field is `DIV41`"]
    #[inline(always)]
    pub fn is_div41(&self) -> bool {
        **self == PLLN_A::DIV41
    }
    #[doc = "Checks if the value of the field is `DIV42`"]
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        **self == PLLN_A::DIV42
    }
    #[doc = "Checks if the value of the field is `DIV43`"]
    #[inline(always)]
    pub fn is_div43(&self) -> bool {
        **self == PLLN_A::DIV43
    }
    #[doc = "Checks if the value of the field is `DIV44`"]
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        **self == PLLN_A::DIV44
    }
    #[doc = "Checks if the value of the field is `DIV45`"]
    #[inline(always)]
    pub fn is_div45(&self) -> bool {
        **self == PLLN_A::DIV45
    }
    #[doc = "Checks if the value of the field is `DIV46`"]
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        **self == PLLN_A::DIV46
    }
    #[doc = "Checks if the value of the field is `DIV47`"]
    #[inline(always)]
    pub fn is_div47(&self) -> bool {
        **self == PLLN_A::DIV47
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        **self == PLLN_A::DIV48
    }
    #[doc = "Checks if the value of the field is `DIV49`"]
    #[inline(always)]
    pub fn is_div49(&self) -> bool {
        **self == PLLN_A::DIV49
    }
    #[doc = "Checks if the value of the field is `DIV50`"]
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        **self == PLLN_A::DIV50
    }
    #[doc = "Checks if the value of the field is `DIV51`"]
    #[inline(always)]
    pub fn is_div51(&self) -> bool {
        **self == PLLN_A::DIV51
    }
    #[doc = "Checks if the value of the field is `DIV52`"]
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        **self == PLLN_A::DIV52
    }
    #[doc = "Checks if the value of the field is `DIV53`"]
    #[inline(always)]
    pub fn is_div53(&self) -> bool {
        **self == PLLN_A::DIV53
    }
    #[doc = "Checks if the value of the field is `DIV54`"]
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        **self == PLLN_A::DIV54
    }
    #[doc = "Checks if the value of the field is `DIV55`"]
    #[inline(always)]
    pub fn is_div55(&self) -> bool {
        **self == PLLN_A::DIV55
    }
    #[doc = "Checks if the value of the field is `DIV56`"]
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        **self == PLLN_A::DIV56
    }
    #[doc = "Checks if the value of the field is `DIV57`"]
    #[inline(always)]
    pub fn is_div57(&self) -> bool {
        **self == PLLN_A::DIV57
    }
    #[doc = "Checks if the value of the field is `DIV58`"]
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        **self == PLLN_A::DIV58
    }
    #[doc = "Checks if the value of the field is `DIV59`"]
    #[inline(always)]
    pub fn is_div59(&self) -> bool {
        **self == PLLN_A::DIV59
    }
    #[doc = "Checks if the value of the field is `DIV60`"]
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        **self == PLLN_A::DIV60
    }
    #[doc = "Checks if the value of the field is `DIV61`"]
    #[inline(always)]
    pub fn is_div61(&self) -> bool {
        **self == PLLN_A::DIV61
    }
    #[doc = "Checks if the value of the field is `DIV62`"]
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        **self == PLLN_A::DIV62
    }
    #[doc = "Checks if the value of the field is `DIV63`"]
    #[inline(always)]
    pub fn is_div63(&self) -> bool {
        **self == PLLN_A::DIV63
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PLLN_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV65`"]
    #[inline(always)]
    pub fn is_div65(&self) -> bool {
        **self == PLLN_A::DIV65
    }
    #[doc = "Checks if the value of the field is `DIV66`"]
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        **self == PLLN_A::DIV66
    }
    #[doc = "Checks if the value of the field is `DIV67`"]
    #[inline(always)]
    pub fn is_div67(&self) -> bool {
        **self == PLLN_A::DIV67
    }
    #[doc = "Checks if the value of the field is `DIV68`"]
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        **self == PLLN_A::DIV68
    }
    #[doc = "Checks if the value of the field is `DIV69`"]
    #[inline(always)]
    pub fn is_div69(&self) -> bool {
        **self == PLLN_A::DIV69
    }
    #[doc = "Checks if the value of the field is `DIV70`"]
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        **self == PLLN_A::DIV70
    }
    #[doc = "Checks if the value of the field is `DIV71`"]
    #[inline(always)]
    pub fn is_div71(&self) -> bool {
        **self == PLLN_A::DIV71
    }
    #[doc = "Checks if the value of the field is `DIV72`"]
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        **self == PLLN_A::DIV72
    }
    #[doc = "Checks if the value of the field is `DIV73`"]
    #[inline(always)]
    pub fn is_div73(&self) -> bool {
        **self == PLLN_A::DIV73
    }
    #[doc = "Checks if the value of the field is `DIV74`"]
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        **self == PLLN_A::DIV74
    }
    #[doc = "Checks if the value of the field is `DIV75`"]
    #[inline(always)]
    pub fn is_div75(&self) -> bool {
        **self == PLLN_A::DIV75
    }
    #[doc = "Checks if the value of the field is `DIV76`"]
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        **self == PLLN_A::DIV76
    }
    #[doc = "Checks if the value of the field is `DIV77`"]
    #[inline(always)]
    pub fn is_div77(&self) -> bool {
        **self == PLLN_A::DIV77
    }
    #[doc = "Checks if the value of the field is `DIV78`"]
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        **self == PLLN_A::DIV78
    }
    #[doc = "Checks if the value of the field is `DIV79`"]
    #[inline(always)]
    pub fn is_div79(&self) -> bool {
        **self == PLLN_A::DIV79
    }
    #[doc = "Checks if the value of the field is `DIV80`"]
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        **self == PLLN_A::DIV80
    }
    #[doc = "Checks if the value of the field is `DIV81`"]
    #[inline(always)]
    pub fn is_div81(&self) -> bool {
        **self == PLLN_A::DIV81
    }
    #[doc = "Checks if the value of the field is `DIV82`"]
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        **self == PLLN_A::DIV82
    }
    #[doc = "Checks if the value of the field is `DIV83`"]
    #[inline(always)]
    pub fn is_div83(&self) -> bool {
        **self == PLLN_A::DIV83
    }
    #[doc = "Checks if the value of the field is `DIV84`"]
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        **self == PLLN_A::DIV84
    }
    #[doc = "Checks if the value of the field is `DIV85`"]
    #[inline(always)]
    pub fn is_div85(&self) -> bool {
        **self == PLLN_A::DIV85
    }
    #[doc = "Checks if the value of the field is `DIV86`"]
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        **self == PLLN_A::DIV86
    }
    #[doc = "Checks if the value of the field is `DIV87`"]
    #[inline(always)]
    pub fn is_div87(&self) -> bool {
        **self == PLLN_A::DIV87
    }
    #[doc = "Checks if the value of the field is `DIV88`"]
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        **self == PLLN_A::DIV88
    }
    #[doc = "Checks if the value of the field is `DIV89`"]
    #[inline(always)]
    pub fn is_div89(&self) -> bool {
        **self == PLLN_A::DIV89
    }
    #[doc = "Checks if the value of the field is `DIV90`"]
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        **self == PLLN_A::DIV90
    }
    #[doc = "Checks if the value of the field is `DIV91`"]
    #[inline(always)]
    pub fn is_div91(&self) -> bool {
        **self == PLLN_A::DIV91
    }
    #[doc = "Checks if the value of the field is `DIV92`"]
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        **self == PLLN_A::DIV92
    }
    #[doc = "Checks if the value of the field is `DIV93`"]
    #[inline(always)]
    pub fn is_div93(&self) -> bool {
        **self == PLLN_A::DIV93
    }
    #[doc = "Checks if the value of the field is `DIV94`"]
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        **self == PLLN_A::DIV94
    }
    #[doc = "Checks if the value of the field is `DIV95`"]
    #[inline(always)]
    pub fn is_div95(&self) -> bool {
        **self == PLLN_A::DIV95
    }
    #[doc = "Checks if the value of the field is `DIV96`"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        **self == PLLN_A::DIV96
    }
    #[doc = "Checks if the value of the field is `DIV97`"]
    #[inline(always)]
    pub fn is_div97(&self) -> bool {
        **self == PLLN_A::DIV97
    }
    #[doc = "Checks if the value of the field is `DIV98`"]
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        **self == PLLN_A::DIV98
    }
    #[doc = "Checks if the value of the field is `DIV99`"]
    #[inline(always)]
    pub fn is_div99(&self) -> bool {
        **self == PLLN_A::DIV99
    }
    #[doc = "Checks if the value of the field is `DIV100`"]
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        **self == PLLN_A::DIV100
    }
    #[doc = "Checks if the value of the field is `DIV101`"]
    #[inline(always)]
    pub fn is_div101(&self) -> bool {
        **self == PLLN_A::DIV101
    }
    #[doc = "Checks if the value of the field is `DIV102`"]
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        **self == PLLN_A::DIV102
    }
    #[doc = "Checks if the value of the field is `DIV103`"]
    #[inline(always)]
    pub fn is_div103(&self) -> bool {
        **self == PLLN_A::DIV103
    }
    #[doc = "Checks if the value of the field is `DIV104`"]
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        **self == PLLN_A::DIV104
    }
    #[doc = "Checks if the value of the field is `DIV105`"]
    #[inline(always)]
    pub fn is_div105(&self) -> bool {
        **self == PLLN_A::DIV105
    }
    #[doc = "Checks if the value of the field is `DIV106`"]
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        **self == PLLN_A::DIV106
    }
    #[doc = "Checks if the value of the field is `DIV107`"]
    #[inline(always)]
    pub fn is_div107(&self) -> bool {
        **self == PLLN_A::DIV107
    }
    #[doc = "Checks if the value of the field is `DIV108`"]
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        **self == PLLN_A::DIV108
    }
    #[doc = "Checks if the value of the field is `DIV109`"]
    #[inline(always)]
    pub fn is_div109(&self) -> bool {
        **self == PLLN_A::DIV109
    }
    #[doc = "Checks if the value of the field is `DIV110`"]
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        **self == PLLN_A::DIV110
    }
    #[doc = "Checks if the value of the field is `DIV111`"]
    #[inline(always)]
    pub fn is_div111(&self) -> bool {
        **self == PLLN_A::DIV111
    }
    #[doc = "Checks if the value of the field is `DIV112`"]
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        **self == PLLN_A::DIV112
    }
    #[doc = "Checks if the value of the field is `DIV113`"]
    #[inline(always)]
    pub fn is_div113(&self) -> bool {
        **self == PLLN_A::DIV113
    }
    #[doc = "Checks if the value of the field is `DIV114`"]
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        **self == PLLN_A::DIV114
    }
    #[doc = "Checks if the value of the field is `DIV115`"]
    #[inline(always)]
    pub fn is_div115(&self) -> bool {
        **self == PLLN_A::DIV115
    }
    #[doc = "Checks if the value of the field is `DIV116`"]
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        **self == PLLN_A::DIV116
    }
    #[doc = "Checks if the value of the field is `DIV117`"]
    #[inline(always)]
    pub fn is_div117(&self) -> bool {
        **self == PLLN_A::DIV117
    }
    #[doc = "Checks if the value of the field is `DIV118`"]
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        **self == PLLN_A::DIV118
    }
    #[doc = "Checks if the value of the field is `DIV119`"]
    #[inline(always)]
    pub fn is_div119(&self) -> bool {
        **self == PLLN_A::DIV119
    }
    #[doc = "Checks if the value of the field is `DIV120`"]
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        **self == PLLN_A::DIV120
    }
    #[doc = "Checks if the value of the field is `DIV121`"]
    #[inline(always)]
    pub fn is_div121(&self) -> bool {
        **self == PLLN_A::DIV121
    }
    #[doc = "Checks if the value of the field is `DIV122`"]
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        **self == PLLN_A::DIV122
    }
    #[doc = "Checks if the value of the field is `DIV123`"]
    #[inline(always)]
    pub fn is_div123(&self) -> bool {
        **self == PLLN_A::DIV123
    }
    #[doc = "Checks if the value of the field is `DIV124`"]
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        **self == PLLN_A::DIV124
    }
    #[doc = "Checks if the value of the field is `DIV125`"]
    #[inline(always)]
    pub fn is_div125(&self) -> bool {
        **self == PLLN_A::DIV125
    }
    #[doc = "Checks if the value of the field is `DIV126`"]
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        **self == PLLN_A::DIV126
    }
    #[doc = "Checks if the value of the field is `DIV127`"]
    #[inline(always)]
    pub fn is_div127(&self) -> bool {
        **self == PLLN_A::DIV127
    }
}
impl core::ops::Deref for PLLN_R {
    type Target = crate::FieldReader<u8, PLLN_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll_n_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLN_A::DIV8)
    }
    #[doc = "pll_n_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLN_A::DIV9)
    }
    #[doc = "pll_n_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLN_A::DIV10)
    }
    #[doc = "pll_n_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLN_A::DIV11)
    }
    #[doc = "pll_n_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLN_A::DIV12)
    }
    #[doc = "pll_n_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLN_A::DIV13)
    }
    #[doc = "pll_n_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLN_A::DIV14)
    }
    #[doc = "pll_n_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLN_A::DIV15)
    }
    #[doc = "pll_n_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLN_A::DIV16)
    }
    #[doc = "pll_n_ck = vco_ck / 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLN_A::DIV17)
    }
    #[doc = "pll_n_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLN_A::DIV18)
    }
    #[doc = "pll_n_ck = vco_ck / 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLN_A::DIV19)
    }
    #[doc = "pll_n_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLN_A::DIV20)
    }
    #[doc = "pll_n_ck = vco_ck / 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLN_A::DIV21)
    }
    #[doc = "pll_n_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLN_A::DIV22)
    }
    #[doc = "pll_n_ck = vco_ck / 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLN_A::DIV23)
    }
    #[doc = "pll_n_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLN_A::DIV24)
    }
    #[doc = "pll_n_ck = vco_ck / 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLN_A::DIV25)
    }
    #[doc = "pll_n_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLN_A::DIV26)
    }
    #[doc = "pll_n_ck = vco_ck / 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLN_A::DIV27)
    }
    #[doc = "pll_n_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLN_A::DIV28)
    }
    #[doc = "pll_n_ck = vco_ck / 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLN_A::DIV29)
    }
    #[doc = "pll_n_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLN_A::DIV30)
    }
    #[doc = "pll_n_ck = vco_ck / 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLN_A::DIV31)
    }
    #[doc = "pll_n_ck = vco_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLN_A::DIV32)
    }
    #[doc = "pll_n_ck = vco_ck / 33"]
    #[inline(always)]
    pub fn div33(self) -> &'a mut W {
        self.variant(PLLN_A::DIV33)
    }
    #[doc = "pll_n_ck = vco_ck / 34"]
    #[inline(always)]
    pub fn div34(self) -> &'a mut W {
        self.variant(PLLN_A::DIV34)
    }
    #[doc = "pll_n_ck = vco_ck / 35"]
    #[inline(always)]
    pub fn div35(self) -> &'a mut W {
        self.variant(PLLN_A::DIV35)
    }
    #[doc = "pll_n_ck = vco_ck / 36"]
    #[inline(always)]
    pub fn div36(self) -> &'a mut W {
        self.variant(PLLN_A::DIV36)
    }
    #[doc = "pll_n_ck = vco_ck / 37"]
    #[inline(always)]
    pub fn div37(self) -> &'a mut W {
        self.variant(PLLN_A::DIV37)
    }
    #[doc = "pll_n_ck = vco_ck / 38"]
    #[inline(always)]
    pub fn div38(self) -> &'a mut W {
        self.variant(PLLN_A::DIV38)
    }
    #[doc = "pll_n_ck = vco_ck / 39"]
    #[inline(always)]
    pub fn div39(self) -> &'a mut W {
        self.variant(PLLN_A::DIV39)
    }
    #[doc = "pll_n_ck = vco_ck / 40"]
    #[inline(always)]
    pub fn div40(self) -> &'a mut W {
        self.variant(PLLN_A::DIV40)
    }
    #[doc = "pll_n_ck = vco_ck / 41"]
    #[inline(always)]
    pub fn div41(self) -> &'a mut W {
        self.variant(PLLN_A::DIV41)
    }
    #[doc = "pll_n_ck = vco_ck / 42"]
    #[inline(always)]
    pub fn div42(self) -> &'a mut W {
        self.variant(PLLN_A::DIV42)
    }
    #[doc = "pll_n_ck = vco_ck / 43"]
    #[inline(always)]
    pub fn div43(self) -> &'a mut W {
        self.variant(PLLN_A::DIV43)
    }
    #[doc = "pll_n_ck = vco_ck / 44"]
    #[inline(always)]
    pub fn div44(self) -> &'a mut W {
        self.variant(PLLN_A::DIV44)
    }
    #[doc = "pll_n_ck = vco_ck / 45"]
    #[inline(always)]
    pub fn div45(self) -> &'a mut W {
        self.variant(PLLN_A::DIV45)
    }
    #[doc = "pll_n_ck = vco_ck / 46"]
    #[inline(always)]
    pub fn div46(self) -> &'a mut W {
        self.variant(PLLN_A::DIV46)
    }
    #[doc = "pll_n_ck = vco_ck / 47"]
    #[inline(always)]
    pub fn div47(self) -> &'a mut W {
        self.variant(PLLN_A::DIV47)
    }
    #[doc = "pll_n_ck = vco_ck / 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(PLLN_A::DIV48)
    }
    #[doc = "pll_n_ck = vco_ck / 49"]
    #[inline(always)]
    pub fn div49(self) -> &'a mut W {
        self.variant(PLLN_A::DIV49)
    }
    #[doc = "pll_n_ck = vco_ck / 50"]
    #[inline(always)]
    pub fn div50(self) -> &'a mut W {
        self.variant(PLLN_A::DIV50)
    }
    #[doc = "pll_n_ck = vco_ck / 51"]
    #[inline(always)]
    pub fn div51(self) -> &'a mut W {
        self.variant(PLLN_A::DIV51)
    }
    #[doc = "pll_n_ck = vco_ck / 52"]
    #[inline(always)]
    pub fn div52(self) -> &'a mut W {
        self.variant(PLLN_A::DIV52)
    }
    #[doc = "pll_n_ck = vco_ck / 53"]
    #[inline(always)]
    pub fn div53(self) -> &'a mut W {
        self.variant(PLLN_A::DIV53)
    }
    #[doc = "pll_n_ck = vco_ck / 54"]
    #[inline(always)]
    pub fn div54(self) -> &'a mut W {
        self.variant(PLLN_A::DIV54)
    }
    #[doc = "pll_n_ck = vco_ck / 55"]
    #[inline(always)]
    pub fn div55(self) -> &'a mut W {
        self.variant(PLLN_A::DIV55)
    }
    #[doc = "pll_n_ck = vco_ck / 56"]
    #[inline(always)]
    pub fn div56(self) -> &'a mut W {
        self.variant(PLLN_A::DIV56)
    }
    #[doc = "pll_n_ck = vco_ck / 57"]
    #[inline(always)]
    pub fn div57(self) -> &'a mut W {
        self.variant(PLLN_A::DIV57)
    }
    #[doc = "pll_n_ck = vco_ck / 58"]
    #[inline(always)]
    pub fn div58(self) -> &'a mut W {
        self.variant(PLLN_A::DIV58)
    }
    #[doc = "pll_n_ck = vco_ck / 59"]
    #[inline(always)]
    pub fn div59(self) -> &'a mut W {
        self.variant(PLLN_A::DIV59)
    }
    #[doc = "pll_n_ck = vco_ck / 60"]
    #[inline(always)]
    pub fn div60(self) -> &'a mut W {
        self.variant(PLLN_A::DIV60)
    }
    #[doc = "pll_n_ck = vco_ck / 61"]
    #[inline(always)]
    pub fn div61(self) -> &'a mut W {
        self.variant(PLLN_A::DIV61)
    }
    #[doc = "pll_n_ck = vco_ck / 62"]
    #[inline(always)]
    pub fn div62(self) -> &'a mut W {
        self.variant(PLLN_A::DIV62)
    }
    #[doc = "pll_n_ck = vco_ck / 63"]
    #[inline(always)]
    pub fn div63(self) -> &'a mut W {
        self.variant(PLLN_A::DIV63)
    }
    #[doc = "pll_n_ck = vco_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PLLN_A::DIV64)
    }
    #[doc = "pll_n_ck = vco_ck / 65"]
    #[inline(always)]
    pub fn div65(self) -> &'a mut W {
        self.variant(PLLN_A::DIV65)
    }
    #[doc = "pll_n_ck = vco_ck / 66"]
    #[inline(always)]
    pub fn div66(self) -> &'a mut W {
        self.variant(PLLN_A::DIV66)
    }
    #[doc = "pll_n_ck = vco_ck / 67"]
    #[inline(always)]
    pub fn div67(self) -> &'a mut W {
        self.variant(PLLN_A::DIV67)
    }
    #[doc = "pll_n_ck = vco_ck / 68"]
    #[inline(always)]
    pub fn div68(self) -> &'a mut W {
        self.variant(PLLN_A::DIV68)
    }
    #[doc = "pll_n_ck = vco_ck / 69"]
    #[inline(always)]
    pub fn div69(self) -> &'a mut W {
        self.variant(PLLN_A::DIV69)
    }
    #[doc = "pll_n_ck = vco_ck / 70"]
    #[inline(always)]
    pub fn div70(self) -> &'a mut W {
        self.variant(PLLN_A::DIV70)
    }
    #[doc = "pll_n_ck = vco_ck / 71"]
    #[inline(always)]
    pub fn div71(self) -> &'a mut W {
        self.variant(PLLN_A::DIV71)
    }
    #[doc = "pll_n_ck = vco_ck / 72"]
    #[inline(always)]
    pub fn div72(self) -> &'a mut W {
        self.variant(PLLN_A::DIV72)
    }
    #[doc = "pll_n_ck = vco_ck / 73"]
    #[inline(always)]
    pub fn div73(self) -> &'a mut W {
        self.variant(PLLN_A::DIV73)
    }
    #[doc = "pll_n_ck = vco_ck / 74"]
    #[inline(always)]
    pub fn div74(self) -> &'a mut W {
        self.variant(PLLN_A::DIV74)
    }
    #[doc = "pll_n_ck = vco_ck / 75"]
    #[inline(always)]
    pub fn div75(self) -> &'a mut W {
        self.variant(PLLN_A::DIV75)
    }
    #[doc = "pll_n_ck = vco_ck / 76"]
    #[inline(always)]
    pub fn div76(self) -> &'a mut W {
        self.variant(PLLN_A::DIV76)
    }
    #[doc = "pll_n_ck = vco_ck / 77"]
    #[inline(always)]
    pub fn div77(self) -> &'a mut W {
        self.variant(PLLN_A::DIV77)
    }
    #[doc = "pll_n_ck = vco_ck / 78"]
    #[inline(always)]
    pub fn div78(self) -> &'a mut W {
        self.variant(PLLN_A::DIV78)
    }
    #[doc = "pll_n_ck = vco_ck / 79"]
    #[inline(always)]
    pub fn div79(self) -> &'a mut W {
        self.variant(PLLN_A::DIV79)
    }
    #[doc = "pll_n_ck = vco_ck / 80"]
    #[inline(always)]
    pub fn div80(self) -> &'a mut W {
        self.variant(PLLN_A::DIV80)
    }
    #[doc = "pll_n_ck = vco_ck / 81"]
    #[inline(always)]
    pub fn div81(self) -> &'a mut W {
        self.variant(PLLN_A::DIV81)
    }
    #[doc = "pll_n_ck = vco_ck / 82"]
    #[inline(always)]
    pub fn div82(self) -> &'a mut W {
        self.variant(PLLN_A::DIV82)
    }
    #[doc = "pll_n_ck = vco_ck / 83"]
    #[inline(always)]
    pub fn div83(self) -> &'a mut W {
        self.variant(PLLN_A::DIV83)
    }
    #[doc = "pll_n_ck = vco_ck / 84"]
    #[inline(always)]
    pub fn div84(self) -> &'a mut W {
        self.variant(PLLN_A::DIV84)
    }
    #[doc = "pll_n_ck = vco_ck / 85"]
    #[inline(always)]
    pub fn div85(self) -> &'a mut W {
        self.variant(PLLN_A::DIV85)
    }
    #[doc = "pll_n_ck = vco_ck / 86"]
    #[inline(always)]
    pub fn div86(self) -> &'a mut W {
        self.variant(PLLN_A::DIV86)
    }
    #[doc = "pll_n_ck = vco_ck / 87"]
    #[inline(always)]
    pub fn div87(self) -> &'a mut W {
        self.variant(PLLN_A::DIV87)
    }
    #[doc = "pll_n_ck = vco_ck / 88"]
    #[inline(always)]
    pub fn div88(self) -> &'a mut W {
        self.variant(PLLN_A::DIV88)
    }
    #[doc = "pll_n_ck = vco_ck / 89"]
    #[inline(always)]
    pub fn div89(self) -> &'a mut W {
        self.variant(PLLN_A::DIV89)
    }
    #[doc = "pll_n_ck = vco_ck / 90"]
    #[inline(always)]
    pub fn div90(self) -> &'a mut W {
        self.variant(PLLN_A::DIV90)
    }
    #[doc = "pll_n_ck = vco_ck / 91"]
    #[inline(always)]
    pub fn div91(self) -> &'a mut W {
        self.variant(PLLN_A::DIV91)
    }
    #[doc = "pll_n_ck = vco_ck / 92"]
    #[inline(always)]
    pub fn div92(self) -> &'a mut W {
        self.variant(PLLN_A::DIV92)
    }
    #[doc = "pll_n_ck = vco_ck / 93"]
    #[inline(always)]
    pub fn div93(self) -> &'a mut W {
        self.variant(PLLN_A::DIV93)
    }
    #[doc = "pll_n_ck = vco_ck / 94"]
    #[inline(always)]
    pub fn div94(self) -> &'a mut W {
        self.variant(PLLN_A::DIV94)
    }
    #[doc = "pll_n_ck = vco_ck / 95"]
    #[inline(always)]
    pub fn div95(self) -> &'a mut W {
        self.variant(PLLN_A::DIV95)
    }
    #[doc = "pll_n_ck = vco_ck / 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(PLLN_A::DIV96)
    }
    #[doc = "pll_n_ck = vco_ck / 97"]
    #[inline(always)]
    pub fn div97(self) -> &'a mut W {
        self.variant(PLLN_A::DIV97)
    }
    #[doc = "pll_n_ck = vco_ck / 98"]
    #[inline(always)]
    pub fn div98(self) -> &'a mut W {
        self.variant(PLLN_A::DIV98)
    }
    #[doc = "pll_n_ck = vco_ck / 99"]
    #[inline(always)]
    pub fn div99(self) -> &'a mut W {
        self.variant(PLLN_A::DIV99)
    }
    #[doc = "pll_n_ck = vco_ck / 100"]
    #[inline(always)]
    pub fn div100(self) -> &'a mut W {
        self.variant(PLLN_A::DIV100)
    }
    #[doc = "pll_n_ck = vco_ck / 101"]
    #[inline(always)]
    pub fn div101(self) -> &'a mut W {
        self.variant(PLLN_A::DIV101)
    }
    #[doc = "pll_n_ck = vco_ck / 102"]
    #[inline(always)]
    pub fn div102(self) -> &'a mut W {
        self.variant(PLLN_A::DIV102)
    }
    #[doc = "pll_n_ck = vco_ck / 103"]
    #[inline(always)]
    pub fn div103(self) -> &'a mut W {
        self.variant(PLLN_A::DIV103)
    }
    #[doc = "pll_n_ck = vco_ck / 104"]
    #[inline(always)]
    pub fn div104(self) -> &'a mut W {
        self.variant(PLLN_A::DIV104)
    }
    #[doc = "pll_n_ck = vco_ck / 105"]
    #[inline(always)]
    pub fn div105(self) -> &'a mut W {
        self.variant(PLLN_A::DIV105)
    }
    #[doc = "pll_n_ck = vco_ck / 106"]
    #[inline(always)]
    pub fn div106(self) -> &'a mut W {
        self.variant(PLLN_A::DIV106)
    }
    #[doc = "pll_n_ck = vco_ck / 107"]
    #[inline(always)]
    pub fn div107(self) -> &'a mut W {
        self.variant(PLLN_A::DIV107)
    }
    #[doc = "pll_n_ck = vco_ck / 108"]
    #[inline(always)]
    pub fn div108(self) -> &'a mut W {
        self.variant(PLLN_A::DIV108)
    }
    #[doc = "pll_n_ck = vco_ck / 109"]
    #[inline(always)]
    pub fn div109(self) -> &'a mut W {
        self.variant(PLLN_A::DIV109)
    }
    #[doc = "pll_n_ck = vco_ck / 110"]
    #[inline(always)]
    pub fn div110(self) -> &'a mut W {
        self.variant(PLLN_A::DIV110)
    }
    #[doc = "pll_n_ck = vco_ck / 111"]
    #[inline(always)]
    pub fn div111(self) -> &'a mut W {
        self.variant(PLLN_A::DIV111)
    }
    #[doc = "pll_n_ck = vco_ck / 112"]
    #[inline(always)]
    pub fn div112(self) -> &'a mut W {
        self.variant(PLLN_A::DIV112)
    }
    #[doc = "pll_n_ck = vco_ck / 113"]
    #[inline(always)]
    pub fn div113(self) -> &'a mut W {
        self.variant(PLLN_A::DIV113)
    }
    #[doc = "pll_n_ck = vco_ck / 114"]
    #[inline(always)]
    pub fn div114(self) -> &'a mut W {
        self.variant(PLLN_A::DIV114)
    }
    #[doc = "pll_n_ck = vco_ck / 115"]
    #[inline(always)]
    pub fn div115(self) -> &'a mut W {
        self.variant(PLLN_A::DIV115)
    }
    #[doc = "pll_n_ck = vco_ck / 116"]
    #[inline(always)]
    pub fn div116(self) -> &'a mut W {
        self.variant(PLLN_A::DIV116)
    }
    #[doc = "pll_n_ck = vco_ck / 117"]
    #[inline(always)]
    pub fn div117(self) -> &'a mut W {
        self.variant(PLLN_A::DIV117)
    }
    #[doc = "pll_n_ck = vco_ck / 118"]
    #[inline(always)]
    pub fn div118(self) -> &'a mut W {
        self.variant(PLLN_A::DIV118)
    }
    #[doc = "pll_n_ck = vco_ck / 119"]
    #[inline(always)]
    pub fn div119(self) -> &'a mut W {
        self.variant(PLLN_A::DIV119)
    }
    #[doc = "pll_n_ck = vco_ck / 120"]
    #[inline(always)]
    pub fn div120(self) -> &'a mut W {
        self.variant(PLLN_A::DIV120)
    }
    #[doc = "pll_n_ck = vco_ck / 121"]
    #[inline(always)]
    pub fn div121(self) -> &'a mut W {
        self.variant(PLLN_A::DIV121)
    }
    #[doc = "pll_n_ck = vco_ck / 122"]
    #[inline(always)]
    pub fn div122(self) -> &'a mut W {
        self.variant(PLLN_A::DIV122)
    }
    #[doc = "pll_n_ck = vco_ck / 123"]
    #[inline(always)]
    pub fn div123(self) -> &'a mut W {
        self.variant(PLLN_A::DIV123)
    }
    #[doc = "pll_n_ck = vco_ck / 124"]
    #[inline(always)]
    pub fn div124(self) -> &'a mut W {
        self.variant(PLLN_A::DIV124)
    }
    #[doc = "pll_n_ck = vco_ck / 125"]
    #[inline(always)]
    pub fn div125(self) -> &'a mut W {
        self.variant(PLLN_A::DIV125)
    }
    #[doc = "pll_n_ck = vco_ck / 126"]
    #[inline(always)]
    pub fn div126(self) -> &'a mut W {
        self.variant(PLLN_A::DIV126)
    }
    #[doc = "pll_n_ck = vco_ck / 127"]
    #[inline(always)]
    pub fn div127(self) -> &'a mut W {
        self.variant(PLLN_A::DIV127)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLM_A {
    #[doc = "0: pll_p_ck = vco_ck / 1"]
    DIV1 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 2"]
    DIV2 = 1,
    #[doc = "2: pll_p_ck = vco_ck / 3"]
    DIV3 = 2,
    #[doc = "3: pll_p_ck = vco_ck / 4"]
    DIV4 = 3,
    #[doc = "4: pll_p_ck = vco_ck / 5"]
    DIV5 = 4,
    #[doc = "5: pll_p_ck = vco_ck / 6"]
    DIV6 = 5,
    #[doc = "6: pll_p_ck = vco_ck / 7"]
    DIV7 = 6,
    #[doc = "7: pll_p_ck = vco_ck / 8"]
    DIV8 = 7,
    #[doc = "8: pll_p_ck = vco_ck / 9"]
    DIV9 = 8,
    #[doc = "9: pll_p_ck = vco_ck / 10"]
    DIV10 = 9,
    #[doc = "10: pll_p_ck = vco_ck / 11"]
    DIV11 = 10,
    #[doc = "11: pll_p_ck = vco_ck / 12"]
    DIV12 = 11,
    #[doc = "12: pll_p_ck = vco_ck / 13"]
    DIV13 = 12,
    #[doc = "13: pll_p_ck = vco_ck / 14"]
    DIV14 = 13,
    #[doc = "14: pll_p_ck = vco_ck / 15"]
    DIV15 = 14,
    #[doc = "15: pll_p_ck = vco_ck / 16"]
    DIV16 = 15,
}
impl From<PLLM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
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
            8 => PLLM_A::DIV9,
            9 => PLLM_A::DIV10,
            10 => PLLM_A::DIV11,
            11 => PLLM_A::DIV12,
            12 => PLLM_A::DIV13,
            13 => PLLM_A::DIV14,
            14 => PLLM_A::DIV15,
            15 => PLLM_A::DIV16,
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
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        **self == PLLM_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PLLM_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        **self == PLLM_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PLLM_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        **self == PLLM_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        **self == PLLM_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        **self == PLLM_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PLLM_A::DIV16
    }
}
impl core::ops::Deref for PLLM_R {
    type Target = crate::FieldReader<u8, PLLM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "pll_p_ck = vco_ck / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLM_A::DIV1)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLM_A::DIV2)
    }
    #[doc = "pll_p_ck = vco_ck / 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLM_A::DIV3)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLM_A::DIV4)
    }
    #[doc = "pll_p_ck = vco_ck / 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLM_A::DIV5)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLM_A::DIV6)
    }
    #[doc = "pll_p_ck = vco_ck / 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLM_A::DIV7)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLM_A::DIV8)
    }
    #[doc = "pll_p_ck = vco_ck / 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLM_A::DIV9)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLM_A::DIV10)
    }
    #[doc = "pll_p_ck = vco_ck / 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLM_A::DIV11)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLM_A::DIV12)
    }
    #[doc = "pll_p_ck = vco_ck / 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLM_A::DIV13)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLM_A::DIV14)
    }
    #[doc = "pll_p_ck = vco_ck / 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLM_A::DIV15)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLM_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Main PLL, PLLSAI1 and PLLSAI2 entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: No clock sent to PLL"]
    NONE = 0,
    #[doc = "2: No clock sent to PLL"]
    HSI16 = 2,
    #[doc = "3: No clock sent to PLL"]
    HSE = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub struct PLLSRC_R(crate::FieldReader<u8, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSRC_A> {
        match self.bits {
            0 => Some(PLLSRC_A::NONE),
            2 => Some(PLLSRC_A::HSI16),
            3 => Some(PLLSRC_A::HSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == PLLSRC_A::NONE
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == PLLSRC_A::HSI16
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PLLSRC_A::HSE
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLSRC_A::NONE)
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI16)
    }
    #[doc = "No clock sent to PLL"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W {
        PLLPDIV_W { w: self }
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W {
        PLLREN_W { w: self }
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W {
        PLLQEN_W { w: self }
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W {
        PLLPEN_W { w: self }
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
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
#[doc = "`reset()` method sets PLLCFGR to value 0x1000"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
