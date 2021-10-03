#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in       the USART_ISR register."]
    B_0X1 = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
pub struct EIE_R(crate::FieldReader<bool, EIE_A>);
impl EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::B_0X0,
            true => EIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EIE_A::B_0X1
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, EIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EIE_A::B_0X0)
    }
    #[doc = "interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: IrDA disabled"]
    B_0X0 = 0,
    #[doc = "1: IrDA enabled"]
    B_0X1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct IREN_R(crate::FieldReader<bool, IREN_A>);
impl IREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::B_0X0,
            true => IREN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IREN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IREN_A::B_0X1
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, IREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IREN_A::B_0X0)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IREN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLP_A {
    #[doc = "0: Normal mode"]
    B_0X0 = 0,
    #[doc = "1: Low-power mode"]
    B_0X1 = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct IRLP_R(crate::FieldReader<bool, IRLP_A>);
impl IRLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::B_0X0,
            true => IRLP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IRLP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IRLP_A::B_0X1
    }
}
impl core::ops::Deref for IRLP_R {
    type Target = crate::FieldReader<bool, IRLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct IRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> IRLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IRLP_A::B_0X0)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IRLP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    #[doc = "0: Half duplex mode is not selected"]
    B_0X0 = 0,
    #[doc = "1: Half duplex mode is selected "]
    B_0X1 = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub struct HDSEL_R(crate::FieldReader<bool, HDSEL_A>);
impl HDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::B_0X0,
            true => HDSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HDSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HDSEL_A::B_0X1
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, HDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub struct HDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HDSEL_A::B_0X0)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HDSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    B_0X0 = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    B_0X1 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct NACK_R(crate::FieldReader<bool, NACK_A>);
impl NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::B_0X0,
            true => NACK_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == NACK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == NACK_A::B_0X1
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, NACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NACK_A::B_0X0)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NACK_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
    #[doc = "0: Smartcard Mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Smartcard Mode enabled"]
    B_0X1 = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct SCEN_R(crate::FieldReader<bool, SCEN_A>);
impl SCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::B_0X0,
            true => SCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SCEN_A::B_0X1
    }
}
impl core::ops::Deref for SCEN_R {
    type Target = crate::FieldReader<bool, SCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct SCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smartcard Mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SCEN_A::B_0X0)
    }
    #[doc = "Smartcard Mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SCEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "DMA enable receiver This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    #[doc = "1: DMA mode is enabled for reception"]
    B_0X1 = 1,
    #[doc = "0: DMA mode is disabled for reception"]
    B_0X0 = 0,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver This bit is set/reset by software"]
pub struct DMAR_R(crate::FieldReader<bool, DMAR_A>);
impl DMAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            true => DMAR_A::B_0X1,
            false => DMAR_A::B_0X0,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAR_A::B_0X0
    }
}
impl core::ops::Deref for DMAR_R {
    type Target = crate::FieldReader<bool, DMAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver This bit is set/reset by software"]
pub struct DMAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAR_A::B_0X1)
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAR_A::B_0X0)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "DMA enable transmitter This bit is set/reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    #[doc = "1: DMA mode is enabled for transmission"]
    B_0X1 = 1,
    #[doc = "0: DMA mode is disabled for transmission"]
    B_0X0 = 0,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software"]
pub struct DMAT_R(crate::FieldReader<bool, DMAT_A>);
impl DMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            true => DMAT_A::B_0X1,
            false => DMAT_A::B_0X0,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAT_A::B_0X0
    }
}
impl core::ops::Deref for DMAT_R {
    type Target = crate::FieldReader<bool, DMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software"]
pub struct DMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAT_A::B_0X1)
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAT_A::B_0X0)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSE_A {
    #[doc = "0: RTS hardware flow control disabled"]
    B_0X0 = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received."]
    B_0X1 = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct RTSE_R(crate::FieldReader<bool, RTSE_A>);
impl RTSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::B_0X0,
            true => RTSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RTSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RTSE_A::B_0X1
    }
}
impl core::ops::Deref for RTSE_R {
    type Target = crate::FieldReader<bool, RTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct RTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTSE_A::B_0X0)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTSE_A::B_0X1)
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
#[doc = "CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSE_A {
    #[doc = "0: CTS hardware flow control disabled"]
    B_0X0 = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted."]
    B_0X1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct CTSE_R(crate::FieldReader<bool, CTSE_A>);
impl CTSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::B_0X0,
            true => CTSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CTSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CTSE_A::B_0X1
    }
}
impl core::ops::Deref for CTSE_R {
    type Target = crate::FieldReader<bool, CTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct CTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CTSE_A::B_0X0)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CTSE_A::B_0X1)
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
#[doc = "CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    #[doc = "0: Interrupt is inhibited"]
    B_0X0 = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF = 1 in the USART_ISR register  "]
    B_0X1 = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct CTSIE_R(crate::FieldReader<bool, CTSIE_A>);
impl CTSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::B_0X0,
            true => CTSIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CTSIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CTSIE_A::B_0X1
    }
}
impl core::ops::Deref for CTSIE_R {
    type Target = crate::FieldReader<bool, CTSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CTSIE_A::B_0X0)
    }
    #[doc = "An interrupt is generated whenever CTSIF = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CTSIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEBIT_A {
    #[doc = "0: Three sample bit method"]
    B_0X0 = 0,
    #[doc = "1: One sample bit method"]
    B_0X1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub struct ONEBIT_R(crate::FieldReader<bool, ONEBIT_A>);
impl ONEBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONEBIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::B_0X0,
            true => ONEBIT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ONEBIT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ONEBIT_A::B_0X1
    }
}
impl core::ops::Deref for ONEBIT_R {
    type Target = crate::FieldReader<bool, ONEBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub struct ONEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEBIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ONEBIT_A::B_0X0)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ONEBIT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDIS_A {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. "]
    B_0X0 = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    B_0X1 = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub struct OVRDIS_R(crate::FieldReader<bool, OVRDIS_A>);
impl OVRDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::B_0X0,
            true => OVRDIS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVRDIS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVRDIS_A::B_0X1
    }
}
impl core::ops::Deref for OVRDIS_R {
    type Target = crate::FieldReader<bool, OVRDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub struct OVRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVRDIS_A::B_0X0)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVRDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    #[doc = "0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode)."]
    B_0X0 = 0,
    #[doc = "1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    B_0X1 = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub struct DDRE_R(crate::FieldReader<bool, DDRE_A>);
impl DDRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::B_0X0,
            true => DDRE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DDRE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DDRE_A::B_0X1
    }
}
impl core::ops::Deref for DDRE_R {
    type Target = crate::FieldReader<bool, DDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub struct DDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRE_A::B_0X0)
    }
    #[doc = "DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    #[doc = "0: DE function is disabled. "]
    B_0X0 = 0,
    #[doc = "1: DE function is enabled. The DE signal is output on the RTS pin."]
    B_0X1 = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub struct DEM_R(crate::FieldReader<bool, DEM_A>);
impl DEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::B_0X0,
            true => DEM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DEM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DEM_A::B_0X1
    }
}
impl core::ops::Deref for DEM_R {
    type Target = crate::FieldReader<bool, DEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub struct DEM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DE function is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DEM_A::B_0X0)
    }
    #[doc = "DE function is enabled. The DE signal is output on the RTS pin."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DEM_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    #[doc = "0: DE signal is active high. "]
    B_0X0 = 0,
    #[doc = "1: DE signal is active low."]
    B_0X1 = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct DEP_R(crate::FieldReader<bool, DEP_A>);
impl DEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::B_0X0,
            true => DEP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DEP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DEP_A::B_0X1
    }
}
impl core::ops::Deref for DEP_R {
    type Target = crate::FieldReader<bool, DEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DE signal is active high."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DEP_A::B_0X0)
    }
    #[doc = "DE signal is active low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DEP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCARCNT_A {
    #[doc = "0: retransmission disabled - No automatic retransmission in transmit mode. "]
    B_0X0 = 0,
    #[doc = "1: number of automatic retransmission attempts (before signaling error)"]
    B_0X1 = 1,
    #[doc = "2: number of automatic retransmission attempts (before signaling error)"]
    B_0X2 = 2,
    #[doc = "3: number of automatic retransmission attempts (before signaling error)"]
    B_0X3 = 3,
    #[doc = "4: number of automatic retransmission attempts (before signaling error)"]
    B_0X4 = 4,
    #[doc = "5: number of automatic retransmission attempts (before signaling error)"]
    B_0X5 = 5,
    #[doc = "6: number of automatic retransmission attempts (before signaling error)"]
    B_0X6 = 6,
    #[doc = "7: number of automatic retransmission attempts (before signaling error)"]
    B_0X7 = 7,
}
impl From<SCARCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SCARCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct SCARCNT_R(crate::FieldReader<u8, SCARCNT_A>);
impl SCARCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCARCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCARCNT_A {
        match self.bits {
            0 => SCARCNT_A::B_0X0,
            1 => SCARCNT_A::B_0X1,
            2 => SCARCNT_A::B_0X2,
            3 => SCARCNT_A::B_0X3,
            4 => SCARCNT_A::B_0X4,
            5 => SCARCNT_A::B_0X5,
            6 => SCARCNT_A::B_0X6,
            7 => SCARCNT_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SCARCNT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SCARCNT_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == SCARCNT_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == SCARCNT_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == SCARCNT_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == SCARCNT_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == SCARCNT_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == SCARCNT_A::B_0X7
    }
}
impl core::ops::Deref for SCARCNT_R {
    type Target = crate::FieldReader<u8, SCARCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct SCARCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCARCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCARCNT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "retransmission disabled - No automatic retransmission in transmit mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X0)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X1)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X2)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X3)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X4)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X5)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X6)
    }
    #[doc = "number of automatic retransmission attempts (before signaling error)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SCARCNT_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUS_A {
    #[doc = "0: WUF active on address match (as defined by ADD\\[7:0\\]
and ADDM7)"]
    B_0X0 = 0,
    #[doc = "2: WUF active on start bit detection"]
    B_0X2 = 2,
    #[doc = "3: WUF active on RXNE/RXFNE. "]
    B_0X3 = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUS` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub struct WUS_R(crate::FieldReader<u8, WUS_A>);
impl WUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::B_0X0),
            2 => Some(WUS_A::B_0X2),
            3 => Some(WUS_A::B_0X3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == WUS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == WUS_A::B_0X3
    }
}
impl core::ops::Deref for WUS_R {
    type Target = crate::FieldReader<u8, WUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUS` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub struct WUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "WUF active on address match (as defined by ADD\\[7:0\\]
and ADDM7)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUS_A::B_0X0)
    }
    #[doc = "WUF active on start bit detection"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(WUS_A::B_0X2)
    }
    #[doc = "WUF active on RXNE/RXFNE."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(WUS_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever WUF = 1 in the USART_ISR register  "]
    B_0X1 = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub struct WUFIE_R(crate::FieldReader<bool, WUFIE_A>);
impl WUFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::B_0X0,
            true => WUFIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUFIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUFIE_A::B_0X1
    }
}
impl core::ops::Deref for WUFIE_R {
    type Target = crate::FieldReader<bool, WUFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub struct WUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUFIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever WUF = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUFIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "TXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    B_0X1 = 1,
}
impl From<TXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub struct TXFTIE_R(crate::FieldReader<bool, TXFTIE_A>);
impl TXFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFTIE_A {
        match self.bits {
            false => TXFTIE_A::B_0X0,
            true => TXFTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXFTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXFTIE_A::B_0X1
    }
}
impl core::ops::Deref for TXFTIE_R {
    type Target = crate::FieldReader<bool, TXFTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub struct TXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXFTIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXFTIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCBGTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<TCBGTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct TCBGTIE_R(crate::FieldReader<bool, TCBGTIE_A>);
impl TCBGTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCBGTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCBGTIE_A {
        match self.bits {
            false => TCBGTIE_A::B_0X0,
            true => TCBGTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TCBGTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TCBGTIE_A::B_0X1
    }
}
impl core::ops::Deref for TCBGTIE_R {
    type Target = crate::FieldReader<bool, TCBGTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub struct TCBGTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBGTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCBGTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCBGTIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever TCBGT=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCBGTIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Receive FIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFTCFG_A {
    #[doc = "0: Receive FIFO reaches 1/8 of its depth"]
    B_0X0 = 0,
    #[doc = "1: Receive FIFO reaches 1/4 of its depth"]
    B_0X1 = 1,
    #[doc = "2: Receive FIFO reaches 1/2 of its depth"]
    B_0X2 = 2,
    #[doc = "3: Receive FIFO reaches 3/4 of its depth"]
    B_0X3 = 3,
    #[doc = "4: Receive FIFO reaches 7/8 of its depth"]
    B_0X4 = 4,
    #[doc = "5: Receive FIFO becomes full"]
    B_0X5 = 5,
}
impl From<RXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub struct RXFTCFG_R(crate::FieldReader<u8, RXFTCFG_A>);
impl RXFTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFTCFG_A> {
        match self.bits {
            0 => Some(RXFTCFG_A::B_0X0),
            1 => Some(RXFTCFG_A::B_0X1),
            2 => Some(RXFTCFG_A::B_0X2),
            3 => Some(RXFTCFG_A::B_0X3),
            4 => Some(RXFTCFG_A::B_0X4),
            5 => Some(RXFTCFG_A::B_0X5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFTCFG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFTCFG_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == RXFTCFG_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == RXFTCFG_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == RXFTCFG_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == RXFTCFG_A::B_0X5
    }
}
impl core::ops::Deref for RXFTCFG_R {
    type Target = crate::FieldReader<u8, RXFTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub struct RXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Receive FIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X0)
    }
    #[doc = "Receive FIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X1)
    }
    #[doc = "Receive FIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X2)
    }
    #[doc = "Receive FIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X3)
    }
    #[doc = "Receive FIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X4)
    }
    #[doc = "Receive FIFO becomes full"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "RXFIFO threshold interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFTIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    B_0X1 = 1,
}
impl From<RXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub struct RXFTIE_R(crate::FieldReader<bool, RXFTIE_A>);
impl RXFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFTIE_A {
        match self.bits {
            false => RXFTIE_A::B_0X0,
            true => RXFTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFTIE_A::B_0X1
    }
}
impl core::ops::Deref for RXFTIE_R {
    type Target = crate::FieldReader<bool, RXFTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub struct RXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFTIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFTIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "TXFIFO threshold configuration Remaining combinations: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFTCFG_A {
    #[doc = "0: TXFIFO reaches 1/8 of its depth"]
    B_0X0 = 0,
    #[doc = "1: TXFIFO reaches 1/4 of its depth"]
    B_0X1 = 1,
    #[doc = "2: TXFIFO reaches 1/2 of its depth"]
    B_0X2 = 2,
    #[doc = "3: TXFIFO reaches 3/4 of its depth"]
    B_0X3 = 3,
    #[doc = "4: TXFIFO reaches 7/8 of its depth"]
    B_0X4 = 4,
    #[doc = "5: TXFIFO becomes empty"]
    B_0X5 = 5,
}
impl From<TXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub struct TXFTCFG_R(crate::FieldReader<u8, TXFTCFG_A>);
impl TXFTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFTCFG_A> {
        match self.bits {
            0 => Some(TXFTCFG_A::B_0X0),
            1 => Some(TXFTCFG_A::B_0X1),
            2 => Some(TXFTCFG_A::B_0X2),
            3 => Some(TXFTCFG_A::B_0X3),
            4 => Some(TXFTCFG_A::B_0X4),
            5 => Some(TXFTCFG_A::B_0X5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXFTCFG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXFTCFG_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TXFTCFG_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TXFTCFG_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == TXFTCFG_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == TXFTCFG_A::B_0X5
    }
}
impl core::ops::Deref for TXFTCFG_R {
    type Target = crate::FieldReader<u8, TXFTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub struct TXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TXFIFO reaches 1/8 of its depth"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X0)
    }
    #[doc = "TXFIFO reaches 1/4 of its depth"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X1)
    }
    #[doc = "TXFIFO reaches 1/2 of its depth"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X2)
    }
    #[doc = "TXFIFO reaches 3/4 of its depth"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X3)
    }
    #[doc = "TXFIFO reaches 7/8 of its depth"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X4)
    }
    #[doc = "TXFIFO becomes empty"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W { w: self }
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W { w: self }
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W { w: self }
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W { w: self }
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W { w: self }
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W {
        RTSE_W { w: self }
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W {
        CTSE_W { w: self }
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W {
        ONEBIT_W { w: self }
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W {
        OVRDIS_W { w: self }
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W {
        DDRE_W { w: self }
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W {
        DEM_W { w: self }
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W {
        SCARCNT_W { w: self }
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W {
        WUS_W { w: self }
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W {
        WUFIE_W { w: self }
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W {
        TXFTIE_W { w: self }
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W {
        TCBGTIE_W { w: self }
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W {
        RXFTCFG_W { w: self }
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W {
        RXFTIE_W { w: self }
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W {
        TXFTCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
