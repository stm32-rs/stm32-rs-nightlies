#[doc = "Reader of register TIMDDIER"]
pub type R = crate::R<u32, super::TIMDDIER>;
#[doc = "Writer for register TIMDDIER"]
pub type W = crate::W<u32, super::TIMDDIER>;
#[doc = "Register TIMDDIER `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMDDIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DLYPRTDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYPRTDE_A {
    #[doc = "0: Delayed protection DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Delayed protection DMA request enabled"]
    ENABLED = 1,
}
impl From<DLYPRTDE_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLYPRTDE`"]
pub type DLYPRTDE_R = crate::R<bool, DLYPRTDE_A>;
impl DLYPRTDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTDE_A {
        match self.bits {
            false => DLYPRTDE_A::DISABLED,
            true => DLYPRTDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTDE_A::ENABLED
    }
}
#[doc = "Write proxy for field `DLYPRTDE`"]
pub struct DLYPRTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYPRTDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Delayed protection DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTDE_A::DISABLED)
    }
    #[doc = "Delayed protection DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "RSTDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTDE_A {
    #[doc = "0: Timer x counter reset/roll-over DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Timer x counter reset/roll-over DMA request enabled"]
    ENABLED = 1,
}
impl From<RSTDE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTDE`"]
pub type RSTDE_R = crate::R<bool, RSTDE_A>;
impl RSTDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTDE_A {
        match self.bits {
            false => RSTDE_A::DISABLED,
            true => RSTDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTDE_A::ENABLED
    }
}
#[doc = "Write proxy for field `RSTDE`"]
pub struct RSTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer x counter reset/roll-over DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTDE_A::DISABLED)
    }
    #[doc = "Timer x counter reset/roll-over DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTDE_A::ENABLED)
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
#[doc = "RSTx2DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTX2DE_A {
    #[doc = "0: Tx output reset DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Tx output reset DMA request enabled"]
    ENABLED = 1,
}
impl From<RSTX2DE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX2DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTx2DE`"]
pub type RSTX2DE_R = crate::R<bool, RSTX2DE_A>;
impl RSTX2DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTX2DE_A {
        match self.bits {
            false => RSTX2DE_A::DISABLED,
            true => RSTX2DE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX2DE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX2DE_A::ENABLED
    }
}
#[doc = "Write proxy for field `RSTx2DE`"]
pub struct RSTX2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output reset DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX2DE_A::DISABLED)
    }
    #[doc = "Tx output reset DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "SETx2DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETX2DE_A {
    #[doc = "0: Tx output set DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Tx output set DMA request enabled"]
    ENABLED = 1,
}
impl From<SETX2DE_A> for bool {
    #[inline(always)]
    fn from(variant: SETX2DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SETx2DE`"]
pub type SETX2DE_R = crate::R<bool, SETX2DE_A>;
impl SETX2DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETX2DE_A {
        match self.bits {
            false => SETX2DE_A::DISABLED,
            true => SETX2DE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX2DE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX2DE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SETx2DE`"]
pub struct SETX2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETX2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output set DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX2DE_A::DISABLED)
    }
    #[doc = "Tx output set DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX2DE_A::ENABLED)
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
#[doc = "RSTx1DE"]
pub type RSTX1DE_A = RSTX2DE_A;
#[doc = "Reader of field `RSTx1DE`"]
pub type RSTX1DE_R = crate::R<bool, RSTX2DE_A>;
#[doc = "Write proxy for field `RSTx1DE`"]
pub struct RSTX1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output reset DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX2DE_A::DISABLED)
    }
    #[doc = "Tx output reset DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "SET1xDE"]
pub type SETX1DE_A = SETX2DE_A;
#[doc = "Reader of field `SETx1DE`"]
pub type SETX1DE_R = crate::R<bool, SETX2DE_A>;
#[doc = "Write proxy for field `SETx1DE`"]
pub struct SETX1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETX1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output set DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX2DE_A::DISABLED)
    }
    #[doc = "Tx output set DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "CPT2DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPT2DE_A {
    #[doc = "0: Capture DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Capture DMA request enabled"]
    ENABLED = 1,
}
impl From<CPT2DE_A> for bool {
    #[inline(always)]
    fn from(variant: CPT2DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPT2DE`"]
pub type CPT2DE_R = crate::R<bool, CPT2DE_A>;
impl CPT2DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPT2DE_A {
        match self.bits {
            false => CPT2DE_A::DISABLED,
            true => CPT2DE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT2DE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT2DE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CPT2DE`"]
pub struct CPT2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT2DE_A::DISABLED)
    }
    #[doc = "Capture DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "CPT1DE"]
pub type CPT1DE_A = CPT2DE_A;
#[doc = "Reader of field `CPT1DE`"]
pub type CPT1DE_R = crate::R<bool, CPT2DE_A>;
#[doc = "Write proxy for field `CPT1DE`"]
pub struct CPT1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT2DE_A::DISABLED)
    }
    #[doc = "Capture DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "UPDDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDDE_A {
    #[doc = "0: Update DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Update DMA request enabled"]
    ENABLED = 1,
}
impl From<UPDDE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDDE`"]
pub type UPDDE_R = crate::R<bool, UPDDE_A>;
impl UPDDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDDE_A {
        match self.bits {
            false => UPDDE_A::DISABLED,
            true => UPDDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDDE_A::ENABLED
    }
}
#[doc = "Write proxy for field `UPDDE`"]
pub struct UPDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDDE_A::DISABLED)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "REPDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPDE_A {
    #[doc = "0: Repetition DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Repetition DMA request enabled"]
    ENABLED = 1,
}
impl From<REPDE_A> for bool {
    #[inline(always)]
    fn from(variant: REPDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REPDE`"]
pub type REPDE_R = crate::R<bool, REPDE_A>;
impl REPDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPDE_A {
        match self.bits {
            false => REPDE_A::DISABLED,
            true => REPDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPDE_A::ENABLED
    }
}
#[doc = "Write proxy for field `REPDE`"]
pub struct REPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Repetition DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPDE_A::DISABLED)
    }
    #[doc = "Repetition DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPDE_A::ENABLED)
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
#[doc = "CMP4DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP4DE_A {
    #[doc = "0: Compare DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: Compare DMA request enabled"]
    ENABLED = 1,
}
impl From<CMP4DE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP4DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP4DE`"]
pub type CMP4DE_R = crate::R<bool, CMP4DE_A>;
impl CMP4DE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP4DE_A {
        match self.bits {
            false => CMP4DE_A::DISABLED,
            true => CMP4DE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP4DE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP4DE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CMP4DE`"]
pub struct CMP4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::DISABLED)
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::ENABLED)
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
#[doc = "CMP3DE"]
pub type CMP3DE_A = CMP4DE_A;
#[doc = "Reader of field `CMP3DE`"]
pub type CMP3DE_R = crate::R<bool, CMP4DE_A>;
#[doc = "Write proxy for field `CMP3DE`"]
pub struct CMP3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP3DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::DISABLED)
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::ENABLED)
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
#[doc = "CMP2DE"]
pub type CMP2DE_A = CMP4DE_A;
#[doc = "Reader of field `CMP2DE`"]
pub type CMP2DE_R = crate::R<bool, CMP4DE_A>;
#[doc = "Write proxy for field `CMP2DE`"]
pub struct CMP2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::DISABLED)
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::ENABLED)
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
#[doc = "CMP1DE"]
pub type CMP1DE_A = CMP4DE_A;
#[doc = "Reader of field `CMP1DE`"]
pub type CMP1DE_R = crate::R<bool, CMP4DE_A>;
#[doc = "Write proxy for field `CMP1DE`"]
pub struct CMP1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::DISABLED)
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4DE_A::ENABLED)
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
#[doc = "DLYPRTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYPRTIE_A {
    #[doc = "0: Delayed protection interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Delayed protection interrupt enabled"]
    ENABLED = 1,
}
impl From<DLYPRTIE_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLYPRTIE`"]
pub type DLYPRTIE_R = crate::R<bool, DLYPRTIE_A>;
impl DLYPRTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTIE_A {
        match self.bits {
            false => DLYPRTIE_A::DISABLED,
            true => DLYPRTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `DLYPRTIE`"]
pub struct DLYPRTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYPRTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Delayed protection interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTIE_A::DISABLED)
    }
    #[doc = "Delayed protection interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "RSTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTIE_A {
    #[doc = "0: Timer x counter/reset roll-over interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Timer x counter/reset roll-over interrupt enabled"]
    ENABLED = 1,
}
impl From<RSTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTIE`"]
pub type RSTIE_R = crate::R<bool, RSTIE_A>;
impl RSTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTIE_A {
        match self.bits {
            false => RSTIE_A::DISABLED,
            true => RSTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `RSTIE`"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer x counter/reset roll-over interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTIE_A::DISABLED)
    }
    #[doc = "Timer x counter/reset roll-over interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "RSTx2IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTX2IE_A {
    #[doc = "0: Tx output reset interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Tx output reset interrupt enabled"]
    ENABLED = 1,
}
impl From<RSTX2IE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTx2IE`"]
pub type RSTX2IE_R = crate::R<bool, RSTX2IE_A>;
impl RSTX2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTX2IE_A {
        match self.bits {
            false => RSTX2IE_A::DISABLED,
            true => RSTX2IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX2IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX2IE_A::ENABLED
    }
}
#[doc = "Write proxy for field `RSTx2IE`"]
pub struct RSTX2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output reset interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX2IE_A::DISABLED)
    }
    #[doc = "Tx output reset interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "SETx2IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETX2IE_A {
    #[doc = "0: Tx output set interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Tx output set interrupt enabled"]
    ENABLED = 1,
}
impl From<SETX2IE_A> for bool {
    #[inline(always)]
    fn from(variant: SETX2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SETx2IE`"]
pub type SETX2IE_R = crate::R<bool, SETX2IE_A>;
impl SETX2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETX2IE_A {
        match self.bits {
            false => SETX2IE_A::DISABLED,
            true => SETX2IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX2IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX2IE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SETx2IE`"]
pub struct SETX2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETX2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output set interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX2IE_A::DISABLED)
    }
    #[doc = "Tx output set interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX2IE_A::ENABLED)
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
#[doc = "RSTx1IE"]
pub type RSTX1IE_A = RSTX2IE_A;
#[doc = "Reader of field `RSTx1IE`"]
pub type RSTX1IE_R = crate::R<bool, RSTX2IE_A>;
#[doc = "Write proxy for field `RSTx1IE`"]
pub struct RSTX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output reset interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX2IE_A::DISABLED)
    }
    #[doc = "Tx output reset interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX2IE_A::ENABLED)
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
#[doc = "SET1xIE"]
pub type SETX1IE_A = SETX2IE_A;
#[doc = "Reader of field `SETx1IE`"]
pub type SETX1IE_R = crate::R<bool, SETX2IE_A>;
#[doc = "Write proxy for field `SETx1IE`"]
pub struct SETX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETX1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETX1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx output set interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX2IE_A::DISABLED)
    }
    #[doc = "Tx output set interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "CPT2IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPT2IE_A {
    #[doc = "0: Capture interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Capture interrupt enabled"]
    ENABLED = 1,
}
impl From<CPT2IE_A> for bool {
    #[inline(always)]
    fn from(variant: CPT2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPT2IE`"]
pub type CPT2IE_R = crate::R<bool, CPT2IE_A>;
impl CPT2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPT2IE_A {
        match self.bits {
            false => CPT2IE_A::DISABLED,
            true => CPT2IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT2IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT2IE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CPT2IE`"]
pub struct CPT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT2IE_A::DISABLED)
    }
    #[doc = "Capture interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CPT1IE"]
pub type CPT1IE_A = CPT2IE_A;
#[doc = "Reader of field `CPT1IE`"]
pub type CPT1IE_R = crate::R<bool, CPT2IE_A>;
#[doc = "Write proxy for field `CPT1IE`"]
pub struct CPT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT2IE_A::DISABLED)
    }
    #[doc = "Capture interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "UPDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDIE_A {
    #[doc = "0: Update interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Update interrupt enabled"]
    ENABLED = 1,
}
impl From<UPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDIE`"]
pub type UPDIE_R = crate::R<bool, UPDIE_A>;
impl UPDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDIE_A {
        match self.bits {
            false => UPDIE_A::DISABLED,
            true => UPDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `UPDIE`"]
pub struct UPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDIE_A::DISABLED)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "REPIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPIE_A {
    #[doc = "0: Repetition interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Repetition interrupt enabled"]
    ENABLED = 1,
}
impl From<REPIE_A> for bool {
    #[inline(always)]
    fn from(variant: REPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REPIE`"]
pub type REPIE_R = crate::R<bool, REPIE_A>;
impl REPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPIE_A {
        match self.bits {
            false => REPIE_A::DISABLED,
            true => REPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `REPIE`"]
pub struct REPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Repetition interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPIE_A::DISABLED)
    }
    #[doc = "Repetition interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPIE_A::ENABLED)
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
#[doc = "CMP4IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP4IE_A {
    #[doc = "0: Compare interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Compare interrupt enabled"]
    ENABLED = 1,
}
impl From<CMP4IE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP4IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP4IE`"]
pub type CMP4IE_R = crate::R<bool, CMP4IE_A>;
impl CMP4IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP4IE_A {
        match self.bits {
            false => CMP4IE_A::DISABLED,
            true => CMP4IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP4IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP4IE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CMP4IE`"]
pub struct CMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::DISABLED)
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::ENABLED)
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
#[doc = "CMP3IE"]
pub type CMP3IE_A = CMP4IE_A;
#[doc = "Reader of field `CMP3IE`"]
pub type CMP3IE_R = crate::R<bool, CMP4IE_A>;
#[doc = "Write proxy for field `CMP3IE`"]
pub struct CMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP3IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::DISABLED)
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "CMP2IE"]
pub type CMP2IE_A = CMP4IE_A;
#[doc = "Reader of field `CMP2IE`"]
pub type CMP2IE_R = crate::R<bool, CMP4IE_A>;
#[doc = "Write proxy for field `CMP2IE`"]
pub struct CMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::DISABLED)
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "CMP1IE"]
pub type CMP1IE_A = CMP4IE_A;
#[doc = "Reader of field `CMP1IE`"]
pub type CMP1IE_R = crate::R<bool, CMP4IE_A>;
#[doc = "Write proxy for field `CMP1IE`"]
pub struct CMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::DISABLED)
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP4IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    pub fn rstx2de(&self) -> RSTX2DE_R {
        RSTX2DE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    pub fn setx2de(&self) -> SETX2DE_R {
        SETX2DE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    pub fn rstx1de(&self) -> RSTX1DE_R {
        RSTX1DE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    pub fn setx1de(&self) -> SETX1DE_R {
        SETX1DE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    pub fn cpt2de(&self) -> CPT2DE_R {
        CPT2DE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    pub fn cpt1de(&self) -> CPT1DE_R {
        CPT1DE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    pub fn cmp4de(&self) -> CMP4DE_R {
        CMP4DE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    pub fn cmp3de(&self) -> CMP3DE_R {
        CMP3DE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    pub fn cmp2de(&self) -> CMP2DE_R {
        CMP2DE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    pub fn cmp1de(&self) -> CMP1DE_R {
        CMP1DE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    pub fn rstx2ie(&self) -> RSTX2IE_R {
        RSTX2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    pub fn setx2ie(&self) -> SETX2IE_R {
        SETX2IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    pub fn rstx1ie(&self) -> RSTX1IE_R {
        RSTX1IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    pub fn setx1ie(&self) -> SETX1IE_R {
        SETX1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPT2IE_R {
        CPT2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPT1IE_R {
        CPT1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMP4IE_R {
        CMP4IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W {
        DLYPRTDE_W { w: self }
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    pub fn rstde(&mut self) -> RSTDE_W {
        RSTDE_W { w: self }
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    pub fn rstx2de(&mut self) -> RSTX2DE_W {
        RSTX2DE_W { w: self }
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    pub fn setx2de(&mut self) -> SETX2DE_W {
        SETX2DE_W { w: self }
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    pub fn rstx1de(&mut self) -> RSTX1DE_W {
        RSTX1DE_W { w: self }
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    pub fn setx1de(&mut self) -> SETX1DE_W {
        SETX1DE_W { w: self }
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    pub fn cpt2de(&mut self) -> CPT2DE_W {
        CPT2DE_W { w: self }
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    pub fn cpt1de(&mut self) -> CPT1DE_W {
        CPT1DE_W { w: self }
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    pub fn updde(&mut self) -> UPDDE_W {
        UPDDE_W { w: self }
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    pub fn repde(&mut self) -> REPDE_W {
        REPDE_W { w: self }
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    pub fn cmp4de(&mut self) -> CMP4DE_W {
        CMP4DE_W { w: self }
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    pub fn cmp3de(&mut self) -> CMP3DE_W {
        CMP3DE_W { w: self }
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    pub fn cmp2de(&mut self) -> CMP2DE_W {
        CMP2DE_W { w: self }
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    pub fn cmp1de(&mut self) -> CMP1DE_W {
        CMP1DE_W { w: self }
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W {
        DLYPRTIE_W { w: self }
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    pub fn rstx2ie(&mut self) -> RSTX2IE_W {
        RSTX2IE_W { w: self }
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    pub fn setx2ie(&mut self) -> SETX2IE_W {
        SETX2IE_W { w: self }
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    pub fn rstx1ie(&mut self) -> RSTX1IE_W {
        RSTX1IE_W { w: self }
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    pub fn setx1ie(&mut self) -> SETX1IE_W {
        SETX1IE_W { w: self }
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    pub fn cpt2ie(&mut self) -> CPT2IE_W {
        CPT2IE_W { w: self }
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    pub fn cpt1ie(&mut self) -> CPT1IE_W {
        CPT1IE_W { w: self }
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W {
        UPDIE_W { w: self }
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    pub fn repie(&mut self) -> REPIE_W {
        REPIE_W { w: self }
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    pub fn cmp4ie(&mut self) -> CMP4IE_W {
        CMP4IE_W { w: self }
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMP3IE_W {
        CMP3IE_W { w: self }
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMP2IE_W {
        CMP2IE_W { w: self }
    }
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMP1IE_W {
        CMP1IE_W { w: self }
    }
}
