#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0x0007_0007"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_0007
    }
}
#[doc = "Master baud rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBR_A {
    #[doc = "0: f_spi_ker_ck / 2"]
    DIV2 = 0,
    #[doc = "1: f_spi_ker_ck / 4"]
    DIV4 = 1,
    #[doc = "2: f_spi_ker_ck / 8"]
    DIV8 = 2,
    #[doc = "3: f_spi_ker_ck / 16"]
    DIV16 = 3,
    #[doc = "4: f_spi_ker_ck / 32"]
    DIV32 = 4,
    #[doc = "5: f_spi_ker_ck / 64"]
    DIV64 = 5,
    #[doc = "6: f_spi_ker_ck / 128"]
    DIV128 = 6,
    #[doc = "7: f_spi_ker_ck / 256"]
    DIV256 = 7,
}
impl From<MBR_A> for u8 {
    #[inline(always)]
    fn from(variant: MBR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MBR`"]
pub type MBR_R = crate::R<u8, MBR_A>;
impl MBR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBR_A {
        match self.bits {
            0 => MBR_A::DIV2,
            1 => MBR_A::DIV4,
            2 => MBR_A::DIV8,
            3 => MBR_A::DIV16,
            4 => MBR_A::DIV32,
            5 => MBR_A::DIV64,
            6 => MBR_A::DIV128,
            7 => MBR_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MBR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MBR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MBR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MBR_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MBR_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MBR_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MBR_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == MBR_A::DIV256
    }
}
#[doc = "Write proxy for field `MBR`"]
pub struct MBR_W<'a> {
    w: &'a mut W,
}
impl<'a> MBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "f_spi_ker_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MBR_A::DIV2)
    }
    #[doc = "f_spi_ker_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MBR_A::DIV4)
    }
    #[doc = "f_spi_ker_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MBR_A::DIV8)
    }
    #[doc = "f_spi_ker_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MBR_A::DIV16)
    }
    #[doc = "f_spi_ker_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MBR_A::DIV32)
    }
    #[doc = "f_spi_ker_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MBR_A::DIV64)
    }
    #[doc = "f_spi_ker_ck / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MBR_A::DIV128)
    }
    #[doc = "f_spi_ker_ck / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(MBR_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Hardware CRC computation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: CRC calculation enabled"]
    ENABLED = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, CRCEN_A>;
impl CRCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CRCSIZE`"]
pub type CRCSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCSIZE`"]
pub struct CRCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Tx DMA stream enable\n\nValue on reset: 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Rx DMA stream enable\n\nValue on reset: 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Detection of underrun condition at slave transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UDRDET_A {
    #[doc = "0: Underrun is detected at begin of data frame"]
    STARTOFFRAME = 0,
    #[doc = "1: Underrun is detected at end of last data frame"]
    ENDOFFRAME = 1,
    #[doc = "2: Underrun is detected at begin of active SS signal"]
    STARTOFSLAVESELECT = 2,
}
impl From<UDRDET_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRDET_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UDRDET`"]
pub type UDRDET_R = crate::R<u8, UDRDET_A>;
impl UDRDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UDRDET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UDRDET_A::STARTOFFRAME),
            1 => Val(UDRDET_A::ENDOFFRAME),
            2 => Val(UDRDET_A::STARTOFSLAVESELECT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STARTOFFRAME`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == UDRDET_A::STARTOFFRAME
    }
    #[doc = "Checks if the value of the field is `ENDOFFRAME`"]
    #[inline(always)]
    pub fn is_end_of_frame(&self) -> bool {
        *self == UDRDET_A::ENDOFFRAME
    }
    #[doc = "Checks if the value of the field is `STARTOFSLAVESELECT`"]
    #[inline(always)]
    pub fn is_start_of_slave_select(&self) -> bool {
        *self == UDRDET_A::STARTOFSLAVESELECT
    }
}
#[doc = "Write proxy for field `UDRDET`"]
pub struct UDRDET_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDRDET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Underrun is detected at begin of data frame"]
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::STARTOFFRAME)
    }
    #[doc = "Underrun is detected at end of last data frame"]
    #[inline(always)]
    pub fn end_of_frame(self) -> &'a mut W {
        self.variant(UDRDET_A::ENDOFFRAME)
    }
    #[doc = "Underrun is detected at begin of active SS signal"]
    #[inline(always)]
    pub fn start_of_slave_select(self) -> &'a mut W {
        self.variant(UDRDET_A::STARTOFSLAVESELECT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Behavior of slave transmitter at underrun condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UDRCFG_A {
    #[doc = "0: Slave sends a constant underrun pattern"]
    CONSTANT = 0,
    #[doc = "1: Slave repeats last received data frame from master"]
    REPEATRECEIVED = 1,
    #[doc = "2: Slave repeats last transmitted data frame"]
    REPEATTRANSMITTED = 2,
}
impl From<UDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: UDRCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UDRCFG`"]
pub type UDRCFG_R = crate::R<u8, UDRCFG_A>;
impl UDRCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UDRCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UDRCFG_A::CONSTANT),
            1 => Val(UDRCFG_A::REPEATRECEIVED),
            2 => Val(UDRCFG_A::REPEATTRANSMITTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT`"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == UDRCFG_A::CONSTANT
    }
    #[doc = "Checks if the value of the field is `REPEATRECEIVED`"]
    #[inline(always)]
    pub fn is_repeat_received(&self) -> bool {
        *self == UDRCFG_A::REPEATRECEIVED
    }
    #[doc = "Checks if the value of the field is `REPEATTRANSMITTED`"]
    #[inline(always)]
    pub fn is_repeat_transmitted(&self) -> bool {
        *self == UDRCFG_A::REPEATTRANSMITTED
    }
}
#[doc = "Write proxy for field `UDRCFG`"]
pub struct UDRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDRCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slave sends a constant underrun pattern"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut W {
        self.variant(UDRCFG_A::CONSTANT)
    }
    #[doc = "Slave repeats last received data frame from master"]
    #[inline(always)]
    pub fn repeat_received(self) -> &'a mut W {
        self.variant(UDRCFG_A::REPEATRECEIVED)
    }
    #[doc = "Slave repeats last transmitted data frame"]
    #[inline(always)]
    pub fn repeat_transmitted(self) -> &'a mut W {
        self.variant(UDRCFG_A::REPEATTRANSMITTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "threshold level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTHLV_A {
    #[doc = "0: 1 frame"]
    ONEFRAME = 0,
    #[doc = "1: 2 frames"]
    TWOFRAMES = 1,
    #[doc = "2: 3 frames"]
    THREEFRAMES = 2,
    #[doc = "3: 4 frames"]
    FOURFRAMES = 3,
    #[doc = "4: 5 frames"]
    FIVEFRAMES = 4,
    #[doc = "5: 6 frames"]
    SIXFRAMES = 5,
    #[doc = "6: 7 frames"]
    SEVENFRAMES = 6,
    #[doc = "7: 8 frames"]
    EIGHTFRAMES = 7,
    #[doc = "8: 9 frames"]
    NINEFRAMES = 8,
    #[doc = "9: 10 frames"]
    TENFRAMES = 9,
    #[doc = "10: 11 frames"]
    ELEVENFRAMES = 10,
    #[doc = "11: 12 frames"]
    TWELVEFRAMES = 11,
    #[doc = "12: 13 frames"]
    THIRTEENFRAMES = 12,
    #[doc = "13: 14 frames"]
    FOURTEENFRAMES = 13,
    #[doc = "14: 15 frames"]
    FIFTEENFRAMES = 14,
    #[doc = "15: 16 frames"]
    SIXTEENFRAMES = 15,
}
impl From<FTHLV_A> for u8 {
    #[inline(always)]
    fn from(variant: FTHLV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTHLV`"]
pub type FTHLV_R = crate::R<u8, FTHLV_A>;
impl FTHLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTHLV_A {
        match self.bits {
            0 => FTHLV_A::ONEFRAME,
            1 => FTHLV_A::TWOFRAMES,
            2 => FTHLV_A::THREEFRAMES,
            3 => FTHLV_A::FOURFRAMES,
            4 => FTHLV_A::FIVEFRAMES,
            5 => FTHLV_A::SIXFRAMES,
            6 => FTHLV_A::SEVENFRAMES,
            7 => FTHLV_A::EIGHTFRAMES,
            8 => FTHLV_A::NINEFRAMES,
            9 => FTHLV_A::TENFRAMES,
            10 => FTHLV_A::ELEVENFRAMES,
            11 => FTHLV_A::TWELVEFRAMES,
            12 => FTHLV_A::THIRTEENFRAMES,
            13 => FTHLV_A::FOURTEENFRAMES,
            14 => FTHLV_A::FIFTEENFRAMES,
            15 => FTHLV_A::SIXTEENFRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONEFRAME`"]
    #[inline(always)]
    pub fn is_one_frame(&self) -> bool {
        *self == FTHLV_A::ONEFRAME
    }
    #[doc = "Checks if the value of the field is `TWOFRAMES`"]
    #[inline(always)]
    pub fn is_two_frames(&self) -> bool {
        *self == FTHLV_A::TWOFRAMES
    }
    #[doc = "Checks if the value of the field is `THREEFRAMES`"]
    #[inline(always)]
    pub fn is_three_frames(&self) -> bool {
        *self == FTHLV_A::THREEFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURFRAMES`"]
    #[inline(always)]
    pub fn is_four_frames(&self) -> bool {
        *self == FTHLV_A::FOURFRAMES
    }
    #[doc = "Checks if the value of the field is `FIVEFRAMES`"]
    #[inline(always)]
    pub fn is_five_frames(&self) -> bool {
        *self == FTHLV_A::FIVEFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXFRAMES`"]
    #[inline(always)]
    pub fn is_six_frames(&self) -> bool {
        *self == FTHLV_A::SIXFRAMES
    }
    #[doc = "Checks if the value of the field is `SEVENFRAMES`"]
    #[inline(always)]
    pub fn is_seven_frames(&self) -> bool {
        *self == FTHLV_A::SEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `EIGHTFRAMES`"]
    #[inline(always)]
    pub fn is_eight_frames(&self) -> bool {
        *self == FTHLV_A::EIGHTFRAMES
    }
    #[doc = "Checks if the value of the field is `NINEFRAMES`"]
    #[inline(always)]
    pub fn is_nine_frames(&self) -> bool {
        *self == FTHLV_A::NINEFRAMES
    }
    #[doc = "Checks if the value of the field is `TENFRAMES`"]
    #[inline(always)]
    pub fn is_ten_frames(&self) -> bool {
        *self == FTHLV_A::TENFRAMES
    }
    #[doc = "Checks if the value of the field is `ELEVENFRAMES`"]
    #[inline(always)]
    pub fn is_eleven_frames(&self) -> bool {
        *self == FTHLV_A::ELEVENFRAMES
    }
    #[doc = "Checks if the value of the field is `TWELVEFRAMES`"]
    #[inline(always)]
    pub fn is_twelve_frames(&self) -> bool {
        *self == FTHLV_A::TWELVEFRAMES
    }
    #[doc = "Checks if the value of the field is `THIRTEENFRAMES`"]
    #[inline(always)]
    pub fn is_thirteen_frames(&self) -> bool {
        *self == FTHLV_A::THIRTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FOURTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fourteen_frames(&self) -> bool {
        *self == FTHLV_A::FOURTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `FIFTEENFRAMES`"]
    #[inline(always)]
    pub fn is_fifteen_frames(&self) -> bool {
        *self == FTHLV_A::FIFTEENFRAMES
    }
    #[doc = "Checks if the value of the field is `SIXTEENFRAMES`"]
    #[inline(always)]
    pub fn is_sixteen_frames(&self) -> bool {
        *self == FTHLV_A::SIXTEENFRAMES
    }
}
#[doc = "Write proxy for field `FTHLV`"]
pub struct FTHLV_W<'a> {
    w: &'a mut W,
}
impl<'a> FTHLV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTHLV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 frame"]
    #[inline(always)]
    pub fn one_frame(self) -> &'a mut W {
        self.variant(FTHLV_A::ONEFRAME)
    }
    #[doc = "2 frames"]
    #[inline(always)]
    pub fn two_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TWOFRAMES)
    }
    #[doc = "3 frames"]
    #[inline(always)]
    pub fn three_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::THREEFRAMES)
    }
    #[doc = "4 frames"]
    #[inline(always)]
    pub fn four_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FOURFRAMES)
    }
    #[doc = "5 frames"]
    #[inline(always)]
    pub fn five_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FIVEFRAMES)
    }
    #[doc = "6 frames"]
    #[inline(always)]
    pub fn six_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SIXFRAMES)
    }
    #[doc = "7 frames"]
    #[inline(always)]
    pub fn seven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SEVENFRAMES)
    }
    #[doc = "8 frames"]
    #[inline(always)]
    pub fn eight_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::EIGHTFRAMES)
    }
    #[doc = "9 frames"]
    #[inline(always)]
    pub fn nine_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::NINEFRAMES)
    }
    #[doc = "10 frames"]
    #[inline(always)]
    pub fn ten_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TENFRAMES)
    }
    #[doc = "11 frames"]
    #[inline(always)]
    pub fn eleven_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::ELEVENFRAMES)
    }
    #[doc = "12 frames"]
    #[inline(always)]
    pub fn twelve_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::TWELVEFRAMES)
    }
    #[doc = "13 frames"]
    #[inline(always)]
    pub fn thirteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::THIRTEENFRAMES)
    }
    #[doc = "14 frames"]
    #[inline(always)]
    pub fn fourteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FOURTEENFRAMES)
    }
    #[doc = "15 frames"]
    #[inline(always)]
    pub fn fifteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::FIFTEENFRAMES)
    }
    #[doc = "16 frames"]
    #[inline(always)]
    pub fn sixteen_frames(self) -> &'a mut W {
        self.variant(FTHLV_A::SIXTEENFRAMES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&mut self) -> MBR_W {
        MBR_W { w: self }
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&mut self) -> CRCSIZE_W {
        CRCSIZE_W { w: self }
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&mut self) -> UDRDET_W {
        UDRDET_W { w: self }
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&mut self) -> UDRCFG_W {
        UDRCFG_W { w: self }
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthlv(&mut self) -> FTHLV_W {
        FTHLV_W { w: self }
    }
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
}
