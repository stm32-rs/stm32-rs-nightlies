#[doc = "Register `PECR` reader"]
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PECR` writer"]
pub struct W(crate::W<PECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PECR_SPEC>;
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
impl From<crate::W<PECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FLASH_PECR and data EEPROM lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PELOCK_A {
    #[doc = "0: The FLASH_PECR register is unlocked"]
    UNLOCKED = 0,
    #[doc = "1: The FLASH_PECR register is locked and no write/erase operation can start"]
    LOCKED = 1,
}
impl From<PELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PELOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PELOCK` reader - FLASH_PECR and data EEPROM lock"]
pub struct PELOCK_R(crate::FieldReader<bool, PELOCK_A>);
impl PELOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PELOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PELOCK_A {
        match self.bits {
            false => PELOCK_A::UNLOCKED,
            true => PELOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == PELOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == PELOCK_A::LOCKED
    }
}
impl core::ops::Deref for PELOCK_R {
    type Target = crate::FieldReader<bool, PELOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PELOCK` writer - FLASH_PECR and data EEPROM lock"]
pub struct PELOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PELOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PELOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FLASH_PECR register is unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(PELOCK_A::UNLOCKED)
    }
    #[doc = "The FLASH_PECR register is locked and no write/erase operation can start"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(PELOCK_A::LOCKED)
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
#[doc = "Program memory lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRGLOCK_A {
    #[doc = "0: The write and erase operations in the Flash program memory are disabled"]
    UNLOCKED = 0,
    #[doc = "1: The write and erase operations in the Flash program memory are enabled"]
    LOCKED = 1,
}
impl From<PRGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PRGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGLOCK` reader - Program memory lock"]
pub struct PRGLOCK_R(crate::FieldReader<bool, PRGLOCK_A>);
impl PRGLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRGLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGLOCK_A {
        match self.bits {
            false => PRGLOCK_A::UNLOCKED,
            true => PRGLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == PRGLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == PRGLOCK_A::LOCKED
    }
}
impl core::ops::Deref for PRGLOCK_R {
    type Target = crate::FieldReader<bool, PRGLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGLOCK` writer - Program memory lock"]
pub struct PRGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRGLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The write and erase operations in the Flash program memory are disabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(PRGLOCK_A::UNLOCKED)
    }
    #[doc = "The write and erase operations in the Flash program memory are enabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(PRGLOCK_A::LOCKED)
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
#[doc = "Option bytes block lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLOCK_A {
    #[doc = "0: The write and erase operations in the Option bytes area are disabled"]
    UNLOCKED = 0,
    #[doc = "1: The write and erase operations in the Option bytes area are enabled"]
    LOCKED = 1,
}
impl From<OPTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTLOCK` reader - Option bytes block lock"]
pub struct OPTLOCK_R(crate::FieldReader<bool, OPTLOCK_A>);
impl OPTLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPTLOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPTLOCK_A {
        match self.bits {
            false => OPTLOCK_A::UNLOCKED,
            true => OPTLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == OPTLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == OPTLOCK_A::LOCKED
    }
}
impl core::ops::Deref for OPTLOCK_R {
    type Target = crate::FieldReader<bool, OPTLOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPTLOCK` writer - Option bytes block lock"]
pub struct OPTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPTLOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The write and erase operations in the Option bytes area are disabled"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::UNLOCKED)
    }
    #[doc = "The write and erase operations in the Option bytes area are enabled"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(OPTLOCK_A::LOCKED)
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
#[doc = "Program memory selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROG_A {
    #[doc = "0: The Flash program memory is not selected"]
    NOTSELECTED = 0,
    #[doc = "1: The Flash program memory is selected"]
    SELECTED = 1,
}
impl From<PROG_A> for bool {
    #[inline(always)]
    fn from(variant: PROG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG` reader - Program memory selection"]
pub struct PROG_R(crate::FieldReader<bool, PROG_A>);
impl PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROG_A {
        match self.bits {
            false => PROG_A::NOTSELECTED,
            true => PROG_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        **self == PROG_A::NOTSELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        **self == PROG_A::SELECTED
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, PROG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROG` writer - Program memory selection"]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Flash program memory is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(PROG_A::NOTSELECTED)
    }
    #[doc = "The Flash program memory is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(PROG_A::SELECTED)
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
#[doc = "Data EEPROM selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_A {
    #[doc = "0: Data EEPROM not selected"]
    NOTSELECTED = 0,
    #[doc = "1: Data memory selected"]
    SELECTED = 1,
}
impl From<DATA_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA` reader - Data EEPROM selection"]
pub struct DATA_R(crate::FieldReader<bool, DATA_A>);
impl DATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_A {
        match self.bits {
            false => DATA_A::NOTSELECTED,
            true => DATA_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        **self == DATA_A::NOTSELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        **self == DATA_A::SELECTED
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<bool, DATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Data EEPROM selection"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data EEPROM not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(DATA_A::NOTSELECTED)
    }
    #[doc = "Data memory selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(DATA_A::SELECTED)
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
#[doc = "Fixed time data write for Byte, Half Word and Word programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIX_A {
    #[doc = "0: An erase phase is automatically performed"]
    AUTOERASE = 0,
    #[doc = "1: The program operation is always performed with a preliminary erase"]
    PRELIMERASE = 1,
}
impl From<FIX_A> for bool {
    #[inline(always)]
    fn from(variant: FIX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIX` reader - Fixed time data write for Byte, Half Word and Word programming"]
pub struct FIX_R(crate::FieldReader<bool, FIX_A>);
impl FIX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIX_A {
        match self.bits {
            false => FIX_A::AUTOERASE,
            true => FIX_A::PRELIMERASE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOERASE`"]
    #[inline(always)]
    pub fn is_auto_erase(&self) -> bool {
        **self == FIX_A::AUTOERASE
    }
    #[doc = "Checks if the value of the field is `PRELIMERASE`"]
    #[inline(always)]
    pub fn is_prelim_erase(&self) -> bool {
        **self == FIX_A::PRELIMERASE
    }
}
impl core::ops::Deref for FIX_R {
    type Target = crate::FieldReader<bool, FIX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIX` writer - Fixed time data write for Byte, Half Word and Word programming"]
pub struct FIX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An erase phase is automatically performed"]
    #[inline(always)]
    pub fn auto_erase(self) -> &'a mut W {
        self.variant(FIX_A::AUTOERASE)
    }
    #[doc = "The program operation is always performed with a preliminary erase"]
    #[inline(always)]
    pub fn prelim_erase(self) -> &'a mut W {
        self.variant(FIX_A::PRELIMERASE)
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
#[doc = "Page or Double Word erase mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASE_A {
    #[doc = "0: No erase operation requested"]
    NOERASE = 0,
    #[doc = "1: Erase operation requested"]
    ERASE = 1,
}
impl From<ERASE_A> for bool {
    #[inline(always)]
    fn from(variant: ERASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASE` reader - Page or Double Word erase mode"]
pub struct ERASE_R(crate::FieldReader<bool, ERASE_A>);
impl ERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASE_A {
        match self.bits {
            false => ERASE_A::NOERASE,
            true => ERASE_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NOERASE`"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        **self == ERASE_A::NOERASE
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == ERASE_A::ERASE
    }
}
impl core::ops::Deref for ERASE_R {
    type Target = crate::FieldReader<bool, ERASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASE` writer - Page or Double Word erase mode"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No erase operation requested"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(ERASE_A::NOERASE)
    }
    #[doc = "Erase operation requested"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASE_A::ERASE)
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
#[doc = "Half Page/Double Word programming mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRG_A {
    #[doc = "0: Half Page programming disabled"]
    DISABLED = 0,
    #[doc = "1: Half Page programming enabled"]
    ENABLED = 1,
}
impl From<FPRG_A> for bool {
    #[inline(always)]
    fn from(variant: FPRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRG` reader - Half Page/Double Word programming mode"]
pub struct FPRG_R(crate::FieldReader<bool, FPRG_A>);
impl FPRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPRG_A {
        match self.bits {
            false => FPRG_A::DISABLED,
            true => FPRG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FPRG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FPRG_A::ENABLED
    }
}
impl core::ops::Deref for FPRG_R {
    type Target = crate::FieldReader<bool, FPRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPRG` writer - Half Page/Double Word programming mode"]
pub struct FPRG_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Half Page programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPRG_A::DISABLED)
    }
    #[doc = "Half Page programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPRG_A::ENABLED)
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
#[doc = "Parallel bank mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARALLELBANK_A {
    #[doc = "0: Parallel bank mode disabled"]
    DISABLED = 0,
    #[doc = "1: Parallel bank mode enabled"]
    ENABLED = 1,
}
impl From<PARALLELBANK_A> for bool {
    #[inline(always)]
    fn from(variant: PARALLELBANK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARALLELBANK` reader - Parallel bank mode"]
pub struct PARALLELBANK_R(crate::FieldReader<bool, PARALLELBANK_A>);
impl PARALLELBANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARALLELBANK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARALLELBANK_A {
        match self.bits {
            false => PARALLELBANK_A::DISABLED,
            true => PARALLELBANK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PARALLELBANK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PARALLELBANK_A::ENABLED
    }
}
impl core::ops::Deref for PARALLELBANK_R {
    type Target = crate::FieldReader<bool, PARALLELBANK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARALLELBANK` writer - Parallel bank mode"]
pub struct PARALLELBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> PARALLELBANK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARALLELBANK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Parallel bank mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PARALLELBANK_A::DISABLED)
    }
    #[doc = "Parallel bank mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PARALLELBANK_A::ENABLED)
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
#[doc = "End of programming interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPIE_A {
    #[doc = "0: End of program interrupt disable"]
    DISABLED = 0,
    #[doc = "1: End of program interrupt enable"]
    ENABLED = 1,
}
impl From<EOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - End of programming interrupt enable"]
pub struct EOPIE_R(crate::FieldReader<bool, EOPIE_A>);
impl EOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOPIE_A {
        match self.bits {
            false => EOPIE_A::DISABLED,
            true => EOPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EOPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EOPIE_A::ENABLED
    }
}
impl core::ops::Deref for EOPIE_R {
    type Target = crate::FieldReader<bool, EOPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPIE` writer - End of programming interrupt enable"]
pub struct EOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOPIE_A::DISABLED)
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOPIE_A::ENABLED)
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
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt disable"]
    DISABLED = 0,
    #[doc = "1: Error interrupt enable"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRIE_A::ENABLED
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
#[doc = "Launch the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBL_LAUNCH_A {
    #[doc = "0: Option byte loaded"]
    COMPLETE = 0,
    #[doc = "1: Option byte loading to be done"]
    NOTCOMPLETE = 1,
}
impl From<OBL_LAUNCH_A> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` reader - Launch the option byte loading"]
pub struct OBL_LAUNCH_R(crate::FieldReader<bool, OBL_LAUNCH_A>);
impl OBL_LAUNCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBL_LAUNCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OBL_LAUNCH_A {
        match self.bits {
            false => OBL_LAUNCH_A::COMPLETE,
            true => OBL_LAUNCH_A::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == OBL_LAUNCH_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == OBL_LAUNCH_A::NOTCOMPLETE
    }
}
impl core::ops::Deref for OBL_LAUNCH_R {
    type Target = crate::FieldReader<bool, OBL_LAUNCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Launch the option byte loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBL_LAUNCH_AW {
    #[doc = "1: Reload option byte"]
    RELOAD = 1,
}
impl From<OBL_LAUNCH_AW> for bool {
    #[inline(always)]
    fn from(variant: OBL_LAUNCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBL_LAUNCH` writer - Launch the option byte loading"]
pub struct OBL_LAUNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OBL_LAUNCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OBL_LAUNCH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reload option byte"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(OBL_LAUNCH_AW::RELOAD)
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
impl R {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&self) -> PELOCK_R {
        PELOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&self) -> PRGLOCK_R {
        PRGLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn fix(&self) -> FIX_R {
        FIX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&self) -> FPRG_R {
        FPRG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&self) -> PARALLELBANK_R {
        PARALLELBANK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_PECR and data EEPROM lock"]
    #[inline(always)]
    pub fn pelock(&mut self) -> PELOCK_W {
        PELOCK_W { w: self }
    }
    #[doc = "Bit 1 - Program memory lock"]
    #[inline(always)]
    pub fn prglock(&mut self) -> PRGLOCK_W {
        PRGLOCK_W { w: self }
    }
    #[doc = "Bit 2 - Option bytes block lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W {
        OPTLOCK_W { w: self }
    }
    #[doc = "Bit 3 - Program memory selection"]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bit 4 - Data EEPROM selection"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bit 8 - Fixed time data write for Byte, Half Word and Word programming"]
    #[inline(always)]
    pub fn fix(&mut self) -> FIX_W {
        FIX_W { w: self }
    }
    #[doc = "Bit 9 - Page or Double Word erase mode"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 10 - Half Page/Double Word programming mode"]
    #[inline(always)]
    pub fn fprg(&mut self) -> FPRG_W {
        FPRG_W { w: self }
    }
    #[doc = "Bit 15 - Parallel bank mode"]
    #[inline(always)]
    pub fn parallelbank(&mut self) -> PARALLELBANK_W {
        PARALLELBANK_W { w: self }
    }
    #[doc = "Bit 16 - End of programming interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W { w: self }
    }
    #[doc = "Bit 17 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 18 - Launch the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W {
        OBL_LAUNCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program/erase control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](index.html) module"]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pecr::R](R) reader structure"]
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pecr::W](W) writer structure"]
impl crate::Writable for PECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PECR to value 0x07"]
impl crate::Resettable for PECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
