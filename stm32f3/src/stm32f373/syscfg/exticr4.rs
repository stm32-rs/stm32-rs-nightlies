#[doc = "Reader of register EXTICR4"]
pub type R = crate::R<u32, super::EXTICR4>;
#[doc = "Writer for register EXTICR4"]
pub type W = crate::W<u32, super::EXTICR4>;
#[doc = "Register EXTICR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTICR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EXTI 15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: Select PA15 as the source input for the EXTI15 external interrupt"]
    PA15 = 0,
    #[doc = "1: Select PB15 as the source input for the EXTI15 external interrupt"]
    PB15 = 1,
    #[doc = "2: Select PC15 as the source input for the EXTI15 external interrupt"]
    PC15 = 2,
    #[doc = "3: Select PD15 as the source input for the EXTI15 external interrupt"]
    PD15 = 3,
    #[doc = "4: Select PE15 as the source input for the EXTI15 external interrupt"]
    PE15 = 4,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI15`"]
pub type EXTI15_R = crate::R<u8, EXTI15_A>;
impl EXTI15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI15_A::PA15),
            1 => Val(EXTI15_A::PB15),
            2 => Val(EXTI15_A::PC15),
            3 => Val(EXTI15_A::PD15),
            4 => Val(EXTI15_A::PE15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_A::PC15
    }
    #[doc = "Checks if the value of the field is `PD15`"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15_A::PD15
    }
    #[doc = "Checks if the value of the field is `PE15`"]
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15_A::PE15
    }
}
#[doc = "Write proxy for field `EXTI15`"]
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::PA15)
    }
    #[doc = "Select PB15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::PB15)
    }
    #[doc = "Select PC15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::PC15)
    }
    #[doc = "Select PD15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(EXTI15_A::PD15)
    }
    #[doc = "Select PE15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pe15(self) -> &'a mut W {
        self.variant(EXTI15_A::PE15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "EXTI 14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: Select PA14 as the source input for the EXTI14 external interrupt"]
    PA14 = 0,
    #[doc = "1: Select PB14 as the source input for the EXTI14 external interrupt"]
    PB14 = 1,
    #[doc = "2: Select PC14 as the source input for the EXTI14 external interrupt"]
    PC14 = 2,
    #[doc = "3: Select PD14 as the source input for the EXTI14 external interrupt"]
    PD14 = 3,
    #[doc = "4: Select PE14 as the source input for the EXTI14 external interrupt"]
    PE14 = 4,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI14`"]
pub type EXTI14_R = crate::R<u8, EXTI14_A>;
impl EXTI14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI14_A::PA14),
            1 => Val(EXTI14_A::PB14),
            2 => Val(EXTI14_A::PC14),
            3 => Val(EXTI14_A::PD14),
            4 => Val(EXTI14_A::PE14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_A::PA14
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_A::PB14
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_A::PC14
    }
    #[doc = "Checks if the value of the field is `PD14`"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14_A::PD14
    }
    #[doc = "Checks if the value of the field is `PE14`"]
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14_A::PE14
    }
}
#[doc = "Write proxy for field `EXTI14`"]
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::PA14)
    }
    #[doc = "Select PB14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::PB14)
    }
    #[doc = "Select PC14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::PC14)
    }
    #[doc = "Select PD14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(EXTI14_A::PD14)
    }
    #[doc = "Select PE14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pe14(self) -> &'a mut W {
        self.variant(EXTI14_A::PE14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "EXTI 13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: Select PA13 as the source input for the EXTI13 external interrupt"]
    PA13 = 0,
    #[doc = "2: Select PC13 as the source input for the EXTI13 external interrupt"]
    PC13 = 2,
    #[doc = "3: Select PD13 as the source input for the EXTI13 external interrupt"]
    PD13 = 3,
    #[doc = "4: Select PE13 as the source input for the EXTI13 external interrupt"]
    PE13 = 4,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI13`"]
pub type EXTI13_R = crate::R<u8, EXTI13_A>;
impl EXTI13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI13_A::PA13),
            2 => Val(EXTI13_A::PC13),
            3 => Val(EXTI13_A::PD13),
            4 => Val(EXTI13_A::PE13),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_A::PA13
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_A::PC13
    }
    #[doc = "Checks if the value of the field is `PD13`"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13_A::PD13
    }
    #[doc = "Checks if the value of the field is `PE13`"]
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13_A::PE13
    }
}
#[doc = "Write proxy for field `EXTI13`"]
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::PA13)
    }
    #[doc = "Select PC13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::PC13)
    }
    #[doc = "Select PD13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(EXTI13_A::PD13)
    }
    #[doc = "Select PE13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pe13(self) -> &'a mut W {
        self.variant(EXTI13_A::PE13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "EXTI 12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: Select PA12 as the source input for the EXTI12 external interrupt"]
    PA12 = 0,
    #[doc = "2: Select PC12 as the source input for the EXTI12 external interrupt"]
    PC12 = 2,
    #[doc = "3: Select PD12 as the source input for the EXTI12 external interrupt"]
    PD12 = 3,
    #[doc = "4: Select PE12 as the source input for the EXTI12 external interrupt"]
    PE12 = 4,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTI12`"]
pub type EXTI12_R = crate::R<u8, EXTI12_A>;
impl EXTI12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTI12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTI12_A::PA12),
            2 => Val(EXTI12_A::PC12),
            3 => Val(EXTI12_A::PD12),
            4 => Val(EXTI12_A::PE12),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_A::PA12
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12_A::PC12
    }
    #[doc = "Checks if the value of the field is `PD12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12_A::PD12
    }
    #[doc = "Checks if the value of the field is `PE12`"]
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12_A::PE12
    }
}
#[doc = "Write proxy for field `EXTI12`"]
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTI12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select PA12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::PA12)
    }
    #[doc = "Select PC12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::PC12)
    }
    #[doc = "Select PD12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(EXTI12_A::PD12)
    }
    #[doc = "Select PE12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pe12(self) -> &'a mut W {
        self.variant(EXTI12_A::PE12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
}
