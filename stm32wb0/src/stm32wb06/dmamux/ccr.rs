///Register `C%sCR` reader
pub type R = crate::R<CCRrs>;
///Register `C%sCR` writer
pub type W = crate::W<CCRrs>;
/**DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID {
    ///2: SPI3_RX
    Spi3rx = 2,
    ///3: SPI3_TX
    Spi3tx = 3,
    ///4: SPI1_RX
    Spi1rx = 4,
    ///5: SPI1_TX
    Spi1tx = 5,
    ///6: SPI2_RX
    Spi2rx = 6,
    ///7: SPI2_TX
    Spi2tx = 7,
    ///8: I2C1_RX
    I2c1rx = 8,
    ///9: I2C1_TX
    I2c1tx = 9,
    ///10: I2C2_RX
    I2c2rx = 10,
    ///11: I2C2_TX
    I2c2tx = 11,
    ///12: USART_RX
    UsartRx = 12,
    ///13: USART_TX
    UsartTx = 13,
    ///14: LPUART_RX
    LpuartRx = 14,
    ///15: LPUART_TX
    LpuartTx = 15,
    ///16: ADC_CH0 (DS channel)
    AdcCh0 = 16,
    ///17: ADC_CH1 (DF channel)
    AdcCh1 = 17,
}
impl From<DMAREQ_ID> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMAREQ_ID {
    type Ux = u8;
}
impl crate::IsEnum for DMAREQ_ID {}
///Field `DMAREQ_ID` reader - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_R = crate::FieldReader<DMAREQ_ID>;
impl DMAREQ_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMAREQ_ID> {
        match self.bits {
            2 => Some(DMAREQ_ID::Spi3rx),
            3 => Some(DMAREQ_ID::Spi3tx),
            4 => Some(DMAREQ_ID::Spi1rx),
            5 => Some(DMAREQ_ID::Spi1tx),
            6 => Some(DMAREQ_ID::Spi2rx),
            7 => Some(DMAREQ_ID::Spi2tx),
            8 => Some(DMAREQ_ID::I2c1rx),
            9 => Some(DMAREQ_ID::I2c1tx),
            10 => Some(DMAREQ_ID::I2c2rx),
            11 => Some(DMAREQ_ID::I2c2tx),
            12 => Some(DMAREQ_ID::UsartRx),
            13 => Some(DMAREQ_ID::UsartTx),
            14 => Some(DMAREQ_ID::LpuartRx),
            15 => Some(DMAREQ_ID::LpuartTx),
            16 => Some(DMAREQ_ID::AdcCh0),
            17 => Some(DMAREQ_ID::AdcCh1),
            _ => None,
        }
    }
    ///SPI3_RX
    #[inline(always)]
    pub fn is_spi3rx(&self) -> bool {
        *self == DMAREQ_ID::Spi3rx
    }
    ///SPI3_TX
    #[inline(always)]
    pub fn is_spi3tx(&self) -> bool {
        *self == DMAREQ_ID::Spi3tx
    }
    ///SPI1_RX
    #[inline(always)]
    pub fn is_spi1rx(&self) -> bool {
        *self == DMAREQ_ID::Spi1rx
    }
    ///SPI1_TX
    #[inline(always)]
    pub fn is_spi1tx(&self) -> bool {
        *self == DMAREQ_ID::Spi1tx
    }
    ///SPI2_RX
    #[inline(always)]
    pub fn is_spi2rx(&self) -> bool {
        *self == DMAREQ_ID::Spi2rx
    }
    ///SPI2_TX
    #[inline(always)]
    pub fn is_spi2tx(&self) -> bool {
        *self == DMAREQ_ID::Spi2tx
    }
    ///I2C1_RX
    #[inline(always)]
    pub fn is_i2c1rx(&self) -> bool {
        *self == DMAREQ_ID::I2c1rx
    }
    ///I2C1_TX
    #[inline(always)]
    pub fn is_i2c1tx(&self) -> bool {
        *self == DMAREQ_ID::I2c1tx
    }
    ///I2C2_RX
    #[inline(always)]
    pub fn is_i2c2rx(&self) -> bool {
        *self == DMAREQ_ID::I2c2rx
    }
    ///I2C2_TX
    #[inline(always)]
    pub fn is_i2c2tx(&self) -> bool {
        *self == DMAREQ_ID::I2c2tx
    }
    ///USART_RX
    #[inline(always)]
    pub fn is_usart_rx(&self) -> bool {
        *self == DMAREQ_ID::UsartRx
    }
    ///USART_TX
    #[inline(always)]
    pub fn is_usart_tx(&self) -> bool {
        *self == DMAREQ_ID::UsartTx
    }
    ///LPUART_RX
    #[inline(always)]
    pub fn is_lpuart_rx(&self) -> bool {
        *self == DMAREQ_ID::LpuartRx
    }
    ///LPUART_TX
    #[inline(always)]
    pub fn is_lpuart_tx(&self) -> bool {
        *self == DMAREQ_ID::LpuartTx
    }
    ///ADC_CH0 (DS channel)
    #[inline(always)]
    pub fn is_adc_ch0(&self) -> bool {
        *self == DMAREQ_ID::AdcCh0
    }
    ///ADC_CH1 (DF channel)
    #[inline(always)]
    pub fn is_adc_ch1(&self) -> bool {
        *self == DMAREQ_ID::AdcCh1
    }
}
///Field `DMAREQ_ID` writer - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DMAREQ_ID>;
impl<'a, REG> DMAREQ_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI3_RX
    #[inline(always)]
    pub fn spi3rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3rx)
    }
    ///SPI3_TX
    #[inline(always)]
    pub fn spi3tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3tx)
    }
    ///SPI1_RX
    #[inline(always)]
    pub fn spi1rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1rx)
    }
    ///SPI1_TX
    #[inline(always)]
    pub fn spi1tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1tx)
    }
    ///SPI2_RX
    #[inline(always)]
    pub fn spi2rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2rx)
    }
    ///SPI2_TX
    #[inline(always)]
    pub fn spi2tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2tx)
    }
    ///I2C1_RX
    #[inline(always)]
    pub fn i2c1rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1rx)
    }
    ///I2C1_TX
    #[inline(always)]
    pub fn i2c1tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1tx)
    }
    ///I2C2_RX
    #[inline(always)]
    pub fn i2c2rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2rx)
    }
    ///I2C2_TX
    #[inline(always)]
    pub fn i2c2tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2tx)
    }
    ///USART_RX
    #[inline(always)]
    pub fn usart_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::UsartRx)
    }
    ///USART_TX
    #[inline(always)]
    pub fn usart_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::UsartTx)
    }
    ///LPUART_RX
    #[inline(always)]
    pub fn lpuart_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::LpuartRx)
    }
    ///LPUART_TX
    #[inline(always)]
    pub fn lpuart_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::LpuartTx)
    }
    ///ADC_CH0 (DS channel)
    #[inline(always)]
    pub fn adc_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::AdcCh0)
    }
    ///ADC_CH1 (DF channel)
    #[inline(always)]
    pub fn adc_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::AdcCh1)
    }
}
impl R {
    ///Bits 0:4 - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dmareq_id", &self.dmareq_id())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMAREQ_ID\[4:0\]: DMA REQuest IDentification Selects the input DMA request. C.f. the DMAMUX table about assignments of multiplexer inputs to resources.
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
}
/**CxCR register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#DMAMUX:C[0]CR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C%sCR to value 0
impl crate::Resettable for CCRrs {}
