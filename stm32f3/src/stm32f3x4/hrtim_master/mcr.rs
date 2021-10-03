#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `BRSTDMA` reader - Burst DMA Update"]
pub struct BRSTDMA_R(crate::FieldReader<u8, BRSTDMA_A>);
impl BRSTDMA_R {
    pub(crate) fn new(bits: u8) -> Self {
        BRSTDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BRSTDMA_A> {
        match self.bits {
            0 => Some(BRSTDMA_A::INDEPENDENT),
            1 => Some(BRSTDMA_A::COMPLETION),
            2 => Some(BRSTDMA_A::ROLLOVER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == BRSTDMA_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `COMPLETION`"]
    #[inline(always)]
    pub fn is_completion(&self) -> bool {
        **self == BRSTDMA_A::COMPLETION
    }
    #[doc = "Checks if the value of the field is `ROLLOVER`"]
    #[inline(always)]
    pub fn is_rollover(&self) -> bool {
        **self == BRSTDMA_A::ROLLOVER
    }
}
impl core::ops::Deref for BRSTDMA_R {
    type Target = crate::FieldReader<u8, BRSTDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSTDMA` writer - Burst DMA Update"]
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
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
#[doc = "Field `MREPU` reader - Master Timer Repetition update"]
pub struct MREPU_R(crate::FieldReader<bool, MREPU_A>);
impl MREPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREPU_R(crate::FieldReader::new(bits))
    }
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
        **self == MREPU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MREPU_A::ENABLED
    }
}
impl core::ops::Deref for MREPU_R {
    type Target = crate::FieldReader<bool, MREPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREPU` writer - Master Timer Repetition update"]
pub struct MREPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPU_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
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
#[doc = "Field `PREEN` reader - Preload enable"]
pub struct PREEN_R(crate::FieldReader<bool, PREEN_A>);
impl PREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREEN_R(crate::FieldReader::new(bits))
    }
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
        **self == PREEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PREEN_A::ENABLED
    }
}
impl core::ops::Deref for PREEN_R {
    type Target = crate::FieldReader<bool, PREEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREEN` writer - Preload enable"]
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
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
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub struct DACSYNC_R(crate::FieldReader<u8, DACSYNC_A>);
impl DACSYNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACSYNC_R(crate::FieldReader::new(bits))
    }
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
        **self == DACSYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DACSYNC1`"]
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        **self == DACSYNC_A::DACSYNC1
    }
    #[doc = "Checks if the value of the field is `DACSYNC2`"]
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        **self == DACSYNC_A::DACSYNC2
    }
    #[doc = "Checks if the value of the field is `DACSYNC3`"]
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        **self == DACSYNC_A::DACSYNC3
    }
}
impl core::ops::Deref for DACSYNC_R {
    type Target = crate::FieldReader<u8, DACSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub struct DACSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACSYNC_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Timer E counter enable"]
pub type TECEN_A = TACEN_A;
#[doc = "Field `TECEN` reader - Timer E counter enable"]
pub type TECEN_R = TACEN_R;
#[doc = "Field `TECEN` writer - Timer E counter enable"]
pub struct TECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Timer D counter enable"]
pub type TDCEN_A = TACEN_A;
#[doc = "Field `TDCEN` reader - Timer D counter enable"]
pub type TDCEN_R = TACEN_R;
#[doc = "Field `TDCEN` writer - Timer D counter enable"]
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDCEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDCEN_A::ENABLED)
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
#[doc = "Timer C counter enable"]
pub type TCCEN_A = TACEN_A;
#[doc = "Field `TCCEN` reader - Timer C counter enable"]
pub type TCCEN_R = TACEN_R;
#[doc = "Field `TCCEN` writer - Timer C counter enable"]
pub struct TCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCCEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCCEN_A::ENABLED)
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
#[doc = "Timer B counter enable"]
pub type TBCEN_A = TACEN_A;
#[doc = "Field `TBCEN` reader - Timer B counter enable"]
pub type TBCEN_R = TACEN_R;
#[doc = "Field `TBCEN` writer - Timer B counter enable"]
pub struct TBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TBCEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TBCEN_A::ENABLED)
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
#[doc = "Timer A counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACEN_A {
    #[doc = "0: Timer counter disabled"]
    DISABLED = 0,
    #[doc = "1: Timer counter enabled"]
    ENABLED = 1,
}
impl From<TACEN_A> for bool {
    #[inline(always)]
    fn from(variant: TACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACEN` reader - Timer A counter enable"]
pub struct TACEN_R(crate::FieldReader<bool, TACEN_A>);
impl TACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACEN_A {
        match self.bits {
            false => TACEN_A::DISABLED,
            true => TACEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TACEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TACEN_A::ENABLED
    }
}
impl core::ops::Deref for TACEN_R {
    type Target = crate::FieldReader<bool, TACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACEN` writer - Timer A counter enable"]
pub struct TACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TACEN_A::DISABLED)
    }
    #[doc = "Timer counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TACEN_A::ENABLED)
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
#[doc = "Field `MCEN` reader - Master Counter enable"]
pub struct MCEN_R(crate::FieldReader<bool, MCEN_A>);
impl MCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCEN_R(crate::FieldReader::new(bits))
    }
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
        **self == MCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCEN_A::ENABLED
    }
}
impl core::ops::Deref for MCEN_R {
    type Target = crate::FieldReader<bool, MCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCEN` writer - Master Counter enable"]
pub struct MCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `SYNCSRC` reader - Synchronization source"]
pub struct SYNCSRC_R(crate::FieldReader<u8, SYNCSRC_A>);
impl SYNCSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCSRC_R(crate::FieldReader::new(bits))
    }
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
        **self == SYNCSRC_A::MASTERSTART
    }
    #[doc = "Checks if the value of the field is `MASTERCOMPARE1`"]
    #[inline(always)]
    pub fn is_master_compare1(&self) -> bool {
        **self == SYNCSRC_A::MASTERCOMPARE1
    }
    #[doc = "Checks if the value of the field is `TIMERASTART`"]
    #[inline(always)]
    pub fn is_timer_astart(&self) -> bool {
        **self == SYNCSRC_A::TIMERASTART
    }
    #[doc = "Checks if the value of the field is `TIMERACOMPARE1`"]
    #[inline(always)]
    pub fn is_timer_acompare1(&self) -> bool {
        **self == SYNCSRC_A::TIMERACOMPARE1
    }
}
impl core::ops::Deref for SYNCSRC_R {
    type Target = crate::FieldReader<u8, SYNCSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSRC` writer - Synchronization source"]
pub struct SYNCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSRC_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
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
#[doc = "Field `SYNCOUT` reader - Synchronization output"]
pub struct SYNCOUT_R(crate::FieldReader<u8, SYNCOUT_A>);
impl SYNCOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCOUT_A> {
        match self.bits {
            0 => Some(SYNCOUT_A::DISABLED),
            2 => Some(SYNCOUT_A::POSITIVEPULSE),
            3 => Some(SYNCOUT_A::NEGATIVEPULSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYNCOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `POSITIVEPULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == SYNCOUT_A::POSITIVEPULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVEPULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == SYNCOUT_A::NEGATIVEPULSE
    }
}
impl core::ops::Deref for SYNCOUT_R {
    type Target = crate::FieldReader<u8, SYNCOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCOUT` writer - Synchronization output"]
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
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
#[doc = "Field `SYNCSTRTM` reader - Synchronization Starts Master"]
pub struct SYNCSTRTM_R(crate::FieldReader<bool, SYNCSTRTM_A>);
impl SYNCSTRTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCSTRTM_R(crate::FieldReader::new(bits))
    }
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
        **self == SYNCSTRTM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYNCSTRTM_A::ENABLED
    }
}
impl core::ops::Deref for SYNCSTRTM_R {
    type Target = crate::FieldReader<bool, SYNCSTRTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCSTRTM` writer - Synchronization Starts Master"]
pub struct SYNCSTRTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSTRTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCSTRTM_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
#[doc = "Field `SYNCRSTM` reader - Synchronization Resets Master"]
pub struct SYNCRSTM_R(crate::FieldReader<bool, SYNCRSTM_A>);
impl SYNCRSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRSTM_R(crate::FieldReader::new(bits))
    }
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
        **self == SYNCRSTM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYNCRSTM_A::ENABLED
    }
}
impl core::ops::Deref for SYNCRSTM_R {
    type Target = crate::FieldReader<bool, SYNCRSTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRSTM` writer - Synchronization Resets Master"]
pub struct SYNCRSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCRSTM_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
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
#[doc = "Field `SYNCIN` reader - ynchronization input"]
pub struct SYNCIN_R(crate::FieldReader<u8, SYNCIN_A>);
impl SYNCIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCIN_A> {
        match self.bits {
            0 => Some(SYNCIN_A::DISABLED),
            2 => Some(SYNCIN_A::INTERNAL),
            3 => Some(SYNCIN_A::EXTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYNCIN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == SYNCIN_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == SYNCIN_A::EXTERNAL
    }
}
impl core::ops::Deref for SYNCIN_R {
    type Target = crate::FieldReader<u8, SYNCIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCIN` writer - ynchronization input"]
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
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
#[doc = "Field `HALF` reader - Half mode enable"]
pub struct HALF_R(crate::FieldReader<bool, HALF_A>);
impl HALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALF_R(crate::FieldReader::new(bits))
    }
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
        **self == HALF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HALF_A::ENABLED
    }
}
impl core::ops::Deref for HALF_R {
    type Target = crate::FieldReader<bool, HALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALF` writer - Half mode enable"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALF_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
#[doc = "Field `RETRIG` reader - Master Re-triggerable mode"]
pub struct RETRIG_R(crate::FieldReader<bool, RETRIG_A>);
impl RETRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRIG_R(crate::FieldReader::new(bits))
    }
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
        **self == RETRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RETRIG_A::ENABLED
    }
}
impl core::ops::Deref for RETRIG_R {
    type Target = crate::FieldReader<bool, RETRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRIG` writer - Master Re-triggerable mode"]
pub struct RETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRIG_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `CONT` reader - Master Continuous mode"]
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
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
        **self == CONT_A::SINGLESHOT
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Master Continuous mode"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CKPSC` reader - HRTIM Master Clock prescaler"]
pub struct CKPSC_R(crate::FieldReader<u8, u8>);
impl CKPSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPSC` writer - HRTIM Master Clock prescaler"]
pub struct CKPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
