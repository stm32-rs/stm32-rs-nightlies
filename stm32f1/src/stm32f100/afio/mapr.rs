#[doc = "Register `MAPR` reader"]
pub struct R(crate::R<MAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPR` writer"]
pub struct W(crate::W<MAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR_SPEC>;
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
impl From<crate::W<MAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_REMAP` reader - SPI1 remapping"]
pub struct SPI1_REMAP_R(crate::FieldReader<bool, bool>);
impl SPI1_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_REMAP` writer - SPI1 remapping"]
pub struct SPI1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `I2C1_REMAP` reader - I2C1 remapping"]
pub struct I2C1_REMAP_R(crate::FieldReader<bool, bool>);
impl I2C1_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_REMAP` writer - I2C1 remapping"]
pub struct I2C1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `USART1_REMAP` reader - USART1 remapping"]
pub struct USART1_REMAP_R(crate::FieldReader<bool, bool>);
impl USART1_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1_REMAP` writer - USART1 remapping"]
pub struct USART1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `USART2_REMAP` reader - USART2 remapping"]
pub struct USART2_REMAP_R(crate::FieldReader<bool, bool>);
impl USART2_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2_REMAP` writer - USART2 remapping"]
pub struct USART2_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `USART3_REMAP` reader - USART3 remapping"]
pub struct USART3_REMAP_R(crate::FieldReader<u8, u8>);
impl USART3_REMAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART3_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3_REMAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3_REMAP` writer - USART3 remapping"]
pub struct USART3_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `TIM1_REMAP` reader - TIM1 remapping"]
pub struct TIM1_REMAP_R(crate::FieldReader<u8, u8>);
impl TIM1_REMAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM1_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_REMAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_REMAP` writer - TIM1 remapping"]
pub struct TIM1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TIM2_REMAP` reader - TIM2 remapping"]
pub struct TIM2_REMAP_R(crate::FieldReader<u8, u8>);
impl TIM2_REMAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM2_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2_REMAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2_REMAP` writer - TIM2 remapping"]
pub struct TIM2_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TIM3_REMAP` reader - TIM3 remapping"]
pub struct TIM3_REMAP_R(crate::FieldReader<u8, u8>);
impl TIM3_REMAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM3_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3_REMAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3_REMAP` writer - TIM3 remapping"]
pub struct TIM3_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `TIM4_REMAP` reader - TIM4 remapping"]
pub struct TIM4_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM4_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4_REMAP` writer - TIM4 remapping"]
pub struct TIM4_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PD01_REMAP` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub struct PD01_REMAP_R(crate::FieldReader<bool, bool>);
impl PD01_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD01_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD01_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD01_REMAP` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub struct PD01_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD01_REMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TIM5CH4_IREMAP` reader - Set and cleared by software"]
pub struct TIM5CH4_IREMAP_R(crate::FieldReader<bool, bool>);
impl TIM5CH4_IREMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5CH4_IREMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5CH4_IREMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5CH4_IREMAP` writer - Set and cleared by software"]
pub struct TIM5CH4_IREMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5CH4_IREMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SWJ_CFG` writer - Serial wire JTAG configuration"]
pub struct SWJ_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWJ_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1_remap(&self) -> SPI1_REMAP_R {
        SPI1_REMAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&self) -> I2C1_REMAP_R {
        I2C1_REMAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3_remap(&self) -> USART3_REMAP_R {
        USART3_REMAP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1_remap(&self) -> TIM1_REMAP_R {
        TIM1_REMAP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2_remap(&self) -> TIM2_REMAP_R {
        TIM2_REMAP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3_remap(&self) -> TIM3_REMAP_R {
        TIM3_REMAP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4_remap(&self) -> TIM4_REMAP_R {
        TIM4_REMAP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    pub fn tim5ch4_iremap(&self) -> TIM5CH4_IREMAP_R {
        TIM5CH4_IREMAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1_remap(&mut self) -> SPI1_REMAP_W {
        SPI1_REMAP_W { w: self }
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1_remap(&mut self) -> I2C1_REMAP_W {
        I2C1_REMAP_W { w: self }
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W {
        USART1_REMAP_W { w: self }
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W {
        USART2_REMAP_W { w: self }
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3_remap(&mut self) -> USART3_REMAP_W {
        USART3_REMAP_W { w: self }
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1_remap(&mut self) -> TIM1_REMAP_W {
        TIM1_REMAP_W { w: self }
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2_remap(&mut self) -> TIM2_REMAP_W {
        TIM2_REMAP_W { w: self }
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3_remap(&mut self) -> TIM3_REMAP_W {
        TIM3_REMAP_W { w: self }
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4_remap(&mut self) -> TIM4_REMAP_W {
        TIM4_REMAP_W { w: self }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W {
        PD01_REMAP_W { w: self }
    }
    #[doc = "Bit 16 - Set and cleared by software"]
    #[inline(always)]
    pub fn tim5ch4_iremap(&mut self) -> TIM5CH4_IREMAP_W {
        TIM5CH4_IREMAP_W { w: self }
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W {
        SWJ_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr](index.html) module"]
pub struct MAPR_SPEC;
impl crate::RegisterSpec for MAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapr::R](R) reader structure"]
impl crate::Readable for MAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapr::W](W) writer structure"]
impl crate::Writable for MAPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAPR to value 0"]
impl crate::Resettable for MAPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
