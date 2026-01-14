///Register `CFGR3` reader
pub type R = crate::R<CFGR3rs>;
///Register `CFGR3` writer
pub type W = crate::W<CFGR3rs>;
/**SPI1_RX DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1_RX_DMA_RMP {
    ///0: SPI1_RX mapped on DMA1 CH2
    MapDma1ch3 = 0,
    ///1: SPI1_RX mapped on DMA1 CH4
    MapDma1ch5 = 1,
    ///2: SPI1_RX mapped on DMA1 CH6
    MapDma1ch7 = 2,
}
impl From<SPI1_RX_DMA_RMP> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_RX_DMA_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1_RX_DMA_RMP {
    type Ux = u8;
}
impl crate::IsEnum for SPI1_RX_DMA_RMP {}
///Field `SPI1_RX_DMA_RMP` reader - SPI1_RX DMA remapping bit
pub type SPI1_RX_DMA_RMP_R = crate::FieldReader<SPI1_RX_DMA_RMP>;
impl SPI1_RX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1_RX_DMA_RMP> {
        match self.bits {
            0 => Some(SPI1_RX_DMA_RMP::MapDma1ch3),
            1 => Some(SPI1_RX_DMA_RMP::MapDma1ch5),
            2 => Some(SPI1_RX_DMA_RMP::MapDma1ch7),
            _ => None,
        }
    }
    ///SPI1_RX mapped on DMA1 CH2
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_RX_DMA_RMP::MapDma1ch3
    }
    ///SPI1_RX mapped on DMA1 CH4
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_RX_DMA_RMP::MapDma1ch5
    }
    ///SPI1_RX mapped on DMA1 CH6
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_RX_DMA_RMP::MapDma1ch7
    }
}
///Field `SPI1_RX_DMA_RMP` writer - SPI1_RX DMA remapping bit
pub type SPI1_RX_DMA_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI1_RX_DMA_RMP>;
impl<'a, REG> SPI1_RX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI1_RX mapped on DMA1 CH2
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_RX_DMA_RMP::MapDma1ch3)
    }
    ///SPI1_RX mapped on DMA1 CH4
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_RX_DMA_RMP::MapDma1ch5)
    }
    ///SPI1_RX mapped on DMA1 CH6
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_RX_DMA_RMP::MapDma1ch7)
    }
}
/**SPI1_TX DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1_TX_DMA_RMP {
    ///0: SPI1_TX mapped on DMA1 CH3
    MapDma1ch3 = 0,
    ///1: SPI1_TX mapped on DMA1 CH5
    MapDma1ch5 = 1,
    ///2: SPI1_TX mapped on DMA1 CH7
    MapDma1ch7 = 2,
}
impl From<SPI1_TX_DMA_RMP> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_TX_DMA_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1_TX_DMA_RMP {
    type Ux = u8;
}
impl crate::IsEnum for SPI1_TX_DMA_RMP {}
///Field `SPI1_TX_DMA_RMP` reader - SPI1_TX DMA remapping bit
pub type SPI1_TX_DMA_RMP_R = crate::FieldReader<SPI1_TX_DMA_RMP>;
impl SPI1_TX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1_TX_DMA_RMP> {
        match self.bits {
            0 => Some(SPI1_TX_DMA_RMP::MapDma1ch3),
            1 => Some(SPI1_TX_DMA_RMP::MapDma1ch5),
            2 => Some(SPI1_TX_DMA_RMP::MapDma1ch7),
            _ => None,
        }
    }
    ///SPI1_TX mapped on DMA1 CH3
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_TX_DMA_RMP::MapDma1ch3
    }
    ///SPI1_TX mapped on DMA1 CH5
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_TX_DMA_RMP::MapDma1ch5
    }
    ///SPI1_TX mapped on DMA1 CH7
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_TX_DMA_RMP::MapDma1ch7
    }
}
///Field `SPI1_TX_DMA_RMP` writer - SPI1_TX DMA remapping bit
pub type SPI1_TX_DMA_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI1_TX_DMA_RMP>;
impl<'a, REG> SPI1_TX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI1_TX mapped on DMA1 CH3
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_TX_DMA_RMP::MapDma1ch3)
    }
    ///SPI1_TX mapped on DMA1 CH5
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_TX_DMA_RMP::MapDma1ch5)
    }
    ///SPI1_TX mapped on DMA1 CH7
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_TX_DMA_RMP::MapDma1ch7)
    }
}
/**I2C1_RX DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1_RX_DMA_RMP {
    ///0: I2C1_RX mapped on DMA1 CH7
    MapDma1ch7 = 0,
    ///1: I2C1_RX mapped on DMA1 CH3
    MapDma1ch3 = 1,
    ///2: I2C1_RX mapped on DMA1 CH5
    MapDma1ch5 = 2,
}
impl From<I2C1_RX_DMA_RMP> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_RX_DMA_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1_RX_DMA_RMP {
    type Ux = u8;
}
impl crate::IsEnum for I2C1_RX_DMA_RMP {}
///Field `I2C1_RX_DMA_RMP` reader - I2C1_RX DMA remapping bit
pub type I2C1_RX_DMA_RMP_R = crate::FieldReader<I2C1_RX_DMA_RMP>;
impl I2C1_RX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1_RX_DMA_RMP> {
        match self.bits {
            0 => Some(I2C1_RX_DMA_RMP::MapDma1ch7),
            1 => Some(I2C1_RX_DMA_RMP::MapDma1ch3),
            2 => Some(I2C1_RX_DMA_RMP::MapDma1ch5),
            _ => None,
        }
    }
    ///I2C1_RX mapped on DMA1 CH7
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == I2C1_RX_DMA_RMP::MapDma1ch7
    }
    ///I2C1_RX mapped on DMA1 CH3
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == I2C1_RX_DMA_RMP::MapDma1ch3
    }
    ///I2C1_RX mapped on DMA1 CH5
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == I2C1_RX_DMA_RMP::MapDma1ch5
    }
}
///Field `I2C1_RX_DMA_RMP` writer - I2C1_RX DMA remapping bit
pub type I2C1_RX_DMA_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1_RX_DMA_RMP>;
impl<'a, REG> I2C1_RX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2C1_RX mapped on DMA1 CH7
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_RX_DMA_RMP::MapDma1ch7)
    }
    ///I2C1_RX mapped on DMA1 CH3
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_RX_DMA_RMP::MapDma1ch3)
    }
    ///I2C1_RX mapped on DMA1 CH5
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_RX_DMA_RMP::MapDma1ch5)
    }
}
/**I2C1_TX DMA remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1_TX_DMA_RMP {
    ///0: I2C1_TX mapped on DMA1 CH6
    MapDma1ch6 = 0,
    ///1: I2C1_TX mapped on DMA1 CH2
    MapDma1ch2 = 1,
    ///2: I2C1_TX mapped on DMA1 CH4
    MapDma1ch4 = 2,
}
impl From<I2C1_TX_DMA_RMP> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_TX_DMA_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1_TX_DMA_RMP {
    type Ux = u8;
}
impl crate::IsEnum for I2C1_TX_DMA_RMP {}
///Field `I2C1_TX_DMA_RMP` reader - I2C1_TX DMA remapping bit
pub type I2C1_TX_DMA_RMP_R = crate::FieldReader<I2C1_TX_DMA_RMP>;
impl I2C1_TX_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1_TX_DMA_RMP> {
        match self.bits {
            0 => Some(I2C1_TX_DMA_RMP::MapDma1ch6),
            1 => Some(I2C1_TX_DMA_RMP::MapDma1ch2),
            2 => Some(I2C1_TX_DMA_RMP::MapDma1ch4),
            _ => None,
        }
    }
    ///I2C1_TX mapped on DMA1 CH6
    #[inline(always)]
    pub fn is_map_dma1ch6(&self) -> bool {
        *self == I2C1_TX_DMA_RMP::MapDma1ch6
    }
    ///I2C1_TX mapped on DMA1 CH2
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == I2C1_TX_DMA_RMP::MapDma1ch2
    }
    ///I2C1_TX mapped on DMA1 CH4
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == I2C1_TX_DMA_RMP::MapDma1ch4
    }
}
///Field `I2C1_TX_DMA_RMP` writer - I2C1_TX DMA remapping bit
pub type I2C1_TX_DMA_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1_TX_DMA_RMP>;
impl<'a, REG> I2C1_TX_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I2C1_TX mapped on DMA1 CH6
    #[inline(always)]
    pub fn map_dma1ch6(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_TX_DMA_RMP::MapDma1ch6)
    }
    ///I2C1_TX mapped on DMA1 CH2
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_TX_DMA_RMP::MapDma1ch2)
    }
    ///I2C1_TX mapped on DMA1 CH4
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_TX_DMA_RMP::MapDma1ch4)
    }
}
/**ADC2 DMA channel remapping bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC2_DMA_RMP {
    ///0: ADC2 mapped on DMA2
    MapDma2 = 0,
    ///2: ADC2 mapped on DMA1 channel 2
    MapDma1ch2 = 2,
    ///3: ADC2 mapped on DMA1 channel 4
    MapDma1ch4 = 3,
}
impl From<ADC2_DMA_RMP> for u8 {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC2_DMA_RMP {
    type Ux = u8;
}
impl crate::IsEnum for ADC2_DMA_RMP {}
///Field `ADC2_DMA_RMP` reader - ADC2 DMA channel remapping bit
pub type ADC2_DMA_RMP_R = crate::FieldReader<ADC2_DMA_RMP>;
impl ADC2_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC2_DMA_RMP> {
        match self.bits {
            0 => Some(ADC2_DMA_RMP::MapDma2),
            2 => Some(ADC2_DMA_RMP::MapDma1ch2),
            3 => Some(ADC2_DMA_RMP::MapDma1ch4),
            _ => None,
        }
    }
    ///ADC2 mapped on DMA2
    #[inline(always)]
    pub fn is_map_dma2(&self) -> bool {
        *self == ADC2_DMA_RMP::MapDma2
    }
    ///ADC2 mapped on DMA1 channel 2
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == ADC2_DMA_RMP::MapDma1ch2
    }
    ///ADC2 mapped on DMA1 channel 4
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == ADC2_DMA_RMP::MapDma1ch4
    }
}
///Field `ADC2_DMA_RMP` writer - ADC2 DMA channel remapping bit
pub type ADC2_DMA_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADC2_DMA_RMP>;
impl<'a, REG> ADC2_DMA_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADC2 mapped on DMA2
    #[inline(always)]
    pub fn map_dma2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC2_DMA_RMP::MapDma2)
    }
    ///ADC2 mapped on DMA1 channel 2
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC2_DMA_RMP::MapDma1ch2)
    }
    ///ADC2 mapped on DMA1 channel 4
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC2_DMA_RMP::MapDma1ch4)
    }
}
impl R {
    ///Bits 0:1 - SPI1_RX DMA remapping bit
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&self) -> SPI1_RX_DMA_RMP_R {
        SPI1_RX_DMA_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SPI1_TX DMA remapping bit
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&self) -> SPI1_TX_DMA_RMP_R {
        SPI1_TX_DMA_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - I2C1_RX DMA remapping bit
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&self) -> I2C1_RX_DMA_RMP_R {
        I2C1_RX_DMA_RMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - I2C1_TX DMA remapping bit
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&self) -> I2C1_TX_DMA_RMP_R {
        I2C1_TX_DMA_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - ADC2 DMA channel remapping bit
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR3")
            .field("spi1_rx_dma_rmp", &self.spi1_rx_dma_rmp())
            .field("spi1_tx_dma_rmp", &self.spi1_tx_dma_rmp())
            .field("i2c1_rx_dma_rmp", &self.i2c1_rx_dma_rmp())
            .field("i2c1_tx_dma_rmp", &self.i2c1_tx_dma_rmp())
            .field("adc2_dma_rmp", &self.adc2_dma_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SPI1_RX DMA remapping bit
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&mut self) -> SPI1_RX_DMA_RMP_W<'_, CFGR3rs> {
        SPI1_RX_DMA_RMP_W::new(self, 0)
    }
    ///Bits 2:3 - SPI1_TX DMA remapping bit
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&mut self) -> SPI1_TX_DMA_RMP_W<'_, CFGR3rs> {
        SPI1_TX_DMA_RMP_W::new(self, 2)
    }
    ///Bits 4:5 - I2C1_RX DMA remapping bit
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&mut self) -> I2C1_RX_DMA_RMP_W<'_, CFGR3rs> {
        I2C1_RX_DMA_RMP_W::new(self, 4)
    }
    ///Bits 6:7 - I2C1_TX DMA remapping bit
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&mut self) -> I2C1_TX_DMA_RMP_W<'_, CFGR3rs> {
        I2C1_TX_DMA_RMP_W::new(self, 6)
    }
    ///Bits 8:9 - ADC2 DMA channel remapping bit
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W<'_, CFGR3rs> {
        ADC2_DMA_RMP_W::new(self, 8)
    }
}
/**SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#SYSCFG:CFGR3)*/
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr3::R`](R) reader structure
impl crate::Readable for CFGR3rs {}
///`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3rs {}
