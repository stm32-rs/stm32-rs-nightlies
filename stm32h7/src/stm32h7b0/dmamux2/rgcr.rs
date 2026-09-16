///Register `RG%sCR` reader
pub type R = crate::R<RGCRrs>;
///Register `RG%sCR` writer
pub type W = crate::W<RGCRrs>;
/**DMA request trigger input selected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID {
    ///0: Signal `dmamux2_evt0` selected as trigger input
    Dmamux2Evt0 = 0,
    ///1: Signal `dmamux2_evt1` selected as trigger input
    Dmamux2Evt1 = 1,
    ///2: Signal `dmamux2_evt2` selected as trigger input
    Dmamux2Evt2 = 2,
    ///3: Signal `dmamux2_evt3` selected as trigger input
    Dmamux2Evt3 = 3,
    ///4: Signal `dmamux2_evt4` selected as trigger input
    Dmamux2Evt4 = 4,
    ///5: Signal `dmamux2_evt5` selected as trigger input
    Dmamux2Evt5 = 5,
    ///6: Signal `dmamux2_evt6` selected as trigger input
    Dmamux2Evt6 = 6,
    ///7: Signal `lpuart_rx_wkup` selected as trigger input
    LpuartRxWkup = 7,
    ///8: Signal `lpuart_tx_wkup` selected as trigger input
    LpuartTxWkup = 8,
    ///9: Signal `lptim2_wkup` selected as trigger input
    Lptim2Wkup = 9,
    ///10: Signal `lptim2_out` selected as trigger input
    Lptim2Out = 10,
    ///11: Signal `lptim3_wkup` selected as trigger input
    Lptim3Wkup = 11,
    ///12: Signal `lptim3_out` selected as trigger input
    Lptim3Out = 12,
    ///13: Signal `lptim4_ait` selected as trigger input
    Lptim4Ait = 13,
    ///14: Signal `lptim5_ait` selected as trigger input
    Lptim5Ait = 14,
    ///15: Signal `i2c4_wkup` selected as trigger input
    I2c4Wkup = 15,
    ///16: Signal `spi6_wkup` selected as trigger input
    Spi6Wkup = 16,
    ///17: Signal `comp1_out` selected as trigger input
    Comp1Out = 17,
    ///18: Signal `comp2_out` selected as trigger input
    Comp2Out = 18,
    ///19: Signal `rtc_wkup` selected as trigger input
    RtcWkup = 19,
    ///20: Signal `syscfg_exti0_mux` selected as trigger input
    SyscfgExti0Mux = 20,
    ///21: Signal `syscfg_exti2_mux` selected as trigger input
    SyscfgExti2Mux = 21,
    ///22: Signal `i2c4_event_it` selected as trigger input
    I2c4EventIt = 22,
    ///23: Signal `spi6_it` selected as trigger input
    Spi6It = 23,
    ///24: Signal `lpuart1_it_t` selected as trigger input
    Lpuart1ItT = 24,
    ///25: Signal `lpuart1_it_r` selected as trigger input
    Lpuart1ItR = 25,
    ///26: Signal `adc3_it` selected as trigger input
    Adc3It = 26,
    ///27: Signal `adc3_awd1` selected as trigger input
    Adc3Awd1 = 27,
    ///28: Signal `bdma_ch0_it` selected as trigger input
    BdmaCh0It = 28,
    ///29: Signal `bdma_ch1_it` selected as trigger input
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
impl crate::IsEnum for SIG_ID {}
///Field `SIG_ID` reader - DMA request trigger input selected
pub type SIG_ID_R = crate::FieldReader<SIG_ID>;
impl SIG_ID_R {
    ///Get enumerated values variant
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
    ///Signal `dmamux2_evt0` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt0
    }
    ///Signal `dmamux2_evt1` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt1
    }
    ///Signal `dmamux2_evt2` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt2
    }
    ///Signal `dmamux2_evt3` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt3
    }
    ///Signal `dmamux2_evt4` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt4
    }
    ///Signal `dmamux2_evt5` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt5
    }
    ///Signal `dmamux2_evt6` selected as trigger input
    #[inline(always)]
    pub fn is_dmamux2_evt6(&self) -> bool {
        *self == SIG_ID::Dmamux2Evt6
    }
    ///Signal `lpuart_rx_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_lpuart_rx_wkup(&self) -> bool {
        *self == SIG_ID::LpuartRxWkup
    }
    ///Signal `lpuart_tx_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_lpuart_tx_wkup(&self) -> bool {
        *self == SIG_ID::LpuartTxWkup
    }
    ///Signal `lptim2_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_lptim2_wkup(&self) -> bool {
        *self == SIG_ID::Lptim2Wkup
    }
    ///Signal `lptim2_out` selected as trigger input
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID::Lptim2Out
    }
    ///Signal `lptim3_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_lptim3_wkup(&self) -> bool {
        *self == SIG_ID::Lptim3Wkup
    }
    ///Signal `lptim3_out` selected as trigger input
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID::Lptim3Out
    }
    ///Signal `lptim4_ait` selected as trigger input
    #[inline(always)]
    pub fn is_lptim4_ait(&self) -> bool {
        *self == SIG_ID::Lptim4Ait
    }
    ///Signal `lptim5_ait` selected as trigger input
    #[inline(always)]
    pub fn is_lptim5_ait(&self) -> bool {
        *self == SIG_ID::Lptim5Ait
    }
    ///Signal `i2c4_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SIG_ID::I2c4Wkup
    }
    ///Signal `spi6_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SIG_ID::Spi6Wkup
    }
    ///Signal `comp1_out` selected as trigger input
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SIG_ID::Comp1Out
    }
    ///Signal `comp2_out` selected as trigger input
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == SIG_ID::Comp2Out
    }
    ///Signal `rtc_wkup` selected as trigger input
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SIG_ID::RtcWkup
    }
    ///Signal `syscfg_exti0_mux` selected as trigger input
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SIG_ID::SyscfgExti0Mux
    }
    ///Signal `syscfg_exti2_mux` selected as trigger input
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SIG_ID::SyscfgExti2Mux
    }
    ///Signal `i2c4_event_it` selected as trigger input
    #[inline(always)]
    pub fn is_i2c4_event_it(&self) -> bool {
        *self == SIG_ID::I2c4EventIt
    }
    ///Signal `spi6_it` selected as trigger input
    #[inline(always)]
    pub fn is_spi6_it(&self) -> bool {
        *self == SIG_ID::Spi6It
    }
    ///Signal `lpuart1_it_t` selected as trigger input
    #[inline(always)]
    pub fn is_lpuart1_it_t(&self) -> bool {
        *self == SIG_ID::Lpuart1ItT
    }
    ///Signal `lpuart1_it_r` selected as trigger input
    #[inline(always)]
    pub fn is_lpuart1_it_r(&self) -> bool {
        *self == SIG_ID::Lpuart1ItR
    }
    ///Signal `adc3_it` selected as trigger input
    #[inline(always)]
    pub fn is_adc3_it(&self) -> bool {
        *self == SIG_ID::Adc3It
    }
    ///Signal `adc3_awd1` selected as trigger input
    #[inline(always)]
    pub fn is_adc3_awd1(&self) -> bool {
        *self == SIG_ID::Adc3Awd1
    }
    ///Signal `bdma_ch0_it` selected as trigger input
    #[inline(always)]
    pub fn is_bdma_ch0_it(&self) -> bool {
        *self == SIG_ID::BdmaCh0It
    }
    ///Signal `bdma_ch1_it` selected as trigger input
    #[inline(always)]
    pub fn is_bdma_ch1_it(&self) -> bool {
        *self == SIG_ID::BdmaCh1It
    }
}
///Field `SIG_ID` writer - DMA request trigger input selected
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SIG_ID>;
impl<'a, REG> SIG_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Signal `dmamux2_evt0` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt0)
    }
    ///Signal `dmamux2_evt1` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt1)
    }
    ///Signal `dmamux2_evt2` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt2)
    }
    ///Signal `dmamux2_evt3` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt3)
    }
    ///Signal `dmamux2_evt4` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt4)
    }
    ///Signal `dmamux2_evt5` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt5)
    }
    ///Signal `dmamux2_evt6` selected as trigger input
    #[inline(always)]
    pub fn dmamux2_evt6(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux2Evt6)
    }
    ///Signal `lpuart_rx_wkup` selected as trigger input
    #[inline(always)]
    pub fn lpuart_rx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::LpuartRxWkup)
    }
    ///Signal `lpuart_tx_wkup` selected as trigger input
    #[inline(always)]
    pub fn lpuart_tx_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::LpuartTxWkup)
    }
    ///Signal `lptim2_wkup` selected as trigger input
    #[inline(always)]
    pub fn lptim2_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Wkup)
    }
    ///Signal `lptim2_out` selected as trigger input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Out)
    }
    ///Signal `lptim3_wkup` selected as trigger input
    #[inline(always)]
    pub fn lptim3_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Wkup)
    }
    ///Signal `lptim3_out` selected as trigger input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Out)
    }
    ///Signal `lptim4_ait` selected as trigger input
    #[inline(always)]
    pub fn lptim4_ait(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim4Ait)
    }
    ///Signal `lptim5_ait` selected as trigger input
    #[inline(always)]
    pub fn lptim5_ait(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim5Ait)
    }
    ///Signal `i2c4_wkup` selected as trigger input
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::I2c4Wkup)
    }
    ///Signal `spi6_wkup` selected as trigger input
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Spi6Wkup)
    }
    ///Signal `comp1_out` selected as trigger input
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Comp1Out)
    }
    ///Signal `comp2_out` selected as trigger input
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Comp2Out)
    }
    ///Signal `rtc_wkup` selected as trigger input
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::RtcWkup)
    }
    ///Signal `syscfg_exti0_mux` selected as trigger input
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::SyscfgExti0Mux)
    }
    ///Signal `syscfg_exti2_mux` selected as trigger input
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::SyscfgExti2Mux)
    }
    ///Signal `i2c4_event_it` selected as trigger input
    #[inline(always)]
    pub fn i2c4_event_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::I2c4EventIt)
    }
    ///Signal `spi6_it` selected as trigger input
    #[inline(always)]
    pub fn spi6_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Spi6It)
    }
    ///Signal `lpuart1_it_t` selected as trigger input
    #[inline(always)]
    pub fn lpuart1_it_t(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lpuart1ItT)
    }
    ///Signal `lpuart1_it_r` selected as trigger input
    #[inline(always)]
    pub fn lpuart1_it_r(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lpuart1ItR)
    }
    ///Signal `adc3_it` selected as trigger input
    #[inline(always)]
    pub fn adc3_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Adc3It)
    }
    ///Signal `adc3_awd1` selected as trigger input
    #[inline(always)]
    pub fn adc3_awd1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Adc3Awd1)
    }
    ///Signal `bdma_ch0_it` selected as trigger input
    #[inline(always)]
    pub fn bdma_ch0_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::BdmaCh0It)
    }
    ///Signal `bdma_ch1_it` selected as trigger input
    #[inline(always)]
    pub fn bdma_ch1_it(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::BdmaCh1It)
    }
}
/**Interrupt enable at trigger event overrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    ///0: Trigger overrun interrupt disabled
    Disabled = 0,
    ///1: Trigger overrun interrupt enabled
    Enabled = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OIE` reader - Interrupt enable at trigger event overrun
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::Disabled,
            true => OIE::Enabled,
        }
    }
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE::Disabled
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE::Enabled
    }
}
///Field `OIE` writer - Interrupt enable at trigger event overrun
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Disabled)
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Enabled)
    }
}
/**DMA request generator channel enable/disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    ///0: DMA request generation disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
///Field `GE` reader - DMA request generator channel enable/disable
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::Disabled,
            true => GE::Enabled,
        }
    }
    ///DMA request generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE::Disabled
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE::Enabled
    }
}
///Field `GE` writer - DMA request generator channel enable/disable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Enabled)
    }
}
/**DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    ///0: No event, i.e. no detection nor generation
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
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
impl crate::IsEnum for GPOL {}
///Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    ///Get enumerated values variant
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
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL::BothEdges
    }
}
///Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GPOL, crate::Safe>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::BothEdges)
    }
}
///Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGCR")
            .field("sig_id", &self.sig_id())
            .field("oie", &self.oie())
            .field("ge", &self.ge())
            .field("gpol", &self.gpol())
            .field("gnbreq", &self.gnbreq())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<'_, RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<'_, RGCRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<'_, RGCRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, RGCRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<'_, RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMAMux - DMA request generator channel x control register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#DMAMUX2:RG[0]CR)*/
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
///`read()` method returns [`rgcr::R`](R) reader structure
impl crate::Readable for RGCRrs {}
///`write(|w| ..)` method takes [`rgcr::W`](W) writer structure
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RG%sCR to value 0
impl crate::Resettable for RGCRrs {}
