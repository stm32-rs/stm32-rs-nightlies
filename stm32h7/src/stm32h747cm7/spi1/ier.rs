#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Additional number of transactions reload interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSERFIE_A {
    #[doc = "0: TSER loaded interrupt masked"]
    MASKED = 0,
    #[doc = "1: TSER loaded interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TSERFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSERFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSERFIE`"]
pub type TSERFIE_R = crate::R<bool, TSERFIE_A>;
impl TSERFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSERFIE_A {
        match self.bits {
            false => TSERFIE_A::MASKED,
            true => TSERFIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSERFIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TSERFIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `TSERFIE`"]
pub struct TSERFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSERFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TSER loaded interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TSERFIE_A::MASKED)
    }
    #[doc = "TSER loaded interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TSERFIE_A::NOTMASKED)
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
#[doc = "Mode Fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFIE_A {
    #[doc = "0: Mode fault interrupt masked"]
    MASKED = 0,
    #[doc = "1: Mode fault interrupt not masked"]
    NOTMASKED = 1,
}
impl From<MODFIE_A> for bool {
    #[inline(always)]
    fn from(variant: MODFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODFIE`"]
pub type MODFIE_R = crate::R<bool, MODFIE_A>;
impl MODFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFIE_A {
        match self.bits {
            false => MODFIE_A::MASKED,
            true => MODFIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MODFIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == MODFIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `MODFIE`"]
pub struct MODFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode fault interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MODFIE_A::MASKED)
    }
    #[doc = "Mode fault interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(MODFIE_A::NOTMASKED)
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
#[doc = "TIFRE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIFREIE_A {
    #[doc = "0: TI frame format error interrupt masked"]
    MASKED = 0,
    #[doc = "1: TI frame format error interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TIFREIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIFREIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIFREIE`"]
pub type TIFREIE_R = crate::R<bool, TIFREIE_A>;
impl TIFREIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIFREIE_A {
        match self.bits {
            false => TIFREIE_A::MASKED,
            true => TIFREIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TIFREIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TIFREIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `TIFREIE`"]
pub struct TIFREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIFREIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TI frame format error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TIFREIE_A::MASKED)
    }
    #[doc = "TI frame format error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TIFREIE_A::NOTMASKED)
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
#[doc = "CRC Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEIE_A {
    #[doc = "0: CRC error interrupt masked"]
    MASKED = 0,
    #[doc = "1: CRC error interrupt not masked"]
    NOTMASKED = 1,
}
impl From<CRCEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCEIE`"]
pub type CRCEIE_R = crate::R<bool, CRCEIE_A>;
impl CRCEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEIE_A {
        match self.bits {
            false => CRCEIE_A::MASKED,
            true => CRCEIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CRCEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == CRCEIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `CRCEIE`"]
pub struct CRCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(CRCEIE_A::MASKED)
    }
    #[doc = "CRC error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(CRCEIE_A::NOTMASKED)
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
#[doc = "OVR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRIE_A {
    #[doc = "0: Overrun interrupt masked"]
    MASKED = 0,
    #[doc = "1: Overrun interrupt not masked"]
    NOTMASKED = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRIE`"]
pub type OVRIE_R = crate::R<bool, OVRIE_A>;
impl OVRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::MASKED,
            true => OVRIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == OVRIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == OVRIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `OVRIE`"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Overrun interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(OVRIE_A::MASKED)
    }
    #[doc = "Overrun interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(OVRIE_A::NOTMASKED)
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
#[doc = "UDR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDRIE_A {
    #[doc = "0: Underrun interrupt masked"]
    MASKED = 0,
    #[doc = "1: Underrun interrupt not masked"]
    NOTMASKED = 1,
}
impl From<UDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: UDRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UDRIE`"]
pub type UDRIE_R = crate::R<bool, UDRIE_A>;
impl UDRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRIE_A {
        match self.bits {
            false => UDRIE_A::MASKED,
            true => UDRIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == UDRIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == UDRIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `UDRIE`"]
pub struct UDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Underrun interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(UDRIE_A::MASKED)
    }
    #[doc = "Underrun interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(UDRIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "TXTFIE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTFIE_A {
    #[doc = "0: Transmission transfer filled interrupt masked"]
    MASKED = 0,
    #[doc = "1: Transmission transfer filled interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TXTFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXTFIE`"]
pub type TXTFIE_R = crate::R<bool, TXTFIE_A>;
impl TXTFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTFIE_A {
        match self.bits {
            false => TXTFIE_A::MASKED,
            true => TXTFIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXTFIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXTFIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `TXTFIE`"]
pub struct TXTFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmission transfer filled interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXTFIE_A::MASKED)
    }
    #[doc = "Transmission transfer filled interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXTFIE_A::NOTMASKED)
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
#[doc = "EOT, SUSP and TXC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTIE_A {
    #[doc = "0: End-of-transfer interrupt masked"]
    MASKED = 0,
    #[doc = "1: End-of-transfer interrupt not masked"]
    NOTMASKED = 1,
}
impl From<EOTIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOTIE`"]
pub type EOTIE_R = crate::R<bool, EOTIE_A>;
impl EOTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOTIE_A {
        match self.bits {
            false => EOTIE_A::MASKED,
            true => EOTIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EOTIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == EOTIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `EOTIE`"]
pub struct EOTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "End-of-transfer interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EOTIE_A::MASKED)
    }
    #[doc = "End-of-transfer interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(EOTIE_A::NOTMASKED)
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
#[doc = "DXP interrupt enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DXPIE_A {
    #[doc = "0: Duplex transfer complete interrupt masked"]
    MASKED = 0,
    #[doc = "1: Duplex transfer complete interrupt not masked"]
    NOTMASKED = 1,
}
impl From<DXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: DXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DXPIE`"]
pub type DXPIE_R = crate::R<bool, DXPIE_A>;
impl DXPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DXPIE_A {
        match self.bits {
            false => DXPIE_A::MASKED,
            true => DXPIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == DXPIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == DXPIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `DXPIE`"]
pub struct DXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DXPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Duplex transfer complete interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(DXPIE_A::MASKED)
    }
    #[doc = "Duplex transfer complete interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(DXPIE_A::NOTMASKED)
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
#[doc = "TXP interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPIE_A {
    #[doc = "0: TX space available interrupt masked"]
    MASKED = 0,
    #[doc = "1: TX space available interrupt not masked"]
    NOTMASKED = 1,
}
impl From<TXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPIE`"]
pub type TXPIE_R = crate::R<bool, TXPIE_A>;
impl TXPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPIE_A {
        match self.bits {
            false => TXPIE_A::MASKED,
            true => TXPIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXPIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXPIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `TXPIE`"]
pub struct TXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TX space available interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXPIE_A::MASKED)
    }
    #[doc = "TX space available interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXPIE_A::NOTMASKED)
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
#[doc = "RXP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPIE_A {
    #[doc = "0: RX data available interrupt masked"]
    MASKED = 0,
    #[doc = "1: RX data available interrupt not masked"]
    NOTMASKED = 1,
}
impl From<RXPIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPIE`"]
pub type RXPIE_R = crate::R<bool, RXPIE_A>;
impl RXPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPIE_A {
        match self.bits {
            false => RXPIE_A::MASKED,
            true => RXPIE_A::NOTMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXPIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXPIE_A::NOTMASKED
    }
}
#[doc = "Write proxy for field `RXPIE`"]
pub struct RXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX data available interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXPIE_A::MASKED)
    }
    #[doc = "RX data available interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXPIE_A::NOTMASKED)
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
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXP interrupt enabled"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXP interrupt enable"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Additional number of transactions reload interrupt enable"]
    #[inline(always)]
    pub fn tserfie(&mut self) -> TSERFIE_W {
        TSERFIE_W { w: self }
    }
    #[doc = "Bit 9 - Mode Fault interrupt enable"]
    #[inline(always)]
    pub fn modfie(&mut self) -> MODFIE_W {
        MODFIE_W { w: self }
    }
    #[doc = "Bit 8 - TIFRE interrupt enable"]
    #[inline(always)]
    pub fn tifreie(&mut self) -> TIFREIE_W {
        TIFREIE_W { w: self }
    }
    #[doc = "Bit 7 - CRC Interrupt enable"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W {
        CRCEIE_W { w: self }
    }
    #[doc = "Bit 6 - OVR interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bit 5 - UDR interrupt enable"]
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W {
        UDRIE_W { w: self }
    }
    #[doc = "Bit 4 - TXTFIE interrupt enable"]
    #[inline(always)]
    pub fn txtfie(&mut self) -> TXTFIE_W {
        TXTFIE_W { w: self }
    }
    #[doc = "Bit 3 - EOT, SUSP and TXC interrupt enable"]
    #[inline(always)]
    pub fn eotie(&mut self) -> EOTIE_W {
        EOTIE_W { w: self }
    }
    #[doc = "Bit 2 - DXP interrupt enabled"]
    #[inline(always)]
    pub fn dxpie(&mut self) -> DXPIE_W {
        DXPIE_W { w: self }
    }
    #[doc = "Bit 1 - TXP interrupt enable"]
    #[inline(always)]
    pub fn txpie(&mut self) -> TXPIE_W {
        TXPIE_W { w: self }
    }
    #[doc = "Bit 0 - RXP Interrupt Enable"]
    #[inline(always)]
    pub fn rxpie(&mut self) -> RXPIE_W {
        RXPIE_W { w: self }
    }
}
