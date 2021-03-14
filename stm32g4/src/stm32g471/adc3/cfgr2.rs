#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPTRIG_A {
    #[doc = "0: Sampling time control trigger mode disabled"]
    DISABLED = 0,
    #[doc = "1: Sampling time control trigger mode enabled"]
    ENABLED = 1,
}
impl From<SMPTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMPTRIG`"]
pub type SMPTRIG_R = crate::R<bool, SMPTRIG_A>;
impl SMPTRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPTRIG_A {
        match self.bits {
            false => SMPTRIG_A::DISABLED,
            true => SMPTRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG_A::ENABLED
    }
}
#[doc = "Write proxy for field `SMPTRIG`"]
pub struct SMPTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPTRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling time control trigger mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::DISABLED)
    }
    #[doc = "Sampling time control trigger mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Bulb sampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BULB_A {
    #[doc = "0: Bulb sampling mode disabled"]
    DISABLED = 0,
    #[doc = "1: Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    ENABLED = 1,
}
impl From<BULB_A> for bool {
    #[inline(always)]
    fn from(variant: BULB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BULB`"]
pub type BULB_R = crate::R<bool, BULB_A>;
impl BULB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BULB_A {
        match self.bits {
            false => BULB_A::DISABLED,
            true => BULB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB_A::ENABLED
    }
}
#[doc = "Write proxy for field `BULB`"]
pub struct BULB_W<'a> {
    w: &'a mut W,
}
impl<'a> BULB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BULB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bulb sampling mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BULB_A::DISABLED)
    }
    #[doc = "Bulb sampling mode enabled. Immediately start sampling after last conversion finishes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BULB_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Software trigger bit for sampling time control trigger mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG_A {
    #[doc = "0: End sampling period and start conversion"]
    DISABLED = 0,
    #[doc = "1: Start sampling period"]
    ENABLED = 1,
}
impl From<SWTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWTRIG`"]
pub type SWTRIG_R = crate::R<bool, SWTRIG_A>;
impl SWTRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWTRIG_A {
        match self.bits {
            false => SWTRIG_A::DISABLED,
            true => SWTRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG_A::ENABLED
    }
}
#[doc = "Write proxy for field `SWTRIG`"]
pub struct SWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "End sampling period and start conversion"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::DISABLED)
    }
    #[doc = "Start sampling period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Gain compensation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCOMP_A {
    #[doc = "0: Regular ADC operating mode"]
    DISABLED = 0,
    #[doc = "1: Gain compensation enabled and applies to all channels"]
    ENABLED = 1,
}
impl From<GCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: GCOMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GCOMP`"]
pub type GCOMP_R = crate::R<bool, GCOMP_A>;
impl GCOMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCOMP_A {
        match self.bits {
            false => GCOMP_A::DISABLED,
            true => GCOMP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP_A::ENABLED
    }
}
#[doc = "Write proxy for field `GCOMP`"]
pub struct GCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> GCOMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCOMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular ADC operating mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCOMP_A::DISABLED)
    }
    #[doc = "Gain compensation enabled and applies to all channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCOMP_A::ENABLED)
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
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVSM_A {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    CONTINUED = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    RESUMED = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVSM`"]
pub type ROVSM_R = crate::R<bool, ROVSM_A>;
impl ROVSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::CONTINUED,
            true => ROVSM_A::RESUMED,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUED`"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM_A::CONTINUED
    }
    #[doc = "Checks if the value of the field is `RESUMED`"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM_A::RESUMED
    }
}
#[doc = "Write proxy for field `ROVSM`"]
pub struct ROVSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut W {
        self.variant(ROVSM_A::CONTINUED)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut W {
        self.variant(ROVSM_A::RESUMED)
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
#[doc = "Triggered Regular Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    AUTOMATIC = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    TRIGGERED = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TROVS`"]
pub type TROVS_R = crate::R<bool, TROVS_A>;
impl TROVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::AUTOMATIC,
            true => TROVS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS_A::TRIGGERED
    }
}
#[doc = "Write proxy for field `TROVS`"]
pub struct TROVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TROVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TROVS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(TROVS_A::AUTOMATIC)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TROVS_A::TRIGGERED)
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
#[doc = "Oversampling shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No right shift applied to oversampling result"]
    NOSHIFT = 0,
    #[doc = "1: Shift oversampling result right by 1 bit"]
    SHIFT1 = 1,
    #[doc = "2: Shift oversampling result right by 2 bits"]
    SHIFT2 = 2,
    #[doc = "3: Shift oversampling result right by 3 bits"]
    SHIFT3 = 3,
    #[doc = "4: Shift oversampling result right by 4 bits"]
    SHIFT4 = 4,
    #[doc = "5: Shift oversampling result right by 5 bits"]
    SHIFT5 = 5,
    #[doc = "6: Shift oversampling result right by 6 bits"]
    SHIFT6 = 6,
    #[doc = "7: Shift oversampling result right by 7 bits"]
    SHIFT7 = 7,
    #[doc = "8: Shift oversampling result right by 8 bits"]
    SHIFT8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVSS`"]
pub type OVSS_R = crate::R<u8, OVSS_A>;
impl OVSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVSS_A::NOSHIFT),
            1 => Val(OVSS_A::SHIFT1),
            2 => Val(OVSS_A::SHIFT2),
            3 => Val(OVSS_A::SHIFT3),
            4 => Val(OVSS_A::SHIFT4),
            5 => Val(OVSS_A::SHIFT5),
            6 => Val(OVSS_A::SHIFT6),
            7 => Val(OVSS_A::SHIFT7),
            8 => Val(OVSS_A::SHIFT8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS_A::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFT1`"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS_A::SHIFT1
    }
    #[doc = "Checks if the value of the field is `SHIFT2`"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS_A::SHIFT2
    }
    #[doc = "Checks if the value of the field is `SHIFT3`"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS_A::SHIFT3
    }
    #[doc = "Checks if the value of the field is `SHIFT4`"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS_A::SHIFT4
    }
    #[doc = "Checks if the value of the field is `SHIFT5`"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS_A::SHIFT5
    }
    #[doc = "Checks if the value of the field is `SHIFT6`"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS_A::SHIFT6
    }
    #[doc = "Checks if the value of the field is `SHIFT7`"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS_A::SHIFT7
    }
    #[doc = "Checks if the value of the field is `SHIFT8`"]
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS_A::SHIFT8
    }
}
#[doc = "Write proxy for field `OVSS`"]
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No right shift applied to oversampling result"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(OVSS_A::NOSHIFT)
    }
    #[doc = "Shift oversampling result right by 1 bit"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT1)
    }
    #[doc = "Shift oversampling result right by 2 bits"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT2)
    }
    #[doc = "Shift oversampling result right by 3 bits"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT3)
    }
    #[doc = "Shift oversampling result right by 4 bits"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT4)
    }
    #[doc = "Shift oversampling result right by 5 bits"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT5)
    }
    #[doc = "Shift oversampling result right by 6 bits"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT6)
    }
    #[doc = "Shift oversampling result right by 7 bits"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT7)
    }
    #[doc = "Shift oversampling result right by 8 bits"]
    #[inline(always)]
    pub fn shift8(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: Oversampling ratio of 2"]
    OS2 = 0,
    #[doc = "1: Oversampling ratio of 4"]
    OS4 = 1,
    #[doc = "2: Oversampling ratio of 8"]
    OS8 = 2,
    #[doc = "3: Oversampling ratio of 16"]
    OS16 = 3,
    #[doc = "4: Oversampling ratio of 32"]
    OS32 = 4,
    #[doc = "5: Oversampling ratio of 64"]
    OS64 = 5,
    #[doc = "6: Oversampling ratio of 128"]
    OS128 = 6,
    #[doc = "7: Oversampling ratio of 256"]
    OS256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVSR`"]
pub type OVSR_R = crate::R<u8, OVSR_A>;
impl OVSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::OS2,
            1 => OVSR_A::OS4,
            2 => OVSR_A::OS8,
            3 => OVSR_A::OS16,
            4 => OVSR_A::OS32,
            5 => OVSR_A::OS64,
            6 => OVSR_A::OS128,
            7 => OVSR_A::OS256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OS2`"]
    #[inline(always)]
    pub fn is_os2(&self) -> bool {
        *self == OVSR_A::OS2
    }
    #[doc = "Checks if the value of the field is `OS4`"]
    #[inline(always)]
    pub fn is_os4(&self) -> bool {
        *self == OVSR_A::OS4
    }
    #[doc = "Checks if the value of the field is `OS8`"]
    #[inline(always)]
    pub fn is_os8(&self) -> bool {
        *self == OVSR_A::OS8
    }
    #[doc = "Checks if the value of the field is `OS16`"]
    #[inline(always)]
    pub fn is_os16(&self) -> bool {
        *self == OVSR_A::OS16
    }
    #[doc = "Checks if the value of the field is `OS32`"]
    #[inline(always)]
    pub fn is_os32(&self) -> bool {
        *self == OVSR_A::OS32
    }
    #[doc = "Checks if the value of the field is `OS64`"]
    #[inline(always)]
    pub fn is_os64(&self) -> bool {
        *self == OVSR_A::OS64
    }
    #[doc = "Checks if the value of the field is `OS128`"]
    #[inline(always)]
    pub fn is_os128(&self) -> bool {
        *self == OVSR_A::OS128
    }
    #[doc = "Checks if the value of the field is `OS256`"]
    #[inline(always)]
    pub fn is_os256(&self) -> bool {
        *self == OVSR_A::OS256
    }
}
#[doc = "Write proxy for field `OVSR`"]
pub struct OVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Oversampling ratio of 2"]
    #[inline(always)]
    pub fn os2(self) -> &'a mut W {
        self.variant(OVSR_A::OS2)
    }
    #[doc = "Oversampling ratio of 4"]
    #[inline(always)]
    pub fn os4(self) -> &'a mut W {
        self.variant(OVSR_A::OS4)
    }
    #[doc = "Oversampling ratio of 8"]
    #[inline(always)]
    pub fn os8(self) -> &'a mut W {
        self.variant(OVSR_A::OS8)
    }
    #[doc = "Oversampling ratio of 16"]
    #[inline(always)]
    pub fn os16(self) -> &'a mut W {
        self.variant(OVSR_A::OS16)
    }
    #[doc = "Oversampling ratio of 32"]
    #[inline(always)]
    pub fn os32(self) -> &'a mut W {
        self.variant(OVSR_A::OS32)
    }
    #[doc = "Oversampling ratio of 64"]
    #[inline(always)]
    pub fn os64(self) -> &'a mut W {
        self.variant(OVSR_A::OS64)
    }
    #[doc = "Oversampling ratio of 128"]
    #[inline(always)]
    pub fn os128(self) -> &'a mut W {
        self.variant(OVSR_A::OS128)
    }
    #[doc = "Oversampling ratio of 256"]
    #[inline(always)]
    pub fn os256(self) -> &'a mut W {
        self.variant(OVSR_A::OS256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Injected Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JOVSE_A {
    #[doc = "0: Injected oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Injected oversampling enabled"]
    ENABLED = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JOVSE`"]
pub type JOVSE_R = crate::R<bool, JOVSE_A>;
impl JOVSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::DISABLED,
            true => JOVSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE_A::ENABLED
    }
}
#[doc = "Write proxy for field `JOVSE`"]
pub struct JOVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JOVSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::DISABLED)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::ENABLED)
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
#[doc = "Regular Oversampling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVSE_A {
    #[doc = "0: Regular oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Regular oversampling enabled"]
    ENABLED = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVSE`"]
pub type ROVSE_R = crate::R<bool, ROVSE_A>;
impl ROVSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::DISABLED,
            true => ROVSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ROVSE`"]
pub struct ROVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::DISABLED)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::ENABLED)
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
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Sampling time control trigger mode"]
    #[inline(always)]
    pub fn smptrig(&mut self) -> SMPTRIG_W {
        SMPTRIG_W { w: self }
    }
    #[doc = "Bit 26 - Bulb sampling mode"]
    #[inline(always)]
    pub fn bulb(&mut self) -> BULB_W {
        BULB_W { w: self }
    }
    #[doc = "Bit 25 - Software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W {
        SWTRIG_W { w: self }
    }
    #[doc = "Bit 16 - Gain compensation mode"]
    #[inline(always)]
    pub fn gcomp(&mut self) -> GCOMP_W {
        GCOMP_W { w: self }
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W {
        ROVSM_W { w: self }
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W {
        TROVS_W { w: self }
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W { w: self }
    }
    #[doc = "Bit 1 - Injected Oversampling Enable"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W {
        JOVSE_W { w: self }
    }
    #[doc = "Bit 0 - Regular Oversampling Enable"]
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W {
        ROVSE_W { w: self }
    }
}
