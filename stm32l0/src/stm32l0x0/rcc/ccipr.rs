#[doc = "Reader of register CCIPR"]
pub type R = crate::R<u32, super::CCIPR>;
#[doc = "Writer for register CCIPR"]
pub type W = crate::W<u32, super::CCIPR>;
#[doc = "Register CCIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low Power Timer clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: APB clock selected as Timer clock"]
    APB = 0,
    #[doc = "1: LSI clock selected as Timer clock"]
    LSI = 1,
    #[doc = "2: HSI16 clock selected as Timer clock"]
    HSI16 = 2,
    #[doc = "3: LSE clock selected as Timer clock"]
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTIM1SEL`"]
pub type LPTIM1SEL_R = crate::R<u8, LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::APB,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI16,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == LPTIM1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL_A::HSI16
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::LSE
    }
}
#[doc = "Write proxy for field `LPTIM1SEL`"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB clock selected as Timer clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::APB)
    }
    #[doc = "LSI clock selected as Timer clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "HSI16 clock selected as Timer clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as Timer clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "I2C3 clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C3SEL_A {
    #[doc = "0: APB clock selected as peripheral clock"]
    APB = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    SYSTEM = 1,
    #[doc = "2: HSI16 clock selected as peripheral clock"]
    HSI16 = 2,
}
impl From<I2C3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C3SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2C3SEL`"]
pub type I2C3SEL_R = crate::R<u8, I2C3SEL_A>;
impl I2C3SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C3SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C3SEL_A::APB),
            1 => Val(I2C3SEL_A::SYSTEM),
            2 => Val(I2C3SEL_A::HSI16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C3SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C3SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C3SEL_A::HSI16
    }
}
#[doc = "Write proxy for field `I2C3SEL`"]
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C3SEL_A::APB)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "I2C1 clock source selection bits"]
pub type I2C1SEL_A = I2C3SEL_A;
#[doc = "Reader of field `I2C1SEL`"]
pub type I2C1SEL_R = crate::R<u8, I2C3SEL_A>;
#[doc = "Write proxy for field `I2C1SEL`"]
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C3SEL_A::APB)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "LPUART1 clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: APB clock selected as peripheral clock"]
    APB = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    SYSTEM = 1,
    #[doc = "2: HSI16 clock selected as peripheral clock"]
    HSI16 = 2,
    #[doc = "3: LSE clock selected as peripheral clock"]
    LSE = 3,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUART1SEL`"]
pub type LPUART1SEL_R = crate::R<u8, LPUART1SEL_A>;
impl LPUART1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SEL_A {
        match self.bits {
            0 => LPUART1SEL_A::APB,
            1 => LPUART1SEL_A::SYSTEM,
            2 => LPUART1SEL_A::HSI16,
            3 => LPUART1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == LPUART1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == LPUART1SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL_A::HSI16
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL_A::LSE
    }
}
#[doc = "Write proxy for field `LPUART1SEL`"]
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::APB)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "USART2 clock source selection bits"]
pub type USART2SEL_A = LPUART1SEL_A;
#[doc = "Reader of field `USART2SEL`"]
pub type USART2SEL_R = crate::R<u8, LPUART1SEL_A>;
#[doc = "Write proxy for field `USART2SEL`"]
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::APB)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "USART1 clock source selection bits"]
pub type USART1SEL_A = LPUART1SEL_A;
#[doc = "Reader of field `USART1SEL`"]
pub type USART1SEL_R = crate::R<u8, LPUART1SEL_A>;
#[doc = "Write proxy for field `USART1SEL`"]
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::APB)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    #[doc = "Bits 0:1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
}
