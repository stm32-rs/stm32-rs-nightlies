#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
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
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Recalibration pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALPF_A {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    PENDING = 1,
}
impl From<RECALPF_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub struct RECALPF_R(crate::FieldReader<bool, RECALPF_A>);
impl RECALPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECALPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPF_A> {
        match self.bits {
            true => Some(RECALPF_A::PENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECALPF_A::PENDING
    }
}
impl core::ops::Deref for RECALPF_R {
    type Target = crate::FieldReader<bool, RECALPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "BCD update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCDU_A {
    #[doc = "0: 1s increment each time SS\\[7:0\\]=0"]
    BIT7 = 0,
    #[doc = "1: 1s increment each time SS\\[8:0\\]=0"]
    BIT8 = 1,
    #[doc = "2: 1s increment each time SS\\[9:0\\]=0"]
    BIT9 = 2,
    #[doc = "3: 1s increment each time SS\\[10:0\\]=0"]
    BIT10 = 3,
    #[doc = "4: 1s increment each time SS\\[11:0\\]=0"]
    BIT11 = 4,
    #[doc = "5: 1s increment each time SS\\[12:0\\]=0"]
    BIT12 = 5,
    #[doc = "6: 1s increment each time SS\\[13:0\\]=0"]
    BIT13 = 6,
    #[doc = "7: 1s increment each time SS\\[14:0\\]=0"]
    BIT14 = 7,
}
impl From<BCDU_A> for u8 {
    #[inline(always)]
    fn from(variant: BCDU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BCDU` reader - BCD update"]
pub struct BCDU_R(crate::FieldReader<u8, BCDU_A>);
impl BCDU_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCDU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDU_A {
        match self.bits {
            0 => BCDU_A::BIT7,
            1 => BCDU_A::BIT8,
            2 => BCDU_A::BIT9,
            3 => BCDU_A::BIT10,
            4 => BCDU_A::BIT11,
            5 => BCDU_A::BIT12,
            6 => BCDU_A::BIT13,
            7 => BCDU_A::BIT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        **self == BCDU_A::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        **self == BCDU_A::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT9`"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        **self == BCDU_A::BIT9
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        **self == BCDU_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT11`"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        **self == BCDU_A::BIT11
    }
    #[doc = "Checks if the value of the field is `BIT12`"]
    #[inline(always)]
    pub fn is_bit12(&self) -> bool {
        **self == BCDU_A::BIT12
    }
    #[doc = "Checks if the value of the field is `BIT13`"]
    #[inline(always)]
    pub fn is_bit13(&self) -> bool {
        **self == BCDU_A::BIT13
    }
    #[doc = "Checks if the value of the field is `BIT14`"]
    #[inline(always)]
    pub fn is_bit14(&self) -> bool {
        **self == BCDU_A::BIT14
    }
}
impl core::ops::Deref for BCDU_R {
    type Target = crate::FieldReader<u8, BCDU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDU` writer - BCD update"]
pub struct BCDU_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCDU_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1s increment each time SS\\[7:0\\]=0"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(BCDU_A::BIT7)
    }
    #[doc = "1s increment each time SS\\[8:0\\]=0"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(BCDU_A::BIT8)
    }
    #[doc = "1s increment each time SS\\[9:0\\]=0"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(BCDU_A::BIT9)
    }
    #[doc = "1s increment each time SS\\[10:0\\]=0"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(BCDU_A::BIT10)
    }
    #[doc = "1s increment each time SS\\[11:0\\]=0"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut W {
        self.variant(BCDU_A::BIT11)
    }
    #[doc = "1s increment each time SS\\[12:0\\]=0"]
    #[inline(always)]
    pub fn bit12(self) -> &'a mut W {
        self.variant(BCDU_A::BIT12)
    }
    #[doc = "1s increment each time SS\\[13:0\\]=0"]
    #[inline(always)]
    pub fn bit13(self) -> &'a mut W {
        self.variant(BCDU_A::BIT13)
    }
    #[doc = "1s increment each time SS\\[14:0\\]=0"]
    #[inline(always)]
    pub fn bit14(self) -> &'a mut W {
        self.variant(BCDU_A::BIT14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Binary mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIN_A {
    #[doc = "0: Free running BCD calendar mode (Binary mode disabled)"]
    BCD = 0,
    #[doc = "1: Free running Binary mode (BCD mode disabled)"]
    BINARY = 1,
    #[doc = "2: Free running BCD calendar and Binary modes"]
    BINBCD = 2,
    #[doc = "3: Free running BCD calendar and Binary modes"]
    BINBCD2 = 3,
}
impl From<BIN_A> for u8 {
    #[inline(always)]
    fn from(variant: BIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIN` reader - Binary mode"]
pub struct BIN_R(crate::FieldReader<u8, BIN_A>);
impl BIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIN_A {
        match self.bits {
            0 => BIN_A::BCD,
            1 => BIN_A::BINARY,
            2 => BIN_A::BINBCD,
            3 => BIN_A::BINBCD2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BCD`"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        **self == BIN_A::BCD
    }
    #[doc = "Checks if the value of the field is `BINARY`"]
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        **self == BIN_A::BINARY
    }
    #[doc = "Checks if the value of the field is `BINBCD`"]
    #[inline(always)]
    pub fn is_bin_bcd(&self) -> bool {
        **self == BIN_A::BINBCD
    }
    #[doc = "Checks if the value of the field is `BINBCD2`"]
    #[inline(always)]
    pub fn is_bin_bcd2(&self) -> bool {
        **self == BIN_A::BINBCD2
    }
}
impl core::ops::Deref for BIN_R {
    type Target = crate::FieldReader<u8, BIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIN` writer - Binary mode"]
pub struct BIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Free running BCD calendar mode (Binary mode disabled)"]
    #[inline(always)]
    pub fn bcd(self) -> &'a mut W {
        self.variant(BIN_A::BCD)
    }
    #[doc = "Free running Binary mode (BCD mode disabled)"]
    #[inline(always)]
    pub fn binary(self) -> &'a mut W {
        self.variant(BIN_A::BINARY)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn bin_bcd(self) -> &'a mut W {
        self.variant(BIN_A::BINBCD)
    }
    #[doc = "Free running BCD calendar and Binary modes"]
    #[inline(always)]
    pub fn bin_bcd2(self) -> &'a mut W {
        self.variant(BIN_A::BINBCD2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    FREERUNNINGMODE = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    INITMODE = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub struct INIT_R(crate::FieldReader<bool, INIT_A>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FREERUNNINGMODE,
            true => INIT_A::INITMODE,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUNNINGMODE`"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        **self == INIT_A::FREERUNNINGMODE
    }
    #[doc = "Checks if the value of the field is `INITMODE`"]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        **self == INIT_A::INITMODE
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FREERUNNINGMODE)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::INITMODE)
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
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITF_A {
    #[doc = "0: Calendar registers update is not allowed"]
    NOTALLOWED = 0,
    #[doc = "1: Calendar registers update is allowed"]
    ALLOWED = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub struct INITF_R(crate::FieldReader<bool, INITF_A>);
impl INITF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::NOTALLOWED,
            true => INITF_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == INITF_A::NOTALLOWED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == INITF_A::ALLOWED
    }
}
impl core::ops::Deref for INITF_R {
    type Target = crate::FieldReader<bool, INITF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NOTSYNCED = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    SYNCED = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub struct RSF_R(crate::FieldReader<bool, RSF_A>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NOTSYNCED,
            true => RSF_A::SYNCED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSYNCED`"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        **self == RSF_A::NOTSYNCED
    }
    #[doc = "Checks if the value of the field is `SYNCED`"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        **self == RSF_A::SYNCED
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, RSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::CLEAR)
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
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITS_A {
    #[doc = "0: Calendar has not been initialized"]
    NOTINITALIZED = 0,
    #[doc = "1: Calendar has been initialized"]
    INITALIZED = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub struct INITS_R(crate::FieldReader<bool, INITS_A>);
impl INITS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::NOTINITALIZED,
            true => INITS_A::INITALIZED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINITALIZED`"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        **self == INITS_A::NOTINITALIZED
    }
    #[doc = "Checks if the value of the field is `INITALIZED`"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        **self == INITS_A::INITALIZED
    }
}
impl core::ops::Deref for INITS_R {
    type Target = crate::FieldReader<bool, INITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHPF_A {
    #[doc = "0: No shift operation is pending"]
    NOSHIFTPENDING = 0,
    #[doc = "1: A shift operation is pending"]
    SHIFTPENDING = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub struct SHPF_R(crate::FieldReader<bool, SHPF_A>);
impl SHPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::NOSHIFTPENDING,
            true => SHPF_A::SHIFTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFTPENDING`"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        **self == SHPF_A::NOSHIFTPENDING
    }
    #[doc = "Checks if the value of the field is `SHIFTPENDING`"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        **self == SHPF_A::SHIFTPENDING
    }
}
impl core::ops::Deref for SHPF_R {
    type Target = crate::FieldReader<bool, SHPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup timer write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTWF_A {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UPDATENOTALLOWED = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UPDATEALLOWED = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub struct WUTWF_R(crate::FieldReader<bool, WUTWF_A>);
impl WUTWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::UPDATENOTALLOWED,
            true => WUTWF_A::UPDATEALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATENOTALLOWED`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        **self == WUTWF_A::UPDATENOTALLOWED
    }
    #[doc = "Checks if the value of the field is `UPDATEALLOWED`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        **self == WUTWF_A::UPDATEALLOWED
    }
}
impl core::ops::Deref for WUTWF_R {
    type Target = crate::FieldReader<bool, WUTWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - BCD update"]
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:12 - BCD update"]
    #[inline(always)]
    pub fn bcdu(&mut self) -> BCDU_W {
        BCDU_W { w: self }
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&mut self) -> BIN_W {
        BIN_W { w: self }
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initialization control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
