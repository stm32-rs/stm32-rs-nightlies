#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Locking the AF configuration of associated IOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOLOCK_A {
    #[doc = "0: IO configuration unlocked"]
    UNLOCKED = 0,
    #[doc = "1: IO configuration locked"]
    LOCKED = 1,
}
impl From<IOLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: IOLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IOLOCK`"]
pub type IOLOCK_R = crate::R<bool, IOLOCK_A>;
impl IOLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOLOCK_A {
        match self.bits {
            false => IOLOCK_A::UNLOCKED,
            true => IOLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == IOLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == IOLOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `IOLOCK`"]
pub struct IOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IO configuration unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(IOLOCK_A::UNLOCKED)
    }
    #[doc = "IO configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(IOLOCK_A::LOCKED)
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
#[doc = "CRC calculation initialization pattern control for transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRCINI_A {
    #[doc = "0: All zeros TX CRC initialization pattern"]
    ALLZEROS = 0,
    #[doc = "1: All ones TX CRC initialization pattern"]
    ALLONES = 1,
}
impl From<TCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: TCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCRCINI`"]
pub type TCRCINI_R = crate::R<bool, TCRCINI_A>;
impl TCRCINI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRCINI_A {
        match self.bits {
            false => TCRCINI_A::ALLZEROS,
            true => TCRCINI_A::ALLONES,
        }
    }
    #[doc = "Checks if the value of the field is `ALLZEROS`"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == TCRCINI_A::ALLZEROS
    }
    #[doc = "Checks if the value of the field is `ALLONES`"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == TCRCINI_A::ALLONES
    }
}
#[doc = "Write proxy for field `TCRCINI`"]
pub struct TCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRCINI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRCINI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All zeros TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut W {
        self.variant(TCRCINI_A::ALLZEROS)
    }
    #[doc = "All ones TX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut W {
        self.variant(TCRCINI_A::ALLONES)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "CRC calculation initialization pattern control for receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCRCINI_A {
    #[doc = "0: All zeros RX CRC initialization pattern"]
    ALLZEROS = 0,
    #[doc = "1: All ones RX CRC initialization pattern"]
    ALLONES = 1,
}
impl From<RCRCINI_A> for bool {
    #[inline(always)]
    fn from(variant: RCRCINI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RCRCINI`"]
pub type RCRCINI_R = crate::R<bool, RCRCINI_A>;
impl RCRCINI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRCINI_A {
        match self.bits {
            false => RCRCINI_A::ALLZEROS,
            true => RCRCINI_A::ALLONES,
        }
    }
    #[doc = "Checks if the value of the field is `ALLZEROS`"]
    #[inline(always)]
    pub fn is_all_zeros(&self) -> bool {
        *self == RCRCINI_A::ALLZEROS
    }
    #[doc = "Checks if the value of the field is `ALLONES`"]
    #[inline(always)]
    pub fn is_all_ones(&self) -> bool {
        *self == RCRCINI_A::ALLONES
    }
}
#[doc = "Write proxy for field `RCRCINI`"]
pub struct RCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRCINI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRCINI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All zeros RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_zeros(self) -> &'a mut W {
        self.variant(RCRCINI_A::ALLZEROS)
    }
    #[doc = "All ones RX CRC initialization pattern"]
    #[inline(always)]
    pub fn all_ones(self) -> &'a mut W {
        self.variant(RCRCINI_A::ALLONES)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "32-bit CRC polynomial configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC33_17_A {
    #[doc = "0: Full size (33/17 bit) CRC polynomial is not used"]
    DISABLED = 0,
    #[doc = "1: Full size (33/17 bit) CRC polynomial is used"]
    ENABLED = 1,
}
impl From<CRC33_17_A> for bool {
    #[inline(always)]
    fn from(variant: CRC33_17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC33_17`"]
pub type CRC33_17_R = crate::R<bool, CRC33_17_A>;
impl CRC33_17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC33_17_A {
        match self.bits {
            false => CRC33_17_A::DISABLED,
            true => CRC33_17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRC33_17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRC33_17_A::ENABLED
    }
}
#[doc = "Write proxy for field `CRC33_17`"]
pub struct CRC33_17_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC33_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC33_17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRC33_17_A::DISABLED)
    }
    #[doc = "Full size (33/17 bit) CRC polynomial is used"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRC33_17_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Internal SS signal input level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_A {
    #[doc = "0: 0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SLAVESELECTED = 0,
    #[doc = "1: 1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    SLAVENOTSELECTED = 1,
}
impl From<SSI_A> for bool {
    #[inline(always)]
    fn from(variant: SSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSI`"]
pub type SSI_R = crate::R<bool, SSI_A>;
impl SSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSI_A {
        match self.bits {
            false => SSI_A::SLAVESELECTED,
            true => SSI_A::SLAVENOTSELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVESELECTED`"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI_A::SLAVESELECTED
    }
    #[doc = "Checks if the value of the field is `SLAVENOTSELECTED`"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI_A::SLAVENOTSELECTED
    }
}
#[doc = "Write proxy for field `SSI`"]
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVESELECTED)
    }
    #[doc = "1 is forced onto the SS signal and the I/O value of the SS pin is ignored"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVENOTSELECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Rx/Tx direction at Half-duplex mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDDIR_A {
    #[doc = "0: Receiver in half duplex mode"]
    RECEIVER = 0,
    #[doc = "1: Transmitter in half duplex mode"]
    TRANSMITTER = 1,
}
impl From<HDDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HDDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDDIR`"]
pub type HDDIR_R = crate::R<bool, HDDIR_A>;
impl HDDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDDIR_A {
        match self.bits {
            false => HDDIR_A::RECEIVER,
            true => HDDIR_A::TRANSMITTER,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVER`"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == HDDIR_A::RECEIVER
    }
    #[doc = "Checks if the value of the field is `TRANSMITTER`"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == HDDIR_A::TRANSMITTER
    }
}
#[doc = "Write proxy for field `HDDIR`"]
pub struct HDDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver in half duplex mode"]
    #[inline(always)]
    pub fn receiver(self) -> &'a mut W {
        self.variant(HDDIR_A::RECEIVER)
    }
    #[doc = "Transmitter in half duplex mode"]
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut W {
        self.variant(HDDIR_A::TRANSMITTER)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Master SUSPend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSUSP_AW {
    #[doc = "0: Do not request master suspend"]
    NOTREQUESTED = 0,
    #[doc = "1: Request master suspend"]
    REQUESTED = 1,
}
impl From<CSUSP_AW> for bool {
    #[inline(always)]
    fn from(variant: CSUSP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CSUSP`"]
pub struct CSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSUSP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not request master suspend"]
    #[inline(always)]
    pub fn not_requested(self) -> &'a mut W {
        self.variant(CSUSP_AW::NOTREQUESTED)
    }
    #[doc = "Request master suspend"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(CSUSP_AW::REQUESTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Master transfer start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTART_A {
    #[doc = "0: Do not start master transfer"]
    NOTSTARTED = 0,
    #[doc = "1: Start master transfer"]
    STARTED = 1,
}
impl From<CSTART_A> for bool {
    #[inline(always)]
    fn from(variant: CSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSTART`"]
pub type CSTART_R = crate::R<bool, CSTART_A>;
impl CSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTART_A {
        match self.bits {
            false => CSTART_A::NOTSTARTED,
            true => CSTART_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == CSTART_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == CSTART_A::STARTED
    }
}
#[doc = "Write proxy for field `CSTART`"]
pub struct CSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not start master transfer"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(CSTART_A::NOTSTARTED)
    }
    #[doc = "Start master transfer"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(CSTART_A::STARTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Master automatic SUSP in Receive mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASRX_A {
    #[doc = "0: Automatic suspend in master receive-only mode disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic suspend in master receive-only mode enabled"]
    ENABLED = 1,
}
impl From<MASRX_A> for bool {
    #[inline(always)]
    fn from(variant: MASRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASRX`"]
pub type MASRX_R = crate::R<bool, MASRX_A>;
impl MASRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASRX_A {
        match self.bits {
            false => MASRX_A::DISABLED,
            true => MASRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `MASRX`"]
pub struct MASRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MASRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic suspend in master receive-only mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASRX_A::DISABLED)
    }
    #[doc = "Automatic suspend in master receive-only mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASRX_A::ENABLED)
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
#[doc = "Serial Peripheral Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, SPE_A>;
impl SPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::DISABLED,
            true => SPE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W {
        IOLOCK_W { w: self }
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W {
        TCRCINI_W { w: self }
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W {
        RCRCINI_W { w: self }
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W {
        CRC33_17_W { w: self }
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W {
        HDDIR_W { w: self }
    }
    #[doc = "Bit 10 - Master SUSPend request"]
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W {
        CSUSP_W { w: self }
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W {
        CSTART_W { w: self }
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W {
        MASRX_W { w: self }
    }
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
}
