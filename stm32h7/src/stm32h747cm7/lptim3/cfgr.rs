#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Encoder mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENC_A {
    #[doc = "0: Encoder mode disabled"]
    DISABLED = 0,
    #[doc = "1: Encoder mode enabled"]
    ENABLED = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<bool, ENC_A>;
impl ENC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::DISABLED,
            true => ENC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENC_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENC_A::DISABLED)
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "counter mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTMODE_A {
    #[doc = "0: The counter is incremented following each internal clock pulse"]
    INTERNAL = 0,
    #[doc = "1: The counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    EXTERNAL = 1,
}
impl From<COUNTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COUNTMODE`"]
pub type COUNTMODE_R = crate::R<bool, COUNTMODE_A>;
impl COUNTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNTMODE_A {
        match self.bits {
            false => COUNTMODE_A::INTERNAL,
            true => COUNTMODE_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == COUNTMODE_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == COUNTMODE_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `COUNTMODE`"]
pub struct COUNTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COUNTMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(COUNTMODE_A::INTERNAL)
    }
    #[doc = "The counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(COUNTMODE_A::EXTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Registers update mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRELOAD_A {
    #[doc = "0: Registers are updated after each APB bus write access"]
    IMMEDIATE = 0,
    #[doc = "1: Registers are updated at the end of the current LPTIM period"]
    ENDOFPERIOD = 1,
}
impl From<PRELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRELOAD`"]
pub type PRELOAD_R = crate::R<bool, PRELOAD_A>;
impl PRELOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRELOAD_A {
        match self.bits {
            false => PRELOAD_A::IMMEDIATE,
            true => PRELOAD_A::ENDOFPERIOD,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == PRELOAD_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `ENDOFPERIOD`"]
    #[inline(always)]
    pub fn is_end_of_period(&self) -> bool {
        *self == PRELOAD_A::ENDOFPERIOD
    }
}
#[doc = "Write proxy for field `PRELOAD`"]
pub struct PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRELOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(PRELOAD_A::IMMEDIATE)
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn end_of_period(self) -> &'a mut W {
        self.variant(PRELOAD_A::ENDOFPERIOD)
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
#[doc = "Waveform shape polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVPOL_A {
    #[doc = "0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    POSITIVE = 0,
    #[doc = "1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    NEGATIVE = 1,
}
impl From<WAVPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAVPOL`"]
pub type WAVPOL_R = crate::R<bool, WAVPOL_A>;
impl WAVPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVPOL_A {
        match self.bits {
            false => WAVPOL_A::POSITIVE,
            true => WAVPOL_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == WAVPOL_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == WAVPOL_A::NEGATIVE
    }
}
#[doc = "Write proxy for field `WAVPOL`"]
pub struct WAVPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(WAVPOL_A::POSITIVE)
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(WAVPOL_A::NEGATIVE)
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
#[doc = "Waveform shape\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE_A {
    #[doc = "0: Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)"]
    INACTIVE = 0,
    #[doc = "1: Activate the Set-once mode"]
    ACTIVE = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, WAVE_A>;
impl WAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVE_A {
        match self.bits {
            false => WAVE_A::INACTIVE,
            true => WAVE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WAVE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == WAVE_A::ACTIVE
    }
}
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(WAVE_A::INACTIVE)
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(WAVE_A::ACTIVE)
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
#[doc = "Timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUT_A {
    #[doc = "0: A trigger event arriving when the timer is already started will be ignored"]
    DISABLED = 0,
    #[doc = "1: A trigger event arriving when the timer is already started will reset and restart the counter"]
    ENABLED = 1,
}
impl From<TIMOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMOUT`"]
pub type TIMOUT_R = crate::R<bool, TIMOUT_A>;
impl TIMOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOUT_A {
        match self.bits {
            false => TIMOUT_A::DISABLED,
            true => TIMOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUT_A::ENABLED
    }
}
#[doc = "Write proxy for field `TIMOUT`"]
pub struct TIMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A trigger event arriving when the timer is already started will be ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMOUT_A::DISABLED)
    }
    #[doc = "A trigger event arriving when the timer is already started will reset and restart the counter"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMOUT_A::ENABLED)
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
#[doc = "Trigger enable and polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGEN_A {
    #[doc = "0: Software trigger (counting start is initiated by software)"]
    SW = 0,
    #[doc = "1: Rising edge is the active edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge is the active edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Both edges are active edges"]
    BOTHEDGES = 3,
}
impl From<TRIGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGEN`"]
pub type TRIGEN_R = crate::R<u8, TRIGEN_A>;
impl TRIGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGEN_A {
        match self.bits {
            0 => TRIGEN_A::SW,
            1 => TRIGEN_A::RISINGEDGE,
            2 => TRIGEN_A::FALLINGEDGE,
            3 => TRIGEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGEN_A::SW
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGEN_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGEN_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == TRIGEN_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `TRIGEN`"]
pub struct TRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGEN_A::SW)
    }
    #[doc = "Rising edge is the active edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TRIGEN_A::RISINGEDGE)
    }
    #[doc = "Falling edge is the active edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TRIGEN_A::FALLINGEDGE)
    }
    #[doc = "Both edges are active edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(TRIGEN_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: lptim_ext_trig0"]
    TRIG0 = 0,
    #[doc = "1: lptim_ext_trig1"]
    TRIG1 = 1,
    #[doc = "2: lptim_ext_trig2"]
    TRIG2 = 2,
    #[doc = "3: lptim_ext_trig3"]
    TRIG3 = 3,
    #[doc = "4: lptim_ext_trig4"]
    TRIG4 = 4,
    #[doc = "5: lptim_ext_trig5"]
    TRIG5 = 5,
    #[doc = "6: lptim_ext_trig6"]
    TRIG6 = 6,
    #[doc = "7: lptim_ext_trig7"]
    TRIG7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, TRIGSEL_A>;
impl TRIGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGSEL_A {
        match self.bits {
            0 => TRIGSEL_A::TRIG0,
            1 => TRIGSEL_A::TRIG1,
            2 => TRIGSEL_A::TRIG2,
            3 => TRIGSEL_A::TRIG3,
            4 => TRIGSEL_A::TRIG4,
            5 => TRIGSEL_A::TRIG5,
            6 => TRIGSEL_A::TRIG6,
            7 => TRIGSEL_A::TRIG7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == TRIGSEL_A::TRIG0
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == TRIGSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `TRIG2`"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == TRIGSEL_A::TRIG2
    }
    #[doc = "Checks if the value of the field is `TRIG3`"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == TRIGSEL_A::TRIG3
    }
    #[doc = "Checks if the value of the field is `TRIG4`"]
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == TRIGSEL_A::TRIG4
    }
    #[doc = "Checks if the value of the field is `TRIG5`"]
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == TRIGSEL_A::TRIG5
    }
    #[doc = "Checks if the value of the field is `TRIG6`"]
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == TRIGSEL_A::TRIG6
    }
    #[doc = "Checks if the value of the field is `TRIG7`"]
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == TRIGSEL_A::TRIG7
    }
}
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG0)
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG1)
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG2)
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG3)
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn trig4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG4)
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn trig5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG5)
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn trig6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG6)
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn trig7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::TRIG7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Clock prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: /1"]
    DIV1 = 0,
    #[doc = "1: /2"]
    DIV2 = 1,
    #[doc = "2: /4"]
    DIV4 = 2,
    #[doc = "3: /8"]
    DIV8 = 3,
    #[doc = "4: /16"]
    DIV16 = 4,
    #[doc = "5: /32"]
    DIV32 = 5,
    #[doc = "6: /64"]
    DIV64 = 6,
    #[doc = "7: /128"]
    DIV128 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV1,
            1 => PRESC_A::DIV2,
            2 => PRESC_A::DIV4,
            3 => PRESC_A::DIV8,
            4 => PRESC_A::DIV16,
            5 => PRESC_A::DIV32,
            6 => PRESC_A::DIV64,
            7 => PRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Configurable digital filter for trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGFLT_A {
    #[doc = "0: Any trigger active level change is considered as a valid trigger"]
    IMMEDIATE = 0,
    #[doc = "1: Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger"]
    CLOCKS2 = 1,
    #[doc = "2: Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger"]
    CLOCKS4 = 2,
    #[doc = "3: Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger"]
    CLOCKS8 = 3,
}
impl From<TRGFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRGFLT`"]
pub type TRGFLT_R = crate::R<u8, TRGFLT_A>;
impl TRGFLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGFLT_A {
        match self.bits {
            0 => TRGFLT_A::IMMEDIATE,
            1 => TRGFLT_A::CLOCKS2,
            2 => TRGFLT_A::CLOCKS4,
            3 => TRGFLT_A::CLOCKS8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TRGFLT_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `CLOCKS2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == TRGFLT_A::CLOCKS2
    }
    #[doc = "Checks if the value of the field is `CLOCKS4`"]
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == TRGFLT_A::CLOCKS4
    }
    #[doc = "Checks if the value of the field is `CLOCKS8`"]
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == TRGFLT_A::CLOCKS8
    }
}
#[doc = "Write proxy for field `TRGFLT`"]
pub struct TRGFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGFLT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(TRGFLT_A::IMMEDIATE)
    }
    #[doc = "Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(TRGFLT_A::CLOCKS2)
    }
    #[doc = "Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut W {
        self.variant(TRGFLT_A::CLOCKS4)
    }
    #[doc = "Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut W {
        self.variant(TRGFLT_A::CLOCKS8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Configurable digital filter for external clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKFLT_A {
    #[doc = "0: Any external clock signal level change is considered as a valid transition"]
    IMMEDIATE = 0,
    #[doc = "1: External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition"]
    CLOCKS2 = 1,
    #[doc = "2: External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition"]
    CLOCKS4 = 2,
    #[doc = "3: External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition"]
    CLOCKS8 = 3,
}
impl From<CKFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKFLT`"]
pub type CKFLT_R = crate::R<u8, CKFLT_A>;
impl CKFLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKFLT_A {
        match self.bits {
            0 => CKFLT_A::IMMEDIATE,
            1 => CKFLT_A::CLOCKS2,
            2 => CKFLT_A::CLOCKS4,
            3 => CKFLT_A::CLOCKS8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == CKFLT_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `CLOCKS2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CKFLT_A::CLOCKS2
    }
    #[doc = "Checks if the value of the field is `CLOCKS4`"]
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == CKFLT_A::CLOCKS4
    }
    #[doc = "Checks if the value of the field is `CLOCKS8`"]
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == CKFLT_A::CLOCKS8
    }
}
#[doc = "Write proxy for field `CKFLT`"]
pub struct CKFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKFLT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(CKFLT_A::IMMEDIATE)
    }
    #[doc = "External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(CKFLT_A::CLOCKS2)
    }
    #[doc = "External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut W {
        self.variant(CKFLT_A::CLOCKS4)
    }
    #[doc = "External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut W {
        self.variant(CKFLT_A::CLOCKS8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKPOL_A {
    #[doc = "0: The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active."]
    RISINGEDGE = 0,
    #[doc = "1: The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active."]
    FALLINGEDGE = 1,
    #[doc = "2: Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active."]
    BOTHEDGES = 2,
}
impl From<CKPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKPOL`"]
pub type CKPOL_R = crate::R<u8, CKPOL_A>;
impl CKPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKPOL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKPOL_A::RISINGEDGE),
            1 => Val(CKPOL_A::FALLINGEDGE),
            2 => Val(CKPOL_A::BOTHEDGES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CKPOL_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `CKPOL`"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::RISINGEDGE)
    }
    #[doc = "The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::FALLINGEDGE)
    }
    #[doc = "Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(CKPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Clock selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSEL_A {
    #[doc = "0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    INTERNAL = 0,
    #[doc = "1: LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    EXTERNAL = 1,
}
impl From<CKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKSEL`"]
pub type CKSEL_R = crate::R<bool, CKSEL_A>;
impl CKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKSEL_A {
        match self.bits {
            false => CKSEL_A::INTERNAL,
            true => CKSEL_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CKSEL_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == CKSEL_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `CKSEL`"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(CKSEL_A::INTERNAL)
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(CKSEL_A::EXTERNAL)
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
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W {
        COUNTMODE_W { w: self }
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W {
        PRELOAD_W { w: self }
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W {
        WAVPOL_W { w: self }
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W { w: self }
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W {
        TRIGEN_W { w: self }
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W {
        TRGFLT_W { w: self }
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W {
        CKFLT_W { w: self }
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
}
