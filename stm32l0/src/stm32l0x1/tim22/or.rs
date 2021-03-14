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
#[doc = "Timer22 ETR remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETR_RMP_A {
    #[doc = "0: TIM2x ETR input connected to GPIO"]
    GPIO = 0,
    #[doc = "1: TIM2x ETR input connected to COMP2_OUT"]
    COMP2_OUT = 1,
    #[doc = "2: TIM2x ETR input connected to COMP1_OUT"]
    COMP1_OUT = 2,
    #[doc = "3: TIM2x ETR input connected to LSE clock"]
    LSE = 3,
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
    pub fn variant(&self) -> ETR_RMP_A {
        match self.bits {
            0 => ETR_RMP_A::GPIO,
            1 => ETR_RMP_A::COMP2_OUT,
            2 => ETR_RMP_A::COMP1_OUT,
            3 => ETR_RMP_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP_A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP_A::LSE
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TIM2x ETR input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(ETR_RMP_A::GPIO)
    }
    #[doc = "TIM2x ETR input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2x ETR input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(ETR_RMP_A::COMP1_OUT)
    }
    #[doc = "TIM2x ETR input connected to LSE clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(ETR_RMP_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Timer22 TI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI1_RMP_A {
    #[doc = "0: TIM2x TI1 input connected to GPIO"]
    GPIO = 0,
    #[doc = "1: TIM2x TI1 input connected to COMP2_OUT"]
    COMP2_OUT = 1,
    #[doc = "2: TIM2x TI1 input connected to COMP1_OUT"]
    COMP1_OUT = 2,
}
impl From<TI1_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TI1_RMP`"]
pub type TI1_RMP_R = crate::R<u8, TI1_RMP_A>;
impl TI1_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TI1_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TI1_RMP_A::GPIO),
            1 => Val(TI1_RMP_A::COMP2_OUT),
            2 => Val(TI1_RMP_A::COMP1_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP_A::GPIO
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI1_RMP_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI1_RMP_A::COMP1_OUT
    }
}
#[doc = "Write proxy for field `TI1_RMP`"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2x TI1 input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TI1_RMP_A::GPIO)
    }
    #[doc = "TIM2x TI1 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(TI1_RMP_A::COMP2_OUT)
    }
    #[doc = "TIM2x TI1 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(TI1_RMP_A::COMP1_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer22 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Timer22 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer22 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer22 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
}
