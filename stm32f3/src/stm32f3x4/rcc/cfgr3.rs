#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SW_A {
    #[doc = "0: PCLK selected as USART clock source"]
    PCLK = 0,
    #[doc = "1: SYSCLK selected as USART clock source"]
    SYSCLK = 1,
    #[doc = "2: LSE selected as USART clock source"]
    LSE = 2,
    #[doc = "3: HSI selected as USART clock source"]
    HSI = 3,
}
impl From<USART1SW_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub struct USART1SW_R(crate::FieldReader<u8, USART1SW_A>);
impl USART1SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SW_A {
        match self.bits {
            0 => USART1SW_A::PCLK,
            1 => USART1SW_A::SYSCLK,
            2 => USART1SW_A::LSE,
            3 => USART1SW_A::HSI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == USART1SW_A::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == USART1SW_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART1SW_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == USART1SW_A::HSI
    }
}
impl core::ops::Deref for USART1SW_R {
    type Target = crate::FieldReader<u8, USART1SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub struct USART1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SW_A::PCLK)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SW_A::SYSCLK)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SW_A::LSE)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SW_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1SW_A {
    #[doc = "0: HSI clock selected as I2C clock source"]
    HSI = 0,
    #[doc = "1: SYSCLK clock selected as I2C clock source"]
    SYSCLK = 1,
}
impl From<I2C1SW_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub struct I2C1SW_R(crate::FieldReader<bool, I2C1SW_A>);
impl I2C1SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1SW_A {
        match self.bits {
            false => I2C1SW_A::HSI,
            true => I2C1SW_A::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == I2C1SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == I2C1SW_A::SYSCLK
    }
}
impl core::ops::Deref for I2C1SW_R {
    type Target = crate::FieldReader<bool, I2C1SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub struct I2C1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1SW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI clock selected as I2C clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SW_A::HSI)
    }
    #[doc = "SYSCLK clock selected as I2C clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SW_A::SYSCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Timer1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1SW_A {
    #[doc = "0: PCLK2 clock (doubled frequency when prescaled)"]
    PCLK2 = 0,
    #[doc = "1: PLL vco output (running up to 144 MHz)"]
    PLL = 1,
}
impl From<TIM1SW_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1SW` reader - Timer1 clock source selection"]
pub struct TIM1SW_R(crate::FieldReader<bool, TIM1SW_A>);
impl TIM1SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1SW_A {
        match self.bits {
            false => TIM1SW_A::PCLK2,
            true => TIM1SW_A::PLL,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK2`"]
    #[inline(always)]
    pub fn is_pclk2(&self) -> bool {
        **self == TIM1SW_A::PCLK2
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == TIM1SW_A::PLL
    }
}
impl core::ops::Deref for TIM1SW_R {
    type Target = crate::FieldReader<bool, TIM1SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1SW` writer - Timer1 clock source selection"]
pub struct TIM1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1SW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PCLK2 clock (doubled frequency when prescaled)"]
    #[inline(always)]
    pub fn pclk2(self) -> &'a mut W {
        self.variant(TIM1SW_A::PCLK2)
    }
    #[doc = "PLL vco output (running up to 144 MHz)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TIM1SW_A::PLL)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&self) -> TIM1SW_R {
        TIM1SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&mut self) -> USART1SW_W {
        USART1SW_W { w: self }
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2C1SW_W {
        I2C1SW_W { w: self }
    }
    #[doc = "Bit 8 - Timer1 clock source selection"]
    #[inline(always)]
    pub fn tim1sw(&mut self) -> TIM1SW_W {
        TIM1SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
