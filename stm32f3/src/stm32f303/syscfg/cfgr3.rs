#[doc = "Reader of register CFGR3"]
pub type R = crate::R<u32, super::CFGR3>;
#[doc = "Writer for register CFGR3"]
pub type W = crate::W<u32, super::CFGR3>;
#[doc = "Register CFGR3 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::CFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
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
#[doc = "Reader of field `SPI1_RX_DMA_RMP`"]
pub type SPI1_RX_DMA_RMP_R = crate::R<u8, SPI1_RX_DMA_RMP_A>;
impl SPI1_RX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI1_RX_DMA_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI1_RX_DMA_RMP_A::MAPDMA1CH3),
            1 => Val(SPI1_RX_DMA_RMP_A::MAPDMA1CH5),
            2 => Val(SPI1_RX_DMA_RMP_A::MAPDMA1CH7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAPDMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAPDMA1CH7
    }
}
#[doc = "Write proxy for field `SPI1_RX_DMA_RMP`"]
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
#[doc = "Reader of field `SPI1_TX_DMA_RMP`"]
pub type SPI1_TX_DMA_RMP_R = crate::R<u8, SPI1_TX_DMA_RMP_A>;
impl SPI1_TX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI1_TX_DMA_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI1_TX_DMA_RMP_A::MAPDMA1CH3),
            1 => Val(SPI1_TX_DMA_RMP_A::MAPDMA1CH5),
            2 => Val(SPI1_TX_DMA_RMP_A::MAPDMA1CH7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAPDMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAPDMA1CH7
    }
}
#[doc = "Write proxy for field `SPI1_TX_DMA_RMP`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
#[doc = "Reader of field `I2C1_RX_DMA_RMP`"]
pub type I2C1_RX_DMA_RMP_R = crate::R<u8, I2C1_RX_DMA_RMP_A>;
impl I2C1_RX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C1_RX_DMA_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C1_RX_DMA_RMP_A::MAPDMA1CH7),
            1 => Val(I2C1_RX_DMA_RMP_A::MAPDMA1CH3),
            2 => Val(I2C1_RX_DMA_RMP_A::MAPDMA1CH5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAPDMA1CH7
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAPDMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAPDMA1CH5
    }
}
#[doc = "Write proxy for field `I2C1_RX_DMA_RMP`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Reader of field `I2C1_TX_DMA_RMP`"]
pub type I2C1_TX_DMA_RMP_R = crate::R<u8, I2C1_TX_DMA_RMP_A>;
impl I2C1_TX_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C1_TX_DMA_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C1_TX_DMA_RMP_A::MAPDMA1CH6),
            1 => Val(I2C1_TX_DMA_RMP_A::MAPDMA1CH2),
            2 => Val(I2C1_TX_DMA_RMP_A::MAPDMA1CH4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH6`"]
    #[inline(always)]
    pub fn is_map_dma1ch6(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAPDMA1CH6
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAPDMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAPDMA1CH4
    }
}
#[doc = "Write proxy for field `I2C1_TX_DMA_RMP`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "ADC2 DMA remapping bit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC2_DMA_RMP_A {
    #[doc = "0: ADC2 mapped on DMA2"]
    MAPDMA2 = 0,
    #[doc = "3: ADC2 mapped on DMA1 channel 2"]
    MAPDMA1CH2 = 3,
    #[doc = "4: DC2 mapped on DMA1 channel 4"]
    MAPDMA1CH4 = 4,
}
impl From<ADC2_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC2_DMA_RMP`"]
pub type ADC2_DMA_RMP_R = crate::R<u8, ADC2_DMA_RMP_A>;
impl ADC2_DMA_RMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC2_DMA_RMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC2_DMA_RMP_A::MAPDMA2),
            3 => Val(ADC2_DMA_RMP_A::MAPDMA1CH2),
            4 => Val(ADC2_DMA_RMP_A::MAPDMA1CH4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAPDMA2`"]
    #[inline(always)]
    pub fn is_map_dma2(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAPDMA2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAPDMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAPDMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAPDMA1CH4
    }
}
#[doc = "Write proxy for field `ADC2_DMA_RMP`"]
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
    #[doc = "DC2 mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAPDMA1CH4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
}
