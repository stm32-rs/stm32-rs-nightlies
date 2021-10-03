#[doc = "Register `CCIPR1` reader"]
pub struct R(crate::R<CCIPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR1` writer"]
pub struct W(crate::W<CCIPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR1_SPEC>;
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
impl From<crate::W<CCIPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub struct ADCSEL_R(crate::FieldReader<u8, u8>);
impl ADCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub struct ADCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CLK48MSEL` reader - 48 MHz clock source selection"]
pub struct CLK48MSEL_R(crate::FieldReader<u8, u8>);
impl CLK48MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK48MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK48MSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK48MSEL` writer - 48 MHz clock source selection"]
pub struct CLK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48MSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `FDCANSEL` reader - FDCAN clock source selection"]
pub struct FDCANSEL_R(crate::FieldReader<u8, u8>);
impl FDCANSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDCANSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANSEL` writer - FDCAN clock source selection"]
pub struct FDCANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `LPTIM3SEL` reader - Low-power timer 3 clock source selection"]
pub struct LPTIM3SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3SEL` writer - Low-power timer 3 clock source selection"]
pub struct LPTIM3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub struct LPTIM2SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub struct LPTIM2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub struct I2C3SEL_R(crate::FieldReader<u8, u8>);
impl I2C3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub struct I2C2SEL_R(crate::FieldReader<u8, u8>);
impl I2C2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub struct I2C2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub struct I2C1SEL_R(crate::FieldReader<u8, u8>);
impl I2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub struct LPUART1SEL_R(crate::FieldReader<u8, u8>);
impl LPUART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `UART5SEL` reader - UART5 clock source selection"]
pub struct UART5SEL_R(crate::FieldReader<u8, u8>);
impl UART5SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART5SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5SEL` writer - UART5 clock source selection"]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `UART4SEL` reader - UART4 clock source selection"]
pub struct UART4SEL_R(crate::FieldReader<u8, u8>);
impl UART4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4SEL` writer - UART4 clock source selection"]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub struct USART3SEL_R(crate::FieldReader<u8, u8>);
impl USART3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub struct USART2SEL_R(crate::FieldReader<u8, u8>);
impl USART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub struct USART1SEL_R(crate::FieldReader<u8, u8>);
impl USART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48msel(&self) -> CLK48MSEL_R {
        CLK48MSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - FDCAN clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Low-power timer 3 clock source selection"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W { w: self }
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48msel(&mut self) -> CLK48MSEL_W {
        CLK48MSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - FDCAN clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W {
        FDCANSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Low-power timer 3 clock source selection"]
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W {
        LPTIM3SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W {
        LPTIM2SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W {
        I2C2SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W {
        UART5SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W {
        UART4SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W {
        USART3SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCIPR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr1](index.html) module"]
pub struct CCIPR1_SPEC;
impl crate::RegisterSpec for CCIPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr1::R](R) reader structure"]
impl crate::Readable for CCIPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr1::W](W) writer structure"]
impl crate::Writable for CCIPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}