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
#[doc = "SPI1_RX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI1_RX_DMA_RMP_A {
    #[doc = "0: SPI1_RX mapped on DMA1 CH2"]
    MAPDMA1CH3 = 0,
    #[doc = "1: SPI1_RX mapped on DMA1 CH4"]
    MAPDMA1CH5 = 1,
    #[doc = "2: SPI1_RX mapped on DMA1 CH6"]
    MAPDMA1CH7 = 2,
}
impl From<SPI1_RX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_RX_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI1_RX_DMA_RMP` reader - SPI1_RX DMA remapping bit"]
pub struct SPI1_RX_DMA_RMP_R(crate::FieldReader<u8, SPI1_RX_DMA_RMP_A>);
impl SPI1_RX_DMA_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_RX_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI1_RX_DMA_RMP_A> {
        match self.bits {
            0 => Some(SPI1_RX_DMA_RMP_A::MAPDMA1CH3),
            1 => Some(SPI1_RX_DMA_RMP_A::MAPDMA1CH5),
            2 => Some(SPI1_RX_DMA_RMP_A::MAPDMA1CH7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        **self == SPI1_RX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        **self == SPI1_RX_DMA_RMP_A::MAPDMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        **self == SPI1_RX_DMA_RMP_A::MAPDMA1CH7
    }
}
impl core::ops::Deref for SPI1_RX_DMA_RMP_R {
    type Target = crate::FieldReader<u8, SPI1_RX_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_RX_DMA_RMP` writer - SPI1_RX DMA remapping bit"]
pub struct SPI1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_RX_DMA_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI1_RX mapped on DMA1 CH2"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAPDMA1CH3)
    }
    #[doc = "SPI1_RX mapped on DMA1 CH4"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAPDMA1CH5)
    }
    #[doc = "SPI1_RX mapped on DMA1 CH6"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAPDMA1CH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "SPI1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI1_TX_DMA_RMP_A {
    #[doc = "0: SPI1_TX mapped on DMA1 CH3"]
    MAPDMA1CH3 = 0,
    #[doc = "1: SPI1_TX mapped on DMA1 CH5"]
    MAPDMA1CH5 = 1,
    #[doc = "2: SPI1_TX mapped on DMA1 CH7"]
    MAPDMA1CH7 = 2,
}
impl From<SPI1_TX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_TX_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI1_TX_DMA_RMP` reader - SPI1_TX DMA remapping bit"]
pub struct SPI1_TX_DMA_RMP_R(crate::FieldReader<u8, SPI1_TX_DMA_RMP_A>);
impl SPI1_TX_DMA_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_TX_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI1_TX_DMA_RMP_A> {
        match self.bits {
            0 => Some(SPI1_TX_DMA_RMP_A::MAPDMA1CH3),
            1 => Some(SPI1_TX_DMA_RMP_A::MAPDMA1CH5),
            2 => Some(SPI1_TX_DMA_RMP_A::MAPDMA1CH7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        **self == SPI1_TX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        **self == SPI1_TX_DMA_RMP_A::MAPDMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        **self == SPI1_TX_DMA_RMP_A::MAPDMA1CH7
    }
}
impl core::ops::Deref for SPI1_TX_DMA_RMP_R {
    type Target = crate::FieldReader<u8, SPI1_TX_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_TX_DMA_RMP` writer - SPI1_TX DMA remapping bit"]
pub struct SPI1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_TX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_TX_DMA_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI1_TX mapped on DMA1 CH3"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAPDMA1CH3)
    }
    #[doc = "SPI1_TX mapped on DMA1 CH5"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAPDMA1CH5)
    }
    #[doc = "SPI1_TX mapped on DMA1 CH7"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAPDMA1CH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "I2C1_RX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1_RX_DMA_RMP_A {
    #[doc = "0: I2C1_RX mapped on DMA1 CH7"]
    MAPDMA1CH7 = 0,
    #[doc = "1: I2C1_RX mapped on DMA1 CH3"]
    MAPDMA1CH3 = 1,
    #[doc = "2: I2C1_RX mapped on DMA1 CH5"]
    MAPDMA1CH5 = 2,
}
impl From<I2C1_RX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_RX_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C1_RX_DMA_RMP` reader - I2C1_RX DMA remapping bit"]
pub struct I2C1_RX_DMA_RMP_R(crate::FieldReader<u8, I2C1_RX_DMA_RMP_A>);
impl I2C1_RX_DMA_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1_RX_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1_RX_DMA_RMP_A> {
        match self.bits {
            0 => Some(I2C1_RX_DMA_RMP_A::MAPDMA1CH7),
            1 => Some(I2C1_RX_DMA_RMP_A::MAPDMA1CH3),
            2 => Some(I2C1_RX_DMA_RMP_A::MAPDMA1CH5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        **self == I2C1_RX_DMA_RMP_A::MAPDMA1CH7
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        **self == I2C1_RX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        **self == I2C1_RX_DMA_RMP_A::MAPDMA1CH5
    }
}
impl core::ops::Deref for I2C1_RX_DMA_RMP_R {
    type Target = crate::FieldReader<u8, I2C1_RX_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_RX_DMA_RMP` writer - I2C1_RX DMA remapping bit"]
pub struct I2C1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_RX_DMA_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "I2C1_RX mapped on DMA1 CH7"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAPDMA1CH7)
    }
    #[doc = "I2C1_RX mapped on DMA1 CH3"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAPDMA1CH3)
    }
    #[doc = "I2C1_RX mapped on DMA1 CH5"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAPDMA1CH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "I2C1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1_TX_DMA_RMP_A {
    #[doc = "0: I2C1_TX mapped on DMA1 CH6"]
    MAPDMA1CH6 = 0,
    #[doc = "1: I2C1_TX mapped on DMA1 CH2"]
    MAPDMA1CH2 = 1,
    #[doc = "2: I2C1_TX mapped on DMA1 CH4"]
    MAPDMA1CH4 = 2,
}
impl From<I2C1_TX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_TX_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C1_TX_DMA_RMP` reader - I2C1_TX DMA remapping bit"]
pub struct I2C1_TX_DMA_RMP_R(crate::FieldReader<u8, I2C1_TX_DMA_RMP_A>);
impl I2C1_TX_DMA_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1_TX_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1_TX_DMA_RMP_A> {
        match self.bits {
            0 => Some(I2C1_TX_DMA_RMP_A::MAPDMA1CH6),
            1 => Some(I2C1_TX_DMA_RMP_A::MAPDMA1CH2),
            2 => Some(I2C1_TX_DMA_RMP_A::MAPDMA1CH4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH6`"]
    #[inline(always)]
    pub fn is_map_dma1ch6(&self) -> bool {
        **self == I2C1_TX_DMA_RMP_A::MAPDMA1CH6
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        **self == I2C1_TX_DMA_RMP_A::MAPDMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        **self == I2C1_TX_DMA_RMP_A::MAPDMA1CH4
    }
}
impl core::ops::Deref for I2C1_TX_DMA_RMP_R {
    type Target = crate::FieldReader<u8, I2C1_TX_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_TX_DMA_RMP` writer - I2C1_TX DMA remapping bit"]
pub struct I2C1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_TX_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_TX_DMA_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "I2C1_TX mapped on DMA1 CH6"]
    #[inline(always)]
    pub fn map_dma1ch6(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAPDMA1CH6)
    }
    #[doc = "I2C1_TX mapped on DMA1 CH2"]
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAPDMA1CH2)
    }
    #[doc = "I2C1_TX mapped on DMA1 CH4"]
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAPDMA1CH4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "ADC2 DMA remapping bit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC2_DMA_RMP_A {
    #[doc = "0: ADC2 mapped on DMA2"]
    MAPDMA2 = 0,
    #[doc = "2: ADC2 mapped on DMA1 channel 2"]
    MAPDMA1CH2 = 2,
    #[doc = "3: ADC2 mapped on DMA1 channel 4"]
    MAPDMA1CH4 = 3,
}
impl From<ADC2_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC2_DMA_RMP` reader - ADC2 DMA remapping bit"]
pub struct ADC2_DMA_RMP_R(crate::FieldReader<u8, ADC2_DMA_RMP_A>);
impl ADC2_DMA_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC2_DMA_RMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC2_DMA_RMP_A> {
        match self.bits {
            0 => Some(ADC2_DMA_RMP_A::MAPDMA2),
            2 => Some(ADC2_DMA_RMP_A::MAPDMA1CH2),
            3 => Some(ADC2_DMA_RMP_A::MAPDMA1CH4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA2`"]
    #[inline(always)]
    pub fn is_map_dma2(&self) -> bool {
        **self == ADC2_DMA_RMP_A::MAPDMA2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        **self == ADC2_DMA_RMP_A::MAPDMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        **self == ADC2_DMA_RMP_A::MAPDMA1CH4
    }
}
impl core::ops::Deref for ADC2_DMA_RMP_R {
    type Target = crate::FieldReader<u8, ADC2_DMA_RMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_DMA_RMP` writer - ADC2 DMA remapping bit"]
pub struct ADC2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_DMA_RMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC2_DMA_RMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC2 mapped on DMA2"]
    #[inline(always)]
    pub fn map_dma2(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAPDMA2)
    }
    #[doc = "ADC2 mapped on DMA1 channel 2"]
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAPDMA1CH2)
    }
    #[doc = "ADC2 mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAPDMA1CH4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&self) -> SPI1_RX_DMA_RMP_R {
        SPI1_RX_DMA_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&self) -> SPI1_TX_DMA_RMP_R {
        SPI1_TX_DMA_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&self) -> I2C1_RX_DMA_RMP_R {
        I2C1_RX_DMA_RMP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&self) -> I2C1_TX_DMA_RMP_R {
        I2C1_TX_DMA_RMP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&mut self) -> SPI1_RX_DMA_RMP_W {
        SPI1_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&mut self) -> SPI1_TX_DMA_RMP_W {
        SPI1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&mut self) -> I2C1_RX_DMA_RMP_W {
        I2C1_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&mut self) -> I2C1_TX_DMA_RMP_W {
        I2C1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W {
        ADC2_DMA_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
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
#[doc = "`reset()` method sets CFGR3 to value 0x0200"]
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
