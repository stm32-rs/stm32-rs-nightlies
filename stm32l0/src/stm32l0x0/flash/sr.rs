#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write/erase operations in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    #[doc = "0: No write/erase operation is in progress"]
    INACTIVE = 0,
    #[doc = "1: No write/erase operation is in progress"]
    ACTIVE = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Write/erase operations in progress"]
pub struct BSY_R(crate::FieldReader<bool, BSY_A>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::INACTIVE,
            true => BSY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == BSY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == BSY_A::ACTIVE
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, BSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_A {
    #[doc = "0: No EOP operation occurred"]
    NOEVENT = 0,
    #[doc = "1: An EOP event occurred"]
    EVENT = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub struct EOP_R(crate::FieldReader<bool, EOP_A>);
impl EOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NOEVENT,
            true => EOP_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == EOP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == EOP_A::EVENT
    }
}
impl core::ops::Deref for EOP_R {
    type Target = crate::FieldReader<bool, EOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of high voltage\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDHV_A {
    #[doc = "0: High voltage is executing a write/erase operation in the NVM"]
    ACTIVE = 0,
    #[doc = "1: High voltage is off, no write/erase operation is ongoing"]
    INACTIVE = 1,
}
impl From<ENDHV_A> for bool {
    #[inline(always)]
    fn from(variant: ENDHV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDHV` reader - End of high voltage"]
pub struct ENDHV_R(crate::FieldReader<bool, ENDHV_A>);
impl ENDHV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDHV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDHV_A {
        match self.bits {
            false => ENDHV_A::ACTIVE,
            true => ENDHV_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ENDHV_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == ENDHV_A::INACTIVE
    }
}
impl core::ops::Deref for ENDHV_R {
    type Target = crate::FieldReader<bool, ENDHV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash memory module ready after low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: The NVM is not ready"]
    NOTREADY = 0,
    #[doc = "1: The NVM is ready"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Flash memory module ready after low power mode"]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOTREADY,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == READY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == READY_A::READY
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_A {
    #[doc = "0: No protection error happened"]
    NOERROR = 0,
    #[doc = "1: One protection error happened"]
    ERROR = 1,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - Write protected error"]
pub struct WRPERR_R(crate::FieldReader<bool, WRPERR_A>);
impl WRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRPERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPERR_A {
        match self.bits {
            false => WRPERR_A::NOERROR,
            true => WRPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == WRPERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == WRPERR_A::ERROR
    }
}
impl core::ops::Deref for WRPERR_R {
    type Target = crate::FieldReader<bool, WRPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<WRPERR_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` writer - Write protected error"]
pub struct WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRPERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERR_AW::CLEAR)
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
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_A {
    #[doc = "0: No alignment error happened"]
    NOERROR = 0,
    #[doc = "1: One alignment error happened"]
    ERROR = 1,
}
impl From<PGAERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub struct PGAERR_R(crate::FieldReader<bool, PGAERR_A>);
impl PGAERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGAERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAERR_A {
        match self.bits {
            false => PGAERR_A::NOERROR,
            true => PGAERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PGAERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PGAERR_A::ERROR
    }
}
impl core::ops::Deref for PGAERR_R {
    type Target = crate::FieldReader<bool, PGAERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<PGAERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub struct PGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGAERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGAERR_AW::CLEAR)
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
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_A {
    #[doc = "0: No size error happened"]
    NOERROR = 0,
    #[doc = "1: One size error happened"]
    ERROR = 1,
}
impl From<SIZERR_A> for bool {
    #[inline(always)]
    fn from(variant: SIZERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` reader - Size error"]
pub struct SIZERR_R(crate::FieldReader<bool, SIZERR_A>);
impl SIZERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIZERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZERR_A {
        match self.bits {
            false => SIZERR_A::NOERROR,
            true => SIZERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == SIZERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == SIZERR_A::ERROR
    }
}
impl core::ops::Deref for SIZERR_R {
    type Target = crate::FieldReader<bool, SIZERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<SIZERR_AW> for bool {
    #[inline(always)]
    fn from(variant: SIZERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` writer - Size error"]
pub struct SIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIZERR_AW::CLEAR)
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
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTVERR_A {
    #[doc = "0: No error happened during the Option bytes loading"]
    NOERROR = 0,
    #[doc = "1: One or more errors happened during the Option bytes loading"]
    ERROR = 1,
}
impl From<OPTVERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTVERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub struct OPTVERR_R(crate::FieldReader<bool, OPTVERR_A>);
impl OPTVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTVERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTVERR_A {
        match self.bits {
            false => OPTVERR_A::NOERROR,
            true => OPTVERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == OPTVERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == OPTVERR_A::ERROR
    }
}
impl core::ops::Deref for OPTVERR_R {
    type Target = crate::FieldReader<bool, OPTVERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTVERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<OPTVERR_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub struct OPTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTVERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPTVERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPTVERR_AW::CLEAR)
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
#[doc = "RDERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_A {
    #[doc = "0: No read protection error happened."]
    NOERROR = 0,
    #[doc = "1: One read protection error happened"]
    ERROR = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` reader - RDERR"]
pub struct RDERR_R(crate::FieldReader<bool, RDERR_A>);
impl RDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDERR_A {
        match self.bits {
            false => RDERR_A::NOERROR,
            true => RDERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == RDERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == RDERR_A::ERROR
    }
}
impl core::ops::Deref for RDERR_R {
    type Target = crate::FieldReader<bool, RDERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RDERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<RDERR_AW> for bool {
    #[inline(always)]
    fn from(variant: RDERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` writer - RDERR"]
pub struct RDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RDERR_AW::CLEAR)
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
#[doc = "NOTZEROERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTZEROERR_A {
    #[doc = "0: The write operation is done in an erased region or the memory interface can apply an erase before a write"]
    NOEVENT = 0,
    #[doc = "1: The write operation is attempting to write to a not-erased region and the memory interface cannot apply an erase before a write"]
    EVENT = 1,
}
impl From<NOTZEROERR_A> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTZEROERR` reader - NOTZEROERR"]
pub struct NOTZEROERR_R(crate::FieldReader<bool, NOTZEROERR_A>);
impl NOTZEROERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOTZEROERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTZEROERR_A {
        match self.bits {
            false => NOTZEROERR_A::NOEVENT,
            true => NOTZEROERR_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == NOTZEROERR_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == NOTZEROERR_A::EVENT
    }
}
impl core::ops::Deref for NOTZEROERR_R {
    type Target = crate::FieldReader<bool, NOTZEROERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "NOTZEROERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTZEROERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<NOTZEROERR_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTZEROERR` writer - NOTZEROERR"]
pub struct NOTZEROERR_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTZEROERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOTZEROERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NOTZEROERR_AW::CLEAR)
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
#[doc = "FWWERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWWERR_A {
    #[doc = "0: No write/erase operation aborted to perform a fetch"]
    NOERROR = 0,
    #[doc = "1: A write/erase operation aborted to perform a fetch"]
    ERROR = 1,
}
impl From<FWWERR_A> for bool {
    #[inline(always)]
    fn from(variant: FWWERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWWERR` reader - FWWERR"]
pub struct FWWERR_R(crate::FieldReader<bool, FWWERR_A>);
impl FWWERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWWERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWWERR_A {
        match self.bits {
            false => FWWERR_A::NOERROR,
            true => FWWERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FWWERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FWWERR_A::ERROR
    }
}
impl core::ops::Deref for FWWERR_R {
    type Target = crate::FieldReader<bool, FWWERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FWWERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWWERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<FWWERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FWWERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWWERR` writer - FWWERR"]
pub struct FWWERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FWWERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWWERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FWWERR_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write/erase operations in progress"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of high voltage"]
    #[inline(always)]
    pub fn endhv(&self) -> ENDHV_R {
        ENDHV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash memory module ready after low power mode"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RDERR"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NOTZEROERR"]
    #[inline(always)]
    pub fn notzeroerr(&self) -> NOTZEROERR_R {
        NOTZEROERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FWWERR"]
    #[inline(always)]
    pub fn fwwerr(&self) -> FWWERR_R {
        FWWERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    #[doc = "Bit 9 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W {
        PGAERR_W { w: self }
    }
    #[doc = "Bit 10 - Size error"]
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W {
        SIZERR_W { w: self }
    }
    #[doc = "Bit 11 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W {
        OPTVERR_W { w: self }
    }
    #[doc = "Bit 13 - RDERR"]
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W {
        RDERR_W { w: self }
    }
    #[doc = "Bit 16 - NOTZEROERR"]
    #[inline(always)]
    pub fn notzeroerr(&mut self) -> NOTZEROERR_W {
        NOTZEROERR_W { w: self }
    }
    #[doc = "Bit 17 - FWWERR"]
    #[inline(always)]
    pub fn fwwerr(&mut self) -> FWWERR_W {
        FWWERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x04"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
