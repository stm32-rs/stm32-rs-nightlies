#[doc = "Register `C2SR` reader"]
pub struct R(crate::R<C2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2SR` writer"]
pub struct W(crate::W<C2SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2SR_SPEC>;
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
impl From<crate::W<C2SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2SR_SPEC>) -> Self {
        W(writer)
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
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<EOP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOP_AW::CLEAR)
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
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERR_A {
    #[doc = "0: No memory opreation error happened"]
    NOERROR = 0,
    #[doc = "1: Memory operation error happened"]
    ERROR = 1,
}
impl From<OPERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - Operation error"]
pub struct OPERR_R(crate::FieldReader<bool, OPERR_A>);
impl OPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERR_A {
        match self.bits {
            false => OPERR_A::NOERROR,
            true => OPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == OPERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == OPERR_A::ERROR
    }
}
impl core::ops::Deref for OPERR_R {
    type Target = crate::FieldReader<bool, OPERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<OPERR_AW> for bool {
    #[inline(always)]
    fn from(variant: OPERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` writer - Operation error"]
pub struct OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPERR_AW::CLEAR)
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
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_A {
    #[doc = "0: No size programming error happened"]
    NOERROR = 0,
    #[doc = "1: Programming error happened"]
    ERROR = 1,
}
impl From<PROGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PROGERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGERR` reader - Programming error"]
pub struct PROGERR_R(crate::FieldReader<bool, PROGERR_A>);
impl PROGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROGERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGERR_A {
        match self.bits {
            false => PROGERR_A::NOERROR,
            true => PROGERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PROGERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PROGERR_A::ERROR
    }
}
impl core::ops::Deref for PROGERR_R {
    type Target = crate::FieldReader<bool, PROGERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<PROGERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PROGERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGERR` writer - Programming error"]
pub struct PROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROGERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROGERR_AW::CLEAR)
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
#[doc = "WRPERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPERR_A {
    #[doc = "0: No write protection error happened"]
    NOERROR = 0,
    #[doc = "1: Write protection error happened"]
    ERROR = 1,
}
impl From<WRPERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - WRPERR"]
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
#[doc = "WRPERR\n\nValue on reset: 0"]
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
#[doc = "Field `WRPERR` writer - WRPERR"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "PGAERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAERR_A {
    #[doc = "0: No programming alignment error happened"]
    NOERROR = 0,
    #[doc = "1: Programming alignment error happened"]
    ERROR = 1,
}
impl From<PGAERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` reader - PGAERR"]
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
#[doc = "PGAERR\n\nValue on reset: 0"]
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
#[doc = "Field `PGAERR` writer - PGAERR"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZERR_A {
    #[doc = "0: No size error happened"]
    NOERROR = 0,
    #[doc = "1: Size error happened"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGSERR_A {
    #[doc = "0: No fast programming sequence error happened"]
    NOERROR = 0,
    #[doc = "1: Fast programming sequence error happened"]
    ERROR = 1,
}
impl From<PGSERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub struct PGSERR_R(crate::FieldReader<bool, PGSERR_A>);
impl PGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGSERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGSERR_A {
        match self.bits {
            false => PGSERR_A::NOERROR,
            true => PGSERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == PGSERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == PGSERR_A::ERROR
    }
}
impl core::ops::Deref for PGSERR_R {
    type Target = crate::FieldReader<bool, PGSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGSERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<PGSERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGSERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub struct PGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGSERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERR_AW::CLEAR)
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
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISSERR_A {
    #[doc = "0: No fast programming data miss error happened"]
    NOERROR = 0,
    #[doc = "1: Fast programming data miss error happened"]
    ERROR = 1,
}
impl From<MISSERR_A> for bool {
    #[inline(always)]
    fn from(variant: MISSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISSERR` reader - Fast programming data miss error"]
pub struct MISSERR_R(crate::FieldReader<bool, MISSERR_A>);
impl MISSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISSERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISSERR_A {
        match self.bits {
            false => MISSERR_A::NOERROR,
            true => MISSERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == MISSERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == MISSERR_A::ERROR
    }
}
impl core::ops::Deref for MISSERR_R {
    type Target = crate::FieldReader<bool, MISSERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast programming data miss error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISSERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<MISSERR_AW> for bool {
    #[inline(always)]
    fn from(variant: MISSERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MISSERR` writer - Fast programming data miss error"]
pub struct MISSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MISSERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MISSERR_AW::CLEAR)
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
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTERR_A {
    #[doc = "0: No fast programming error happened"]
    NOERROR = 0,
    #[doc = "1: Fast programming error happened"]
    ERROR = 1,
}
impl From<FASTERR_A> for bool {
    #[inline(always)]
    fn from(variant: FASTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub struct FASTERR_R(crate::FieldReader<bool, FASTERR_A>);
impl FASTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTERR_A {
        match self.bits {
            false => FASTERR_A::NOERROR,
            true => FASTERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FASTERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FASTERR_A::ERROR
    }
}
impl core::ops::Deref for FASTERR_R {
    type Target = crate::FieldReader<bool, FASTERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTERR_AW {
    #[doc = "1: Clear the flag"]
    CLEAR = 1,
}
impl From<FASTERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FASTERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub struct FASTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FASTERR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FASTERR_AW::CLEAR)
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
#[doc = "PCROP read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_A {
    #[doc = "0: No read-only error happened"]
    NOERROR = 0,
    #[doc = "1: Read-only error happened"]
    ERROR = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` reader - PCROP read error"]
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
#[doc = "PCROP read error\n\nValue on reset: 0"]
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
#[doc = "Field `RDERR` writer - PCROP read error"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "BSY\n\nValue on reset: 0"]
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
#[doc = "Field `BSY` reader - BSY"]
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
#[doc = "CFGBSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGBSY_A {
    #[doc = "0: PG, PNB, PER, MER bits available for writing"]
    FREE = 0,
    #[doc = "1: PG, PNB, PER, MER bits not available for writing (operation ongoing)"]
    BUSY = 1,
}
impl From<CFGBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CFGBSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGBSY` reader - CFGBSY"]
pub struct CFGBSY_R(crate::FieldReader<bool, CFGBSY_A>);
impl CFGBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGBSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGBSY_A {
        match self.bits {
            false => CFGBSY_A::FREE,
            true => CFGBSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        **self == CFGBSY_A::FREE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == CFGBSY_A::BUSY
    }
}
impl core::ops::Deref for CFGBSY_R {
    type Target = crate::FieldReader<bool, CFGBSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PESD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD_A {
    #[doc = "0: Flash program and erase operations granted"]
    GRANTED = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when at least one PES bit in FLASH_ACR or FLASH_C2ACR is set."]
    SUSPENDED = 1,
}
impl From<PESD_A> for bool {
    #[inline(always)]
    fn from(variant: PESD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESD` reader - PESD"]
pub struct PESD_R(crate::FieldReader<bool, PESD_A>);
impl PESD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD_A {
        match self.bits {
            false => PESD_A::GRANTED,
            true => PESD_A::SUSPENDED,
        }
    }
    #[doc = "Checks if the value of the field is `GRANTED`"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        **self == PESD_A::GRANTED
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        **self == PESD_A::SUSPENDED
    }
}
impl core::ops::Deref for PESD_R {
    type Target = crate::FieldReader<bool, PESD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WRPERR"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PGAERR"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSY"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CFGBSY"]
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PESD"]
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W { w: self }
    }
    #[doc = "Bit 4 - WRPERR"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    #[doc = "Bit 5 - PGAERR"]
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W {
        PGAERR_W { w: self }
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W {
        SIZERR_W { w: self }
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W {
        PGSERR_W { w: self }
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn misserr(&mut self) -> MISSERR_W {
        MISSERR_W { w: self }
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W {
        FASTERR_W { w: self }
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W {
        RDERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash CPU2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2sr](index.html) module"]
pub struct C2SR_SPEC;
impl crate::RegisterSpec for C2SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2sr::R](R) reader structure"]
impl crate::Readable for C2SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2sr::W](W) writer structure"]
impl crate::Writable for C2SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2SR to value 0"]
impl crate::Resettable for C2SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
