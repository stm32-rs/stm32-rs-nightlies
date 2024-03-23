#[doc = "Register `C%sCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `C%sCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Input DMA request line selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID {
    #[doc = "0: No signal selected as request input"]
    None = 0,
    #[doc = "1: Signal `dmamux2_req_gen0` selected as request input"]
    Dmamux2ReqGen0 = 1,
    #[doc = "2: Signal `dmamux2_req_gen1` selected as request input"]
    Dmamux2ReqGen1 = 2,
    #[doc = "3: Signal `dmamux2_req_gen2` selected as request input"]
    Dmamux2ReqGen2 = 3,
    #[doc = "4: Signal `dmamux2_req_gen3` selected as request input"]
    Dmamux2ReqGen3 = 4,
    #[doc = "5: Signal `dmamux2_req_gen4` selected as request input"]
    Dmamux2ReqGen4 = 5,
    #[doc = "6: Signal `dmamux2_req_gen5` selected as request input"]
    Dmamux2ReqGen5 = 6,
    #[doc = "7: Signal `dmamux2_req_gen6` selected as request input"]
    Dmamux2ReqGen6 = 7,
    #[doc = "8: Signal `dmamux2_req_gen7` selected as request input"]
    Dmamux2ReqGen7 = 8,
    #[doc = "9: Signal `lpuart1_rx_dma` selected as request input"]
    Lpuart1RxDma = 9,
    #[doc = "10: Signal `lpuart1_tx_dma` selected as request input"]
    Lpuart1TxDma = 10,
    #[doc = "11: Signal `spi6_rx_dma` selected as request input"]
    Spi6RxDma = 11,
    #[doc = "12: Signal `spi6_tx_dma` selected as request input"]
    Spi6TxDma = 12,
    #[doc = "13: Signal `i2c4_rx_dma` selected as request input"]
    I2c4RxDma = 13,
    #[doc = "14: Signal `i2c4_tx_dma` selected as request input"]
    I2c4TxDma = 14,
    #[doc = "15: Signal `sai4_a_dma` selected as request input"]
    Sai4ADma = 15,
    #[doc = "16: Signal `sai4_b_dma` selected as request input"]
    Sai4BDma = 16,
    #[doc = "17: Signal `adc3_dma` selected as request input"]
    Adc3Dma = 17,
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
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DMAREQ_ID_R = crate::FieldReader<DMAREQ_ID>;
impl DMAREQ_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMAREQ_ID> {
        match self.bits {
            0 => Some(DMAREQ_ID::None),
            1 => Some(DMAREQ_ID::Dmamux2ReqGen0),
            2 => Some(DMAREQ_ID::Dmamux2ReqGen1),
            3 => Some(DMAREQ_ID::Dmamux2ReqGen2),
            4 => Some(DMAREQ_ID::Dmamux2ReqGen3),
            5 => Some(DMAREQ_ID::Dmamux2ReqGen4),
            6 => Some(DMAREQ_ID::Dmamux2ReqGen5),
            7 => Some(DMAREQ_ID::Dmamux2ReqGen6),
            8 => Some(DMAREQ_ID::Dmamux2ReqGen7),
            9 => Some(DMAREQ_ID::Lpuart1RxDma),
            10 => Some(DMAREQ_ID::Lpuart1TxDma),
            11 => Some(DMAREQ_ID::Spi6RxDma),
            12 => Some(DMAREQ_ID::Spi6TxDma),
            13 => Some(DMAREQ_ID::I2c4RxDma),
            14 => Some(DMAREQ_ID::I2c4TxDma),
            15 => Some(DMAREQ_ID::Sai4ADma),
            16 => Some(DMAREQ_ID::Sai4BDma),
            17 => Some(DMAREQ_ID::Adc3Dma),
            _ => None,
        }
    }
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID::None
    }
    #[doc = "Signal `dmamux2_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen0(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen0
    }
    #[doc = "Signal `dmamux2_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen1(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen1
    }
    #[doc = "Signal `dmamux2_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen2(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen2
    }
    #[doc = "Signal `dmamux2_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen3(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen3
    }
    #[doc = "Signal `dmamux2_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen4(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen4
    }
    #[doc = "Signal `dmamux2_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen5(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen5
    }
    #[doc = "Signal `dmamux2_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen6(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen6
    }
    #[doc = "Signal `dmamux2_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen7(&self) -> bool {
        *self == DMAREQ_ID::Dmamux2ReqGen7
    }
    #[doc = "Signal `lpuart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Lpuart1RxDma
    }
    #[doc = "Signal `lpuart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Lpuart1TxDma
    }
    #[doc = "Signal `spi6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi6RxDma
    }
    #[doc = "Signal `spi6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi6TxDma
    }
    #[doc = "Signal `i2c4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c4RxDma
    }
    #[doc = "Signal `i2c4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c4TxDma
    }
    #[doc = "Signal `sai4_a_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai4_a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai4ADma
    }
    #[doc = "Signal `sai4_b_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai4_b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai4BDma
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc3Dma
    }
}
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DMAREQ_ID>;
impl<'a, REG> DMAREQ_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::None)
    }
    #[doc = "Signal `dmamux2_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen0)
    }
    #[doc = "Signal `dmamux2_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen1)
    }
    #[doc = "Signal `dmamux2_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen2)
    }
    #[doc = "Signal `dmamux2_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen3)
    }
    #[doc = "Signal `dmamux2_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen4)
    }
    #[doc = "Signal `dmamux2_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen5(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen5)
    }
    #[doc = "Signal `dmamux2_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen6(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen6)
    }
    #[doc = "Signal `dmamux2_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen7(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux2ReqGen7)
    }
    #[doc = "Signal `lpuart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Lpuart1RxDma)
    }
    #[doc = "Signal `lpuart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Lpuart1TxDma)
    }
    #[doc = "Signal `spi6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi6RxDma)
    }
    #[doc = "Signal `spi6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi6TxDma)
    }
    #[doc = "Signal `i2c4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c4RxDma)
    }
    #[doc = "Signal `i2c4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c4TxDma)
    }
    #[doc = "Signal `sai4_a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai4ADma)
    }
    #[doc = "Signal `sai4_b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai4BDma)
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc3Dma)
    }
}
#[doc = "Interrupt enable at synchronization event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    Enabled = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::Disabled,
            true => SOIE::Enabled,
        }
    }
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE::Disabled
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE::Enabled
    }
}
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Disabled)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Enabled)
    }
}
#[doc = "Event generation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    #[doc = "0: Event generation disabled"]
    Disabled = 0,
    #[doc = "1: Event generation enabled"]
    Enabled = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::Disabled,
            true => EGE::Enabled,
        }
    }
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE::Disabled
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE::Enabled
    }
}
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Disabled)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Enabled)
    }
}
#[doc = "Synchronous operating mode enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    #[doc = "0: Synchronization disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization enabled"]
    Enabled = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::Disabled,
            true => SE::Enabled,
        }
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE::Disabled
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE::Enabled
    }
}
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Disabled)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Enabled)
    }
}
#[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
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
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL::NoEdge
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL::BothEdges
    }
}
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPOL>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::BothEdges)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_R = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Synchronization input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID {
    #[doc = "0: Signal `dmamux2_evt0` selected as synchronization input"]
    Dmamux2Evt0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as synchronization input"]
    Dmamux2Evt1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as synchronization input"]
    Dmamux2Evt2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as synchronization input"]
    Dmamux2Evt3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as synchronization input"]
    Dmamux2Evt4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as synchronization input"]
    Dmamux2Evt5 = 5,
    #[doc = "6: Signal `lpuart1_rx_wkup` selected as synchronization input"]
    Lpuart1RxWkup = 6,
    #[doc = "7: Signal `lpuart1_tx_wkup` selected as synchronization input"]
    Lpuart1TxWkup = 7,
    #[doc = "8: Signal `lptim2_out` selected as synchronization input"]
    Lptim2Out = 8,
    #[doc = "9: Signal `lptim3_out` selected as synchronization input"]
    Lptim3Out = 9,
    #[doc = "10: Signal `i2c4_wkup` selected as synchronization input"]
    I2c4Wkup = 10,
    #[doc = "11: Signal `spi6_wkup` selected as synchronization input"]
    Spi6Wkup = 11,
    #[doc = "12: Signal `comp1_out` selected as synchronization input"]
    Comp1Out = 12,
    #[doc = "13: Signal `rtc_wkup` selected as synchronization input"]
    RtcWkup = 13,
    #[doc = "14: Signal `syscfg_exti0_mux` selected as synchronization input"]
    SyscfgExti0Mux = 14,
    #[doc = "15: Signal `syscfg_exti2_mux` selected as synchronization input"]
    SyscfgExti2Mux = 15,
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
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SYNC_ID_R = crate::FieldReader<SYNC_ID>;
impl SYNC_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNC_ID> {
        match self.bits {
            0 => Some(SYNC_ID::Dmamux2Evt0),
            1 => Some(SYNC_ID::Dmamux2Evt1),
            2 => Some(SYNC_ID::Dmamux2Evt2),
            3 => Some(SYNC_ID::Dmamux2Evt3),
            4 => Some(SYNC_ID::Dmamux2Evt4),
            5 => Some(SYNC_ID::Dmamux2Evt5),
            6 => Some(SYNC_ID::Lpuart1RxWkup),
            7 => Some(SYNC_ID::Lpuart1TxWkup),
            8 => Some(SYNC_ID::Lptim2Out),
            9 => Some(SYNC_ID::Lptim3Out),
            10 => Some(SYNC_ID::I2c4Wkup),
            11 => Some(SYNC_ID::Spi6Wkup),
            12 => Some(SYNC_ID::Comp1Out),
            13 => Some(SYNC_ID::RtcWkup),
            14 => Some(SYNC_ID::SyscfgExti0Mux),
            15 => Some(SYNC_ID::SyscfgExti2Mux),
            _ => None,
        }
    }
    #[doc = "Signal `dmamux2_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt0
    }
    #[doc = "Signal `dmamux2_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt1
    }
    #[doc = "Signal `dmamux2_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt2
    }
    #[doc = "Signal `dmamux2_evt3` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt3
    }
    #[doc = "Signal `dmamux2_evt4` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt4
    }
    #[doc = "Signal `dmamux2_evt5` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SYNC_ID::Dmamux2Evt5
    }
    #[doc = "Signal `lpuart1_rx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lpuart1_rx_wkup(&self) -> bool {
        *self == SYNC_ID::Lpuart1RxWkup
    }
    #[doc = "Signal `lpuart1_tx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lpuart1_tx_wkup(&self) -> bool {
        *self == SYNC_ID::Lpuart1TxWkup
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID::Lptim2Out
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID::Lptim3Out
    }
    #[doc = "Signal `i2c4_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SYNC_ID::I2c4Wkup
    }
    #[doc = "Signal `spi6_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SYNC_ID::Spi6Wkup
    }
    #[doc = "Signal `comp1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SYNC_ID::Comp1Out
    }
    #[doc = "Signal `rtc_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SYNC_ID::RtcWkup
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SYNC_ID::SyscfgExti0Mux
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SYNC_ID::SyscfgExti2Mux
    }
}
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SYNC_ID>;
impl<'a, REG> SYNC_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal `dmamux2_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux2Evt5)
    }
    #[doc = "Signal `lpuart1_rx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_rx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lpuart1RxWkup)
    }
    #[doc = "Signal `lpuart1_tx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_tx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lpuart1TxWkup)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim3Out)
    }
    #[doc = "Signal `i2c4_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::I2c4Wkup)
    }
    #[doc = "Signal `spi6_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Spi6Wkup)
    }
    #[doc = "Signal `comp1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Comp1Out)
    }
    #[doc = "Signal `rtc_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::RtcWkup)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::SyscfgExti0Mux)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::SyscfgExti2Mux)
    }
}
impl R {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<CCRrs> {
        SOIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<CCRrs> {
        EGE_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<CCRrs> {
        SE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<CCRrs> {
        SPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<CCRrs> {
        NBREQ_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
