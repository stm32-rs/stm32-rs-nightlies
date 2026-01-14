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
    ///8: I2C1_RX
    I2c1rx = 8,
    ///9: I2C1_TX
    I2c1tx = 9,
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
    ///18: TIM2_CH1
    Tim2ch1 = 18,
    ///19: TIM2_CH2
    Tim2ch2 = 19,
    ///20: TIM2_CH3
    Tim2ch3 = 20,
    ///21: TIM2_CH4
    Tim2ch4 = 21,
    ///22: TIM2_UP
    Tim2up = 22,
    ///23: TIM16_CH1
    Tim16ch1 = 23,
    ///24: TIM16_UP
    Tim16up = 24,
    ///25: TIM17_CH1
    Tim17ch1 = 25,
    ///26: TIM17_UP
    Tim17up = 26,
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
            8 => Some(DMAREQ_ID::I2c1rx),
            9 => Some(DMAREQ_ID::I2c1tx),
            12 => Some(DMAREQ_ID::UsartRx),
            13 => Some(DMAREQ_ID::UsartTx),
            14 => Some(DMAREQ_ID::LpuartRx),
            15 => Some(DMAREQ_ID::LpuartTx),
            16 => Some(DMAREQ_ID::AdcCh0),
            18 => Some(DMAREQ_ID::Tim2ch1),
            19 => Some(DMAREQ_ID::Tim2ch2),
            20 => Some(DMAREQ_ID::Tim2ch3),
            21 => Some(DMAREQ_ID::Tim2ch4),
            22 => Some(DMAREQ_ID::Tim2up),
            23 => Some(DMAREQ_ID::Tim16ch1),
            24 => Some(DMAREQ_ID::Tim16up),
            25 => Some(DMAREQ_ID::Tim17ch1),
            26 => Some(DMAREQ_ID::Tim17up),
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
    ///TIM2_CH1
    #[inline(always)]
    pub fn is_tim2ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim2ch1
    }
    ///TIM2_CH2
    #[inline(always)]
    pub fn is_tim2ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim2ch2
    }
    ///TIM2_CH3
    #[inline(always)]
    pub fn is_tim2ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim2ch3
    }
    ///TIM2_CH4
    #[inline(always)]
    pub fn is_tim2ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim2ch4
    }
    ///TIM2_UP
    #[inline(always)]
    pub fn is_tim2up(&self) -> bool {
        *self == DMAREQ_ID::Tim2up
    }
    ///TIM16_CH1
    #[inline(always)]
    pub fn is_tim16ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim16ch1
    }
    ///TIM16_UP
    #[inline(always)]
    pub fn is_tim16up(&self) -> bool {
        *self == DMAREQ_ID::Tim16up
    }
    ///TIM17_CH1
    #[inline(always)]
    pub fn is_tim17ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim17ch1
    }
    ///TIM17_UP
    #[inline(always)]
    pub fn is_tim17up(&self) -> bool {
        *self == DMAREQ_ID::Tim17up
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
    ///TIM2_CH1
    #[inline(always)]
    pub fn tim2ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2ch1)
    }
    ///TIM2_CH2
    #[inline(always)]
    pub fn tim2ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2ch2)
    }
    ///TIM2_CH3
    #[inline(always)]
    pub fn tim2ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2ch3)
    }
    ///TIM2_CH4
    #[inline(always)]
    pub fn tim2ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2ch4)
    }
    ///TIM2_UP
    #[inline(always)]
    pub fn tim2up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2up)
    }
    ///TIM16_CH1
    #[inline(always)]
    pub fn tim16ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16ch1)
    }
    ///TIM16_UP
    #[inline(always)]
    pub fn tim16up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16up)
    }
    ///TIM17_CH1
    #[inline(always)]
    pub fn tim17ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17ch1)
    }
    ///TIM17_UP
    #[inline(always)]
    pub fn tim17up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17up)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#DMAMUX:C[0]CR)*/
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
