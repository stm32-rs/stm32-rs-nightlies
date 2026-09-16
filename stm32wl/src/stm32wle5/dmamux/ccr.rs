///Register `CCR%s` reader
pub type R = crate::R<CCRrs>;
///Register `CCR%s` writer
pub type W = crate::W<CCRrs>;
/**DMA request identification

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID {
    ///0: No signal selected as request input
    None = 0,
    ///1: Signal `dmamux1_req_gen0` selected as request input
    Dmamux1ReqGen0 = 1,
    ///2: Signal `dmamux1_req_gen1` selected as request input
    Dmamux1ReqGen1 = 2,
    ///3: Signal `dmamux1_req_gen2` selected as request input
    Dmamux1ReqGen2 = 3,
    ///4: Signal `dmamux1_req_gen3` selected as request input
    Dmamux1ReqGen3 = 4,
    ///5: Signal `adc1_dma` selected as request input
    Adc = 5,
    ///6: Signal `dac_out1_dma` selected as request input
    DatOut1 = 6,
    ///7: Signal `spi1_rx_dma` selected as request input
    Spi1RxDma = 7,
    ///8: Signal `spi1_tx_dma` selected as request input
    Spi1TxDma = 8,
    ///9: Signal `spi2_rx_dma` selected as request input
    Spi2RxDma = 9,
    ///10: Signal `spi2_tx_dma` selected as request input
    Spi2TxDma = 10,
    ///11: Signal `i2c1_rx_dma` selected as request input
    I2c1RxDma = 11,
    ///12: Signal `i2c1_tx_dma` selected as request input
    I2c1TxDma = 12,
    ///13: Signal `i2c2_rx_dma` selected as request input
    I2c2RxDma = 13,
    ///14: Signal `i2c2_tx_dma` selected as request input
    I2c2TxDma = 14,
    ///15: Signal `i2c3_rx_dma` selected as request input
    I2c3RxDma = 15,
    ///16: Signal `i2c3_tx_dma` selected as request input
    I2c3TxDma = 16,
    ///17: Signal `usart1_rx_dma` selected as request input
    Usart1RxDma = 17,
    ///18: Signal `usart1_tx_dma` selected as request input
    Usart1TxDma = 18,
    ///19: Signal `usart2_rx_dma` selected as request input
    Usart2RxDma = 19,
    ///20: Signal `usart2_tx_dma` selected as request input
    Usart2TxDma = 20,
    ///21: Signal `lpuart1_rx_dma` selected as request input
    Lpuart1RxDma = 21,
    ///22: Signal `lpuart1_tx_dma` selected as request input
    Lpuart1TxDma = 22,
    ///23: Signal `tim1_ch1` selected as request input
    Tim1Ch1 = 23,
    ///24: Signal `tim1_ch2` selected as request input
    Tim1Ch2 = 24,
    ///25: Signal `tim1_ch3` selected as request input
    Tim1Ch3 = 25,
    ///26: Signal `tim1_ch4` selected as request input
    Tim1Ch4 = 26,
    ///27: Signal `tim1_up` selected as request input
    Tim1Up = 27,
    ///28: Signal `tim1_trig` selected as request input
    Tim1Trig = 28,
    ///29: Signal `tim1_com` selected as request input
    Tim1Com = 29,
    ///30: Signal `tim2_ch1` selected as request input
    Tim2Ch1 = 30,
    ///31: Signal `tim2_ch2` selected as request input
    Tim2Ch2 = 31,
    ///32: Signal `tim2_ch3` selected as request input
    Tim2Ch3 = 32,
    ///33: Signal `tim2_ch4` selected as request input
    Tim2Ch4 = 33,
    ///34: Signal `tim2_up` selected as request input
    Tim2Up = 34,
    ///35: Signal `tim16_ch1` selected as request input
    Tim16Ch1 = 35,
    ///36: Signal `tim16_up` selected as request input
    Tim16Up = 36,
    ///37: Signal `tim17_ch1` selected as request input
    Tim17Ch1 = 37,
    ///38: Signal `tim17_up` selected as request input
    Tim17Up = 38,
    ///39: Signal `aes_in` selected as request input
    AesIn = 39,
    ///40: Signal `aes_out` selected as request input
    AesOut = 40,
    ///41: Signal `subghzspi_rx` selected as request input
    SubghzspiRx = 41,
    ///42: Signal `subghzspi_tx` selected as request input
    SubghzspiTx = 42,
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
///Field `DMAREQ_ID` reader - DMA request identification
pub type DMAREQ_ID_R = crate::FieldReader<DMAREQ_ID>;
impl DMAREQ_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMAREQ_ID> {
        match self.bits {
            0 => Some(DMAREQ_ID::None),
            1 => Some(DMAREQ_ID::Dmamux1ReqGen0),
            2 => Some(DMAREQ_ID::Dmamux1ReqGen1),
            3 => Some(DMAREQ_ID::Dmamux1ReqGen2),
            4 => Some(DMAREQ_ID::Dmamux1ReqGen3),
            5 => Some(DMAREQ_ID::Adc),
            6 => Some(DMAREQ_ID::DatOut1),
            7 => Some(DMAREQ_ID::Spi1RxDma),
            8 => Some(DMAREQ_ID::Spi1TxDma),
            9 => Some(DMAREQ_ID::Spi2RxDma),
            10 => Some(DMAREQ_ID::Spi2TxDma),
            11 => Some(DMAREQ_ID::I2c1RxDma),
            12 => Some(DMAREQ_ID::I2c1TxDma),
            13 => Some(DMAREQ_ID::I2c2RxDma),
            14 => Some(DMAREQ_ID::I2c2TxDma),
            15 => Some(DMAREQ_ID::I2c3RxDma),
            16 => Some(DMAREQ_ID::I2c3TxDma),
            17 => Some(DMAREQ_ID::Usart1RxDma),
            18 => Some(DMAREQ_ID::Usart1TxDma),
            19 => Some(DMAREQ_ID::Usart2RxDma),
            20 => Some(DMAREQ_ID::Usart2TxDma),
            21 => Some(DMAREQ_ID::Lpuart1RxDma),
            22 => Some(DMAREQ_ID::Lpuart1TxDma),
            23 => Some(DMAREQ_ID::Tim1Ch1),
            24 => Some(DMAREQ_ID::Tim1Ch2),
            25 => Some(DMAREQ_ID::Tim1Ch3),
            26 => Some(DMAREQ_ID::Tim1Ch4),
            27 => Some(DMAREQ_ID::Tim1Up),
            28 => Some(DMAREQ_ID::Tim1Trig),
            29 => Some(DMAREQ_ID::Tim1Com),
            30 => Some(DMAREQ_ID::Tim2Ch1),
            31 => Some(DMAREQ_ID::Tim2Ch2),
            32 => Some(DMAREQ_ID::Tim2Ch3),
            33 => Some(DMAREQ_ID::Tim2Ch4),
            34 => Some(DMAREQ_ID::Tim2Up),
            35 => Some(DMAREQ_ID::Tim16Ch1),
            36 => Some(DMAREQ_ID::Tim16Up),
            37 => Some(DMAREQ_ID::Tim17Ch1),
            38 => Some(DMAREQ_ID::Tim17Up),
            39 => Some(DMAREQ_ID::AesIn),
            40 => Some(DMAREQ_ID::AesOut),
            41 => Some(DMAREQ_ID::SubghzspiRx),
            42 => Some(DMAREQ_ID::SubghzspiTx),
            _ => None,
        }
    }
    ///No signal selected as request input
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID::None
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen0
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen1
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen2
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen3
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DMAREQ_ID::Adc
    }
    ///Signal `dac_out1_dma` selected as request input
    #[inline(always)]
    pub fn is_dat_out1(&self) -> bool {
        *self == DMAREQ_ID::DatOut1
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1RxDma
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1TxDma
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2RxDma
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2TxDma
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1RxDma
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1TxDma
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2RxDma
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2TxDma
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3RxDma
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3TxDma
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1RxDma
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1TxDma
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2RxDma
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2TxDma
    }
    ///Signal `lpuart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Lpuart1RxDma
    }
    ///Signal `lpuart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Lpuart1TxDma
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch1
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch2
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch3
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch4
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID::Tim1Up
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim1Trig
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID::Tim1Com
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch1
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch2
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch3
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch4
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID::Tim2Up
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim16Ch1
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID::Tim16Up
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim17Ch1
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID::Tim17Up
    }
    ///Signal `aes_in` selected as request input
    #[inline(always)]
    pub fn is_aes_in(&self) -> bool {
        *self == DMAREQ_ID::AesIn
    }
    ///Signal `aes_out` selected as request input
    #[inline(always)]
    pub fn is_aes_out(&self) -> bool {
        *self == DMAREQ_ID::AesOut
    }
    ///Signal `subghzspi_rx` selected as request input
    #[inline(always)]
    pub fn is_subghzspi_rx(&self) -> bool {
        *self == DMAREQ_ID::SubghzspiRx
    }
    ///Signal `subghzspi_tx` selected as request input
    #[inline(always)]
    pub fn is_subghzspi_tx(&self) -> bool {
        *self == DMAREQ_ID::SubghzspiTx
    }
}
///Field `DMAREQ_ID` writer - DMA request identification
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DMAREQ_ID>;
impl<'a, REG> DMAREQ_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No signal selected as request input
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::None)
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen0)
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen1)
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen2)
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen3)
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc)
    }
    ///Signal `dac_out1_dma` selected as request input
    #[inline(always)]
    pub fn dat_out1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DatOut1)
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1RxDma)
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1TxDma)
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2RxDma)
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2TxDma)
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1RxDma)
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1TxDma)
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2RxDma)
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2TxDma)
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3RxDma)
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3TxDma)
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1RxDma)
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1TxDma)
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2RxDma)
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2TxDma)
    }
    ///Signal `lpuart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Lpuart1RxDma)
    }
    ///Signal `lpuart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Lpuart1TxDma)
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch1)
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch2)
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch3)
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch4)
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Up)
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Trig)
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Com)
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch1)
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch2)
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch3)
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch4)
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Up)
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Ch1)
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Up)
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Ch1)
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Up)
    }
    ///Signal `aes_in` selected as request input
    #[inline(always)]
    pub fn aes_in(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::AesIn)
    }
    ///Signal `aes_out` selected as request input
    #[inline(always)]
    pub fn aes_out(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::AesOut)
    }
    ///Signal `subghzspi_rx` selected as request input
    #[inline(always)]
    pub fn subghzspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SubghzspiRx)
    }
    ///Signal `subghzspi_tx` selected as request input
    #[inline(always)]
    pub fn subghzspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SubghzspiTx)
    }
}
/**Synchronization overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    ///0: Synchronization overrun interrupt disabled
    Disabled = 0,
    ///1: Synchronization overrun interrupt enabled
    Enabled = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::Disabled,
            true => SOIE::Enabled,
        }
    }
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE::Disabled
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE::Enabled
    }
}
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Disabled)
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Enabled)
    }
}
/**Event generation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    ///0: Event generation disabled
    Disabled = 0,
    ///1: Event generation enabled
    Enabled = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
///Field `EGE` reader - Event generation enable
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::Disabled,
            true => EGE::Enabled,
        }
    }
    ///Event generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE::Disabled
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE::Enabled
    }
}
///Field `EGE` writer - Event generation enable
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Disabled)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Enabled)
    }
}
/**Synchronization enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    ///0: Synchronization disabled
    Disabled = 0,
    ///1: Synchronization enabled
    Enabled = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
///Field `SE` reader - Synchronization enable
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::Disabled,
            true => SE::Enabled,
        }
    }
    ///Synchronization disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE::Disabled
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE::Enabled
    }
}
///Field `SE` writer - Synchronization enable
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Disabled)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Enabled)
    }
}
/**Synchronization polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    ///0: No event, i.e. no synchronization nor detection
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
    BothEdges = 3,
}
impl From<SPOL> for u8 {
    #[inline(always)]
    fn from(variant: SPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPOL {
    type Ux = u8;
}
impl crate::IsEnum for SPOL {}
///Field `SPOL` reader - Synchronization polarity
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPOL {
        match self.bits {
            0 => SPOL::NoEdge,
            1 => SPOL::RisingEdge,
            2 => SPOL::FallingEdge,
            3 => SPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL::BothEdges
    }
}
///Field `SPOL` writer - Synchronization polarity
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPOL, crate::Safe>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::BothEdges)
    }
}
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Synchronization identification

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID {
    ///0: Signal `EXTIx` selected as synchronization input
    Exti0 = 0,
    ///1: Signal `EXTIx` selected as synchronization input
    Exti1 = 1,
    ///2: Signal `EXTIx` selected as synchronization input
    Exti2 = 2,
    ///3: Signal `EXTIx` selected as synchronization input
    Exti3 = 3,
    ///4: Signal `EXTIx` selected as synchronization input
    Exti4 = 4,
    ///5: Signal `EXTIx` selected as synchronization input
    Exti5 = 5,
    ///6: Signal `EXTIx` selected as synchronization input
    Exti6 = 6,
    ///7: Signal `EXTIx` selected as synchronization input
    Exti7 = 7,
    ///8: Signal `EXTIx` selected as synchronization input
    Exti8 = 8,
    ///9: Signal `EXTIx` selected as synchronization input
    Exti9 = 9,
    ///10: Signal `EXTIx` selected as synchronization input
    Exti10 = 10,
    ///11: Signal `EXTIx` selected as synchronization input
    Exti11 = 11,
    ///12: Signal `EXTIx` selected as synchronization input
    Exti12 = 12,
    ///13: Signal `EXTIx` selected as synchronization input
    Exti13 = 13,
    ///14: Signal `EXTIx` selected as synchronization input
    Exti14 = 14,
    ///15: Signal `EXTIx` selected as synchronization input
    Exti15 = 15,
    ///16: Signal `dmamux1_evt0` selected as synchronization input
    Dmamux1Evt0 = 16,
    ///17: Signal `dmamux1_evt1` selected as synchronization input
    Dmamux1Evt1 = 17,
    ///18: Signal `lptim1_out` selected as synchronization input
    Lptim1Out = 18,
    ///19: Signal `lptim2_out` selected as synchronization input
    Lptim2Out = 19,
    ///20: Signal `lptim3_out` selected as synchronization input
    Lptim3Out = 20,
}
impl From<SYNC_ID> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNC_ID {
    type Ux = u8;
}
impl crate::IsEnum for SYNC_ID {}
///Field `SYNC_ID` reader - Synchronization identification
pub type SYNC_ID_R = crate::FieldReader<SYNC_ID>;
impl SYNC_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNC_ID> {
        match self.bits {
            0 => Some(SYNC_ID::Exti0),
            1 => Some(SYNC_ID::Exti1),
            2 => Some(SYNC_ID::Exti2),
            3 => Some(SYNC_ID::Exti3),
            4 => Some(SYNC_ID::Exti4),
            5 => Some(SYNC_ID::Exti5),
            6 => Some(SYNC_ID::Exti6),
            7 => Some(SYNC_ID::Exti7),
            8 => Some(SYNC_ID::Exti8),
            9 => Some(SYNC_ID::Exti9),
            10 => Some(SYNC_ID::Exti10),
            11 => Some(SYNC_ID::Exti11),
            12 => Some(SYNC_ID::Exti12),
            13 => Some(SYNC_ID::Exti13),
            14 => Some(SYNC_ID::Exti14),
            15 => Some(SYNC_ID::Exti15),
            16 => Some(SYNC_ID::Dmamux1Evt0),
            17 => Some(SYNC_ID::Dmamux1Evt1),
            18 => Some(SYNC_ID::Lptim1Out),
            19 => Some(SYNC_ID::Lptim2Out),
            20 => Some(SYNC_ID::Lptim3Out),
            _ => None,
        }
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == SYNC_ID::Exti0
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == SYNC_ID::Exti1
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == SYNC_ID::Exti2
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == SYNC_ID::Exti3
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == SYNC_ID::Exti4
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == SYNC_ID::Exti5
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == SYNC_ID::Exti6
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == SYNC_ID::Exti7
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        *self == SYNC_ID::Exti8
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == SYNC_ID::Exti9
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == SYNC_ID::Exti10
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == SYNC_ID::Exti11
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        *self == SYNC_ID::Exti12
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        *self == SYNC_ID::Exti13
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        *self == SYNC_ID::Exti14
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == SYNC_ID::Exti15
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt0
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt1
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID::Lptim1Out
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID::Lptim2Out
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID::Lptim3Out
    }
}
///Field `SYNC_ID` writer - Synchronization identification
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SYNC_ID>;
impl<'a, REG> SYNC_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti0)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti1)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti2)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti3(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti3)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti4)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti5(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti5)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti6(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti6)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti7(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti7)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti8(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti8)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti9)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti10(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti10)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti11)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti12(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti12)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti13(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti13)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti14(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti14)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Exti15)
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt1)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim3Out)
    }
}
impl R {
    ///Bits 0:7 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("sync_id", &self.sync_id())
            .field("nbreq", &self.nbreq())
            .field("spol", &self.spol())
            .field("se", &self.se())
            .field("ege", &self.ege())
            .field("soie", &self.soie())
            .field("dmareq_id", &self.dmareq_id())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<'_, CCRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<'_, CCRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<'_, CCRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<'_, CCRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<'_, CCRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:28 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<'_, CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**DMA Multiplexer Channel %s Control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#DMAMUX:CCR[0])*/
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
///`reset()` method sets CCR%s to value 0
impl crate::Resettable for CCRrs {}
