#[doc = "Register `CNTR` reader"]
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR` writer"]
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force USB Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRES_A {
    #[doc = "0: Clear USB reset"]
    NORESET = 0,
    #[doc = "1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    RESET = 1,
}
impl From<FRES_A> for bool {
    #[inline(always)]
    fn from(variant: FRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRES` reader - Force USB Reset"]
pub struct FRES_R(crate::FieldReader<bool, FRES_A>);
impl FRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRES_A {
        match self.bits {
            false => FRES_A::NORESET,
            true => FRES_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == FRES_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == FRES_A::RESET
    }
}
impl core::ops::Deref for FRES_R {
    type Target = crate::FieldReader<bool, FRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRES` writer - Force USB Reset"]
pub struct FRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear USB reset"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FRES_A::NORESET)
    }
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRES_A::RESET)
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
#[doc = "Power down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWN_A {
    #[doc = "0: No power down"]
    DISABLED = 0,
    #[doc = "1: Enter power down mode"]
    ENABLED = 1,
}
impl From<PDWN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDWN` reader - Power down"]
pub struct PDWN_R(crate::FieldReader<bool, PDWN_A>);
impl PDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDWN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDWN_A {
        match self.bits {
            false => PDWN_A::DISABLED,
            true => PDWN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PDWN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PDWN_A::ENABLED
    }
}
impl core::ops::Deref for PDWN_R {
    type Target = crate::FieldReader<bool, PDWN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDWN` writer - Power down"]
pub struct PDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDWN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No power down"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDWN_A::DISABLED)
    }
    #[doc = "Enter power down mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDWN_A::ENABLED)
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
#[doc = "Low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODE_A {
    #[doc = "0: No low-power mode"]
    DISABLED = 0,
    #[doc = "1: Enter low-power mode"]
    ENABLED = 1,
}
impl From<LPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMODE` reader - Low-power mode"]
pub struct LPMODE_R(crate::FieldReader<bool, LPMODE_A>);
impl LPMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            false => LPMODE_A::DISABLED,
            true => LPMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LPMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LPMODE_A::ENABLED
    }
}
impl core::ops::Deref for LPMODE_R {
    type Target = crate::FieldReader<bool, LPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMODE` writer - Low-power mode"]
pub struct LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMODE_A::DISABLED)
    }
    #[doc = "Enter low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMODE_A::ENABLED)
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
#[doc = "Force suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSUSP_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    SUSPEND = 1,
}
impl From<FSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSUSP` reader - Force suspend"]
pub struct FSUSP_R(crate::FieldReader<bool, FSUSP_A>);
impl FSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSUSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSUSP_A {
        match self.bits {
            false => FSUSP_A::NOEFFECT,
            true => FSUSP_A::SUSPEND,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == FSUSP_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == FSUSP_A::SUSPEND
    }
}
impl core::ops::Deref for FSUSP_R {
    type Target = crate::FieldReader<bool, FSUSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSUSP` writer - Force suspend"]
pub struct FSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSUSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FSUSP_A::NOEFFECT)
    }
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(FSUSP_A::SUSPEND)
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
#[doc = "Resume request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUME_A {
    #[doc = "1: Resume requested"]
    REQUESTED = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Resume request"]
pub struct RESUME_R(crate::FieldReader<bool, RESUME_A>);
impl RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESUME_A> {
        match self.bits {
            true => Some(RESUME_A::REQUESTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        **self == RESUME_A::REQUESTED
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, RESUME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Resume request"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESUME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Resume requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(RESUME_A::REQUESTED)
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
#[doc = "LPM L1 Resume request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1RESUME_A {
    #[doc = "1: LPM L1 request requested"]
    REQUESTED = 1,
}
impl From<L1RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: L1RESUME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1RESUME` reader - LPM L1 Resume request"]
pub struct L1RESUME_R(crate::FieldReader<bool, L1RESUME_A>);
impl L1RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1RESUME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<L1RESUME_A> {
        match self.bits {
            true => Some(L1RESUME_A::REQUESTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        **self == L1RESUME_A::REQUESTED
    }
}
impl core::ops::Deref for L1RESUME_R {
    type Target = crate::FieldReader<bool, L1RESUME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1RESUME` writer - LPM L1 Resume request"]
pub struct L1RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> L1RESUME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1RESUME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPM L1 request requested"]
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(L1RESUME_A::REQUESTED)
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
#[doc = "LPM L1 state request interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQM_A {
    #[doc = "0: L1REQ Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<L1REQM_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub struct L1REQM_R(crate::FieldReader<bool, L1REQM_A>);
impl L1REQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1REQM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L1REQM_A {
        match self.bits {
            false => L1REQM_A::DISABLED,
            true => L1REQM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == L1REQM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == L1REQM_A::ENABLED
    }
}
impl core::ops::Deref for L1REQM_R {
    type Target = crate::FieldReader<bool, L1REQM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub struct L1REQM_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L1REQM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "L1REQ Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(L1REQM_A::DISABLED)
    }
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(L1REQM_A::ENABLED)
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
#[doc = "Expected start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFM_A {
    #[doc = "0: ESOF Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<ESOFM_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub struct ESOFM_R(crate::FieldReader<bool, ESOFM_A>);
impl ESOFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESOFM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESOFM_A {
        match self.bits {
            false => ESOFM_A::DISABLED,
            true => ESOFM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ESOFM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ESOFM_A::ENABLED
    }
}
impl core::ops::Deref for ESOFM_R {
    type Target = crate::FieldReader<bool, ESOFM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub struct ESOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESOFM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ESOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESOFM_A::DISABLED)
    }
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESOFM_A::ENABLED)
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
#[doc = "Start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFM_A {
    #[doc = "0: SOF Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<SOFM_A> for bool {
    #[inline(always)]
    fn from(variant: SOFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub struct SOFM_R(crate::FieldReader<bool, SOFM_A>);
impl SOFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFM_A {
        match self.bits {
            false => SOFM_A::DISABLED,
            true => SOFM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SOFM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SOFM_A::ENABLED
    }
}
impl core::ops::Deref for SOFM_R {
    type Target = crate::FieldReader<bool, SOFM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SOF Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOFM_A::DISABLED)
    }
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOFM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "USB reset interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETM_A {
    #[doc = "0: RESET Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<RESETM_A> for bool {
    #[inline(always)]
    fn from(variant: RESETM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETM` reader - USB reset interrupt mask"]
pub struct RESETM_R(crate::FieldReader<bool, RESETM_A>);
impl RESETM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETM_A {
        match self.bits {
            false => RESETM_A::DISABLED,
            true => RESETM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RESETM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RESETM_A::ENABLED
    }
}
impl core::ops::Deref for RESETM_R {
    type Target = crate::FieldReader<bool, RESETM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETM` writer - USB reset interrupt mask"]
pub struct RESETM_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RESET Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETM_A::DISABLED)
    }
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETM_A::ENABLED)
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
#[doc = "Suspend mode interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPM_A {
    #[doc = "0: Suspend Mode Request SUSP Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<SUSPM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub struct SUSPM_R(crate::FieldReader<bool, SUSPM_A>);
impl SUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPM_A {
        match self.bits {
            false => SUSPM_A::DISABLED,
            true => SUSPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SUSPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SUSPM_A::ENABLED
    }
}
impl core::ops::Deref for SUSPM_R {
    type Target = crate::FieldReader<bool, SUSPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub struct SUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUSPM_A::DISABLED)
    }
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUSPM_A::ENABLED)
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
#[doc = "Wakeup interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPM_A {
    #[doc = "0: WKUP Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<WKUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub struct WKUPM_R(crate::FieldReader<bool, WKUPM_A>);
impl WKUPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPM_A {
        match self.bits {
            false => WKUPM_A::DISABLED,
            true => WKUPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPM_A::ENABLED
    }
}
impl core::ops::Deref for WKUPM_R {
    type Target = crate::FieldReader<bool, WKUPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub struct WKUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WKUP Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUPM_A::DISABLED)
    }
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUPM_A::ENABLED)
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
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRM_A {
    #[doc = "0: ERR Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<ERRM_A> for bool {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub struct ERRM_R(crate::FieldReader<bool, ERRM_A>);
impl ERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRM_A {
        match self.bits {
            false => ERRM_A::DISABLED,
            true => ERRM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRM_A::ENABLED
    }
}
impl core::ops::Deref for ERRM_R {
    type Target = crate::FieldReader<bool, ERRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub struct ERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRM_A::DISABLED)
    }
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRM_A::ENABLED)
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
#[doc = "Packet memory area over / underrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRM_A {
    #[doc = "0: PMAOVR Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<PMAOVRM_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub struct PMAOVRM_R(crate::FieldReader<bool, PMAOVRM_A>);
impl PMAOVRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMAOVRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAOVRM_A {
        match self.bits {
            false => PMAOVRM_A::DISABLED,
            true => PMAOVRM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PMAOVRM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PMAOVRM_A::ENABLED
    }
}
impl core::ops::Deref for PMAOVRM_R {
    type Target = crate::FieldReader<bool, PMAOVRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub struct PMAOVRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMAOVRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PMAOVR Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::DISABLED)
    }
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::ENABLED)
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
#[doc = "Correct transfer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRM_A {
    #[doc = "0: Correct Transfer (CTR) Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED = 1,
}
impl From<CTRM_A> for bool {
    #[inline(always)]
    fn from(variant: CTRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub struct CTRM_R(crate::FieldReader<bool, CTRM_A>);
impl CTRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRM_A {
        match self.bits {
            false => CTRM_A::DISABLED,
            true => CTRM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTRM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTRM_A::ENABLED
    }
}
impl core::ops::Deref for CTRM_R {
    type Target = crate::FieldReader<bool, CTRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub struct CTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTRM_A::DISABLED)
    }
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTRM_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W {
        FRES_W { w: self }
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W {
        PDWN_W { w: self }
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W { w: self }
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W {
        FSUSP_W { w: self }
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W {
        L1RESUME_W { w: self }
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1REQM_W {
        L1REQM_W { w: self }
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W {
        ESOFM_W { w: self }
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W {
        RESETM_W { w: self }
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W {
        SUSPM_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W {
        WKUPM_W { w: self }
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W {
        ERRM_W { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W {
        PMAOVRM_W { w: self }
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W {
        CTRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](index.html) module"]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr::R](R) reader structure"]
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr::W](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTR to value 0x03"]
impl crate::Resettable for CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
