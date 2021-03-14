#[doc = "Reader of register EXTICR3"]
pub type R = crate::R<u32, super::EXTICR3>;
#[doc = "Writer for register EXTICR3"]
pub type W = crate::W<u32, super::EXTICR3>;
#[doc = "Register EXTICR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO port selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI0_7_A {
    #[doc = "0: GPIO port A selected"]
    PA = 0,
    #[doc = "1: GPIO port B selected"]
    PB = 1,
    #[doc = "2: GPIO port C selected"]
    PC = 2,
    #[doc = "3: GPIO port D selected"]
    PD = 3,
    #[doc = "5: GPIO port F selected"]
    PF = 5,
}
impl From<EXTI0_7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI0_7`"]
pub type EXTI0_7_R = crate::R<u8, EXTI0_7_A>;
impl EXTI0_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI0_7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI0_7_A::PA),
            1 => Val(EXTI0_7_A::PB),
            2 => Val(EXTI0_7_A::PC),
            3 => Val(EXTI0_7_A::PD),
            5 => Val(EXTI0_7_A::PF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA`"]
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI0_7_A::PA
    }
    #[doc = "Checks if the value of the field is `PB`"]
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI0_7_A::PB
    }
    #[doc = "Checks if the value of the field is `PC`"]
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI0_7_A::PC
    }
    #[doc = "Checks if the value of the field is `PD`"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI0_7_A::PD
    }
    #[doc = "Checks if the value of the field is `PF`"]
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI0_7_A::PF
    }
}
#[doc = "Write proxy for field `EXTI0_7`"]
pub struct EXTI0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI0_7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI8_15_A = EXTI0_7_A;
#[doc = "Reader of field `EXTI8_15`"]
pub type EXTI8_15_R = crate::R<u8, EXTI0_7_A>;
#[doc = "Write proxy for field `EXTI8_15`"]
pub struct EXTI8_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI8_15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI16_23_A = EXTI0_7_A;
#[doc = "Reader of field `EXTI16_23`"]
pub type EXTI16_23_R = crate::R<u8, EXTI0_7_A>;
#[doc = "Write proxy for field `EXTI16_23`"]
pub struct EXTI16_23_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI16_23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI16_23_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "GPIO port selection"]
pub type EXTI24_31_A = EXTI0_7_A;
#[doc = "Reader of field `EXTI24_31`"]
pub type EXTI24_31_R = crate::R<u8, EXTI0_7_A>;
#[doc = "Write proxy for field `EXTI24_31`"]
pub struct EXTI24_31_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI24_31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI24_31_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PA)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PB)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PC)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PD)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut W {
        self.variant(EXTI0_7_A::PF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&mut self) -> EXTI0_7_W {
        EXTI0_7_W { w: self }
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&mut self) -> EXTI8_15_W {
        EXTI8_15_W { w: self }
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&mut self) -> EXTI16_23_W {
        EXTI16_23_W { w: self }
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&mut self) -> EXTI24_31_W {
        EXTI24_31_W { w: self }
    }
}
