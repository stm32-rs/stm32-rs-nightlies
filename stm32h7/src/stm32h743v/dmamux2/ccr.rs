#[doc = "Register `CCR%s` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR%s` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input DMA request line selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    #[doc = "0: No signal selected as request input"]
    NONE = 0,
    #[doc = "1: Signal `dmamux2_req_gen0` selected as request input"]
    DMAMUX2_REQ_GEN0 = 1,
    #[doc = "2: Signal `dmamux2_req_gen1` selected as request input"]
    DMAMUX2_REQ_GEN1 = 2,
    #[doc = "3: Signal `dmamux2_req_gen2` selected as request input"]
    DMAMUX2_REQ_GEN2 = 3,
    #[doc = "4: Signal `dmamux2_req_gen3` selected as request input"]
    DMAMUX2_REQ_GEN3 = 4,
    #[doc = "5: Signal `dmamux2_req_gen4` selected as request input"]
    DMAMUX2_REQ_GEN4 = 5,
    #[doc = "6: Signal `dmamux2_req_gen5` selected as request input"]
    DMAMUX2_REQ_GEN5 = 6,
    #[doc = "7: Signal `dmamux2_req_gen6` selected as request input"]
    DMAMUX2_REQ_GEN6 = 7,
    #[doc = "8: Signal `dmamux2_req_gen7` selected as request input"]
    DMAMUX2_REQ_GEN7 = 8,
    #[doc = "9: Signal `lpuart1_rx_dma` selected as request input"]
    LPUART1_RX_DMA = 9,
    #[doc = "10: Signal `lpuart1_tx_dma` selected as request input"]
    LPUART1_TX_DMA = 10,
    #[doc = "11: Signal `spi6_rx_dma` selected as request input"]
    SPI6_RX_DMA = 11,
    #[doc = "12: Signal `spi6_tx_dma` selected as request input"]
    SPI6_TX_DMA = 12,
    #[doc = "13: Signal `i2c4_rx_dma` selected as request input"]
    I2C4_RX_DMA = 13,
    #[doc = "14: Signal `i2c4_tx_dma` selected as request input"]
    I2C4_TX_DMA = 14,
    #[doc = "15: Signal `sai4_a_dma` selected as request input"]
    SAI4_A_DMA = 15,
    #[doc = "16: Signal `sai4_b_dma` selected as request input"]
    SAI4_B_DMA = 16,
    #[doc = "17: Signal `adc3_dma` selected as request input"]
    ADC3_DMA = 17,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub struct DMAREQ_ID_R(crate::FieldReader<u8, DMAREQ_ID_A>);
impl DMAREQ_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMAREQ_ID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAREQ_ID_A> {
        match self.bits {
            0 => Some(DMAREQ_ID_A::NONE),
            1 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN0),
            2 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN1),
            3 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN2),
            4 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN3),
            5 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN4),
            6 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN5),
            7 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN6),
            8 => Some(DMAREQ_ID_A::DMAMUX2_REQ_GEN7),
            9 => Some(DMAREQ_ID_A::LPUART1_RX_DMA),
            10 => Some(DMAREQ_ID_A::LPUART1_TX_DMA),
            11 => Some(DMAREQ_ID_A::SPI6_RX_DMA),
            12 => Some(DMAREQ_ID_A::SPI6_TX_DMA),
            13 => Some(DMAREQ_ID_A::I2C4_RX_DMA),
            14 => Some(DMAREQ_ID_A::I2C4_TX_DMA),
            15 => Some(DMAREQ_ID_A::SAI4_A_DMA),
            16 => Some(DMAREQ_ID_A::SAI4_B_DMA),
            17 => Some(DMAREQ_ID_A::ADC3_DMA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == DMAREQ_ID_A::NONE
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN0`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen0(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN0
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN1`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen1(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN1
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN2`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen2(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN2
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN3`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen3(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN3
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN4`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen4(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN4
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN5`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen5(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN5
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN6`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen6(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN6
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_REQ_GEN7`"]
    #[inline(always)]
    pub fn is_dmamux2_req_gen7(&self) -> bool {
        **self == DMAREQ_ID_A::DMAMUX2_REQ_GEN7
    }
    #[doc = "Checks if the value of the field is `LPUART1_RX_DMA`"]
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::LPUART1_RX_DMA
    }
    #[doc = "Checks if the value of the field is `LPUART1_TX_DMA`"]
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::LPUART1_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI6_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi6_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI6_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI6_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi6_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SPI6_TX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C4_RX_DMA`"]
    #[inline(always)]
    pub fn is_i2c4_rx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C4_RX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C4_TX_DMA`"]
    #[inline(always)]
    pub fn is_i2c4_tx_dma(&self) -> bool {
        **self == DMAREQ_ID_A::I2C4_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SAI4_A_DMA`"]
    #[inline(always)]
    pub fn is_sai4_a_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SAI4_A_DMA
    }
    #[doc = "Checks if the value of the field is `SAI4_B_DMA`"]
    #[inline(always)]
    pub fn is_sai4_b_dma(&self) -> bool {
        **self == DMAREQ_ID_A::SAI4_B_DMA
    }
    #[doc = "Checks if the value of the field is `ADC3_DMA`"]
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        **self == DMAREQ_ID_A::ADC3_DMA
    }
}
impl core::ops::Deref for DMAREQ_ID_R {
    type Target = crate::FieldReader<u8, DMAREQ_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub struct DMAREQ_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREQ_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::NONE)
    }
    #[doc = "Signal `dmamux2_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN0)
    }
    #[doc = "Signal `dmamux2_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN1)
    }
    #[doc = "Signal `dmamux2_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN2)
    }
    #[doc = "Signal `dmamux2_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN3)
    }
    #[doc = "Signal `dmamux2_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN4)
    }
    #[doc = "Signal `dmamux2_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN5)
    }
    #[doc = "Signal `dmamux2_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN6)
    }
    #[doc = "Signal `dmamux2_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn dmamux2_req_gen7(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX2_REQ_GEN7)
    }
    #[doc = "Signal `lpuart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::LPUART1_RX_DMA)
    }
    #[doc = "Signal `lpuart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::LPUART1_TX_DMA)
    }
    #[doc = "Signal `spi6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI6_RX_DMA)
    }
    #[doc = "Signal `spi6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi6_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI6_TX_DMA)
    }
    #[doc = "Signal `i2c4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C4_RX_DMA)
    }
    #[doc = "Signal `i2c4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C4_TX_DMA)
    }
    #[doc = "Signal `sai4_a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI4_A_DMA)
    }
    #[doc = "Signal `sai4_b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai4_b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI4_B_DMA)
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::ADC3_DMA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Interrupt enable at synchronization event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOIE_A {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    ENABLED = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub struct SOIE_R(crate::FieldReader<bool, SOIE_A>);
impl SOIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::DISABLED,
            true => SOIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SOIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SOIE_A::ENABLED
    }
}
impl core::ops::Deref for SOIE_R {
    type Target = crate::FieldReader<bool, SOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub struct SOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::DISABLED)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Event generation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGE_A {
    #[doc = "0: Event generation disabled"]
    DISABLED = 0,
    #[doc = "1: Event generation enabled"]
    ENABLED = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub struct EGE_R(crate::FieldReader<bool, EGE_A>);
impl EGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::DISABLED,
            true => EGE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EGE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EGE_A::ENABLED
    }
}
impl core::ops::Deref for EGE_R {
    type Target = crate::FieldReader<bool, EGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub struct EGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::DISABLED)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Synchronous operating mode enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    #[doc = "0: Synchronization disabled"]
    DISABLED = 0,
    #[doc = "1: Synchronization enabled"]
    ENABLED = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub struct SE_R(crate::FieldReader<bool, SE_A>);
impl SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::DISABLED,
            true => SE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SE_A::ENABLED
    }
}
impl core::ops::Deref for SE_R {
    type Target = crate::FieldReader<bool, SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::DISABLED)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NOEDGE = 0,
    #[doc = "1: Rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub struct SPOL_R(crate::FieldReader<u8, SPOL_A>);
impl SPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NOEDGE,
            1 => SPOL_A::RISINGEDGE,
            2 => SPOL_A::FALLINGEDGE,
            3 => SPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOEDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        **self == SPOL_A::NOEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == SPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == SPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == SPOL_A::BOTHEDGES
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<u8, SPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NOEDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RISINGEDGE)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FALLINGEDGE)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub struct NBREQ_R(crate::FieldReader<u8, u8>);
impl NBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub struct NBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Synchronization input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    #[doc = "0: Signal `dmamux2_evt0` selected as synchronization input"]
    DMAMUX2_EVT0 = 0,
    #[doc = "1: Signal `dmamux2_evt1` selected as synchronization input"]
    DMAMUX2_EVT1 = 1,
    #[doc = "2: Signal `dmamux2_evt2` selected as synchronization input"]
    DMAMUX2_EVT2 = 2,
    #[doc = "3: Signal `dmamux2_evt3` selected as synchronization input"]
    DMAMUX2_EVT3 = 3,
    #[doc = "4: Signal `dmamux2_evt4` selected as synchronization input"]
    DMAMUX2_EVT4 = 4,
    #[doc = "5: Signal `dmamux2_evt5` selected as synchronization input"]
    DMAMUX2_EVT5 = 5,
    #[doc = "6: Signal `lpuart1_rx_wkup` selected as synchronization input"]
    LPUART1_RX_WKUP = 6,
    #[doc = "7: Signal `lpuart1_tx_wkup` selected as synchronization input"]
    LPUART1_TX_WKUP = 7,
    #[doc = "8: Signal `lptim2_out` selected as synchronization input"]
    LPTIM2_OUT = 8,
    #[doc = "9: Signal `lptim3_out` selected as synchronization input"]
    LPTIM3_OUT = 9,
    #[doc = "10: Signal `i2c4_wkup` selected as synchronization input"]
    I2C4_WKUP = 10,
    #[doc = "11: Signal `spi6_wkup` selected as synchronization input"]
    SPI6_WKUP = 11,
    #[doc = "12: Signal `comp1_out` selected as synchronization input"]
    COMP1_OUT = 12,
    #[doc = "13: Signal `rtc_wkup` selected as synchronization input"]
    RTC_WKUP = 13,
    #[doc = "14: Signal `syscfg_exti0_mux` selected as synchronization input"]
    SYSCFG_EXTI0_MUX = 14,
    #[doc = "15: Signal `syscfg_exti2_mux` selected as synchronization input"]
    SYSCFG_EXTI2_MUX = 15,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub struct SYNC_ID_R(crate::FieldReader<u8, SYNC_ID_A>);
impl SYNC_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC_ID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_ID_A> {
        match self.bits {
            0 => Some(SYNC_ID_A::DMAMUX2_EVT0),
            1 => Some(SYNC_ID_A::DMAMUX2_EVT1),
            2 => Some(SYNC_ID_A::DMAMUX2_EVT2),
            3 => Some(SYNC_ID_A::DMAMUX2_EVT3),
            4 => Some(SYNC_ID_A::DMAMUX2_EVT4),
            5 => Some(SYNC_ID_A::DMAMUX2_EVT5),
            6 => Some(SYNC_ID_A::LPUART1_RX_WKUP),
            7 => Some(SYNC_ID_A::LPUART1_TX_WKUP),
            8 => Some(SYNC_ID_A::LPTIM2_OUT),
            9 => Some(SYNC_ID_A::LPTIM3_OUT),
            10 => Some(SYNC_ID_A::I2C4_WKUP),
            11 => Some(SYNC_ID_A::SPI6_WKUP),
            12 => Some(SYNC_ID_A::COMP1_OUT),
            13 => Some(SYNC_ID_A::RTC_WKUP),
            14 => Some(SYNC_ID_A::SYSCFG_EXTI0_MUX),
            15 => Some(SYNC_ID_A::SYSCFG_EXTI2_MUX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT0`"]
    #[inline(always)]
    pub fn is_dmamux2_evt0(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT0
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT1`"]
    #[inline(always)]
    pub fn is_dmamux2_evt1(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT1
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT2`"]
    #[inline(always)]
    pub fn is_dmamux2_evt2(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT2
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT3`"]
    #[inline(always)]
    pub fn is_dmamux2_evt3(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT3
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT4`"]
    #[inline(always)]
    pub fn is_dmamux2_evt4(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT4
    }
    #[doc = "Checks if the value of the field is `DMAMUX2_EVT5`"]
    #[inline(always)]
    pub fn is_dmamux2_evt5(&self) -> bool {
        **self == SYNC_ID_A::DMAMUX2_EVT5
    }
    #[doc = "Checks if the value of the field is `LPUART1_RX_WKUP`"]
    #[inline(always)]
    pub fn is_lpuart1_rx_wkup(&self) -> bool {
        **self == SYNC_ID_A::LPUART1_RX_WKUP
    }
    #[doc = "Checks if the value of the field is `LPUART1_TX_WKUP`"]
    #[inline(always)]
    pub fn is_lpuart1_tx_wkup(&self) -> bool {
        **self == SYNC_ID_A::LPUART1_TX_WKUP
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == SYNC_ID_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == SYNC_ID_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `I2C4_WKUP`"]
    #[inline(always)]
    pub fn is_i2c4_wkup(&self) -> bool {
        **self == SYNC_ID_A::I2C4_WKUP
    }
    #[doc = "Checks if the value of the field is `SPI6_WKUP`"]
    #[inline(always)]
    pub fn is_spi6_wkup(&self) -> bool {
        **self == SYNC_ID_A::SPI6_WKUP
    }
    #[doc = "Checks if the value of the field is `COMP1_OUT`"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        **self == SYNC_ID_A::COMP1_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_WKUP`"]
    #[inline(always)]
    pub fn is_rtc_wkup(&self) -> bool {
        **self == SYNC_ID_A::RTC_WKUP
    }
    #[doc = "Checks if the value of the field is `SYSCFG_EXTI0_MUX`"]
    #[inline(always)]
    pub fn is_syscfg_exti0_mux(&self) -> bool {
        **self == SYNC_ID_A::SYSCFG_EXTI0_MUX
    }
    #[doc = "Checks if the value of the field is `SYSCFG_EXTI2_MUX`"]
    #[inline(always)]
    pub fn is_syscfg_exti2_mux(&self) -> bool {
        **self == SYNC_ID_A::SYSCFG_EXTI2_MUX
    }
}
impl core::ops::Deref for SYNC_ID_R {
    type Target = crate::FieldReader<u8, SYNC_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub struct SYNC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal `dmamux2_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT0)
    }
    #[doc = "Signal `dmamux2_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT1)
    }
    #[doc = "Signal `dmamux2_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT2)
    }
    #[doc = "Signal `dmamux2_evt3` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt3(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT3)
    }
    #[doc = "Signal `dmamux2_evt4` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt4(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT4)
    }
    #[doc = "Signal `dmamux2_evt5` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux2_evt5(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX2_EVT5)
    }
    #[doc = "Signal `lpuart1_rx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_rx_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPUART1_RX_WKUP)
    }
    #[doc = "Signal `lpuart1_tx_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn lpuart1_tx_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPUART1_TX_WKUP)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM2_OUT)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM3_OUT)
    }
    #[doc = "Signal `i2c4_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn i2c4_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::I2C4_WKUP)
    }
    #[doc = "Signal `spi6_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn spi6_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::SPI6_WKUP)
    }
    #[doc = "Signal `comp1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::COMP1_OUT)
    }
    #[doc = "Signal `rtc_wkup` selected as synchronization input"]
    #[inline(always)]
    pub fn rtc_wkup(self) -> &'a mut W {
        self.variant(SYNC_ID_A::RTC_WKUP)
    }
    #[doc = "Signal `syscfg_exti0_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti0_mux(self) -> &'a mut W {
        self.variant(SYNC_ID_A::SYSCFG_EXTI0_MUX)
    }
    #[doc = "Signal `syscfg_exti2_mux` selected as synchronization input"]
    #[inline(always)]
    pub fn syscfg_exti2_mux(self) -> &'a mut W {
        self.variant(SYNC_ID_A::SYSCFG_EXTI2_MUX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
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
        SOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 0x03) as u8)
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
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W {
        DMAREQ_ID_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W {
        SOIE_W { w: self }
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W {
        EGE_W { w: self }
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W {
        NBREQ_W { w: self }
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W {
        SYNC_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR%s to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
