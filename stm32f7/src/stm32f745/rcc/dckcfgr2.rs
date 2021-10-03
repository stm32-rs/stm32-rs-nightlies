#[doc = "Register `DCKCFGR2` reader"]
pub struct R(crate::R<DCKCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCKCFGR2` writer"]
pub struct W(crate::W<DCKCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DCKCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR2_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `USART1SEL` reader - USART 1 clock source selection"]
pub struct USART1SEL_R(crate::FieldReader<u8, USART1SEL_A>);
impl USART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == USART1SEL_A::APB2
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == USART1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == USART1SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART1SEL_A::LSE
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, USART1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL` writer - USART 1 clock source selection"]
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
#[doc = "Field `USART2SEL` reader - USART 2 clock source selection"]
pub struct USART2SEL_R(crate::FieldReader<u8, USART2SEL_A>);
impl USART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART2SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == USART2SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == USART2SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == USART2SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART2SEL_A::LSE
    }
}
impl core::ops::Deref for USART2SEL_R {
    type Target = crate::FieldReader<u8, USART2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL` writer - USART 2 clock source selection"]
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "USART 3 clock source selection"]
pub type USART3SEL_A = USART2SEL_A;
#[doc = "Field `USART3SEL` reader - USART 3 clock source selection"]
pub type USART3SEL_R = USART2SEL_R;
#[doc = "Field `USART3SEL` writer - USART 3 clock source selection"]
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART3SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART3SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART3SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART3SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "UART 4 clock source selection"]
pub type UART4SEL_A = USART2SEL_A;
#[doc = "Field `UART4SEL` reader - UART 4 clock source selection"]
pub type UART4SEL_R = USART2SEL_R;
#[doc = "Field `UART4SEL` writer - UART 4 clock source selection"]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(UART4SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(UART4SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(UART4SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART4SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "UART 5 clock source selection"]
pub type UART5SEL_A = USART2SEL_A;
#[doc = "Field `UART5SEL` reader - UART 5 clock source selection"]
pub type UART5SEL_R = USART2SEL_R;
#[doc = "Field `UART5SEL` writer - UART 5 clock source selection"]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(UART5SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(UART5SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(UART5SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART5SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "USART 6 clock source selection"]
pub type USART6SEL_A = USART1SEL_A;
#[doc = "Field `USART6SEL` reader - USART 6 clock source selection"]
pub type USART6SEL_R = USART1SEL_R;
#[doc = "Field `USART6SEL` writer - USART 6 clock source selection"]
pub struct USART6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(USART6SEL_A::APB2)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART6SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART6SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART6SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "UART 7 clock source selection"]
pub type UART7SEL_A = USART2SEL_A;
#[doc = "Field `UART7SEL` reader - UART 7 clock source selection"]
pub type UART7SEL_R = USART2SEL_R;
#[doc = "Field `UART7SEL` writer - UART 7 clock source selection"]
pub struct UART7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(UART7SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(UART7SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(UART7SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART7SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "UART 8 clock source selection"]
pub type UART8SEL_A = USART2SEL_A;
#[doc = "Field `UART8SEL` reader - UART 8 clock source selection"]
pub type UART8SEL_R = USART2SEL_R;
#[doc = "Field `UART8SEL` writer - UART 8 clock source selection"]
pub struct UART8SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART8SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(UART8SEL_A::APB1)
    }
    #[doc = "System clock is selected as USART clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(UART8SEL_A::SYSCLK)
    }
    #[doc = "HSI clock is selected as USART clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(UART8SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as USART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART8SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
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
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub struct I2C1SEL_R(crate::FieldReader<u8, I2C1SEL_A>);
impl I2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::APB),
            1 => Some(I2C1SEL_A::SYSCLK),
            2 => Some(I2C1SEL_A::HSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == I2C1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == I2C1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == I2C1SEL_A::HSI
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, I2C1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "I2C2 clock source selection"]
pub type I2C2SEL_A = I2C1SEL_A;
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub type I2C2SEL_R = I2C1SEL_R;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
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
        self.variant(I2C2SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C2SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C2SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "I2C3 clock source selection"]
pub type I2C3SEL_A = I2C1SEL_A;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub type I2C3SEL_R = I2C1SEL_R;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
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
        self.variant(I2C3SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "I2C4 clock source selection"]
pub type I2C4SEL_A = I2C1SEL_A;
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub type I2C4SEL_R = I2C1SEL_R;
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
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
        self.variant(I2C4SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C4SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
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
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_R(crate::FieldReader<u8, LPTIM1SEL_A>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == LPTIM1SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == LPTIM1SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM1SEL_A::LSE
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, LPTIM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
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
#[doc = "Field `CECSEL` reader - HDMI-CEC clock source selection"]
pub struct CECSEL_R(crate::FieldReader<bool, CECSEL_A>);
impl CECSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == CECSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI_DIV488`"]
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        **self == CECSEL_A::HSI_DIV488
    }
}
impl core::ops::Deref for CECSEL_R {
    type Target = crate::FieldReader<bool, CECSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSEL` writer - HDMI-CEC clock source selection"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
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
#[doc = "Field `CK48MSEL` reader - 48MHz clock source selection"]
pub struct CK48MSEL_R(crate::FieldReader<bool, CK48MSEL_A>);
impl CK48MSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CK48MSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == CK48MSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        **self == CK48MSEL_A::PLLSAI
    }
}
impl core::ops::Deref for CK48MSEL_R {
    type Target = crate::FieldReader<bool, CK48MSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK48MSEL` writer - 48MHz clock source selection"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK48MSEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
#[doc = "Field `SDMMC1SEL` reader - SDMMC clock source selection"]
pub struct SDMMC1SEL_R(crate::FieldReader<bool, SDMMC1SEL_A>);
impl SDMMC1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1SEL_R(crate::FieldReader::new(bits))
    }
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
        **self == SDMMC1SEL_A::CK48M
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == SDMMC1SEL_A::SYSCLK
    }
}
impl core::ops::Deref for SDMMC1SEL_R {
    type Target = crate::FieldReader<bool, SDMMC1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1SEL` writer - SDMMC clock source selection"]
pub struct SDMMC1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1SEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dedicated clocks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr2](index.html) module"]
pub struct DCKCFGR2_SPEC;
impl crate::RegisterSpec for DCKCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dckcfgr2::R](R) reader structure"]
impl crate::Readable for DCKCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dckcfgr2::W](W) writer structure"]
impl crate::Writable for DCKCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCKCFGR2 to value 0"]
impl crate::Resettable for DCKCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
