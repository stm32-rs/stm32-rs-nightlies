#[doc = "Reader of register RGCR%s"]
pub type R = crate::R<u32, super::RGCR>;
#[doc = "Writer for register RGCR%s"]
pub type W = crate::W<u32, super::RGCR>;
#[doc = "Register RGCR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::RGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA request trigger input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIG_ID_A {
    #[doc = "0: Signal `dmamux2_evt0` selected as trigger input"]
    DMAMUX2_EVT0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as trigger input"]
    DMAMUX2_EVT1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as trigger input"]
    DMAMUX2_EVT2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as trigger input"]
    DMAMUX2_EVT3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as trigger input"]
    DMAMUX2_EVT4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as trigger input"]
    DMAMUX2_EVT5 = 5,
    #[doc = "6: Signal `dmamux2_evt6` selected as trigger input"]
    DMAMUX2_EVT6 = 6,
    #[doc = "7: Signal `lpuart_rx_wkup` selected as trigger input"]
    LPUART_RX_WKUP = 7,
    #[doc = "8: Signal `lpuart_tx_wkup` selected as trigger input"]
    LPUART_TX_WKUP = 8,
    #[doc = "9: Signal `lptim2_wkup` selected as trigger input"]
    LPTIM2_WKUP = 9,
    #[doc = "10: Signal `lptim2_out` selected as trigger input"]
    LPTIM2_OUT = 10,
    #[doc = "11: Signal `lptim3_wkup` selected as trigger input"]
    LPTIM3_WKUP = 11,
    #[doc = "12: Signal `lptim3_out` selected as trigger input"]
    LPTIM3_OUT = 12,
    #[doc = "13: Signal `lptim4_ait` selected as trigger input"]
    LPTIM4_AIT = 13,
    #[doc = "14: Signal `lptim5_ait` selected as trigger input"]
    LPTIM5_AIT = 14,
    #[doc = "15: Signal `i2c4_wkup` selected as trigger input"]
    I2C4_WKUP = 15,
    #[doc = "16: Signal `spi6_wkup` selected as trigger input"]
    SPI6_WKUP = 16,
    #[doc = "17: Signal `comp1_out` selected as trigger input"]
    COMP1_OUT = 17,
    #[doc = "18: Signal `comp2_out` selected as trigger input"]
    COMP2_OUT = 18,
    #[doc = "19: Signal `rtc_wkup` selected as trigger input"]
    RTC_WKUP = 19,
    #[doc = "20: Signal `syscfg_exti0_mux` selected as trigger input"]
    SYSCFG_EXTI0_MUX = 20,
    #[doc = "21: Signal `syscfg_exti2_mux` selected as trigger input"]
    SYSCFG_EXTI2_MUX = 21,
    #[doc = "22: Signal `i2c4_event_it` selected as trigger input"]
    I2C4_EVENT_IT = 22,
    #[doc = "23: Signal `spi6_it` selected as trigger input"]
    SPI6_IT = 23,
    #[doc = "24: Signal `lpuart1_it_t` selected as trigger input"]
    LPUART1_IT_T = 24,
    #[doc = "25: Signal `lpuart1_it_r` selected as trigger input"]
    LPUART1_IT_R = 25,
    #[doc = "26: Signal `adc3_it` selected as trigger input"]
    ADC3_IT = 26,
    #[doc = "27: Signal `adc3_awd1` selected as trigger input"]
    ADC3_AWD1 = 27,
    #[doc = "28: Signal `bdma_ch0_it` selected as trigger input"]
    BDMA_CH0_IT = 28,
    #[doc = "29: Signal `bdma_ch1_it` selected as trigger input"]
    BDMA_CH1_IT = 29,
}
impl From<SIG_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIG_ID`"]
pub type SIG_ID_R = crate::R<u8, SIG_ID_A>;
impl SIG_ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIG_ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIG_ID_A::DMAMUX2_EVT0),
            1 => Val(SIG_ID_A::DMAMUX2_EVT1),
            2 => Val(SIG_ID_A::DMAMUX2_EVT2),
            3 => Val(SIG_ID_A::DMAMUX2_EVT3),
            4 => Val(SIG_ID_A::DMAMUX2_EVT4),
            5 => Val(SIG_ID_A::DMAMUX2_EVT5),
            6 => Val(SIG_ID_A::DMAMUX2_EVT6),
            7 => Val(SIG_ID_A::LPUART_RX_WKUP),
            8 => Val(SIG_ID_A::LPUART_TX_WKUP),
            9 => Val(SIG_ID_A::LPTIM2_WKUP),
            10 => Val(SIG_ID_A::LPTIM2_OUT),
            11 => Val(SIG_ID_A::LPTIM3_WKUP),
            12 => Val(SIG_ID_A::LPTIM3_OUT),
            13 => Val(SIG_ID_A::LPTIM4_AIT),
            14 => Val(SIG_ID_A::LPTIM5_AIT),
            15 => Val(SIG_ID_A::I2C4_WKUP),
            16 => Val(SIG_ID_A::SPI6_WKUP),
            17 => Val(SIG_ID_A::COMP1_OUT),
            18 => Val(SIG_ID_A::COMP2_OUT),
            19 => Val(SIG_ID_A::RTC_WKUP),
            20 => Val(SIG_ID_A::SYSCFG_EXTI0_MUX),
            21 => Val(SIG_ID_A::SYSCFG_EXTI2_MUX),
            22 => Val(SIG_ID_A::I2C4_EVENT_IT),
            23 => Val(SIG_ID_A::SPI6_IT),
            24 => Val(SIG_ID_A::LPUART1_IT_T),
            25 => Val(SIG_ID_A::LPUART1_IT_R),
            26 => Val(SIG_ID_A::ADC3_IT),
            27 => Val(SIG_ID_A::ADC3_AWD1),
            28 => Val(SIG_ID_A::BDMA_CH0_IT),
            29 => Val(SIG_ID_A::BDMA_CH1_IT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT0`"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT0
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT1`"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT1
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT2`"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT2
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT3`"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT3
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT4`"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT4
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT5`"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT5
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT6`"]
    #[inline(always)]
    pub fn is_dmamux2_evt6(&self) -> bool {
        *self == SIG_ID_A::DMAMUX2_EVT6
    }
    #[doc = "Checks if the value of the field is `LPUART_RX_WKUP`"]
    #[inline(always)]
    pub fn is_lpuart_rx_wkup(&self) -> bool {
        *self == SIG_ID_A::LPUART_RX_WKUP
    }
    #[doc = "Checks if the value of the field is `LPUART_TX_WKUP`"]
    #[inline(always)]
    pub fn is_lpuart_tx_wkup(&self) -> bool {
        *self == SIG_ID_A::LPUART_TX_WKUP
    }
    #[doc = "Checks if the value of the field is `LPTIM2_WKUP`"]
    #[inline(always)]
    pub fn is_lptim2_wkup(&self) -> bool {
        *self == SIG_ID_A::LPTIM2_WKUP
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_WKUP`"]
    #[inline(always)]
    pub fn is_lptim3_wkup(&self) -> bool {
        *self == SIG_ID_A::LPTIM3_WKUP
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM4_AIT`"]
    #[inline(always)]
    pub fn is_lptim4_ait(&self) -> bool {
        *self == SIG_ID_A::LPTIM4_AIT
    }
    #[doc = "Checks if the value of the field is `LPTIM5_AIT`"]
    #[inline(always)]
    pub fn is_lptim5_ait(&self) -> bool {
        *self == SIG_ID_A::LPTIM5_AIT
    }
    #[doc = "Checks if the value of the field is `I2C4_WKUP`"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        *self == SIG_ID_A::I2C4_WKUP
    }
    #[doc = "Checks if the value of the field is `SPI6_WKUP`"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        *self == SIG_ID_A::SPI6_WKUP
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == SIG_ID_A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `COMP2_OUT`"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == SIG_ID_A::COMP2_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_WKUP`"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        *self == SIG_ID_A::RTC_WKUP
    }
    #[doc = "Checks if the value of the field is `SYSCFG_EXTI0_MUX`"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        *self == SIG_ID_A::SYSCFG_EXTI0_MUX
    }
    #[doc = "Checks if the value of the field is `SYSCFG_EXTI2_MUX`"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        *self == SIG_ID_A::SYSCFG_EXTI2_MUX
    }
    #[doc = "Checks if the value of the field is `I2C4_EVENT_IT`"]
    #[inline(always)]
    pub fn is_i2c4_event_it(&self) -> bool {
        *self == SIG_ID_A::I2C4_EVENT_IT
    }
    #[doc = "Checks if the value of the field is `SPI6_IT`"]
    #[inline(always)]
    pub fn is_spi6_it(&self) -> bool {
        *self == SIG_ID_A::SPI6_IT
    }
    #[doc = "Checks if the value of the field is `LPUART1_IT_T`"]
    #[inline(always)]
    pub fn is_lpuart1_it_t(&self) -> bool {
        *self == SIG_ID_A::LPUART1_IT_T
    }
    #[doc = "Checks if the value of the field is `LPUART1_IT_R`"]
    #[inline(always)]
    pub fn is_lpuart1_it_r(&self) -> bool {
        *self == SIG_ID_A::LPUART1_IT_R
    }
    #[doc = "Checks if the value of the field is `ADC3_IT`"]
    #[inline(always)]
    pub fn is_adc3_it(&self) -> bool {
        *self == SIG_ID_A::ADC3_IT
    }
    #[doc = "Checks if the value of the field is `ADC3_AWD1`"]
    #[inline(always)]
    pub fn is_adc3_awd1(&self) -> bool {
        *self == SIG_ID_A::ADC3_AWD1
    }
    #[doc = "Checks if the value of the field is `BDMA_CH0_IT`"]
    #[inline(always)]
    pub fn is_bdma_ch0_it(&self) -> bool {
        *self == SIG_ID_A::BDMA_CH0_IT
    }
    #[doc = "Checks if the value of the field is `BDMA_CH1_IT`"]
    #[inline(always)]
    pub fn is_bdma_ch1_it(&self) -> bool {
        *self == SIG_ID_A::BDMA_CH1_IT
    }
}
#[doc = "Write proxy for field `SIG_ID`"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal `dmamux2_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT5)
    }
    #[doc = "Signal `dmamux2_evt6` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux2_evt6(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX2_EVT6)
    }
    #[doc = "Signal `lpuart_rx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_rx_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPUART_RX_WKUP)
    }
    #[doc = "Signal `lpuart_tx_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart_tx_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPUART_TX_WKUP)
    }
    #[doc = "Signal `lptim2_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM2_WKUP)
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM2_OUT)
    }
    #[doc = "Signal `lptim3_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM3_WKUP)
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM3_OUT)
    }
    #[doc = "Signal `lptim4_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim4_ait(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM4_AIT)
    }
    #[doc = "Signal `lptim5_ait` selected as trigger input"]
    #[inline(always)]
    pub fn lptim5_ait(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM5_AIT)
    }
    #[doc = "Signal `i2c4_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::I2C4_WKUP)
    }
    #[doc = "Signal `spi6_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::SPI6_WKUP)
    }
    #[doc = "Signal `comp1_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::COMP1_OUT)
    }
    #[doc = "Signal `comp2_out` selected as trigger input"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::COMP2_OUT)
    }
    #[doc = "Signal `rtc_wkup` selected as trigger input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut W {
        self.variant(SIG_ID_A::RTC_WKUP)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut W {
        self.variant(SIG_ID_A::SYSCFG_EXTI0_MUX)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as trigger input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut W {
        self.variant(SIG_ID_A::SYSCFG_EXTI2_MUX)
    }
    #[doc = "Signal `i2c4_event_it` selected as trigger input"]
    #[inline(always)]
    pub fn i2c4_event_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::I2C4_EVENT_IT)
    }
    #[doc = "Signal `spi6_it` selected as trigger input"]
    #[inline(always)]
    pub fn spi6_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::SPI6_IT)
    }
    #[doc = "Signal `lpuart1_it_t` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_t(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPUART1_IT_T)
    }
    #[doc = "Signal `lpuart1_it_r` selected as trigger input"]
    #[inline(always)]
    pub fn lpuart1_it_r(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPUART1_IT_R)
    }
    #[doc = "Signal `adc3_it` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::ADC3_IT)
    }
    #[doc = "Signal `adc3_awd1` selected as trigger input"]
    #[inline(always)]
    pub fn adc3_awd1(self) -> &'a mut W {
        self.variant(SIG_ID_A::ADC3_AWD1)
    }
    #[doc = "Signal `bdma_ch0_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch0_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::BDMA_CH0_IT)
    }
    #[doc = "Signal `bdma_ch1_it` selected as trigger input"]
    #[inline(always)]
    pub fn bdma_ch1_it(self) -> &'a mut W {
        self.variant(SIG_ID_A::BDMA_CH1_IT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Interrupt enable at trigger event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIE_A {
    #[doc = "0: Trigger overrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    ENABLED = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OIE`"]
pub type OIE_R = crate::R<bool, OIE_A>;
impl OIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::DISABLED,
            true => OIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `OIE`"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIE_A::DISABLED)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "DMA request generator channel enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GE_A {
    #[doc = "0: DMA request generation disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request enabled"]
    ENABLED = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GE`"]
pub type GE_R = crate::R<bool, GE_A>;
impl GE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::DISABLED,
            true => GE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE_A::ENABLED
    }
}
#[doc = "Write proxy for field `GE`"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GE_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NOEDGE = 0,
    #[doc = "1: Rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPOL`"]
pub type GPOL_R = crate::R<u8, GPOL_A>;
impl GPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NOEDGE,
            1 => GPOL_A::RISINGEDGE,
            2 => GPOL_A::FALLINGEDGE,
            3 => GPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOEDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL_A::NOEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `GPOL`"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPOL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(GPOL_A::NOEDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(GPOL_A::RISINGEDGE)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(GPOL_A::FALLINGEDGE)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `GNBREQ`"]
pub type GNBREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GNBREQ`"]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
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
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
}
