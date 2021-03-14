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
#[doc = "EXTI 11 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: Select PA11 as the source input for the EXTI11 external interrupt"]
    PA11 = 0,
    #[doc = "1: Select PB11 as the source input for the EXTI11 external interrupt"]
    PB11 = 1,
    #[doc = "2: Select PC11 as the source input for the EXTI11 external interrupt"]
    PC11 = 2,
    #[doc = "3: Select PD11 as the source input for the EXTI11 external interrupt"]
    PD11 = 3,
    #[doc = "5: Select PF11 as the source input for the EXTI11 external interrupt"]
    PF11 = 5,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI11`"]
pub type EXTI11_R = crate::R<u8, EXTI11_A>;
impl EXTI11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI11_A::PA11),
            1 => Val(EXTI11_A::PB11),
            2 => Val(EXTI11_A::PC11),
            3 => Val(EXTI11_A::PD11),
            5 => Val(EXTI11_A::PF11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11_A::PA11
    }
    #[doc = "Checks if the value of the field is `PB11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11_A::PB11
    }
    #[doc = "Checks if the value of the field is `PC11`"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == EXTI11_A::PC11
    }
    #[doc = "Checks if the value of the field is `PD11`"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == EXTI11_A::PD11
    }
    #[doc = "Checks if the value of the field is `PF11`"]
    #[inline(always)]
    pub fn is_pf11(&self) -> bool {
        *self == EXTI11_A::PF11
    }
}
#[doc = "Write proxy for field `EXTI11`"]
pub struct EXTI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::PA11)
    }
    #[doc = "Select PB11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::PB11)
    }
    #[doc = "Select PC11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(EXTI11_A::PC11)
    }
    #[doc = "Select PD11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(EXTI11_A::PD11)
    }
    #[doc = "Select PF11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pf11(self) -> &'a mut W {
        self.variant(EXTI11_A::PF11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 10 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: Select PA10 as the source input for the EXTI10 external interrupt"]
    PA10 = 0,
    #[doc = "1: Select PB10 as the source input for the EXTI10 external interrupt"]
    PB10 = 1,
    #[doc = "2: Select PC10 as the source input for the EXTI10 external interrupt"]
    PC10 = 2,
    #[doc = "3: Select PD10 as the source input for the EXTI10 external interrupt"]
    PD10 = 3,
    #[doc = "5: Select PF10 as the source input for the EXTI10 external interrupt"]
    PF10 = 5,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI10`"]
pub type EXTI10_R = crate::R<u8, EXTI10_A>;
impl EXTI10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI10_A::PA10),
            1 => Val(EXTI10_A::PB10),
            2 => Val(EXTI10_A::PC10),
            3 => Val(EXTI10_A::PD10),
            5 => Val(EXTI10_A::PF10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10_A::PA10
    }
    #[doc = "Checks if the value of the field is `PB10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10_A::PB10
    }
    #[doc = "Checks if the value of the field is `PC10`"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == EXTI10_A::PC10
    }
    #[doc = "Checks if the value of the field is `PD10`"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == EXTI10_A::PD10
    }
    #[doc = "Checks if the value of the field is `PF10`"]
    #[inline(always)]
    pub fn is_pf10(&self) -> bool {
        *self == EXTI10_A::PF10
    }
}
#[doc = "Write proxy for field `EXTI10`"]
pub struct EXTI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::PA10)
    }
    #[doc = "Select PB10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::PB10)
    }
    #[doc = "Select PC10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(EXTI10_A::PC10)
    }
    #[doc = "Select PD10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(EXTI10_A::PD10)
    }
    #[doc = "Select PF10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pf10(self) -> &'a mut W {
        self.variant(EXTI10_A::PF10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 9 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: Select PA9 as the source input for the EXTI9 external interrupt"]
    PA9 = 0,
    #[doc = "1: Select PB9 as the source input for the EXTI9 external interrupt"]
    PB9 = 1,
    #[doc = "2: Select PC9 as the source input for the EXTI9 external interrupt"]
    PC9 = 2,
    #[doc = "3: Select PD9 as the source input for the EXTI9 external interrupt"]
    PD9 = 3,
    #[doc = "5: Select PF9 as the source input for the EXTI9 external interrupt"]
    PF9 = 5,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI9`"]
pub type EXTI9_R = crate::R<u8, EXTI9_A>;
impl EXTI9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI9_A::PA9),
            1 => Val(EXTI9_A::PB9),
            2 => Val(EXTI9_A::PC9),
            3 => Val(EXTI9_A::PD9),
            5 => Val(EXTI9_A::PF9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9_A::PA9
    }
    #[doc = "Checks if the value of the field is `PB9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9_A::PB9
    }
    #[doc = "Checks if the value of the field is `PC9`"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == EXTI9_A::PC9
    }
    #[doc = "Checks if the value of the field is `PD9`"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == EXTI9_A::PD9
    }
    #[doc = "Checks if the value of the field is `PF9`"]
    #[inline(always)]
    pub fn is_pf9(&self) -> bool {
        *self == EXTI9_A::PF9
    }
}
#[doc = "Write proxy for field `EXTI9`"]
pub struct EXTI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::PA9)
    }
    #[doc = "Select PB9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::PB9)
    }
    #[doc = "Select PC9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(EXTI9_A::PC9)
    }
    #[doc = "Select PD9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(EXTI9_A::PD9)
    }
    #[doc = "Select PF9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pf9(self) -> &'a mut W {
        self.variant(EXTI9_A::PF9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 8 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: Select PA8 as the source input for the EXTI8 external interrupt"]
    PA8 = 0,
    #[doc = "1: Select PB8 as the source input for the EXTI8 external interrupt"]
    PB8 = 1,
    #[doc = "2: Select PC8 as the source input for the EXTI8 external interrupt"]
    PC8 = 2,
    #[doc = "3: Select PD8 as the source input for the EXTI8 external interrupt"]
    PD8 = 3,
    #[doc = "5: Select PF8 as the source input for the EXTI8 external interrupt"]
    PF8 = 5,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI8`"]
pub type EXTI8_R = crate::R<u8, EXTI8_A>;
impl EXTI8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI8_A::PA8),
            1 => Val(EXTI8_A::PB8),
            2 => Val(EXTI8_A::PC8),
            3 => Val(EXTI8_A::PD8),
            5 => Val(EXTI8_A::PF8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8_A::PA8
    }
    #[doc = "Checks if the value of the field is `PB8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8_A::PB8
    }
    #[doc = "Checks if the value of the field is `PC8`"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == EXTI8_A::PC8
    }
    #[doc = "Checks if the value of the field is `PD8`"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == EXTI8_A::PD8
    }
    #[doc = "Checks if the value of the field is `PF8`"]
    #[inline(always)]
    pub fn is_pf8(&self) -> bool {
        *self == EXTI8_A::PF8
    }
}
#[doc = "Write proxy for field `EXTI8`"]
pub struct EXTI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::PA8)
    }
    #[doc = "Select PB8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::PB8)
    }
    #[doc = "Select PC8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(EXTI8_A::PC8)
    }
    #[doc = "Select PD8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(EXTI8_A::PD8)
    }
    #[doc = "Select PF8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pf8(self) -> &'a mut W {
        self.variant(EXTI8_A::PF8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W {
        EXTI11_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W {
        EXTI10_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W {
        EXTI9_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W {
        EXTI8_W { w: self }
    }
}
