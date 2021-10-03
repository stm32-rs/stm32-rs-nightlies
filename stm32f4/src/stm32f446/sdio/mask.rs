#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAILIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<CCRCFAILIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub struct CCRCFAILIE_R(crate::FieldReader<bool, CCRCFAILIE_A>);
impl CCRCFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAILIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRCFAILIE_A {
        match self.bits {
            false => CCRCFAILIE_A::DISABLED,
            true => CCRCFAILIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CCRCFAILIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CCRCFAILIE_A::ENABLED
    }
}
impl core::ops::Deref for CCRCFAILIE_R {
    type Target = crate::FieldReader<bool, CCRCFAILIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub struct CCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCFAILIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCRCFAILIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCRCFAILIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCRCFAILIE_A::ENABLED)
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
#[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DCRCFAILIE_A = CCRCFAILIE_A;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DCRCFAILIE_R = CCRCFAILIE_R;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub struct DCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCFAILIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCRCFAILIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCRCFAILIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCRCFAILIE_A::ENABLED)
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
#[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CTIMEOUTIE_A = CCRCFAILIE_A;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CTIMEOUTIE_R = CCRCFAILIE_R;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub struct CTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMEOUTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMEOUTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIMEOUTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIMEOUTIE_A::ENABLED)
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
#[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DTIMEOUTIE_A = CCRCFAILIE_A;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DTIMEOUTIE_R = CCRCFAILIE_R;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub struct DTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIMEOUTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTIMEOUTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTIMEOUTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTIMEOUTIE_A::ENABLED)
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
#[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TXUNDERRIE_A = CCRCFAILIE_A;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TXUNDERRIE_R = CCRCFAILIE_R;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub struct TXUNDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUNDERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXUNDERRIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXUNDERRIE_A::ENABLED)
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
#[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RXOVERRIE_A = CCRCFAILIE_A;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RXOVERRIE_R = CCRCFAILIE_R;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub struct RXOVERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXOVERRIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXOVERRIE_A::ENABLED)
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
#[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CMDRENDIE_A = CCRCFAILIE_A;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CMDRENDIE_R = CCRCFAILIE_R;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub struct CMDRENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDRENDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMDRENDIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDRENDIE_A::ENABLED)
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
#[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CMDSENTIE_A = CCRCFAILIE_A;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CMDSENTIE_R = CCRCFAILIE_R;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub struct CMDSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDSENTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMDSENTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDSENTIE_A::ENABLED)
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
#[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DATAENDIE_A = CCRCFAILIE_A;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DATAENDIE_R = CCRCFAILIE_R;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub struct DATAENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAENDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DATAENDIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DATAENDIE_A::ENABLED)
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
#[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DBCKENDIE_A = CCRCFAILIE_A;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DBCKENDIE_R = CCRCFAILIE_R;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub struct DBCKENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCKENDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBCKENDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBCKENDIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBCKENDIE_A::ENABLED)
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
#[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type CMDACTIE_A = CCRCFAILIE_A;
#[doc = "Field `CMDACTIE` reader - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type CMDACTIE_R = CCRCFAILIE_R;
#[doc = "Field `CMDACTIE` writer - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub struct CMDACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDACTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDACTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMDACTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMDACTIE_A::ENABLED)
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
#[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TXFIFOHEIE_A = CCRCFAILIE_A;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TXFIFOHEIE_R = CCRCFAILIE_R;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub struct TXFIFOHEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOHEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFOHEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIFOHEIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIFOHEIE_A::ENABLED)
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
#[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RXFIFOHFIE_A = CCRCFAILIE_A;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RXFIFOHFIE_R = CCRCFAILIE_R;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub struct RXFIFOHFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOHFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFOHFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFIFOHFIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFIFOHFIE_A::ENABLED)
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
#[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RXFIFOFIE_A = CCRCFAILIE_A;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RXFIFOFIE_R = CCRCFAILIE_R;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub struct RXFIFOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFOFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFIFOFIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFIFOFIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TXFIFOEIE_A = CCRCFAILIE_A;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TXFIFOEIE_R = CCRCFAILIE_R;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub struct TXFIFOEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFOEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIFOEIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIFOEIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type RXDAVLIE_A = CCRCFAILIE_A;
#[doc = "Field `RXDAVLIE` reader - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type RXDAVLIE_R = CCRCFAILIE_R;
#[doc = "Field `RXDAVLIE` writer - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub struct RXDAVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDAVLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDAVLIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDAVLIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDAVLIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SDIOITIE_A = CCRCFAILIE_A;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SDIOITIE_R = CCRCFAILIE_R;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub struct SDIOITIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOITIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOITIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOITIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOITIE_A::ENABLED)
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
#[doc = "Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub type TXDAVLIE_A = CCRCFAILIE_A;
#[doc = "Field `TXDAVLIE` reader - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub type TXDAVLIE_R = CCRCFAILIE_R;
#[doc = "Field `TXDAVLIE` writer - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub struct TXDAVLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDAVLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDAVLIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDAVLIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDAVLIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub type RXFIFOEIE_A = CCRCFAILIE_A;
#[doc = "Field `RXFIFOEIE` reader - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub type RXFIFOEIE_R = CCRCFAILIE_R;
#[doc = "Field `RXFIFOEIE` writer - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub struct RXFIFOEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFOEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFIFOEIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFIFOEIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub type TXFIFOFIE_A = CCRCFAILIE_A;
#[doc = "Field `TXFIFOFIE` reader - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub type TXFIFOFIE_R = CCRCFAILIE_R;
#[doc = "Field `TXFIFOFIE` writer - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub struct TXFIFOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFOFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIFOFIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIFOFIE_A::ENABLED)
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
#[doc = "Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub type RXACTIE_A = CCRCFAILIE_A;
#[doc = "Field `RXACTIE` reader - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub type RXACTIE_R = CCRCFAILIE_R;
#[doc = "Field `RXACTIE` writer - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub struct RXACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXACTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXACTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXACTIE_A::ENABLED)
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
#[doc = "Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub type TXACTIE_A = CCRCFAILIE_A;
#[doc = "Field `TXACTIE` reader - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub type TXACTIE_R = CCRCFAILIE_R;
#[doc = "Field `TXACTIE` writer - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub struct TXACTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXACTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXACTIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXACTIE_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W {
        CCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W {
        DCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W {
        CTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W {
        DTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W {
        TXUNDERRIE_W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W {
        RXOVERRIE_W { w: self }
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W {
        CMDRENDIE_W { w: self }
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W {
        CMDSENTIE_W { w: self }
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W {
        DATAENDIE_W { w: self }
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W {
        DBCKENDIE_W { w: self }
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W {
        CMDACTIE_W { w: self }
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W {
        TXFIFOHEIE_W { w: self }
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W {
        RXFIFOHFIE_W { w: self }
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W {
        RXFIFOFIE_W { w: self }
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W {
        TXFIFOEIE_W { w: self }
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W {
        RXDAVLIE_W { w: self }
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W {
        SDIOITIE_W { w: self }
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W {
        TXDAVLIE_W { w: self }
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W {
        RXFIFOEIE_W { w: self }
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W {
        TXFIFOFIE_W { w: self }
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W {
        RXACTIE_W { w: self }
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W {
        TXACTIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
