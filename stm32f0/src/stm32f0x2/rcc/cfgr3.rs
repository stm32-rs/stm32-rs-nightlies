#[doc = "Reader of register CFGR3"]
pub type R = crate::R<u32, super::CFGR3>;
#[doc = "Writer for register CFGR3"]
pub type W = crate::W<u32, super::CFGR3>;
#[doc = "Register CFGR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `USART1SW`"]
pub type USART1SW_R = crate::R<u8, USART1SW_A>;
impl USART1SW_R {
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
        *self == USART1SW_A::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW_A::HSI
    }
}
#[doc = "Write proxy for field `USART1SW`"]
pub struct USART1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1SW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
#[doc = "Reader of field `I2C1SW`"]
pub type I2C1SW_R = crate::R<bool, I2C1SW_A>;
impl I2C1SW_R {
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
        *self == I2C1SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW_A::SYSCLK
    }
}
#[doc = "Write proxy for field `I2C1SW`"]
pub struct I2C1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "HDMI CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSW_A {
    #[doc = "0: HSI clock divided by 244 selected as CEC clock source"]
    HSI_DIV244 = 0,
    #[doc = "1: LSE clock selected as CEC clock source"]
    LSE = 1,
}
impl From<CECSW_A> for bool {
    #[inline(always)]
    fn from(variant: CECSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CECSW`"]
pub type CECSW_R = crate::R<bool, CECSW_A>;
impl CECSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSW_A {
        match self.bits {
            false => CECSW_A::HSI_DIV244,
            true => CECSW_A::LSE,
        }
    }
    #[doc = "Checks if the value of the field is `HSI_DIV244`"]
    #[inline(always)]
    pub fn is_hsi_div244(&self) -> bool {
        *self == CECSW_A::HSI_DIV244
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSW_A::LSE
    }
}
#[doc = "Write proxy for field `CECSW`"]
pub struct CECSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn hsi_div244(self) -> &'a mut W {
        self.variant(CECSW_A::HSI_DIV244)
    }
    #[doc = "LSE clock selected as CEC clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSW_A::LSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "USB clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSW_A {
    #[doc = "0: HSI48 selected as USB clock source"]
    HSI48 = 0,
    #[doc = "1: PLL clock selected as USB clock source"]
    PLLCLK = 1,
}
impl From<USBSW_A> for bool {
    #[inline(always)]
    fn from(variant: USBSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBSW`"]
pub type USBSW_R = crate::R<bool, USBSW_A>;
impl USBSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSW_A {
        match self.bits {
            false => USBSW_A::HSI48,
            true => USBSW_A::PLLCLK,
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSW_A::HSI48
    }
    #[doc = "Checks if the value of the field is `PLLCLK`"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == USBSW_A::PLLCLK
    }
}
#[doc = "Write proxy for field `USBSW`"]
pub struct USBSW_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI48 selected as USB clock source"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(USBSW_A::HSI48)
    }
    #[doc = "PLL clock selected as USB clock source"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut W {
        self.variant(USBSW_A::PLLCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADCSW`"]
pub type ADCSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSW`"]
pub struct ADCSW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "USART2 clock source selection"]
pub type USART2SW_A = USART1SW_A;
#[doc = "Reader of field `USART2SW`"]
pub type USART2SW_R = crate::R<u8, USART1SW_A>;
#[doc = "Write proxy for field `USART2SW`"]
pub struct USART2SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2SW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "USART3 clock source"]
pub type USART3SW_A = USART1SW_A;
#[doc = "Reader of field `USART3SW`"]
pub type USART3SW_R = crate::R<u8, USART1SW_A>;
#[doc = "Write proxy for field `USART3SW`"]
pub struct USART3SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3SW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&self) -> CECSW_R {
        CECSW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 0x03) as u8)
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
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&mut self) -> CECSW_W {
        CECSW_W { w: self }
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&mut self) -> USBSW_W {
        USBSW_W { w: self }
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&mut self) -> ADCSW_W {
        ADCSW_W { w: self }
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&mut self) -> USART2SW_W {
        USART2SW_W { w: self }
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&mut self) -> USART3SW_W {
        USART3SW_W { w: self }
    }
}
