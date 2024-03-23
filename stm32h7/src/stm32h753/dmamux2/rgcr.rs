#[doc = "Register `RG%sCR` reader"]
pub type R = crate::R<RGCRrs>;
#[doc = "Register `RG%sCR` writer"]
pub type W = crate::W<RGCRrs>;
#[doc = "DMA request trigger input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID {
    #[doc = "0: Signal `dmamux2_evt0` selected as trigger input"]
    Dmamux2Evt0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as trigger input"]
    Dmamux2Evt1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as trigger input"]
    Dmamux2Evt2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as trigger input"]
    Dmamux2Evt3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as trigger input"]
    Dmamux2Evt4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as trigger input"]
    Dmamux2Evt5 = 5,
    #[doc = "6: Signal `dmamux2_evt6` selected as trigger input"]
    Dmamux2Evt6 = 6,
    #[doc = "7: Signal `lpuart_rx_wkup` selected as trigger input"]
    LpuartRxWkup = 7,
    #[doc = "8: Signal `lpuart_tx_wkup` selected as trigger input"]
    LpuartTxWkup = 8,
    #[doc = "9: Signal `lptim2_wkup` selected as trigger input"]
    Lptim2Wkup = 9,
    #[doc = "10: Signal `lptim2_out` selected as trigger input"]
    Lptim2Out = 10,
    #[doc = "11: Signal `lptim3_wkup` selected as trigger input"]
    Lptim3Wkup = 11,
    #[doc = "12: Signal `lptim3_out` selected as trigger input"]
    Lptim3Out = 12,
    #[doc = "13: Signal `lptim4_ait` selected as trigger input"]
    Lptim4Ait = 13,
    #[doc = "14: Signal `lptim5_ait` selected as trigger input"]
    Lptim5Ait = 14,
    #[doc = "15: Signal `i2c4_wkup` selected as trigger input"]
    I2c4Wkup = 15,
    #[doc = "16: Signal `spi6_wkup` selected as trigger input"]
    Spi6Wkup = 16,
    #[doc = "17: Signal `comp1_out` selected as trigger input"]
    Comp1Out = 17,
    #[doc = "18: Signal `comp2_out` selected as trigger input"]
    Comp2Out = 18,
    #[doc = "19: Signal `rtc_wkup` selected as trigger input"]
    RtcWkup = 19,
    #[doc = "20: Signal `syscfg_exti0_mux` selected as trigger input"]
    SyscfgExti0Mux = 20,
    #[doc = "21: Signal `syscfg_exti2_mux` selected as trigger input"]
    SyscfgExti2Mux = 21,
    #[doc = "22: Signal `i2c4_event_it` selected as trigger input"]
    I2c4EventIt = 22,
    #[doc = "23: Signal `spi6_it` selected as trigger input"]
    Spi6It = 23,
    #[doc = "24: Signal `lpuart1_it_t` selected as trigger input"]
    Lpuart1ItT = 24,
    #[doc = "25: Signal `lpuart1_it_r` selected as trigger input"]
    Lpuart1ItR = 25,
    #[doc = "26: Signal `adc3_it` selected as trigger input"]
    Adc3It = 26,
    #[doc = "27: Signal `adc3_awd1` selected as trigger input"]
    Adc3Awd1 = 27,
    #[doc = "28: Signal `bdma_ch0_it` selected as trigger input"]
    BdmaCh0It = 28,
    #[doc = "29: Signal `bdma_ch1_it` selected as trigger input"]
    BdmaCh1It = 29,
}
impl From<SIG_ID> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIG_ID {
    type Ux = u8;
}
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SIG_ID_R = crate::FieldReader<SIG_ID>;
impl SIG_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIG_ID> {
        match self.bits {
            0 => Some(SIG_ID::Dmamux2Evt0),
            1 => Some(SIG_ID::Dmamux2Evt1),
            2 => Some(SIG_ID::Dmamux2Evt2),
            3 => Some(SIG_ID::Dmamux2Evt3),
            4 => Some(SIG_ID::Dmamux2Evt4),
            5 => Some(SIG_ID::Dmamux2Evt5),
            6 => Some(SIG_ID::Dmamux2Evt6),
            7 => Some(SIG_ID::LpuartRxWkup),
            8 => Some(SIG_ID::LpuartTxWkup),
            9 => Some(SIG_ID::Lptim2Wkup),
            10 => Some(SIG_ID::Lptim2Out),
            11 => Some(SIG_ID::Lptim3Wkup),
            12 => Some(SIG_ID::Lptim3Out),
            13 => Some(SIG_ID::Lptim4Ait),
            14 => Some(SIG_ID::Lptim5Ait),
            15 => Some(SIG_ID::I2c4Wkup),
            16 => Some(SIG_ID::Spi6Wkup),
            17 => Some(SIG_ID::Comp1Out),
            18 => Some(SIG_ID::Comp2Out),
            19 => Some(SIG_ID::RtcWkup),
            20 => Some(SIG_ID::SyscfgExti0Mux),
            21 => Some(SIG_ID::SyscfgExti2Mux),
            22 => Some(SIG_ID::I2c4EventIt),
            23 => Some(SIG_ID::Spi6It),
            24 => Some(SIG_ID::Lpuart1ItT),
            25 => Some(SIG_ID::Lpuart1ItR),
            26 => Some(SIG_ID::Adc3It),
            27 => Some(SIG_ID::Adc3Awd1),
            28 => Some(SIG_ID::BdmaCh0It),
            29 => Some(SIG_ID::BdmaCh1It),
            _ => None,
        }
    }
    #[doc = "Signal `dmamux2_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt0
    }
    #[doc = "Signal `dmamux2_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt1
    }
    #[doc = "Signal `dmamux2_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt2
    }
    #[doc = "Signal `dmamux2_evt3` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt3
    }
    #[doc = "Signal `dmamux2_evt4` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt4
    }
    #[doc = "Signal `dmamux2_evt5` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt5
    }
    #[doc = "Signal `dmamux2_evt6` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux2_evt6(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt6
    }
    #[doc = "Signal `lpuart_rx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_lpuart_rx_wkup(&self) -> bool {
        *self == SIG_ID::LpuartRxWkup
    }
    #[doc = "Signal `lpuart_tx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_lpuart_tx_wkup(&self) -> bool {
        *self == SIG_ID::LpuartTxWkup
    }
    #[doc = "Signal `lptim2_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim2_wkup(&self) -> bool {
        *self == SIG_ID::Lptim2Wkup
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID::Lptim2Out
    }
    #[doc = "Signal `lptim3_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim3_wkup(&self) -> bool {
        *self == SIG_ID::Lptim3Wkup
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID::Lptim3Out
    }
    #[doc = "Signal `lptim4_ait` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim4_ait(&self) -> bool {
        *self == SIG_ID::Lptim4Ait
    }
    #[doc = "Signal `lptim5_ait` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim5_ait(&self) -> bool {
        *self == SIG_ID::Lptim5Ait
    }
    #[doc = "Signal `i2c4_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SIG_ID::I2c4Wkup
    }
    #[doc = "Signal `spi6_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SIG_ID::Spi6Wkup
    }
    #[doc = "Signal `comp1_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SIG_ID::Comp1Out
    }
    #[doc = "Signal `comp2_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == SIG_ID::Comp2Out
    }
    #[doc = "Signal `rtc_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SIG_ID::RtcWkup
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as trigger input"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SIG_ID::SyscfgExti0Mux
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as trigger input"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SIG_ID::SyscfgExti2Mux
    }
    #[doc = "Signal `i2c4_event_it` selected as trigger input"]
    #[inline(always)]
    pub fn is_i2c4_event_it(&self) -> bool {
        *self == SIG_ID::I2c4EventIt
    }
    #[doc = "Signal `spi6_it` selected as trigger input"]
    #[inline(always)]
    pub fn is_spi6_it(&self) -> bool {
        *self == SIG_ID::Spi6It
    }
    #[doc = "Signal `lpuart1_it_t` selected as trigger input"]
    #[inline(always)]
    pub fn is_lpuart1_it_t(&self) -> bool {
        *self == SIG_ID::Lpuart1ItT
    }
    #[doc = "Signal `lpuart1_it_r` selected as trigger input"]
    #[inline(always)]
    pub fn is_lpuart1_it_r(&self) -> bool {
        *self == SIG_ID::Lpuart1ItR
    }
    #[doc = "Signal `adc3_it` selected as trigger input"]
    #[inline(always)]
    pub fn is_adc3_it(&self) -> bool {
        *self == SIG_ID::Adc3It
    }
    #[doc = "Signal `adc3_awd1` selected as trigger input"]
    #[inline(always)]
    pub fn is_adc3_awd1(&self) -> bool {
        *self == SIG_ID::Adc3Awd1
    }
    #[doc = "Signal `bdma_ch0_it` selected as trigger input"]
    #[inline(always)]
    pub fn is_bdma_ch0_it(&self) -> bool {
        *self == SIG_ID::BdmaCh0It
    }
    #[doc = "Signal `bdma_ch1_it` selected as trigger input"]
    #[inline(always)]
    pub fn is_bdma_ch1_it(&self) -> bool {
        *self == SIG_ID::BdmaCh1It
    }
}
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SIG_ID>;
impl<'a, REG> SIG_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal `dmamux2_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt5)
    }
    #[doc = "Signal `dmamux2_evt6` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt6(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt6)
    }
    #[doc = "Signal `lpuart_rx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_rx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::LpuartRxWkup)
    }
    #[doc = "Signal `lpuart_tx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_tx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::LpuartTxWkup)
    }
    #[doc = "Signal `lptim2_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Wkup)
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Out)
    }
    #[doc = "Signal `lptim3_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Wkup)
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Out)
    }
    #[doc = "Signal `lptim4_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim4_ait(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim4Ait)
    }
    #[doc = "Signal `lptim5_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim5_ait(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim5Ait)
    }
    #[doc = "Signal `i2c4_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::I2c4Wkup)
    }
    #[doc = "Signal `spi6_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Spi6Wkup)
    }
    #[doc = "Signal `comp1_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Comp1Out)
    }
    #[doc = "Signal `comp2_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Comp2Out)
    }
    #[doc = "Signal `rtc_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::RtcWkup)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::SyscfgExti0Mux)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::SyscfgExti2Mux)
    }
    #[doc = "Signal `i2c4_event_it` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_event_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::I2c4EventIt)
    }
    #[doc = "Signal `spi6_it` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Spi6It)
    }
    #[doc = "Signal `lpuart1_it_t` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_t(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lpuart1ItT)
    }
    #[doc = "Signal `lpuart1_it_r` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_r(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lpuart1ItR)
    }
    #[doc = "Signal `adc3_it` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Adc3It)
    }
    #[doc = "Signal `adc3_awd1` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_awd1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Adc3Awd1)
    }
    #[doc = "Signal `bdma_ch0_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch0_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::BdmaCh0It)
    }
    #[doc = "Signal `bdma_ch1_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch1_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::BdmaCh1It)
    }
}
#[doc = "Interrupt enable at trigger event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    #[doc = "0: Trigger overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    Enabled = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::Disabled,
            true => OIE::Enabled,
        }
    }
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE::Disabled
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE::Enabled
    }
}
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Disabled)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Enabled)
    }
}
#[doc = "DMA request generator channel enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    #[doc = "0: DMA request generation disabled"]
    Disabled = 0,
    #[doc = "1: DMA request enabled"]
    Enabled = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::Disabled,
            true => GE::Enabled,
        }
    }
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE::Disabled
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE::Enabled
    }
}
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Disabled)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Enabled)
    }
}
#[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<GPOL> for u8 {
    #[inline(always)]
    fn from(variant: GPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPOL {
    type Ux = u8;
}
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPOL {
        match self.bits {
            0 => GPOL::NoEdge,
            1 => GPOL::RisingEdge,
            2 => GPOL::FallingEdge,
            3 => GPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL::NoEdge
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL::BothEdges
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GPOL>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::BothEdges)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<RGCRrs> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<RGCRrs> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<RGCRrs> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgcr::R`](R) reader structure"]
impl crate::Readable for RGCRrs {}
#[doc = "`write(|w| ..)` method takes [`rgcr::W`](W) writer structure"]
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RG%sCR to value 0"]
impl crate::Resettable for RGCRrs {
    const RESET_VALUE: u32 = 0;
}
