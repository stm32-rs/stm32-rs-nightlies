#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PREDIV division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREDIV_A {
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
impl From<PREDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREDIV`"]
pub type PREDIV_R = crate::R<u8, PREDIV_A>;
impl PREDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDIV_A {
        match self.bits {
            0 => PREDIV_A::DIV1,
            1 => PREDIV_A::DIV2,
            2 => PREDIV_A::DIV3,
            3 => PREDIV_A::DIV4,
            4 => PREDIV_A::DIV5,
            5 => PREDIV_A::DIV6,
            6 => PREDIV_A::DIV7,
            7 => PREDIV_A::DIV8,
            8 => PREDIV_A::DIV9,
            9 => PREDIV_A::DIV10,
            10 => PREDIV_A::DIV11,
            11 => PREDIV_A::DIV12,
            12 => PREDIV_A::DIV13,
            13 => PREDIV_A::DIV14,
            14 => PREDIV_A::DIV15,
            15 => PREDIV_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV_A::DIV16
    }
}
#[doc = "Write proxy for field `PREDIV`"]
pub struct PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "ADC1 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC1PRES_A {
    #[doc = "0: No clock"]
    NOCLOCK = 0,
    #[doc = "16: PLL clock not divided"]
    DIV1 = 16,
    #[doc = "17: PLL clock divided by 2"]
    DIV2 = 17,
    #[doc = "18: PLL clock divided by 4"]
    DIV4 = 18,
    #[doc = "19: PLL clock divided by 6"]
    DIV6 = 19,
    #[doc = "20: PLL clock divided by 8"]
    DIV8 = 20,
    #[doc = "21: PLL clock divided by 10"]
    DIV10 = 21,
    #[doc = "22: PLL clock divided by 12"]
    DIV12 = 22,
    #[doc = "23: PLL clock divided by 16"]
    DIV16 = 23,
    #[doc = "24: PLL clock divided by 32"]
    DIV32 = 24,
    #[doc = "25: PLL clock divided by 64"]
    DIV64 = 25,
    #[doc = "26: PLL clock divided by 128"]
    DIV128 = 26,
    #[doc = "27: PLL clock divided by 256"]
    DIV256 = 27,
}
impl From<ADC1PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC1PRES`"]
pub type ADC1PRES_R = crate::R<u8, ADC1PRES_A>;
impl ADC1PRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC1PRES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC1PRES_A::NOCLOCK),
            16 => Val(ADC1PRES_A::DIV1),
            17 => Val(ADC1PRES_A::DIV2),
            18 => Val(ADC1PRES_A::DIV4),
            19 => Val(ADC1PRES_A::DIV6),
            20 => Val(ADC1PRES_A::DIV8),
            21 => Val(ADC1PRES_A::DIV10),
            22 => Val(ADC1PRES_A::DIV12),
            23 => Val(ADC1PRES_A::DIV16),
            24 => Val(ADC1PRES_A::DIV32),
            25 => Val(ADC1PRES_A::DIV64),
            26 => Val(ADC1PRES_A::DIV128),
            27 => Val(ADC1PRES_A::DIV256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADC1PRES_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ADC1PRES_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADC1PRES_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADC1PRES_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADC1PRES_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADC1PRES_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == ADC1PRES_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == ADC1PRES_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ADC1PRES_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == ADC1PRES_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == ADC1PRES_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == ADC1PRES_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == ADC1PRES_A::DIV256
    }
}
#[doc = "Write proxy for field `ADC1PRES`"]
pub struct ADC1PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1PRES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(ADC1PRES_A::NOCLOCK)
    }
    #[doc = "PLL clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV1)
    }
    #[doc = "PLL clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV2)
    }
    #[doc = "PLL clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV4)
    }
    #[doc = "PLL clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV6)
    }
    #[doc = "PLL clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV8)
    }
    #[doc = "PLL clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV10)
    }
    #[doc = "PLL clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV12)
    }
    #[doc = "PLL clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV16)
    }
    #[doc = "PLL clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV32)
    }
    #[doc = "PLL clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV64)
    }
    #[doc = "PLL clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV128)
    }
    #[doc = "PLL clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(ADC1PRES_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADC1 prescaler"]
    #[inline(always)]
    pub fn adc1pres(&self) -> ADC1PRES_R {
        ADC1PRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W {
        PREDIV_W { w: self }
    }
    #[doc = "Bits 4:7 - ADC1 prescaler"]
    #[inline(always)]
    pub fn adc1pres(&mut self) -> ADC1PRES_W {
        ADC1PRES_W { w: self }
    }
}
