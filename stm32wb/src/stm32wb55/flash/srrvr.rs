#[doc = "Reader of register SRRVR"]
pub type R = crate::R<u32, super::SRRVR>;
#[doc = "Writer for register SRRVR"]
pub type W = crate::W<u32, super::SRRVR>;
#[doc = "Register SRRVR `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::SRRVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `SBRV`"]
pub type SBRV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SBRV`"]
pub struct SBRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `SBRSA`"]
pub type SBRSA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBRSA`"]
pub struct SBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SBRSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `BRSD`"]
pub type BRSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRSD`"]
pub struct BRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSD_W<'a> {
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
#[doc = "Reader of field `SNBRSA`"]
pub type SNBRSA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SNBRSA`"]
pub struct SNBRSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SNBRSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `C2OPT`"]
pub type C2OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C2OPT`"]
pub struct C2OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `NBRSD`"]
pub type NBRSD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NBRSD`"]
pub struct NBRSD_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRSD_W<'a> {
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
impl R {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&mut self) -> SBRV_W {
        SBRV_W { w: self }
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&mut self) -> SBRSA_W {
        SBRSA_W { w: self }
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&mut self) -> BRSD_W {
        BRSD_W { w: self }
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&mut self) -> SNBRSA_W {
        SNBRSA_W { w: self }
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&mut self) -> C2OPT_W {
        C2OPT_W { w: self }
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&mut self) -> NBRSD_W {
        NBRSD_W { w: self }
    }
}
