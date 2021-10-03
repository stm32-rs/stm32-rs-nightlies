#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog watchdog enable on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDEN_A {
    #[doc = "0: Analog watchdog disabled on regular channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog enabled on regular channels"]
    ENABLED = 1,
}
impl From<AWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AWDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDEN` reader - Analog watchdog enable on regular channels"]
pub struct AWDEN_R(crate::FieldReader<bool, AWDEN_A>);
impl AWDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDEN_A {
        match self.bits {
            false => AWDEN_A::DISABLED,
            true => AWDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWDEN_A::ENABLED
    }
}
impl core::ops::Deref for AWDEN_R {
    type Target = crate::FieldReader<bool, AWDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDEN` writer - Analog watchdog enable on regular channels"]
pub struct AWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Analog watchdog enable on injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAWDEN_A {
    #[doc = "0: Analog watchdog disabled on injected channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog enabled on injected channels"]
    ENABLED = 1,
}
impl From<JAWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAWDEN` reader - Analog watchdog enable on injected channels"]
pub struct JAWDEN_R(crate::FieldReader<bool, JAWDEN_A>);
impl JAWDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAWDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAWDEN_A {
        match self.bits {
            false => JAWDEN_A::DISABLED,
            true => JAWDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JAWDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JAWDEN_A::ENABLED
    }
}
impl core::ops::Deref for JAWDEN_R {
    type Target = crate::FieldReader<bool, JAWDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAWDEN` writer - Analog watchdog enable on injected channels"]
pub struct JAWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JAWDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JAWDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::DISABLED)
    }
    #[doc = "Analog watchdog enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Dual mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DUALMOD_A {
    #[doc = "0: Independent mode"]
    INDEPENDENT = 0,
    #[doc = "1: Combined regular simultaneous + injected simultaneous mode"]
    REGULARINJECTED = 1,
    #[doc = "2: Combined regular simultaneous + alternate trigger mode"]
    REGULARALTERNATETRIGGER = 2,
    #[doc = "3: Combined injected simultaneous + fast interleaved mode"]
    INJECTEDFASTINTERLEAVED = 3,
    #[doc = "4: Combined injected simultaneous + slow interleaved mode"]
    INJECTEDSLOWINTERLEAVED = 4,
    #[doc = "5: Injected simultaneous mode only"]
    INJECTED = 5,
    #[doc = "6: Regular simultaneous mode only"]
    REGULAR = 6,
    #[doc = "7: Fast interleaved mode only"]
    FASTINTERLEAVED = 7,
    #[doc = "8: Slow interleaved mode only"]
    SLOWINTERLEAVED = 8,
    #[doc = "9: Alternate trigger mode only"]
    ALTERNATETRIGGER = 9,
}
impl From<DUALMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: DUALMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DUALMOD` reader - Dual mode selection"]
pub struct DUALMOD_R(crate::FieldReader<u8, DUALMOD_A>);
impl DUALMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DUALMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUALMOD_A> {
        match self.bits {
            0 => Some(DUALMOD_A::INDEPENDENT),
            1 => Some(DUALMOD_A::REGULARINJECTED),
            2 => Some(DUALMOD_A::REGULARALTERNATETRIGGER),
            3 => Some(DUALMOD_A::INJECTEDFASTINTERLEAVED),
            4 => Some(DUALMOD_A::INJECTEDSLOWINTERLEAVED),
            5 => Some(DUALMOD_A::INJECTED),
            6 => Some(DUALMOD_A::REGULAR),
            7 => Some(DUALMOD_A::FASTINTERLEAVED),
            8 => Some(DUALMOD_A::SLOWINTERLEAVED),
            9 => Some(DUALMOD_A::ALTERNATETRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        **self == DUALMOD_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `REGULARINJECTED`"]
    #[inline(always)]
    pub fn is_regular_injected(&self) -> bool {
        **self == DUALMOD_A::REGULARINJECTED
    }
    #[doc = "Checks if the value of the field is `REGULARALTERNATETRIGGER`"]
    #[inline(always)]
    pub fn is_regular_alternate_trigger(&self) -> bool {
        **self == DUALMOD_A::REGULARALTERNATETRIGGER
    }
    #[doc = "Checks if the value of the field is `INJECTEDFASTINTERLEAVED`"]
    #[inline(always)]
    pub fn is_injected_fast_interleaved(&self) -> bool {
        **self == DUALMOD_A::INJECTEDFASTINTERLEAVED
    }
    #[doc = "Checks if the value of the field is `INJECTEDSLOWINTERLEAVED`"]
    #[inline(always)]
    pub fn is_injected_slow_interleaved(&self) -> bool {
        **self == DUALMOD_A::INJECTEDSLOWINTERLEAVED
    }
    #[doc = "Checks if the value of the field is `INJECTED`"]
    #[inline(always)]
    pub fn is_injected(&self) -> bool {
        **self == DUALMOD_A::INJECTED
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        **self == DUALMOD_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `FASTINTERLEAVED`"]
    #[inline(always)]
    pub fn is_fast_interleaved(&self) -> bool {
        **self == DUALMOD_A::FASTINTERLEAVED
    }
    #[doc = "Checks if the value of the field is `SLOWINTERLEAVED`"]
    #[inline(always)]
    pub fn is_slow_interleaved(&self) -> bool {
        **self == DUALMOD_A::SLOWINTERLEAVED
    }
    #[doc = "Checks if the value of the field is `ALTERNATETRIGGER`"]
    #[inline(always)]
    pub fn is_alternate_trigger(&self) -> bool {
        **self == DUALMOD_A::ALTERNATETRIGGER
    }
}
impl core::ops::Deref for DUALMOD_R {
    type Target = crate::FieldReader<u8, DUALMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALMOD` writer - Dual mode selection"]
pub struct DUALMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUALMOD_A::INDEPENDENT)
    }
    #[doc = "Combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn regular_injected(self) -> &'a mut W {
        self.variant(DUALMOD_A::REGULARINJECTED)
    }
    #[doc = "Combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn regular_alternate_trigger(self) -> &'a mut W {
        self.variant(DUALMOD_A::REGULARALTERNATETRIGGER)
    }
    #[doc = "Combined injected simultaneous + fast interleaved mode"]
    #[inline(always)]
    pub fn injected_fast_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::INJECTEDFASTINTERLEAVED)
    }
    #[doc = "Combined injected simultaneous + slow interleaved mode"]
    #[inline(always)]
    pub fn injected_slow_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::INJECTEDSLOWINTERLEAVED)
    }
    #[doc = "Injected simultaneous mode only"]
    #[inline(always)]
    pub fn injected(self) -> &'a mut W {
        self.variant(DUALMOD_A::INJECTED)
    }
    #[doc = "Regular simultaneous mode only"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(DUALMOD_A::REGULAR)
    }
    #[doc = "Fast interleaved mode only"]
    #[inline(always)]
    pub fn fast_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::FASTINTERLEAVED)
    }
    #[doc = "Slow interleaved mode only"]
    #[inline(always)]
    pub fn slow_interleaved(self) -> &'a mut W {
        self.variant(DUALMOD_A::SLOWINTERLEAVED)
    }
    #[doc = "Alternate trigger mode only"]
    #[inline(always)]
    pub fn alternate_trigger(self) -> &'a mut W {
        self.variant(DUALMOD_A::ALTERNATETRIGGER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count"]
pub struct DISCNUM_R(crate::FieldReader<u8, u8>);
impl DISCNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DISCNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count"]
pub struct DISCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Discontinuous mode on injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    ENABLED = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels"]
pub struct JDISCEN_R(crate::FieldReader<bool, JDISCEN_A>);
impl JDISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDISCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::DISABLED,
            true => JDISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JDISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JDISCEN_A::ENABLED
    }
}
impl core::ops::Deref for JDISCEN_R {
    type Target = crate::FieldReader<bool, JDISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels"]
pub struct JDISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDISCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JDISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::ENABLED)
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
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode on regular channels"]
pub struct DISCEN_R(crate::FieldReader<bool, DISCEN_A>);
impl DISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISCEN_A::ENABLED
    }
}
impl core::ops::Deref for DISCEN_R {
    type Target = crate::FieldReader<bool, DISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode on regular channels"]
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
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
#[doc = "Automatic injected group conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    ENABLED = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAUTO` reader - Automatic injected group conversion"]
pub struct JAUTO_R(crate::FieldReader<bool, JAUTO_A>);
impl JAUTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAUTO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::DISABLED,
            true => JAUTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JAUTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JAUTO_A::ENABLED
    }
}
impl core::ops::Deref for JAUTO_R {
    type Target = crate::FieldReader<bool, JAUTO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAUTO` writer - Automatic injected group conversion"]
pub struct JAUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> JAUTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JAUTO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::DISABLED)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::ENABLED)
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
#[doc = "Enable the watchdog on a single channel in scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDSGL_A {
    #[doc = "0: Analog watchdog enabled on all channels"]
    ALL = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    SINGLE = 1,
}
impl From<AWDSGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode"]
pub struct AWDSGL_R(crate::FieldReader<bool, AWDSGL_A>);
impl AWDSGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDSGL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDSGL_A {
        match self.bits {
            false => AWDSGL_A::ALL,
            true => AWDSGL_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == AWDSGL_A::ALL
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == AWDSGL_A::SINGLE
    }
}
impl core::ops::Deref for AWDSGL_R {
    type Target = crate::FieldReader<bool, AWDSGL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode"]
pub struct AWDSGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDSGL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWDSGL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWDSGL_A::ALL)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWDSGL_A::SINGLE)
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
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCAN_A {
    #[doc = "0: Scan mode disabled"]
    DISABLED = 0,
    #[doc = "1: Scan mode enabled"]
    ENABLED = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN` reader - Scan mode"]
pub struct SCAN_R(crate::FieldReader<bool, SCAN_A>);
impl SCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::DISABLED,
            true => SCAN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SCAN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SCAN_A::ENABLED
    }
}
impl core::ops::Deref for SCAN_R {
    type Target = crate::FieldReader<bool, SCAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN` writer - Scan mode"]
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCAN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCAN_A::DISABLED)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCAN_A::ENABLED)
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
#[doc = "Interrupt enable for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCIE_A {
    #[doc = "0: JEOC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set"]
    ENABLED = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOCIE` reader - Interrupt enable for injected channels"]
pub struct JEOCIE_R(crate::FieldReader<bool, JEOCIE_A>);
impl JEOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::DISABLED,
            true => JEOCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JEOCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JEOCIE_A::ENABLED
    }
}
impl core::ops::Deref for JEOCIE_R {
    type Target = crate::FieldReader<bool, JEOCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOCIE` writer - Interrupt enable for injected channels"]
pub struct JEOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEOCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "JEOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::DISABLED)
    }
    #[doc = "JEOC interrupt enabled. An interrupt is generated when the JEOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEOCIE_A::ENABLED)
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
#[doc = "Analog watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDIE_A {
    #[doc = "0: Analog watchdog interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog interrupt enabled"]
    ENABLED = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub struct AWDIE_R(crate::FieldReader<bool, AWDIE_A>);
impl AWDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::DISABLED,
            true => AWDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWDIE_A::ENABLED
    }
}
impl core::ops::Deref for AWDIE_R {
    type Target = crate::FieldReader<bool, AWDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub struct AWDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog watchdog interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIE_A::DISABLED)
    }
    #[doc = "Analog watchdog interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCIE_A {
    #[doc = "0: EOC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    ENABLED = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub struct EOCIE_R(crate::FieldReader<bool, EOCIE_A>);
impl EOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::DISABLED,
            true => EOCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOCIE_A::ENABLED
    }
}
impl core::ops::Deref for EOCIE_R {
    type Target = crate::FieldReader<bool, EOCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::DISABLED)
    }
    #[doc = "EOC interrupt enabled. An interrupt is generated when the EOC bit is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::ENABLED)
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
#[doc = "Field `AWDCH` reader - Analog watchdog channel select bits"]
pub struct AWDCH_R(crate::FieldReader<u8, u8>);
impl AWDCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDCH` writer - Analog watchdog channel select bits"]
pub struct AWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&self) -> DUALMOD_R {
        DUALMOD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W {
        AWDEN_W { w: self }
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn jawden(&mut self) -> JAWDEN_W {
        JAWDEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&mut self) -> DUALMOD_W {
        DUALMOD_W { w: self }
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W {
        DISCNUM_W { w: self }
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W {
        JDISCEN_W { w: self }
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W {
        JAUTO_W { w: self }
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W {
        AWDSGL_W { w: self }
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W {
        JEOCIE_W { w: self }
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W {
        AWDIE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
