#[doc = "Reader of register BDMUPR"]
pub type R = crate::R<u32, super::BDMUPR>;
#[doc = "Writer for register BDMUPR"]
pub type W = crate::W<u32, super::BDMUPR>;
#[doc = "Register BDMUPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BDMUPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCMP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMP4_A {
    #[doc = "0: Register not updated by burst DMA access"]
    NOTUPDATED = 0,
    #[doc = "1: Register updated by burst DMA access"]
    UPDATED = 1,
}
impl From<MCMP4_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCMP4`"]
pub type MCMP4_R = crate::R<bool, MCMP4_A>;
impl MCMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMP4_A {
        match self.bits {
            false => MCMP4_A::NOTUPDATED,
            true => MCMP4_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTUPDATED`"]
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        *self == MCMP4_A::NOTUPDATED
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == MCMP4_A::UPDATED
    }
}
#[doc = "Write proxy for field `MCMP4`"]
pub struct MCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MCMP3"]
pub type MCMP3_A = MCMP4_A;
#[doc = "Reader of field `MCMP3`"]
pub type MCMP3_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MCMP3`"]
pub struct MCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MCMP2"]
pub type MCMP2_A = MCMP4_A;
#[doc = "Reader of field `MCMP2`"]
pub type MCMP2_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MCMP2`"]
pub struct MCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MCMP1"]
pub type MCMP1_A = MCMP4_A;
#[doc = "Reader of field `MCMP1`"]
pub type MCMP1_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MCMP1`"]
pub struct MCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MREP"]
pub type MREP_A = MCMP4_A;
#[doc = "Reader of field `MREP`"]
pub type MREP_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MREP`"]
pub struct MREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MPER"]
pub type MPER_A = MCMP4_A;
#[doc = "Reader of field `MPER`"]
pub type MPER_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MPER`"]
pub struct MPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MCNT"]
pub type MCNT_A = MCMP4_A;
#[doc = "Reader of field `MCNT`"]
pub type MCNT_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MCNT`"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MDIER"]
pub type MDIER_A = MCMP4_A;
#[doc = "Reader of field `MDIER`"]
pub type MDIER_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MDIER`"]
pub struct MDIER_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MICR"]
pub type MICR_A = MCMP4_A;
#[doc = "Reader of field `MICR`"]
pub type MICR_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MICR`"]
pub struct MICR_W<'a> {
    w: &'a mut W,
}
impl<'a> MICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MICR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
#[doc = "MCR"]
pub type MCR_A = MCMP4_A;
#[doc = "Reader of field `MCR`"]
pub type MCR_R = crate::R<bool, MCMP4_A>;
#[doc = "Write proxy for field `MCR`"]
pub struct MCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(MCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(MCMP4_A::UPDATED)
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
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&mut self) -> MCMP4_W {
        MCMP4_W { w: self }
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&mut self) -> MCMP3_W {
        MCMP3_W { w: self }
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&mut self) -> MCMP2_W {
        MCMP2_W { w: self }
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&mut self) -> MCMP1_W {
        MCMP1_W { w: self }
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&mut self) -> MREP_W {
        MREP_W { w: self }
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&mut self) -> MPER_W {
        MPER_W { w: self }
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&mut self) -> MDIER_W {
        MDIER_W { w: self }
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&mut self) -> MICR_W {
        MICR_W { w: self }
    }
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&mut self) -> MCR_W {
        MCR_W { w: self }
    }
}
