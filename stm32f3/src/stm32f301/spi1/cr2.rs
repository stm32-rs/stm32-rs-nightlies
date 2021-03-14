#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDMAEN`"]
pub type RXDMAEN_R = crate::R<bool, RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXDMAEN`"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
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
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDMAEN`"]
pub type TXDMAEN_R = crate::R<bool, TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXDMAEN`"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    DISABLED = 0,
    #[doc = "1: SS output is enabled in master mode"]
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSOE`"]
pub type SSOE_R = crate::R<bool, SSOE_A>;
impl SSOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SSOE`"]
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "NSS pulse management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSP_A {
    #[doc = "0: No NSS pulse"]
    NOPULSE = 0,
    #[doc = "1: NSS pulse generated"]
    PULSEGENERATED = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSSP`"]
pub type NSSP_R = crate::R<bool, NSSP_A>;
impl NSSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::NOPULSE,
            true => NSSP_A::PULSEGENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOPULSE`"]
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSP_A::NOPULSE
    }
    #[doc = "Checks if the value of the field is `PULSEGENERATED`"]
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSP_A::PULSEGENERATED
    }
}
#[doc = "Write proxy for field `NSSP`"]
pub struct NSSP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSP_A::NOPULSE)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut W {
        self.variant(NSSP_A::PULSEGENERATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    MOTOROLA = 0,
    #[doc = "1: SPI TI mode"]
    TI = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<bool, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::MOTOROLA,
            true => FRF_A::TI,
        }
    }
    #[doc = "Checks if the value of the field is `MOTOROLA`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF_A::MOTOROLA
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::TI
    }
}
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt masked"]
    MASKED = 0,
    #[doc = "1: Error interrupt not masked"]
    NOTMASKED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::MASKED,
            true => ERRIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::MASKED)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    #[doc = "0: RXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: RXE interrupt not masked"]
    NOTMASKED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXNEIE`"]
pub type RXNEIE_R = crate::R<bool, RXNEIE_A>;
impl RXNEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::MASKED,
            true => RXNEIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `RXNEIE`"]
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::MASKED)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: TXE interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEIE`"]
pub type TXEIE_R = crate::R<bool, TXEIE_A>;
impl TXEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::MASKED,
            true => TXEIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `TXEIE`"]
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::MASKED)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DS_A {
    #[doc = "3: 4-bit"]
    FOURBIT = 3,
    #[doc = "4: 5-bit"]
    FIVEBIT = 4,
    #[doc = "5: 6-bit"]
    SIXBIT = 5,
    #[doc = "6: 7-bit"]
    SEVENBIT = 6,
    #[doc = "7: 8-bit"]
    EIGHTBIT = 7,
    #[doc = "8: 9-bit"]
    NINEBIT = 8,
    #[doc = "9: 10-bit"]
    TENBIT = 9,
    #[doc = "10: 11-bit"]
    ELEVENBIT = 10,
    #[doc = "11: 12-bit"]
    TWELVEBIT = 11,
    #[doc = "12: 13-bit"]
    THIRTEENBIT = 12,
    #[doc = "13: 14-bit"]
    FOURTEENBIT = 13,
    #[doc = "14: 15-bit"]
    FIFTEENBIT = 14,
    #[doc = "15: 16-bit"]
    SIXTEENBIT = 15,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, DS_A>;
impl DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DS_A::FOURBIT),
            4 => Val(DS_A::FIVEBIT),
            5 => Val(DS_A::SIXBIT),
            6 => Val(DS_A::SEVENBIT),
            7 => Val(DS_A::EIGHTBIT),
            8 => Val(DS_A::NINEBIT),
            9 => Val(DS_A::TENBIT),
            10 => Val(DS_A::ELEVENBIT),
            11 => Val(DS_A::TWELVEBIT),
            12 => Val(DS_A::THIRTEENBIT),
            13 => Val(DS_A::FOURTEENBIT),
            14 => Val(DS_A::FIFTEENBIT),
            15 => Val(DS_A::SIXTEENBIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOURBIT`"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == DS_A::FOURBIT
    }
    #[doc = "Checks if the value of the field is `FIVEBIT`"]
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == DS_A::FIVEBIT
    }
    #[doc = "Checks if the value of the field is `SIXBIT`"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == DS_A::SIXBIT
    }
    #[doc = "Checks if the value of the field is `SEVENBIT`"]
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == DS_A::SEVENBIT
    }
    #[doc = "Checks if the value of the field is `EIGHTBIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == DS_A::EIGHTBIT
    }
    #[doc = "Checks if the value of the field is `NINEBIT`"]
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == DS_A::NINEBIT
    }
    #[doc = "Checks if the value of the field is `TENBIT`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == DS_A::TENBIT
    }
    #[doc = "Checks if the value of the field is `ELEVENBIT`"]
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DS_A::ELEVENBIT
    }
    #[doc = "Checks if the value of the field is `TWELVEBIT`"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DS_A::TWELVEBIT
    }
    #[doc = "Checks if the value of the field is `THIRTEENBIT`"]
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DS_A::THIRTEENBIT
    }
    #[doc = "Checks if the value of the field is `FOURTEENBIT`"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DS_A::FOURTEENBIT
    }
    #[doc = "Checks if the value of the field is `FIFTEENBIT`"]
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DS_A::FIFTEENBIT
    }
    #[doc = "Checks if the value of the field is `SIXTEENBIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DS_A::SIXTEENBIT
    }
}
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(DS_A::FOURBIT)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(DS_A::FIVEBIT)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(DS_A::SIXBIT)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(DS_A::SEVENBIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(DS_A::EIGHTBIT)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(DS_A::NINEBIT)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(DS_A::TENBIT)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut W {
        self.variant(DS_A::ELEVENBIT)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(DS_A::TWELVEBIT)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut W {
        self.variant(DS_A::THIRTEENBIT)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FOURTEENBIT)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FIFTEENBIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DS_A::SIXTEENBIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "FIFO reception threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRXTH_A {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    HALF = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    QUARTER = 1,
}
impl From<FRXTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRXTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRXTH`"]
pub type FRXTH_R = crate::R<bool, FRXTH_A>;
impl FRXTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRXTH_A {
        match self.bits {
            false => FRXTH_A::HALF,
            true => FRXTH_A::QUARTER,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRXTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTH_A::QUARTER
    }
}
#[doc = "Write proxy for field `FRXTH`"]
pub struct FRXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRXTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRXTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FRXTH_A::HALF)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FRXTH_A::QUARTER)
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
#[doc = "Last DMA transfer for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_RX_A {
    #[doc = "0: Number of data to transfer for receive is even"]
    EVEN = 0,
    #[doc = "1: Number of data to transfer for receive is odd"]
    ODD = 1,
}
impl From<LDMA_RX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDMA_RX`"]
pub type LDMA_RX_R = crate::R<bool, LDMA_RX_A>;
impl LDMA_RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_RX_A {
        match self.bits {
            false => LDMA_RX_A::EVEN,
            true => LDMA_RX_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RX_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RX_A::ODD
    }
}
#[doc = "Write proxy for field `LDMA_RX`"]
pub struct LDMA_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDMA_RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Number of data to transfer for receive is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RX_A::EVEN)
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RX_A::ODD)
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
#[doc = "Last DMA transfer for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_TX_A {
    #[doc = "0: Number of data to transfer for transmit is even"]
    EVEN = 0,
    #[doc = "1: Number of data to transfer for transmit is odd"]
    ODD = 1,
}
impl From<LDMA_TX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDMA_TX`"]
pub type LDMA_TX_R = crate::R<bool, LDMA_TX_A>;
impl LDMA_TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_TX_A {
        match self.bits {
            false => LDMA_TX_A::EVEN,
            true => LDMA_TX_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TX_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TX_A::ODD
    }
}
#[doc = "Write proxy for field `LDMA_TX`"]
pub struct LDMA_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDMA_TX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TX_A::EVEN)
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TX_A::ODD)
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
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W { w: self }
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W {
        FRXTH_W { w: self }
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W {
        LDMA_RX_W { w: self }
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W {
        LDMA_TX_W { w: self }
    }
}
