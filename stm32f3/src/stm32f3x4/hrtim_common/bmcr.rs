#[doc = "Reader of register BMCR"]
pub type R = crate::R<u32, super::BMCR>;
#[doc = "Writer for register BMCR"]
pub type W = crate::W<u32, super::BMCR>;
#[doc = "Register BMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::BMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Burst Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMSTAT_A {
    #[doc = "0: Normal operation"]
    NORMAL = 0,
    #[doc = "1: Burst operation ongoing"]
    BURST = 1,
}
impl From<BMSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: BMSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BMSTAT`"]
pub type BMSTAT_R = crate::R<bool, BMSTAT_A>;
impl BMSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMSTAT_A {
        match self.bits {
            false => BMSTAT_A::NORMAL,
            true => BMSTAT_A::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BMSTAT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BMSTAT_A::BURST
    }
}
#[doc = "Burst Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMSTAT_AW {
    #[doc = "0: Terminate burst mode"]
    CANCEL = 0,
}
impl From<BMSTAT_AW> for bool {
    #[inline(always)]
    fn from(variant: BMSTAT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BMSTAT`"]
pub struct BMSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BMSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMSTAT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Terminate burst mode"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut W {
        self.variant(BMSTAT_AW::CANCEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Timer E Burst Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEBM_A {
    #[doc = "0: Counter clock is maintained and timer operates normally"]
    NORMAL = 0,
    #[doc = "1: Counter clock is stopped and counter is reset"]
    STOPPED = 1,
}
impl From<TEBM_A> for bool {
    #[inline(always)]
    fn from(variant: TEBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEBM`"]
pub type TEBM_R = crate::R<bool, TEBM_A>;
impl TEBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEBM_A {
        match self.bits {
            false => TEBM_A::NORMAL,
            true => TEBM_A::STOPPED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TEBM_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == TEBM_A::STOPPED
    }
}
#[doc = "Write proxy for field `TEBM`"]
pub struct TEBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Timer D Burst Mode"]
pub type TDBM_A = TEBM_A;
#[doc = "Reader of field `TDBM`"]
pub type TDBM_R = crate::R<bool, TEBM_A>;
#[doc = "Write proxy for field `TDBM`"]
pub struct TDBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TDBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Timer C Burst Mode"]
pub type TCBM_A = TEBM_A;
#[doc = "Reader of field `TCBM`"]
pub type TCBM_R = crate::R<bool, TEBM_A>;
#[doc = "Write proxy for field `TCBM`"]
pub struct TCBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Timer B Burst Mode"]
pub type TBBM_A = TEBM_A;
#[doc = "Reader of field `TBBM`"]
pub type TBBM_R = crate::R<bool, TEBM_A>;
#[doc = "Write proxy for field `TBBM`"]
pub struct TBBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Timer A Burst Mode"]
pub type TABM_A = TEBM_A;
#[doc = "Reader of field `TABM`"]
pub type TABM_R = crate::R<bool, TEBM_A>;
#[doc = "Write proxy for field `TABM`"]
pub struct TABM_W<'a> {
    w: &'a mut W,
}
impl<'a> TABM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TABM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Master Timer Burst Mode"]
pub type MTBM_A = TEBM_A;
#[doc = "Reader of field `MTBM`"]
pub type MTBM_R = crate::R<bool, TEBM_A>;
#[doc = "Write proxy for field `MTBM`"]
pub struct MTBM_W<'a> {
    w: &'a mut W,
}
impl<'a> MTBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is maintained and timer operates normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TEBM_A::NORMAL)
    }
    #[doc = "Counter clock is stopped and counter is reset"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(TEBM_A::STOPPED)
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
#[doc = "Burst Mode Preload Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPREN_A {
    #[doc = "0: Preload disabled: the write access is directly done into active registers"]
    DISABLED = 0,
    #[doc = "1: Preload enabled: the write access is done into preload registers"]
    ENABLED = 1,
}
impl From<BMPREN_A> for bool {
    #[inline(always)]
    fn from(variant: BMPREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BMPREN`"]
pub type BMPREN_R = crate::R<bool, BMPREN_A>;
impl BMPREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPREN_A {
        match self.bits {
            false => BMPREN_A::DISABLED,
            true => BMPREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `BMPREN`"]
pub struct BMPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Preload disabled: the write access is directly done into active registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BMPREN_A::DISABLED)
    }
    #[doc = "Preload enabled: the write access is done into preload registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BMPREN_A::ENABLED)
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
#[doc = "Burst Mode Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BMPRSC_A {
    #[doc = "0: Clock not divided"]
    DIV1 = 0,
    #[doc = "1: Division by 2"]
    DIV2 = 1,
    #[doc = "2: Division by 4"]
    DIV4 = 2,
    #[doc = "3: Division by 8"]
    DIV8 = 3,
    #[doc = "4: Division by 16"]
    DIV16 = 4,
    #[doc = "5: Division by 32"]
    DIV32 = 5,
    #[doc = "6: Division by 64"]
    DIV64 = 6,
    #[doc = "7: Division by 128"]
    DIV128 = 7,
    #[doc = "8: Division by 256"]
    DIV256 = 8,
    #[doc = "9: Division by 512"]
    DIV512 = 9,
    #[doc = "10: Division by 1024"]
    DIV1024 = 10,
    #[doc = "11: Division by 2048"]
    DIV2048 = 11,
    #[doc = "12: Division by 4096"]
    DIV4096 = 12,
    #[doc = "13: Division by 8192"]
    DIV8192 = 13,
    #[doc = "14: Division by 16384"]
    DIV16384 = 14,
    #[doc = "15: Division by 32768"]
    DIV32768 = 15,
}
impl From<BMPRSC_A> for u8 {
    #[inline(always)]
    fn from(variant: BMPRSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BMPRSC`"]
pub type BMPRSC_R = crate::R<u8, BMPRSC_A>;
impl BMPRSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPRSC_A {
        match self.bits {
            0 => BMPRSC_A::DIV1,
            1 => BMPRSC_A::DIV2,
            2 => BMPRSC_A::DIV4,
            3 => BMPRSC_A::DIV8,
            4 => BMPRSC_A::DIV16,
            5 => BMPRSC_A::DIV32,
            6 => BMPRSC_A::DIV64,
            7 => BMPRSC_A::DIV128,
            8 => BMPRSC_A::DIV256,
            9 => BMPRSC_A::DIV512,
            10 => BMPRSC_A::DIV1024,
            11 => BMPRSC_A::DIV2048,
            12 => BMPRSC_A::DIV4096,
            13 => BMPRSC_A::DIV8192,
            14 => BMPRSC_A::DIV16384,
            15 => BMPRSC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == BMPRSC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BMPRSC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BMPRSC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BMPRSC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BMPRSC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BMPRSC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BMPRSC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BMPRSC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BMPRSC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == BMPRSC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == BMPRSC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == BMPRSC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == BMPRSC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == BMPRSC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == BMPRSC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == BMPRSC_A::DIV32768
    }
}
#[doc = "Write proxy for field `BMPRSC`"]
pub struct BMPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPRSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPRSC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV2)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV8)
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV16)
    }
    #[doc = "Division by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV32)
    }
    #[doc = "Division by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV64)
    }
    #[doc = "Division by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV128)
    }
    #[doc = "Division by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV256)
    }
    #[doc = "Division by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV512)
    }
    #[doc = "Division by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV1024)
    }
    #[doc = "Division by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV2048)
    }
    #[doc = "Division by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV4096)
    }
    #[doc = "Division by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV8192)
    }
    #[doc = "Division by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV16384)
    }
    #[doc = "Division by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(BMPRSC_A::DIV32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Burst Mode Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BMCLK_A {
    #[doc = "0: Master timer reset/roll-over"]
    MASTER = 0,
    #[doc = "1: Timer A counter reset/roll-over"]
    TIMERA = 1,
    #[doc = "2: Timer B counter reset/roll-over"]
    TIMERB = 2,
    #[doc = "3: Timer C counter reset/roll-over"]
    TIMERC = 3,
    #[doc = "4: Timer D counter reset/roll-over"]
    TIMERD = 4,
    #[doc = "5: Timer E counter reset/roll-over"]
    TIMERE = 5,
    #[doc = "6: On-chip Event 1 (BMClk\\[1\\]), acting as a burst mode counter clock"]
    EVENT1 = 6,
    #[doc = "7: On-chip Event 2 (BMClk\\[2\\]), acting as a burst mode counter clock"]
    EVENT2 = 7,
    #[doc = "8: On-chip Event 3 (BMClk\\[3\\]), acting as a burst mode counter clock"]
    EVENT3 = 8,
    #[doc = "9: On-chip Event 4 (BMClk\\[4\\]), acting as a burst mode counter clock"]
    EVENT4 = 9,
    #[doc = "10: Prescaled f_HRTIM clock (as per BMPRSC\\[3:0\\]
setting"]
    CLOCK = 10,
}
impl From<BMCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: BMCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BMCLK`"]
pub type BMCLK_R = crate::R<u8, BMCLK_A>;
impl BMCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BMCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BMCLK_A::MASTER),
            1 => Val(BMCLK_A::TIMERA),
            2 => Val(BMCLK_A::TIMERB),
            3 => Val(BMCLK_A::TIMERC),
            4 => Val(BMCLK_A::TIMERD),
            5 => Val(BMCLK_A::TIMERE),
            6 => Val(BMCLK_A::EVENT1),
            7 => Val(BMCLK_A::EVENT2),
            8 => Val(BMCLK_A::EVENT3),
            9 => Val(BMCLK_A::EVENT4),
            10 => Val(BMCLK_A::CLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == BMCLK_A::MASTER
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == BMCLK_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == BMCLK_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERC`"]
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == BMCLK_A::TIMERC
    }
    #[doc = "Checks if the value of the field is `TIMERD`"]
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == BMCLK_A::TIMERD
    }
    #[doc = "Checks if the value of the field is `TIMERE`"]
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == BMCLK_A::TIMERE
    }
    #[doc = "Checks if the value of the field is `EVENT1`"]
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == BMCLK_A::EVENT1
    }
    #[doc = "Checks if the value of the field is `EVENT2`"]
    #[inline(always)]
    pub fn is_event2(&self) -> bool {
        *self == BMCLK_A::EVENT2
    }
    #[doc = "Checks if the value of the field is `EVENT3`"]
    #[inline(always)]
    pub fn is_event3(&self) -> bool {
        *self == BMCLK_A::EVENT3
    }
    #[doc = "Checks if the value of the field is `EVENT4`"]
    #[inline(always)]
    pub fn is_event4(&self) -> bool {
        *self == BMCLK_A::EVENT4
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == BMCLK_A::CLOCK
    }
}
#[doc = "Write proxy for field `BMCLK`"]
pub struct BMCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master timer reset/roll-over"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(BMCLK_A::MASTER)
    }
    #[doc = "Timer A counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(BMCLK_A::TIMERA)
    }
    #[doc = "Timer B counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(BMCLK_A::TIMERB)
    }
    #[doc = "Timer C counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(BMCLK_A::TIMERC)
    }
    #[doc = "Timer D counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(BMCLK_A::TIMERD)
    }
    #[doc = "Timer E counter reset/roll-over"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(BMCLK_A::TIMERE)
    }
    #[doc = "On-chip Event 1 (BMClk\\[1\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event1(self) -> &'a mut W {
        self.variant(BMCLK_A::EVENT1)
    }
    #[doc = "On-chip Event 2 (BMClk\\[2\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event2(self) -> &'a mut W {
        self.variant(BMCLK_A::EVENT2)
    }
    #[doc = "On-chip Event 3 (BMClk\\[3\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event3(self) -> &'a mut W {
        self.variant(BMCLK_A::EVENT3)
    }
    #[doc = "On-chip Event 4 (BMClk\\[4\\]), acting as a burst mode counter clock"]
    #[inline(always)]
    pub fn event4(self) -> &'a mut W {
        self.variant(BMCLK_A::EVENT4)
    }
    #[doc = "Prescaled f_HRTIM clock (as per BMPRSC\\[3:0\\]
setting"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut W {
        self.variant(BMCLK_A::CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Burst Mode operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMOM_A {
    #[doc = "0: Single-shot mode"]
    SINGLESHOT = 0,
    #[doc = "1: Continuous operation"]
    CONTINUOUS = 1,
}
impl From<BMOM_A> for bool {
    #[inline(always)]
    fn from(variant: BMOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BMOM`"]
pub type BMOM_R = crate::R<bool, BMOM_A>;
impl BMOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMOM_A {
        match self.bits {
            false => BMOM_A::SINGLESHOT,
            true => BMOM_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLESHOT`"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == BMOM_A::SINGLESHOT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == BMOM_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `BMOM`"]
pub struct BMOM_W<'a> {
    w: &'a mut W,
}
impl<'a> BMOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-shot mode"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(BMOM_A::SINGLESHOT)
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(BMOM_A::CONTINUOUS)
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
#[doc = "Burst Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BME_A {
    #[doc = "0: Burst mode disabled"]
    DISABLED = 0,
    #[doc = "1: Burst mode enabled"]
    ENABLED = 1,
}
impl From<BME_A> for bool {
    #[inline(always)]
    fn from(variant: BME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BME`"]
pub type BME_R = crate::R<bool, BME_A>;
impl BME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BME_A {
        match self.bits {
            false => BME_A::DISABLED,
            true => BME_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BME_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BME_A::ENABLED
    }
}
#[doc = "Write proxy for field `BME`"]
pub struct BME_W<'a> {
    w: &'a mut W,
}
impl<'a> BME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BME_A::DISABLED)
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BME_A::ENABLED)
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
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&mut self) -> BMSTAT_W {
        BMSTAT_W { w: self }
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&mut self) -> TEBM_W {
        TEBM_W { w: self }
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&mut self) -> TDBM_W {
        TDBM_W { w: self }
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&mut self) -> TCBM_W {
        TCBM_W { w: self }
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&mut self) -> TBBM_W {
        TBBM_W { w: self }
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&mut self) -> TABM_W {
        TABM_W { w: self }
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&mut self) -> MTBM_W {
        MTBM_W { w: self }
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&mut self) -> BMPREN_W {
        BMPREN_W { w: self }
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&mut self) -> BMPRSC_W {
        BMPRSC_W { w: self }
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&mut self) -> BMCLK_W {
        BMCLK_W { w: self }
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&mut self) -> BMOM_W {
        BMOM_W { w: self }
    }
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&mut self) -> BME_W {
        BME_W { w: self }
    }
}
