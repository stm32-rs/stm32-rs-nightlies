#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low speed clock output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSCOSEL_A {
    #[doc = "0: LSI clock selected"]
    LSI = 0,
    #[doc = "1: LSE clock selected"]
    LSE = 1,
}
impl From<LSCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOSEL` reader - Low speed clock output selection"]
pub struct LSCOSEL_R(crate::FieldReader<bool, LSCOSEL_A>);
impl LSCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSCOSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOSEL_A {
        match self.bits {
            false => LSCOSEL_A::LSI,
            true => LSCOSEL_A::LSE,
        }
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LSCOSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LSCOSEL_A::LSE
    }
}
impl core::ops::Deref for LSCOSEL_R {
    type Target = crate::FieldReader<bool, LSCOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSCOSEL` writer - Low speed clock output selection"]
pub struct LSCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LSCOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSCOSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LSCOSEL_A::LSI)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LSCOSEL_A::LSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Low speed clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSCOEN_A {
    #[doc = "0: LSCO disabled"]
    DISABLED = 0,
    #[doc = "1: LSCO enabled"]
    ENABLED = 1,
}
impl From<LSCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOEN` reader - Low speed clock output enable"]
pub struct LSCOEN_R(crate::FieldReader<bool, LSCOEN_A>);
impl LSCOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSCOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSCOEN_A {
        match self.bits {
            false => LSCOEN_A::DISABLED,
            true => LSCOEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSCOEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSCOEN_A::ENABLED
    }
}
impl core::ops::Deref for LSCOEN_R {
    type Target = crate::FieldReader<bool, LSCOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSCOEN` writer - Low speed clock output enable"]
pub struct LSCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSCOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSCOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSCO disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::DISABLED)
    }
    #[doc = "LSCO enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BDRST_A {
    #[doc = "0: Reset not activated"]
    NOTACTIVE = 0,
    #[doc = "1: Entire Backup domain reset"]
    RESET = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub struct BDRST_R(crate::FieldReader<bool, BDRST_A>);
impl BDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BDRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::NOTACTIVE,
            true => BDRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        **self == BDRST_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == BDRST_A::RESET
    }
}
impl core::ops::Deref for BDRST_R {
    type Target = crate::FieldReader<bool, BDRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub struct BDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(BDRST_A::NOTACTIVE)
    }
    #[doc = "Entire Backup domain reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRST_A::RESET)
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
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: RTC kernel clock disabled"]
    DISABLED = 0,
    #[doc = "1: RTC kernel clock enabled"]
    ENABLED = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub struct RTCEN_R(crate::FieldReader<bool, RTCEN_A>);
impl RTCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RTCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RTCEN_A::ENABLED
    }
}
impl core::ops::Deref for RTCEN_R {
    type Target = crate::FieldReader<bool, RTCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC kernel clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    #[doc = "RTC kernel clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
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
#[doc = "LSE system clock ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSESYSRDY_A {
    #[doc = "0: LSE system clock not ready"]
    NOTREADY = 0,
    #[doc = "1: LSE system clock ready"]
    READY = 1,
}
impl From<LSESYSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSRDY` reader - LSE system clock ready"]
pub struct LSESYSRDY_R(crate::FieldReader<bool, LSESYSRDY_A>);
impl LSESYSRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSESYSRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSESYSRDY_A {
        match self.bits {
            false => LSESYSRDY_A::NOTREADY,
            true => LSESYSRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == LSESYSRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LSESYSRDY_A::READY
    }
}
impl core::ops::Deref for LSESYSRDY_R {
    type Target = crate::FieldReader<bool, LSESYSRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    NOCLOCK = 0,
    #[doc = "1: LSE oscillator clock selected"]
    LSE = 1,
    #[doc = "2: LSI oscillator clock selected"]
    LSI = 2,
    #[doc = "3: HSE32 oscillator clock divided by 32 selected"]
    HSE32 = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub struct RTCSEL_R(crate::FieldReader<u8, RTCSEL_A>);
impl RTCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NOCLOCK,
            1 => RTCSEL_A::LSE,
            2 => RTCSEL_A::LSI,
            3 => RTCSEL_A::HSE32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == RTCSEL_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == RTCSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == RTCSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSE32`"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        **self == RTCSEL_A::HSE32
    }
}
impl core::ops::Deref for RTCSEL_R {
    type Target = crate::FieldReader<u8, RTCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NOCLOCK)
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSE)
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSI)
    }
    #[doc = "HSE32 oscillator clock divided by 32 selected"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(RTCSEL_A::HSE32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "LSE system clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSESYSEN_A {
    #[doc = "0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    DISABLED = 0,
    #[doc = "1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    ENABLED = 1,
}
impl From<LSESYSEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSEN` reader - LSE system clock enable"]
pub struct LSESYSEN_R(crate::FieldReader<bool, LSESYSEN_A>);
impl LSESYSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSESYSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSESYSEN_A {
        match self.bits {
            false => LSESYSEN_A::DISABLED,
            true => LSESYSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSESYSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSESYSEN_A::ENABLED
    }
}
impl core::ops::Deref for LSESYSEN_R {
    type Target = crate::FieldReader<bool, LSESYSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSESYSEN` writer - LSE system clock enable"]
pub struct LSESYSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSESYSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSESYSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::DISABLED)
    }
    #[doc = "LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::ENABLED)
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
#[doc = "CSS on LSE failure Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSD_A {
    #[doc = "0: No failure detected on LSE"]
    NOFAILURE = 0,
    #[doc = "1: Failure detected on LSE"]
    FAILURE = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection"]
pub struct LSECSSD_R(crate::FieldReader<bool, LSECSSD_A>);
impl LSECSSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::NOFAILURE,
            true => LSECSSD_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAILURE`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        **self == LSECSSD_A::NOFAILURE
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        **self == LSECSSD_A::FAILURE
    }
}
impl core::ops::Deref for LSECSSD_R {
    type Target = crate::FieldReader<bool, LSECSSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CSS on LSE enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSON_A {
    #[doc = "0: CSS on LSE disabled"]
    DISABLED = 0,
    #[doc = "1: CSS on LSE enabled"]
    ENABLED = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable"]
pub struct LSECSSON_R(crate::FieldReader<bool, LSECSSON_A>);
impl LSECSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSECSSON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::DISABLED,
            true => LSECSSON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSECSSON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSECSSON_A::ENABLED
    }
}
impl core::ops::Deref for LSECSSON_R {
    type Target = crate::FieldReader<bool, LSECSSON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable"]
pub struct LSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSECSSON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CSS on LSE disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::DISABLED)
    }
    #[doc = "CSS on LSE enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::ENABLED)
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
#[doc = "LSE oscillator drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Xtal mode lower driving capability"]
    LOW = 0,
    #[doc = "1: Xtal mode medium-low driving capability"]
    MEDLOW = 1,
    #[doc = "2: Xtal mode medium-high driving capability"]
    MEDHIGH = 2,
    #[doc = "3: Xtal mode higher driving capability"]
    HIGH = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability"]
pub struct LSEDRV_R(crate::FieldReader<u8, LSEDRV_A>);
impl LSEDRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSEDRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::LOW,
            1 => LSEDRV_A::MEDLOW,
            2 => LSEDRV_A::MEDHIGH,
            3 => LSEDRV_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == LSEDRV_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDLOW`"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        **self == LSEDRV_A::MEDLOW
    }
    #[doc = "Checks if the value of the field is `MEDHIGH`"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        **self == LSEDRV_A::MEDHIGH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == LSEDRV_A::HIGH
    }
}
impl core::ops::Deref for LSEDRV_R {
    type Target = crate::FieldReader<u8, LSEDRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability"]
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEDRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::LOW)
    }
    #[doc = "Xtal mode medium-low driving capability"]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDLOW)
    }
    #[doc = "Xtal mode medium-high driving capability"]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDHIGH)
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "LSE oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed"]
    DISABLED = 0,
    #[doc = "1: LSE oscillator bypassed"]
    ENABLED = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub struct LSEBYP_R(crate::FieldReader<bool, LSEBYP_A>);
impl LSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSEBYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::DISABLED,
            true => LSEBYP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSEBYP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSEBYP_A::ENABLED
    }
}
impl core::ops::Deref for LSEBYP_R {
    type Target = crate::FieldReader<bool, LSEBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::DISABLED)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::ENABLED)
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
#[doc = "LSE oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: LSE oscillator ready"]
    READY = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub struct LSERDY_R(crate::FieldReader<bool, LSERDY_A>);
impl LSERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSERDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NOTREADY,
            true => LSERDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == LSERDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LSERDY_A::READY
    }
}
impl core::ops::Deref for LSERDY_R {
    type Target = crate::FieldReader<bool, LSERDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LSE oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator off"]
    OFF = 0,
    #[doc = "1: LSE oscillator on"]
    ON = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable"]
pub struct LSEON_R(crate::FieldReader<bool, LSEON_A>);
impl LSEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSEON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::OFF,
            true => LSEON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == LSEON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == LSEON_A::ON
    }
}
impl core::ops::Deref for LSEON_R {
    type Target = crate::FieldReader<bool, LSEON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSE oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::OFF)
    }
    #[doc = "LSE oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::ON)
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
impl R {
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LSE system clock ready"]
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W {
        LSCOSEL_W { w: self }
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W {
        LSCOEN_W { w: self }
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W {
        BDRST_W { w: self }
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    pub fn lsesysen(&mut self) -> LSESYSEN_W {
        LSESYSEN_W { w: self }
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W {
        LSECSSON_W { w: self }
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
