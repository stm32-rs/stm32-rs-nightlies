#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup from Stop mode clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUCF_A {
    #[doc = "1: Clears the WUF flag in the ISR register"]
    CLEAR = 1,
}
impl From<WUCF_A> for bool {
    #[inline(always)]
    fn from(variant: WUCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUCF` reader - Wakeup from Stop mode clear flag"]
pub struct WUCF_R(crate::FieldReader<bool, WUCF_A>);
impl WUCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCF_A> {
        match self.bits {
            true => Some(WUCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WUCF_A::CLEAR
    }
}
impl core::ops::Deref for WUCF_R {
    type Target = crate::FieldReader<bool, WUCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUCF` writer - Wakeup from Stop mode clear flag"]
pub struct WUCF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the WUF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCF_A::CLEAR)
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
#[doc = "Character match clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMCF_A {
    #[doc = "1: Clears the CMF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CMCF_A> for bool {
    #[inline(always)]
    fn from(variant: CMCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMCF` reader - Character match clear flag"]
pub struct CMCF_R(crate::FieldReader<bool, CMCF_A>);
impl CMCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMCF_A> {
        match self.bits {
            true => Some(CMCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CMCF_A::CLEAR
    }
}
impl core::ops::Deref for CMCF_R {
    type Target = crate::FieldReader<bool, CMCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMCF` writer - Character match clear flag"]
pub struct CMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the CMF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCF_A::CLEAR)
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
#[doc = "End of timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBCF_A {
    #[doc = "1: Clears the EOBF flag in the ISR register"]
    CLEAR = 1,
}
impl From<EOBCF_A> for bool {
    #[inline(always)]
    fn from(variant: EOBCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBCF` reader - End of timeout clear flag"]
pub struct EOBCF_R(crate::FieldReader<bool, EOBCF_A>);
impl EOBCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOBCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EOBCF_A> {
        match self.bits {
            true => Some(EOBCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == EOBCF_A::CLEAR
    }
}
impl core::ops::Deref for EOBCF_R {
    type Target = crate::FieldReader<bool, EOBCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOBCF` writer - End of timeout clear flag"]
pub struct EOBCF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOBCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOBCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the EOBF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOBCF_A::CLEAR)
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
#[doc = "Receiver timeout clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOCF_A {
    #[doc = "1: Clears the RTOF flag in the ISR register"]
    CLEAR = 1,
}
impl From<RTOCF_A> for bool {
    #[inline(always)]
    fn from(variant: RTOCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOCF` reader - Receiver timeout clear flag"]
pub struct RTOCF_R(crate::FieldReader<bool, RTOCF_A>);
impl RTOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTOCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTOCF_A> {
        match self.bits {
            true => Some(RTOCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RTOCF_A::CLEAR
    }
}
impl core::ops::Deref for RTOCF_R {
    type Target = crate::FieldReader<bool, RTOCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag"]
pub struct RTOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTOCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the RTOF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RTOCF_A::CLEAR)
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
#[doc = "CTS clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSCF_A {
    #[doc = "1: Clears the CTSIF flag in the ISR register"]
    CLEAR = 1,
}
impl From<CTSCF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` reader - CTS clear flag"]
pub struct CTSCF_R(crate::FieldReader<bool, CTSCF_A>);
impl CTSCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSCF_A> {
        match self.bits {
            true => Some(CTSCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CTSCF_A::CLEAR
    }
}
impl core::ops::Deref for CTSCF_R {
    type Target = crate::FieldReader<bool, CTSCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSCF` writer - CTS clear flag"]
pub struct CTSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the CTSIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCF_A::CLEAR)
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
#[doc = "LIN break detection clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBDCF_A {
    #[doc = "1: Clears the LBDF flag in the ISR register"]
    CLEAR = 1,
}
impl From<LBDCF_A> for bool {
    #[inline(always)]
    fn from(variant: LBDCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDCF` reader - LIN break detection clear flag"]
pub struct LBDCF_R(crate::FieldReader<bool, LBDCF_A>);
impl LBDCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBDCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LBDCF_A> {
        match self.bits {
            true => Some(LBDCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == LBDCF_A::CLEAR
    }
}
impl core::ops::Deref for LBDCF_R {
    type Target = crate::FieldReader<bool, LBDCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBDCF` writer - LIN break detection clear flag"]
pub struct LBDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBDCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the LBDF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LBDCF_A::CLEAR)
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
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCF_A {
    #[doc = "1: Clears the TC flag in the ISR register"]
    CLEAR = 1,
}
impl From<TCCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCCF` reader - Transmission complete clear flag"]
pub struct TCCF_R(crate::FieldReader<bool, TCCF_A>);
impl TCCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCCF_A> {
        match self.bits {
            true => Some(TCCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TCCF_A::CLEAR
    }
}
impl core::ops::Deref for TCCF_R {
    type Target = crate::FieldReader<bool, TCCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCF` writer - Transmission complete clear flag"]
pub struct TCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the TC flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCF_A::CLEAR)
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
#[doc = "Idle line detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECF_A {
    #[doc = "1: Clears the IDLE flag in the ISR register"]
    CLEAR = 1,
}
impl From<IDLECF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLECF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLECF` reader - Idle line detected clear flag"]
pub struct IDLECF_R(crate::FieldReader<bool, IDLECF_A>);
impl IDLECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLECF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDLECF_A> {
        match self.bits {
            true => Some(IDLECF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == IDLECF_A::CLEAR
    }
}
impl core::ops::Deref for IDLECF_R {
    type Target = crate::FieldReader<bool, IDLECF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLECF` writer - Idle line detected clear flag"]
pub struct IDLECF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the IDLE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECF_A::CLEAR)
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
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORECF_A {
    #[doc = "1: Clears the ORE flag in the ISR register"]
    CLEAR = 1,
}
impl From<ORECF_A> for bool {
    #[inline(always)]
    fn from(variant: ORECF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORECF` reader - Overrun error clear flag"]
pub struct ORECF_R(crate::FieldReader<bool, ORECF_A>);
impl ORECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ORECF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ORECF_A> {
        match self.bits {
            true => Some(ORECF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == ORECF_A::CLEAR
    }
}
impl core::ops::Deref for ORECF_R {
    type Target = crate::FieldReader<bool, ORECF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ORECF` writer - Overrun error clear flag"]
pub struct ORECF_W<'a> {
    w: &'a mut W,
}
impl<'a> ORECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORECF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the ORE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECF_A::CLEAR)
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
#[doc = "Noise detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCF_A {
    #[doc = "1: Clears the NF flag in the ISR register"]
    CLEAR = 1,
}
impl From<NCF_A> for bool {
    #[inline(always)]
    fn from(variant: NCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCF` reader - Noise detected clear flag"]
pub struct NCF_R(crate::FieldReader<bool, NCF_A>);
impl NCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NCF_A> {
        match self.bits {
            true => Some(NCF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == NCF_A::CLEAR
    }
}
impl core::ops::Deref for NCF_R {
    type Target = crate::FieldReader<bool, NCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCF` writer - Noise detected clear flag"]
pub struct NCF_W<'a> {
    w: &'a mut W,
}
impl<'a> NCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the NF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCF_A::CLEAR)
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
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FECF_A {
    #[doc = "1: Clears the FE flag in the ISR register"]
    CLEAR = 1,
}
impl From<FECF_A> for bool {
    #[inline(always)]
    fn from(variant: FECF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FECF` reader - Framing error clear flag"]
pub struct FECF_R(crate::FieldReader<bool, FECF_A>);
impl FECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FECF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FECF_A> {
        match self.bits {
            true => Some(FECF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == FECF_A::CLEAR
    }
}
impl core::ops::Deref for FECF_R {
    type Target = crate::FieldReader<bool, FECF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FECF` writer - Framing error clear flag"]
pub struct FECF_W<'a> {
    w: &'a mut W,
}
impl<'a> FECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FECF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the FE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECF_A::CLEAR)
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
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECF_A {
    #[doc = "1: Clears the PE flag in the ISR register"]
    CLEAR = 1,
}
impl From<PECF_A> for bool {
    #[inline(always)]
    fn from(variant: PECF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECF` reader - Parity error clear flag"]
pub struct PECF_R(crate::FieldReader<bool, PECF_A>);
impl PECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PECF_A> {
        match self.bits {
            true => Some(PECF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == PECF_A::CLEAR
    }
}
impl core::ops::Deref for PECF_R {
    type Target = crate::FieldReader<bool, PECF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECF` writer - Parity error clear flag"]
pub struct PECF_W<'a> {
    w: &'a mut W,
}
impl<'a> PECF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the PE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECF_A::CLEAR)
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
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub fn wucf(&self) -> WUCF_R {
        WUCF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    pub fn cmcf(&self) -> CMCF_R {
        CMCF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline(always)]
    pub fn eobcf(&self) -> EOBCF_R {
        EOBCF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    pub fn rtocf(&self) -> RTOCF_R {
        RTOCF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    pub fn lbdcf(&self) -> LBDCF_R {
        LBDCF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tccf(&self) -> TCCF_R {
        TCCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    pub fn idlecf(&self) -> IDLECF_R {
        IDLECF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orecf(&self) -> ORECF_R {
        ORECF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    pub fn ncf(&self) -> NCF_R {
        NCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fecf(&self) -> FECF_R {
        FECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pecf(&self) -> PECF_R {
        PECF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W {
        WUCF_W { w: self }
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W {
        CMCF_W { w: self }
    }
    #[doc = "Bit 12 - End of timeout clear flag"]
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W {
        EOBCF_W { w: self }
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W {
        RTOCF_W { w: self }
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W {
        CTSCF_W { w: self }
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W {
        LBDCF_W { w: self }
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W {
        TCCF_W { w: self }
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W {
        IDLECF_W { w: self }
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W {
        ORECF_W { w: self }
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W {
        NCF_W { w: self }
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W {
        FECF_W { w: self }
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W {
        PECF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
