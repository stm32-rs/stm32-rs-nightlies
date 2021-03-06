#[doc = "Reader of register EXTICR1"]
pub type R = crate::R<u32, super::EXTICR1>;
#[doc = "Writer for register EXTICR1"]
pub type W = crate::W<u32, super::EXTICR1>;
#[doc = "Register EXTICR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EXTI 3 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI3_A {
    #[doc = "0: Select PA3 as the source input for the EXTI3 external interrupt"]
    PA3 = 0,
    #[doc = "1: Select PB3 as the source input for the EXTI3 external interrupt"]
    PB3 = 1,
    #[doc = "2: Select PC3 as the source input for the EXTI3 external interrupt"]
    PC3 = 2,
    #[doc = "3: Select PD3 as the source input for the EXTI3 external interrupt"]
    PD3 = 3,
    #[doc = "5: Select PF3 as the source input for the EXTI3 external interrupt"]
    PF3 = 5,
}
impl From<EXTI3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI3`"]
pub type EXTI3_R = crate::R<u8, EXTI3_A>;
impl EXTI3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI3_A::PA3),
            1 => Val(EXTI3_A::PB3),
            2 => Val(EXTI3_A::PC3),
            3 => Val(EXTI3_A::PD3),
            5 => Val(EXTI3_A::PF3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3_A::PA3
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3_A::PB3
    }
    #[doc = "Checks if the value of the field is `PC3`"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3_A::PC3
    }
    #[doc = "Checks if the value of the field is `PD3`"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == EXTI3_A::PD3
    }
    #[doc = "Checks if the value of the field is `PF3`"]
    #[inline(always)]
    pub fn is_pf3(&self) -> bool {
        *self == EXTI3_A::PF3
    }
}
#[doc = "Write proxy for field `EXTI3`"]
pub struct EXTI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(EXTI3_A::PA3)
    }
    #[doc = "Select PB3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(EXTI3_A::PB3)
    }
    #[doc = "Select PC3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(EXTI3_A::PC3)
    }
    #[doc = "Select PD3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut W {
        self.variant(EXTI3_A::PD3)
    }
    #[doc = "Select PF3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pf3(self) -> &'a mut W {
        self.variant(EXTI3_A::PF3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 2 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI2_A {
    #[doc = "0: Select PA2 as the source input for the EXTI2 external interrupt"]
    PA2 = 0,
    #[doc = "1: Select PB2 as the source input for the EXTI2 external interrupt"]
    PB2 = 1,
    #[doc = "2: Select PC2 as the source input for the EXTI2 external interrupt"]
    PC2 = 2,
    #[doc = "3: Select PD2 as the source input for the EXTI2 external interrupt"]
    PD2 = 3,
    #[doc = "5: Select PF2 as the source input for the EXTI2 external interrupt"]
    PF2 = 5,
}
impl From<EXTI2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI2`"]
pub type EXTI2_R = crate::R<u8, EXTI2_A>;
impl EXTI2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI2_A::PA2),
            1 => Val(EXTI2_A::PB2),
            2 => Val(EXTI2_A::PC2),
            3 => Val(EXTI2_A::PD2),
            5 => Val(EXTI2_A::PF2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2_A::PA2
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2_A::PB2
    }
    #[doc = "Checks if the value of the field is `PC2`"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2_A::PC2
    }
    #[doc = "Checks if the value of the field is `PD2`"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2_A::PD2
    }
    #[doc = "Checks if the value of the field is `PF2`"]
    #[inline(always)]
    pub fn is_pf2(&self) -> bool {
        *self == EXTI2_A::PF2
    }
}
#[doc = "Write proxy for field `EXTI2`"]
pub struct EXTI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(EXTI2_A::PA2)
    }
    #[doc = "Select PB2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(EXTI2_A::PB2)
    }
    #[doc = "Select PC2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(EXTI2_A::PC2)
    }
    #[doc = "Select PD2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(EXTI2_A::PD2)
    }
    #[doc = "Select PF2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pf2(self) -> &'a mut W {
        self.variant(EXTI2_A::PF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 1 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI1_A {
    #[doc = "0: Select PA1 as the source input for the EXTI1 external interrupt"]
    PA1 = 0,
    #[doc = "1: Select PB1 as the source input for the EXTI1 external interrupt"]
    PB1 = 1,
    #[doc = "2: Select PC1 as the source input for the EXTI1 external interrupt"]
    PC1 = 2,
    #[doc = "3: Select PD1 as the source input for the EXTI1 external interrupt"]
    PD1 = 3,
    #[doc = "5: Select PF1 as the source input for the EXTI1 external interrupt"]
    PF1 = 5,
}
impl From<EXTI1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI1`"]
pub type EXTI1_R = crate::R<u8, EXTI1_A>;
impl EXTI1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI1_A::PA1),
            1 => Val(EXTI1_A::PB1),
            2 => Val(EXTI1_A::PC1),
            3 => Val(EXTI1_A::PD1),
            5 => Val(EXTI1_A::PF1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1_A::PA1
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1_A::PB1
    }
    #[doc = "Checks if the value of the field is `PC1`"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1_A::PC1
    }
    #[doc = "Checks if the value of the field is `PD1`"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == EXTI1_A::PD1
    }
    #[doc = "Checks if the value of the field is `PF1`"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1_A::PF1
    }
}
#[doc = "Write proxy for field `EXTI1`"]
pub struct EXTI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(EXTI1_A::PA1)
    }
    #[doc = "Select PB1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(EXTI1_A::PB1)
    }
    #[doc = "Select PC1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(EXTI1_A::PC1)
    }
    #[doc = "Select PD1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut W {
        self.variant(EXTI1_A::PD1)
    }
    #[doc = "Select PF1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut W {
        self.variant(EXTI1_A::PF1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 0 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI0_A {
    #[doc = "0: Select PA0 as the source input for the EXTI0 external interrupt"]
    PA0 = 0,
    #[doc = "1: Select PB0 as the source input for the EXTI0 external interrupt"]
    PB0 = 1,
    #[doc = "2: Select PC0 as the source input for the EXTI0 external interrupt"]
    PC0 = 2,
    #[doc = "3: Select PD0 as the source input for the EXTI0 external interrupt"]
    PD0 = 3,
    #[doc = "5: Select PF0 as the source input for the EXTI0 external interrupt"]
    PF0 = 5,
}
impl From<EXTI0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI0`"]
pub type EXTI0_R = crate::R<u8, EXTI0_A>;
impl EXTI0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI0_A::PA0),
            1 => Val(EXTI0_A::PB0),
            2 => Val(EXTI0_A::PC0),
            3 => Val(EXTI0_A::PD0),
            5 => Val(EXTI0_A::PF0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0_A::PA0
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0_A::PB0
    }
    #[doc = "Checks if the value of the field is `PC0`"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0_A::PC0
    }
    #[doc = "Checks if the value of the field is `PD0`"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == EXTI0_A::PD0
    }
    #[doc = "Checks if the value of the field is `PF0`"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0_A::PF0
    }
}
#[doc = "Write proxy for field `EXTI0`"]
pub struct EXTI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(EXTI0_A::PA0)
    }
    #[doc = "Select PB0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(EXTI0_A::PB0)
    }
    #[doc = "Select PC0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(EXTI0_A::PC0)
    }
    #[doc = "Select PD0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut W {
        self.variant(EXTI0_A::PD0)
    }
    #[doc = "Select PF0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut W {
        self.variant(EXTI0_A::PF0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W {
        EXTI3_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W {
        EXTI2_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W {
        EXTI1_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W {
        EXTI0_W { w: self }
    }
}
