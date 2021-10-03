#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-power reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWRRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<LPWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub struct LPWRRSTF_R(crate::FieldReader<bool, LPWRRSTF_A>);
impl LPWRRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWRRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWRRSTF_A {
        match self.bits {
            false => LPWRRSTF_A::NORESET,
            true => LPWRRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == LPWRRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == LPWRRSTF_A::RESET
    }
}
impl core::ops::Deref for LPWRRSTF_R {
    type Target = crate::FieldReader<bool, LPWRRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDGRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<WWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub struct WWDGRSTF_R(crate::FieldReader<bool, WWDGRSTF_A>);
impl WWDGRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDGRSTF_A {
        match self.bits {
            false => WWDGRSTF_A::NORESET,
            true => WWDGRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == WWDGRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == WWDGRSTF_A::RESET
    }
}
impl core::ops::Deref for WWDGRSTF_R {
    type Target = crate::FieldReader<bool, WWDGRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Independent window watchdog reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDGRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<IWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub struct IWDGRSTF_R(crate::FieldReader<bool, IWDGRSTF_A>);
impl IWDGRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDGRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDGRSTF_A {
        match self.bits {
            false => IWDGRSTF_A::NORESET,
            true => IWDGRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == IWDGRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == IWDGRSTF_A::RESET
    }
}
impl core::ops::Deref for IWDGRSTF_R {
    type Target = crate::FieldReader<bool, IWDGRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Software reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<SFTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub struct SFTRSTF_R(crate::FieldReader<bool, SFTRSTF_A>);
impl SFTRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTRSTF_A {
        match self.bits {
            false => SFTRSTF_A::NORESET,
            true => SFTRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == SFTRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SFTRSTF_A::RESET
    }
}
impl core::ops::Deref for SFTRSTF_R {
    type Target = crate::FieldReader<bool, SFTRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "BOR flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BORRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<BORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR flag"]
pub struct BORRSTF_R(crate::FieldReader<bool, BORRSTF_A>);
impl BORRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORRSTF_A {
        match self.bits {
            false => BORRSTF_A::NORESET,
            true => BORRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == BORRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == BORRSTF_A::RESET
    }
}
impl core::ops::Deref for BORRSTF_R {
    type Target = crate::FieldReader<bool, BORRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Pin reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<PINRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub struct PINRSTF_R(crate::FieldReader<bool, PINRSTF_A>);
impl PINRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINRSTF_A {
        match self.bits {
            false => PINRSTF_A::NORESET,
            true => PINRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == PINRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == PINRSTF_A::RESET
    }
}
impl core::ops::Deref for PINRSTF_R {
    type Target = crate::FieldReader<bool, PINRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Option byte loader reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBLRSTF_A {
    #[doc = "0: No reset occurred"]
    NORESET = 0,
    #[doc = "1: Reset occurred"]
    RESET = 1,
}
impl From<OBLRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub struct OBLRSTF_R(crate::FieldReader<bool, OBLRSTF_A>);
impl OBLRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBLRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBLRSTF_A {
        match self.bits {
            false => OBLRSTF_A::NORESET,
            true => OBLRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == OBLRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == OBLRSTF_A::RESET
    }
}
impl core::ops::Deref for OBLRSTF_R {
    type Target = crate::FieldReader<bool, OBLRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Radio illegal access flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFILARSTF_A {
    #[doc = "0: No SUBGHZ radio illegal command occurred"]
    NOILLEGALCOMMAND = 0,
    #[doc = "1: SUBGHZ radio illegal command occurred"]
    ILLEGALCOMMAND = 1,
}
impl From<RFILARSTF_A> for bool {
    #[inline(always)]
    fn from(variant: RFILARSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFILARSTF` reader - Radio illegal access flag"]
pub struct RFILARSTF_R(crate::FieldReader<bool, RFILARSTF_A>);
impl RFILARSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFILARSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFILARSTF_A {
        match self.bits {
            false => RFILARSTF_A::NOILLEGALCOMMAND,
            true => RFILARSTF_A::ILLEGALCOMMAND,
        }
    }
    #[doc = "Checks if the value of the field is `NOILLEGALCOMMAND`"]
    #[inline(always)]
    pub fn is_no_illegal_command(&self) -> bool {
        **self == RFILARSTF_A::NOILLEGALCOMMAND
    }
    #[doc = "Checks if the value of the field is `ILLEGALCOMMAND`"]
    #[inline(always)]
    pub fn is_illegal_command(&self) -> bool {
        **self == RFILARSTF_A::ILLEGALCOMMAND
    }
}
impl core::ops::Deref for RFILARSTF_R {
    type Target = crate::FieldReader<bool, RFILARSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVF_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Reset flags reset"]
    CLEAR = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub struct RMVF_R(crate::FieldReader<bool, RMVF_A>);
impl RMVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::NOEFFECT,
            true => RMVF_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == RMVF_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RMVF_A::CLEAR
    }
}
impl core::ops::Deref for RMVF_R {
    type Target = crate::FieldReader<bool, RMVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RMVF_A::NOEFFECT)
    }
    #[doc = "Reset flags reset"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::CLEAR)
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
#[doc = "Radio reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFRST_A {
    #[doc = "0: Sub-GHz radio software reset removed"]
    REMOVED = 0,
    #[doc = "1: Sub-GHz radio software reset active"]
    RESET = 1,
}
impl From<RFRST_A> for bool {
    #[inline(always)]
    fn from(variant: RFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRST` reader - Radio reset"]
pub struct RFRST_R(crate::FieldReader<bool, RFRST_A>);
impl RFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRST_A {
        match self.bits {
            false => RFRST_A::REMOVED,
            true => RFRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `REMOVED`"]
    #[inline(always)]
    pub fn is_removed(&self) -> bool {
        **self == RFRST_A::REMOVED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RFRST_A::RESET
    }
}
impl core::ops::Deref for RFRST_R {
    type Target = crate::FieldReader<bool, RFRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFRST` writer - Radio reset"]
pub struct RFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sub-GHz radio software reset removed"]
    #[inline(always)]
    pub fn removed(self) -> &'a mut W {
        self.variant(RFRST_A::REMOVED)
    }
    #[doc = "Sub-GHz radio software reset active"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RFRST_A::RESET)
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
#[doc = "Radio in reset status flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFRSTF_A {
    #[doc = "0: Sub-GHz radio out of reset"]
    NORESET = 0,
    #[doc = "1: Sub-GHz radio in reset"]
    RESET = 1,
}
impl From<RFRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: RFRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFRSTF` reader - Radio in reset status flag"]
pub struct RFRSTF_R(crate::FieldReader<bool, RFRSTF_A>);
impl RFRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFRSTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFRSTF_A {
        match self.bits {
            false => RFRSTF_A::NORESET,
            true => RFRSTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == RFRSTF_A::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RFRSTF_A::RESET
    }
}
impl core::ops::Deref for RFRSTF_R {
    type Target = crate::FieldReader<bool, RFRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSISRANGE_A {
    #[doc = "4: Range 4 around 1 MHz"]
    F_1MHZ = 4,
    #[doc = "5: Range 5 around 2 MHz"]
    F_2MHZ = 5,
    #[doc = "6: Range 6 around 4 MHz (reset value)"]
    F_4MHZ = 6,
    #[doc = "7: Range 7 around 8 MHz"]
    F_8MHZ = 7,
}
impl From<MSISRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSISRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSISRANGE` reader - MSI clock ranges"]
pub struct MSISRANGE_R(crate::FieldReader<u8, MSISRANGE_A>);
impl MSISRANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSISRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSISRANGE_A> {
        match self.bits {
            4 => Some(MSISRANGE_A::F_1MHZ),
            5 => Some(MSISRANGE_A::F_2MHZ),
            6 => Some(MSISRANGE_A::F_4MHZ),
            7 => Some(MSISRANGE_A::F_8MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `F_1MHZ`"]
    #[inline(always)]
    pub fn is_f_1mhz(&self) -> bool {
        **self == MSISRANGE_A::F_1MHZ
    }
    #[doc = "Checks if the value of the field is `F_2MHZ`"]
    #[inline(always)]
    pub fn is_f_2mhz(&self) -> bool {
        **self == MSISRANGE_A::F_2MHZ
    }
    #[doc = "Checks if the value of the field is `F_4MHZ`"]
    #[inline(always)]
    pub fn is_f_4mhz(&self) -> bool {
        **self == MSISRANGE_A::F_4MHZ
    }
    #[doc = "Checks if the value of the field is `F_8MHZ`"]
    #[inline(always)]
    pub fn is_f_8mhz(&self) -> bool {
        **self == MSISRANGE_A::F_8MHZ
    }
}
impl core::ops::Deref for MSISRANGE_R {
    type Target = crate::FieldReader<u8, MSISRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSISRANGE` writer - MSI clock ranges"]
pub struct MSISRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSISRANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Range 4 around 1 MHz"]
    #[inline(always)]
    pub fn f_1mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F_1MHZ)
    }
    #[doc = "Range 5 around 2 MHz"]
    #[inline(always)]
    pub fn f_2mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F_2MHZ)
    }
    #[doc = "Range 6 around 4 MHz (reset value)"]
    #[inline(always)]
    pub fn f_4mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F_4MHZ)
    }
    #[doc = "Range 7 around 8 MHz"]
    #[inline(always)]
    pub fn f_8mhz(self) -> &'a mut W {
        self.variant(MSISRANGE_A::F_8MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "LSI frequency prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIPRE_A {
    #[doc = "0: LSI clock not divided"]
    DIV1 = 0,
    #[doc = "1: LSI clock divided by 128"]
    DIV128 = 1,
}
impl From<LSIPRE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIPRE` reader - LSI frequency prescaler"]
pub struct LSIPRE_R(crate::FieldReader<bool, LSIPRE_A>);
impl LSIPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIPRE_A {
        match self.bits {
            false => LSIPRE_A::DIV1,
            true => LSIPRE_A::DIV128,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == LSIPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == LSIPRE_A::DIV128
    }
}
impl core::ops::Deref for LSIPRE_R {
    type Target = crate::FieldReader<bool, LSIPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIPRE` writer - LSI frequency prescaler"]
pub struct LSIPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIPRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSI clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LSIPRE_A::DIV1)
    }
    #[doc = "LSI clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LSIPRE_A::DIV128)
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
#[doc = "LSI oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: LSI oscillator ready"]
    READY = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub struct LSIRDY_R(crate::FieldReader<bool, LSIRDY_A>);
impl LSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NOTREADY,
            true => LSIRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == LSIRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LSIRDY_A::READY
    }
}
impl core::ops::Deref for LSIRDY_R {
    type Target = crate::FieldReader<bool, LSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LSI oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator off"]
    OFF = 0,
    #[doc = "1: LSI oscillator on"]
    ON = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub struct LSION_R(crate::FieldReader<bool, LSION_A>);
impl LSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::OFF,
            true => LSION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == LSION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == LSION_A::ON
    }
}
impl core::ops::Deref for LSION_R {
    type Target = crate::FieldReader<bool, LSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSI oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::OFF)
    }
    #[doc = "LSI oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::ON)
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
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Radio illegal access flag"]
    #[inline(always)]
    pub fn rfilarstf(&self) -> RFILARSTF_R {
        RFILARSTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Radio in reset status flag"]
    #[inline(always)]
    pub fn rfrstf(&self) -> RFRSTF_R {
        RFRSTF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    pub fn lsipre(&self) -> LSIPRE_R {
        LSIPRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    #[doc = "Bit 15 - Radio reset"]
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W {
        RFRST_W { w: self }
    }
    #[doc = "Bits 8:11 - MSI clock ranges"]
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W {
        MSISRANGE_W { w: self }
    }
    #[doc = "Bit 4 - LSI frequency prescaler"]
    #[inline(always)]
    pub fn lsipre(&mut self) -> LSIPRE_W {
        LSIPRE_W { w: self }
    }
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x0c01_c600"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c01_c600
    }
}
