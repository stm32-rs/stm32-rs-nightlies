#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Writer for register OR"]
pub type W = crate::W<u32, super::OR>;
#[doc = "Register OR `reset()`'s with value 0"]
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer2 ETR remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETR_RMP_A {
    #[doc = "7: TIM2 ETR input is connected to COMP1_OUT"]
    COMP1_OUT = 7,
    #[doc = "6: TIM2 ETR input is connected to COMP2_OUT"]
    COMP2_OUT = 6,
    #[doc = "5: TIM2 ETR input is connected to LSE"]
    LSE = 5,
    #[doc = "3: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    HSI = 3,
}
impl From<ETR_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ETR_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETR_RMP`"]
pub type ETR_RMP_R = crate::R<u8, ETR_RMP_A>;
impl ETR_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETR_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(ETR_RMP_A::COMP1_OUT),
            6 => Val(ETR_RMP_A::COMP2_OUT),
            5 => Val(ETR_RMP_A::LSE),
            3 => Val(ETR_RMP_A::HSI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP_A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ETR_RMP_A::HSI
    }
}
#[doc = "Write proxy for field `ETR_RMP`"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETR_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2 ETR input is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP1_OUT)
    }
    #[doc = "TIM2 ETR input is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2 ETR input is connected to LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(ETR_RMP_A::LSE)
    }
    #[doc = "TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(ETR_RMP_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Internal trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI4_RMP_A {
    #[doc = "1: TIM2 TI4 input connected to COMP2_OUT"]
    COMP2_OUT = 1,
    #[doc = "2: TIM2 TI4 input connected to COMP1_OUT"]
    COMP1_OUT = 2,
}
impl From<TI4_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TI4_RMP`"]
pub type TI4_RMP_R = crate::R<u8, TI4_RMP_A>;
impl TI4_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TI4_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(TI4_RMP_A::COMP2_OUT),
            2 => Val(TI4_RMP_A::COMP1_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI4_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI4_RMP_A::COMP1_OUT
    }
}
#[doc = "Write proxy for field `TI4_RMP`"]
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI4_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2 TI4 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2 TI4 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(TI4_RMP_A::COMP1_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
}
