#[doc = "Reader of register DCKCFGR2"]
pub type R = crate::R<u32, super::DCKCFGR2>;
#[doc = "Writer for register DCKCFGR2"]
pub type W = crate::W<u32, super::DCKCFGR2>;
#[doc = "Register DCKCFGR2 `reset()`'s with value 0x2000_3000"]
impl crate::ResetValue for super::DCKCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
#[doc = "USART 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SEL_A {
    #[doc = "0: APB2 clock (PCLK2) is selected as USART clock"]
    APB2 = 0,
    #[doc = "1: System clock is selected as USART clock"]
    SYSCLK = 1,
    #[doc = "2: HSI clock is selected as USART clock"]
    HSI = 2,
    #[doc = "3: LSE clock is selected as USART clock"]
    LSE = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USART1SEL`"]
pub type USART1SEL_R = crate::R<u8, USART1SEL_A>;
impl USART1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::APB2,
            1 => USART1SEL_A::SYSCLK,
            2 => USART1SEL_A::HSI,
            3 => USART1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART1SEL_A::APB2
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL_A::LSE
    }
}
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
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(USART1SEL_A::APB2)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "USART 2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART2SEL_A {
    #[doc = "0: APB1 clock (PCLK1) is selected as USART clock"]
    APB1 = 0,
    #[doc = "1: System clock is selected as USART clock"]
    SYSCLK = 1,
    #[doc = "2: HSI clock is selected as USART clock"]
    HSI = 2,
    #[doc = "3: LSE clock is selected as USART clock"]
    LSE = 3,
}
impl From<USART2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USART2SEL`"]
pub type USART2SEL_R = crate::R<u8, USART2SEL_A>;
impl USART2SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2SEL_A {
        match self.bits {
            0 => USART2SEL_A::APB1,
            1 => USART2SEL_A::SYSCLK,
            2 => USART2SEL_A::HSI,
            3 => USART2SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB1`"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == USART2SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART2SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART2SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL_A::LSE
    }
}
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
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "USART 3 clock source selection"]
pub type USART3SEL_A = USART2SEL_A;
#[doc = "Reader of field `USART3SEL`"]
pub type USART3SEL_R = crate::R<u8, USART2SEL_A>;
#[doc = "Write proxy for field `USART3SEL`"]
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "UART 4 clock source selection"]
pub type UART4SEL_A = USART2SEL_A;
#[doc = "Reader of field `UART4SEL`"]
pub type UART4SEL_R = crate::R<u8, USART2SEL_A>;
#[doc = "Write proxy for field `UART4SEL`"]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "UART 5 clock source selection"]
pub type UART5SEL_A = USART2SEL_A;
#[doc = "Reader of field `UART5SEL`"]
pub type UART5SEL_R = crate::R<u8, USART2SEL_A>;
#[doc = "Write proxy for field `UART5SEL`"]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "USART 6 clock source selection"]
pub type USART6SEL_A = USART1SEL_A;
#[doc = "Reader of field `USART6SEL`"]
pub type USART6SEL_R = crate::R<u8, USART1SEL_A>;
#[doc = "Write proxy for field `USART6SEL`"]
pub struct USART6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(USART1SEL_A::APB2)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "UART 7 clock source selection"]
pub type UART7SEL_A = USART2SEL_A;
#[doc = "Reader of field `UART7SEL`"]
pub type UART7SEL_R = crate::R<u8, USART2SEL_A>;
#[doc = "Write proxy for field `UART7SEL`"]
pub struct UART7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "UART 8 clock source selection"]
pub type UART8SEL_A = USART2SEL_A;
#[doc = "Reader of field `UART8SEL`"]
pub type UART8SEL_R = crate::R<u8, USART2SEL_A>;
#[doc = "Write proxy for field `UART8SEL`"]
pub struct UART8SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART8SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: APB clock selected as I2C clock"]
    APB = 0,
    #[doc = "1: System clock selected as I2C clock"]
    SYSCLK = 1,
    #[doc = "2: HSI clock selected as I2C clock"]
    HSI = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2C1SEL`"]
pub type I2C1SEL_R = crate::R<u8, I2C1SEL_A>;
impl I2C1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C1SEL_A::APB),
            1 => Val(I2C1SEL_A::SYSCLK),
            2 => Val(I2C1SEL_A::HSI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SEL_A::HSI
    }
}
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
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "I2C2 clock source selection"]
pub type I2C2SEL_A = I2C1SEL_A;
#[doc = "Reader of field `I2C2SEL`"]
pub type I2C2SEL_R = crate::R<u8, I2C1SEL_A>;
#[doc = "Write proxy for field `I2C2SEL`"]
pub struct I2C2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "I2C3 clock source selection"]
pub type I2C3SEL_A = I2C1SEL_A;
#[doc = "Reader of field `I2C3SEL`"]
pub type I2C3SEL_R = crate::R<u8, I2C1SEL_A>;
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
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "I2C4 clock source selection"]
pub type I2C4SEL_A = I2C1SEL_A;
#[doc = "Reader of field `I2C4SEL`"]
pub type I2C4SEL_R = crate::R<u8, I2C1SEL_A>;
#[doc = "Write proxy for field `I2C4SEL`"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Low power timer 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: APB1 clock (PCLK1) selected as LPTILM1 clock"]
    APB1 = 0,
    #[doc = "1: LSI clock is selected as LPTILM1 clock"]
    LSI = 1,
    #[doc = "2: HSI clock is selected as LPTILM1 clock"]
    HSI = 2,
    #[doc = "3: LSE clock is selected as LPTILM1 clock"]
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
            0 => LPTIM1SEL_A::APB1,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB1`"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL_A::HSI
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
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::APB1)
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "HDMI-CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSEL_A {
    #[doc = "0: LSE clock is selected as HDMI-CEC clock"]
    LSE = 0,
    #[doc = "1: HSI divided by 488 clock is selected as HDMI-CEC clock"]
    HSI_DIV488 = 1,
}
impl From<CECSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CECSEL`"]
pub type CECSEL_R = crate::R<bool, CECSEL_A>;
impl CECSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSEL_A {
        match self.bits {
            false => CECSEL_A::LSE,
            true => CECSEL_A::HSI_DIV488,
        }
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI_DIV488`"]
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        *self == CECSEL_A::HSI_DIV488
    }
}
#[doc = "Write proxy for field `CECSEL`"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::LSE)
    }
    #[doc = "HSI divided by 488 clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn hsi_div488(self) -> &'a mut W {
        self.variant(CECSEL_A::HSI_DIV488)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "48MHz clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK48MSEL_A {
    #[doc = "0: 48MHz clock from PLL is selected"]
    PLL = 0,
    #[doc = "1: 48MHz clock from PLLSAI is selected"]
    PLLSAI = 1,
}
impl From<CK48MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CK48MSEL`"]
pub type CK48MSEL_R = crate::R<bool, CK48MSEL_A>;
impl CK48MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK48MSEL_A {
        match self.bits {
            false => CK48MSEL_A::PLL,
            true => CK48MSEL_A::PLLSAI,
        }
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL_A::PLLSAI
    }
}
#[doc = "Write proxy for field `CK48MSEL`"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK48MSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CK48MSEL_A::PLL)
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(CK48MSEL_A::PLLSAI)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "SDMMC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC1SEL_A {
    #[doc = "0: 48 MHz clock is selected as SD clock"]
    CK48M = 0,
    #[doc = "1: System clock is selected as SD clock"]
    SYSCLK = 1,
}
impl From<SDMMC1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC1SEL`"]
pub type SDMMC1SEL_R = crate::R<bool, SDMMC1SEL_A>;
impl SDMMC1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1SEL_A {
        match self.bits {
            false => SDMMC1SEL_A::CK48M,
            true => SDMMC1SEL_A::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `CK48M`"]
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDMMC1SEL_A::CK48M
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDMMC1SEL_A::SYSCLK
    }
}
#[doc = "Write proxy for field `SDMMC1SEL`"]
pub struct SDMMC1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut W {
        self.variant(SDMMC1SEL_A::CK48M)
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SDMMC1SEL_A::SYSCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W {
        USART3SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W {
        UART4SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W {
        UART5SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    pub fn usart6sel(&mut self) -> USART6SEL_W {
        USART6SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    pub fn uart7sel(&mut self) -> UART7SEL_W {
        UART7SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    pub fn uart8sel(&mut self) -> UART8SEL_W {
        UART8SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W {
        I2C2SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline(always)]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W {
        SDMMC1SEL_W { w: self }
    }
}
