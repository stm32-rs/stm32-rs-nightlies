#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Burst DMA Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BRSTDMA_A {
    #[doc = "0: Update done independently from the DMA burst transfer completion"]
    INDEPENDENT = 0,
    #[doc = "1: Update done when the DMA burst transfer is completed"]
    COMPLETION = 1,
    #[doc = "2: Update done on master timer roll-over following a DMA burst transfer completion"]
    ROLLOVER = 2,
}
impl From<BRSTDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: BRSTDMA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BRSTDMA`"]
pub type BRSTDMA_R = crate::R<u8, BRSTDMA_A>;
impl BRSTDMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BRSTDMA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BRSTDMA_A::INDEPENDENT),
            1 => Val(BRSTDMA_A::COMPLETION),
            2 => Val(BRSTDMA_A::ROLLOVER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == BRSTDMA_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `COMPLETION`"]
    #[inline(always)]
    pub fn is_completion(&self) -> bool {
        *self == BRSTDMA_A::COMPLETION
    }
    #[doc = "Checks if the value of the field is `ROLLOVER`"]
    #[inline(always)]
    pub fn is_rollover(&self) -> bool {
        *self == BRSTDMA_A::ROLLOVER
    }
}
#[doc = "Write proxy for field `BRSTDMA`"]
pub struct BRSTDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSTDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSTDMA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Update done independently from the DMA burst transfer completion"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(BRSTDMA_A::INDEPENDENT)
    }
    #[doc = "Update done when the DMA burst transfer is completed"]
    #[inline(always)]
    pub fn completion(self) -> &'a mut W {
        self.variant(BRSTDMA_A::COMPLETION)
    }
    #[doc = "Update done on master timer roll-over following a DMA burst transfer completion"]
    #[inline(always)]
    pub fn rollover(self) -> &'a mut W {
        self.variant(BRSTDMA_A::ROLLOVER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Master Timer Repetition update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MREPU_A {
    #[doc = "0: Update on repetition disabled"]
    DISABLED = 0,
    #[doc = "1: Update on repetition enabled"]
    ENABLED = 1,
}
impl From<MREPU_A> for bool {
    #[inline(always)]
    fn from(variant: MREPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MREPU`"]
pub type MREPU_R = crate::R<bool, MREPU_A>;
impl MREPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MREPU_A {
        match self.bits {
            false => MREPU_A::DISABLED,
            true => MREPU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MREPU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MREPU_A::ENABLED
    }
}
#[doc = "Write proxy for field `MREPU`"]
pub struct MREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update on repetition disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MREPU_A::DISABLED)
    }
    #[doc = "Update on repetition enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MREPU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREEN_A {
    #[doc = "0: Preload disabled: the write access is directly done into the active register"]
    DISABLED = 0,
    #[doc = "1: Preload enabled: the write access is done into the preload register"]
    ENABLED = 1,
}
impl From<PREEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREEN`"]
pub type PREEN_R = crate::R<bool, PREEN_A>;
impl PREEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREEN_A {
        match self.bits {
            false => PREEN_A::DISABLED,
            true => PREEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREEN`"]
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Preload disabled: the write access is directly done into the active register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREEN_A::DISABLED)
    }
    #[doc = "Preload enabled: the write access is done into the preload register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREEN_A::ENABLED)
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
#[doc = "AC Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DACSYNC_A {
    #[doc = "0: No DAC trigger generated"]
    DISABLED = 0,
    #[doc = "1: Trigger generated on DACSync1"]
    DACSYNC1 = 1,
    #[doc = "2: Trigger generated on DACSync2"]
    DACSYNC2 = 2,
    #[doc = "3: Trigger generated on DACSync3"]
    DACSYNC3 = 3,
}
impl From<DACSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: DACSYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DACSYNC`"]
pub type DACSYNC_R = crate::R<u8, DACSYNC_A>;
impl DACSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACSYNC_A {
        match self.bits {
            0 => DACSYNC_A::DISABLED,
            1 => DACSYNC_A::DACSYNC1,
            2 => DACSYNC_A::DACSYNC2,
            3 => DACSYNC_A::DACSYNC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DACSYNC1`"]
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC_A::DACSYNC1
    }
    #[doc = "Checks if the value of the field is `DACSYNC2`"]
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC_A::DACSYNC2
    }
    #[doc = "Checks if the value of the field is `DACSYNC3`"]
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC_A::DACSYNC3
    }
}
#[doc = "Write proxy for field `DACSYNC`"]
pub struct DACSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSYNC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No DAC trigger generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACSYNC_A::DISABLED)
    }
    #[doc = "Trigger generated on DACSync1"]
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC1)
    }
    #[doc = "Trigger generated on DACSync2"]
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC2)
    }
    #[doc = "Trigger generated on DACSync3"]
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut W {
        self.variant(DACSYNC_A::DACSYNC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Timer E counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TECEN_A {
    #[doc = "0: Timer counter disabled"]
    DISABLED = 0,
    #[doc = "1: Timer counter enabled"]
    ENABLED = 1,
}
impl From<TECEN_A> for bool {
    #[inline(always)]
    fn from(variant: TECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TECEN`"]
pub type TECEN_R = crate::R<bool, TECEN_A>;
impl TECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TECEN_A {
        match self.bits {
            false => TECEN_A::DISABLED,
            true => TECEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TECEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TECEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `TECEN`"]
pub struct TECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TECEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TECEN_A::ENABLED)
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
#[doc = "Timer D counter enable"]
pub type TDCEN_A = TECEN_A;
#[doc = "Reader of field `TDCEN`"]
pub type TDCEN_R = crate::R<bool, TECEN_A>;
#[doc = "Write proxy for field `TDCEN`"]
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TECEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TECEN_A::ENABLED)
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
#[doc = "Timer C counter enable"]
pub type TCCEN_A = TECEN_A;
#[doc = "Reader of field `TCCEN`"]
pub type TCCEN_R = crate::R<bool, TECEN_A>;
#[doc = "Write proxy for field `TCCEN`"]
pub struct TCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TECEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TECEN_A::ENABLED)
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
#[doc = "Timer B counter enable"]
pub type TBCEN_A = TECEN_A;
#[doc = "Reader of field `TBCEN`"]
pub type TBCEN_R = crate::R<bool, TECEN_A>;
#[doc = "Write proxy for field `TBCEN`"]
pub struct TBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TECEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TECEN_A::ENABLED)
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
#[doc = "Timer A counter enable"]
pub type TACEN_A = TECEN_A;
#[doc = "Reader of field `TACEN`"]
pub type TACEN_R = crate::R<bool, TECEN_A>;
#[doc = "Write proxy for field `TACEN`"]
pub struct TACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TECEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TECEN_A::ENABLED)
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
#[doc = "Master Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCEN_A {
    #[doc = "0: Master timer counter disabled"]
    DISABLED = 0,
    #[doc = "1: Master timer counter enabled"]
    ENABLED = 1,
}
impl From<MCEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCEN`"]
pub type MCEN_R = crate::R<bool, MCEN_A>;
impl MCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCEN_A {
        match self.bits {
            false => MCEN_A::DISABLED,
            true => MCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MCEN`"]
pub struct MCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCEN_A::DISABLED)
    }
    #[doc = "Master timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCEN_A::ENABLED)
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
#[doc = "Synchronization source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCSRC_A {
    #[doc = "0: Master timer Start"]
    MASTERSTART = 0,
    #[doc = "1: Master timer Compare 1 event"]
    MASTERCOMPARE1 = 1,
    #[doc = "2: Timer A start/reset"]
    TIMERASTART = 2,
    #[doc = "3: Timer A Compare 1 event"]
    TIMERACOMPARE1 = 3,
}
impl From<SYNCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCSRC`"]
pub type SYNCSRC_R = crate::R<u8, SYNCSRC_A>;
impl SYNCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSRC_A {
        match self.bits {
            0 => SYNCSRC_A::MASTERSTART,
            1 => SYNCSRC_A::MASTERCOMPARE1,
            2 => SYNCSRC_A::TIMERASTART,
            3 => SYNCSRC_A::TIMERACOMPARE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MASTERSTART`"]
    #[inline(always)]
    pub fn is_master_start(&self) -> bool {
        *self == SYNCSRC_A::MASTERSTART
    }
    #[doc = "Checks if the value of the field is `MASTERCOMPARE1`"]
    #[inline(always)]
    pub fn is_master_compare1(&self) -> bool {
        *self == SYNCSRC_A::MASTERCOMPARE1
    }
    #[doc = "Checks if the value of the field is `TIMERASTART`"]
    #[inline(always)]
    pub fn is_timer_astart(&self) -> bool {
        *self == SYNCSRC_A::TIMERASTART
    }
    #[doc = "Checks if the value of the field is `TIMERACOMPARE1`"]
    #[inline(always)]
    pub fn is_timer_acompare1(&self) -> bool {
        *self == SYNCSRC_A::TIMERACOMPARE1
    }
}
#[doc = "Write proxy for field `SYNCSRC`"]
pub struct SYNCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Master timer Start"]
    #[inline(always)]
    pub fn master_start(self) -> &'a mut W {
        self.variant(SYNCSRC_A::MASTERSTART)
    }
    #[doc = "Master timer Compare 1 event"]
    #[inline(always)]
    pub fn master_compare1(self) -> &'a mut W {
        self.variant(SYNCSRC_A::MASTERCOMPARE1)
    }
    #[doc = "Timer A start/reset"]
    #[inline(always)]
    pub fn timer_astart(self) -> &'a mut W {
        self.variant(SYNCSRC_A::TIMERASTART)
    }
    #[doc = "Timer A Compare 1 event"]
    #[inline(always)]
    pub fn timer_acompare1(self) -> &'a mut W {
        self.variant(SYNCSRC_A::TIMERACOMPARE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Synchronization output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCOUT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    POSITIVEPULSE = 2,
    #[doc = "3: Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    NEGATIVEPULSE = 3,
}
impl From<SYNCOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCOUT`"]
pub type SYNCOUT_R = crate::R<u8, SYNCOUT_A>;
impl SYNCOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNCOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNCOUT_A::DISABLED),
            2 => Val(SYNCOUT_A::POSITIVEPULSE),
            3 => Val(SYNCOUT_A::NEGATIVEPULSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `POSITIVEPULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == SYNCOUT_A::POSITIVEPULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVEPULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == SYNCOUT_A::NEGATIVEPULSE
    }
}
#[doc = "Write proxy for field `SYNCOUT`"]
pub struct SYNCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCOUT_A::DISABLED)
    }
    #[doc = "Positive pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(SYNCOUT_A::POSITIVEPULSE)
    }
    #[doc = "Negative pulse on SCOUT output (16x f_HRTIM clock cycles)"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(SYNCOUT_A::NEGATIVEPULSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Synchronization Starts Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCSTRTM_A {
    #[doc = "0: No effect on the master timer"]
    DISABLED = 0,
    #[doc = "1: A synchroniation input event starts the master timer"]
    ENABLED = 1,
}
impl From<SYNCSTRTM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCSTRTM`"]
pub type SYNCSTRTM_R = crate::R<bool, SYNCSTRTM_A>;
impl SYNCSTRTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCSTRTM_A {
        match self.bits {
            false => SYNCSTRTM_A::DISABLED,
            true => SYNCSTRTM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCSTRTM_A::ENABLED
    }
}
#[doc = "Write proxy for field `SYNCSTRTM`"]
pub struct SYNCSTRTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSTRTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCSTRTM_A::DISABLED)
    }
    #[doc = "A synchroniation input event starts the master timer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCSTRTM_A::ENABLED)
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
#[doc = "Synchronization Resets Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCRSTM_A {
    #[doc = "0: No effect on the master timer"]
    DISABLED = 0,
    #[doc = "1: A synchroniation input event resets the master timer"]
    ENABLED = 1,
}
impl From<SYNCRSTM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCRSTM`"]
pub type SYNCRSTM_R = crate::R<bool, SYNCRSTM_A>;
impl SYNCRSTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCRSTM_A {
        match self.bits {
            false => SYNCRSTM_A::DISABLED,
            true => SYNCRSTM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCRSTM_A::ENABLED
    }
}
#[doc = "Write proxy for field `SYNCRSTM`"]
pub struct SYNCRSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCRSTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect on the master timer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCRSTM_A::DISABLED)
    }
    #[doc = "A synchroniation input event resets the master timer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCRSTM_A::ENABLED)
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
#[doc = "ynchronization input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCIN_A {
    #[doc = "0: Disabled. HRTIM is not synchronized and runs in standalone mode"]
    DISABLED = 0,
    #[doc = "2: Internal event: the HRTIM is synchronized with the on-chip timer"]
    INTERNAL = 2,
    #[doc = "3: External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
    EXTERNAL = 3,
}
impl From<SYNCIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCIN`"]
pub type SYNCIN_R = crate::R<u8, SYNCIN_A>;
impl SYNCIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNCIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNCIN_A::DISABLED),
            2 => Val(SYNCIN_A::INTERNAL),
            3 => Val(SYNCIN_A::EXTERNAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCIN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCIN_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCIN_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `SYNCIN`"]
pub struct SYNCIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled. HRTIM is not synchronized and runs in standalone mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCIN_A::DISABLED)
    }
    #[doc = "Internal event: the HRTIM is synchronized with the on-chip timer"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SYNCIN_A::INTERNAL)
    }
    #[doc = "External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SYNCIN_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Half mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALF_A {
    #[doc = "0: Half mode disabled"]
    DISABLED = 0,
    #[doc = "1: Half mode enabled"]
    ENABLED = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALF`"]
pub type HALF_R = crate::R<bool, HALF_A>;
impl HALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::DISABLED,
            true => HALF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF_A::ENABLED
    }
}
#[doc = "Write proxy for field `HALF`"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Half mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HALF_A::DISABLED)
    }
    #[doc = "Half mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HALF_A::ENABLED)
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
#[doc = "Master Re-triggerable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETRIG_A {
    #[doc = "0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    DISABLED = 0,
    #[doc = "1: The timer is retriggerable: a counter reset is done whatever the counter state"]
    ENABLED = 1,
}
impl From<RETRIG_A> for bool {
    #[inline(always)]
    fn from(variant: RETRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETRIG`"]
pub type RETRIG_R = crate::R<bool, RETRIG_A>;
impl RETRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRIG_A {
        match self.bits {
            false => RETRIG_A::DISABLED,
            true => RETRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG_A::ENABLED
    }
}
#[doc = "Write proxy for field `RETRIG`"]
pub struct RETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer is not re-triggerable: a counter reset can be done only if the counter is stopped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RETRIG_A::DISABLED)
    }
    #[doc = "The timer is retriggerable: a counter reset is done whatever the counter state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RETRIG_A::ENABLED)
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
#[doc = "Master Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: The timer operates in single-shot mode and stops when it reaches the MPER value"]
    SINGLESHOT = 0,
    #[doc = "1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONT`"]
pub type CONT_R = crate::R<bool, CONT_A>;
impl CONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLESHOT,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLESHOT`"]
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT_A::SINGLESHOT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::CONTINUOUS
    }
}
#[doc = "Write proxy for field `CONT`"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer operates in single-shot mode and stops when it reaches the MPER value"]
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(CONT_A::SINGLESHOT)
    }
    #[doc = "The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
#[doc = "Reader of field `CKPSC`"]
pub type CKPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKPSC`"]
pub struct CKPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&mut self) -> BRSTDMA_W {
        BRSTDMA_W { w: self }
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&mut self) -> MREPU_W {
        MREPU_W { w: self }
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W {
        PREEN_W { w: self }
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W {
        DACSYNC_W { w: self }
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&mut self) -> TECEN_W {
        TECEN_W { w: self }
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W {
        TDCEN_W { w: self }
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&mut self) -> TCCEN_W {
        TCCEN_W { w: self }
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&mut self) -> TBCEN_W {
        TBCEN_W { w: self }
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&mut self) -> TACEN_W {
        TACEN_W { w: self }
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W {
        MCEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W {
        SYNCSRC_W { w: self }
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W {
        SYNCOUT_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W {
        SYNCSTRTM_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W {
        SYNCRSTM_W { w: self }
    }
    #[doc = "Bits 8:9 - ynchronization input"]
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W {
        SYNCIN_W { w: self }
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W {
        RETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ckpsc(&mut self) -> CKPSC_W {
        CKPSC_W { w: self }
    }
}
