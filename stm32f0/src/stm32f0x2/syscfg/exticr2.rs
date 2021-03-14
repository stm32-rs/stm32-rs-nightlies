#[doc = "Reader of register EXTICR2"]
pub type R = crate::R<u32, super::EXTICR2>;
#[doc = "Writer for register EXTICR2"]
pub type W = crate::W<u32, super::EXTICR2>;
#[doc = "Register EXTICR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EXTI 7 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI7_A {
    #[doc = "4: Select PE7 as the source input for the EXTI7 external interrupt"]
    PE7 = 4,
    #[doc = "0: Select PA7 as the source input for the EXTI7 external interrupt"]
    PA7 = 0,
    #[doc = "1: Select PB7 as the source input for the EXTI7 external interrupt"]
    PB7 = 1,
    #[doc = "2: Select PC7 as the source input for the EXTI7 external interrupt"]
    PC7 = 2,
    #[doc = "3: Select PD7 as the source input for the EXTI7 external interrupt"]
    PD7 = 3,
    #[doc = "5: Select PF7 as the source input for the EXTI7 external interrupt"]
    PF7 = 5,
}
impl From<EXTI7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI7`"]
pub type EXTI7_R = crate::R<u8, EXTI7_A>;
impl EXTI7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI7_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(EXTI7_A::PE7),
            0 => Val(EXTI7_A::PA7),
            1 => Val(EXTI7_A::PB7),
            2 => Val(EXTI7_A::PC7),
            3 => Val(EXTI7_A::PD7),
            5 => Val(EXTI7_A::PF7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PE7`"]
    #[inline(always)]
    pub fn is_pe7(&self) -> bool {
        *self == EXTI7_A::PE7
    }
    #[doc = "Checks if the value of the field is `PA7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7_A::PA7
    }
    #[doc = "Checks if the value of the field is `PB7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7_A::PB7
    }
    #[doc = "Checks if the value of the field is `PC7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7_A::PC7
    }
    #[doc = "Checks if the value of the field is `PD7`"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == EXTI7_A::PD7
    }
    #[doc = "Checks if the value of the field is `PF7`"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7_A::PF7
    }
}
#[doc = "Write proxy for field `EXTI7`"]
pub struct EXTI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PE7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pe7(self) -> &'a mut W {
        self.variant(EXTI7_A::PE7)
    }
    #[doc = "Select PA7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(EXTI7_A::PA7)
    }
    #[doc = "Select PB7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(EXTI7_A::PB7)
    }
    #[doc = "Select PC7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(EXTI7_A::PC7)
    }
    #[doc = "Select PD7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut W {
        self.variant(EXTI7_A::PD7)
    }
    #[doc = "Select PF7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut W {
        self.variant(EXTI7_A::PF7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 6 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI6_A {
    #[doc = "4: Select PE6 as the source input for the EXTI6 external interrupt"]
    PE6 = 4,
    #[doc = "0: Select PA6 as the source input for the EXTI6 external interrupt"]
    PA6 = 0,
    #[doc = "1: Select PB6 as the source input for the EXTI6 external interrupt"]
    PB6 = 1,
    #[doc = "2: Select PC6 as the source input for the EXTI6 external interrupt"]
    PC6 = 2,
    #[doc = "3: Select PD6 as the source input for the EXTI6 external interrupt"]
    PD6 = 3,
    #[doc = "5: Select PF6 as the source input for the EXTI6 external interrupt"]
    PF6 = 5,
}
impl From<EXTI6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI6`"]
pub type EXTI6_R = crate::R<u8, EXTI6_A>;
impl EXTI6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI6_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(EXTI6_A::PE6),
            0 => Val(EXTI6_A::PA6),
            1 => Val(EXTI6_A::PB6),
            2 => Val(EXTI6_A::PC6),
            3 => Val(EXTI6_A::PD6),
            5 => Val(EXTI6_A::PF6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PE6`"]
    #[inline(always)]
    pub fn is_pe6(&self) -> bool {
        *self == EXTI6_A::PE6
    }
    #[doc = "Checks if the value of the field is `PA6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6_A::PA6
    }
    #[doc = "Checks if the value of the field is `PB6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6_A::PB6
    }
    #[doc = "Checks if the value of the field is `PC6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6_A::PC6
    }
    #[doc = "Checks if the value of the field is `PD6`"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == EXTI6_A::PD6
    }
    #[doc = "Checks if the value of the field is `PF6`"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6_A::PF6
    }
}
#[doc = "Write proxy for field `EXTI6`"]
pub struct EXTI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PE6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pe6(self) -> &'a mut W {
        self.variant(EXTI6_A::PE6)
    }
    #[doc = "Select PA6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(EXTI6_A::PA6)
    }
    #[doc = "Select PB6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(EXTI6_A::PB6)
    }
    #[doc = "Select PC6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(EXTI6_A::PC6)
    }
    #[doc = "Select PD6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut W {
        self.variant(EXTI6_A::PD6)
    }
    #[doc = "Select PF6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut W {
        self.variant(EXTI6_A::PF6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 5 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI5_A {
    #[doc = "4: Select PE5 as the source input for the EXTI5 external interrupt"]
    PE5 = 4,
    #[doc = "0: Select PA5 as the source input for the EXTI5 external interrupt"]
    PA5 = 0,
    #[doc = "1: Select PB5 as the source input for the EXTI5 external interrupt"]
    PB5 = 1,
    #[doc = "2: Select PC5 as the source input for the EXTI5 external interrupt"]
    PC5 = 2,
    #[doc = "3: Select PD5 as the source input for the EXTI5 external interrupt"]
    PD5 = 3,
    #[doc = "5: Select PF5 as the source input for the EXTI5 external interrupt"]
    PF5 = 5,
}
impl From<EXTI5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI5`"]
pub type EXTI5_R = crate::R<u8, EXTI5_A>;
impl EXTI5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI5_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(EXTI5_A::PE5),
            0 => Val(EXTI5_A::PA5),
            1 => Val(EXTI5_A::PB5),
            2 => Val(EXTI5_A::PC5),
            3 => Val(EXTI5_A::PD5),
            5 => Val(EXTI5_A::PF5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PE5`"]
    #[inline(always)]
    pub fn is_pe5(&self) -> bool {
        *self == EXTI5_A::PE5
    }
    #[doc = "Checks if the value of the field is `PA5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5_A::PA5
    }
    #[doc = "Checks if the value of the field is `PB5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5_A::PB5
    }
    #[doc = "Checks if the value of the field is `PC5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5_A::PC5
    }
    #[doc = "Checks if the value of the field is `PD5`"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == EXTI5_A::PD5
    }
    #[doc = "Checks if the value of the field is `PF5`"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5_A::PF5
    }
}
#[doc = "Write proxy for field `EXTI5`"]
pub struct EXTI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PE5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pe5(self) -> &'a mut W {
        self.variant(EXTI5_A::PE5)
    }
    #[doc = "Select PA5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(EXTI5_A::PA5)
    }
    #[doc = "Select PB5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(EXTI5_A::PB5)
    }
    #[doc = "Select PC5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(EXTI5_A::PC5)
    }
    #[doc = "Select PD5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut W {
        self.variant(EXTI5_A::PD5)
    }
    #[doc = "Select PF5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut W {
        self.variant(EXTI5_A::PF5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 4 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI4_A {
    #[doc = "4: Select PE4 as the source input for the EXTI4 external interrupt"]
    PE4 = 4,
    #[doc = "0: Select PA4 as the source input for the EXTI4 external interrupt"]
    PA4 = 0,
    #[doc = "1: Select PB4 as the source input for the EXTI4 external interrupt"]
    PB4 = 1,
    #[doc = "2: Select PC4 as the source input for the EXTI4 external interrupt"]
    PC4 = 2,
    #[doc = "3: Select PD4 as the source input for the EXTI4 external interrupt"]
    PD4 = 3,
    #[doc = "5: Select PF4 as the source input for the EXTI4 external interrupt"]
    PF4 = 5,
}
impl From<EXTI4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI4`"]
pub type EXTI4_R = crate::R<u8, EXTI4_A>;
impl EXTI4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI4_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(EXTI4_A::PE4),
            0 => Val(EXTI4_A::PA4),
            1 => Val(EXTI4_A::PB4),
            2 => Val(EXTI4_A::PC4),
            3 => Val(EXTI4_A::PD4),
            5 => Val(EXTI4_A::PF4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PE4`"]
    #[inline(always)]
    pub fn is_pe4(&self) -> bool {
        *self == EXTI4_A::PE4
    }
    #[doc = "Checks if the value of the field is `PA4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4_A::PA4
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4_A::PB4
    }
    #[doc = "Checks if the value of the field is `PC4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4_A::PC4
    }
    #[doc = "Checks if the value of the field is `PD4`"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == EXTI4_A::PD4
    }
    #[doc = "Checks if the value of the field is `PF4`"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4_A::PF4
    }
}
#[doc = "Write proxy for field `EXTI4`"]
pub struct EXTI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PE4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pe4(self) -> &'a mut W {
        self.variant(EXTI4_A::PE4)
    }
    #[doc = "Select PA4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(EXTI4_A::PA4)
    }
    #[doc = "Select PB4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(EXTI4_A::PB4)
    }
    #[doc = "Select PC4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(EXTI4_A::PC4)
    }
    #[doc = "Select PD4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut W {
        self.variant(EXTI4_A::PD4)
    }
    #[doc = "Select PF4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut W {
        self.variant(EXTI4_A::PF4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W {
        EXTI7_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W {
        EXTI6_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W {
        EXTI5_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W {
        EXTI4_W { w: self }
    }
}
