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
#[doc = "Field `CECSW` reader - HDMI CEC clock source selection"]
pub struct CECSW_R(crate::FieldReader<bool, CECSW_A>);
impl CECSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECSW_R(crate::FieldReader::new(bits))
    }
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
        **self == CECSW_A::HSI_DIV244
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == CECSW_A::LSE
    }
}
impl core::ops::Deref for CECSW_R {
    type Target = crate::FieldReader<bool, CECSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSW` writer - HDMI CEC clock source selection"]
pub struct CECSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSW_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
#[doc = "Field `USBSW` reader - USB clock source selection"]
pub struct USBSW_R(crate::FieldReader<bool, USBSW_A>);
impl USBSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSW_R(crate::FieldReader::new(bits))
    }
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
        **self == USBSW_A::HSI48
    }
    #[doc = "Checks if the value of the field is `PLLCLK`"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        **self == USBSW_A::PLLCLK
    }
}
impl core::ops::Deref for USBSW_R {
    type Target = crate::FieldReader<bool, USBSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSW` writer - USB clock source selection"]
pub struct USBSW_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSW_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ADCSW` reader - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub struct ADCSW_R(crate::FieldReader<bool, bool>);
impl ADCSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSW` writer - ADCSW is deprecated. See ADC field in CFGR2 register."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "USART2 clock source selection"]
pub type USART2SW_A = USART1SW_A;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub type USART2SW_R = USART1SW_R;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub struct USART2SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART2SW_A::PCLK)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SW_A::SYSCLK)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SW_A::LSE)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SW_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "USART3 clock source"]
pub type USART3SW_A = USART1SW_A;
#[doc = "Field `USART3SW` reader - USART3 clock source"]
pub type USART3SW_R = USART1SW_R;
#[doc = "Field `USART3SW` writer - USART3 clock source"]
pub struct USART3SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART3SW_A::PCLK)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART3SW_A::SYSCLK)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART3SW_A::LSE)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART3SW_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
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
