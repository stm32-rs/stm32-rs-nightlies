#[doc = "Reader of register SMCR"]
pub type R = crate::R<u32, super::SMCR>;
#[doc = "Writer for register SMCR"]
pub type W = crate::W<u32, super::SMCR>;
#[doc = "Register SMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKPRWDPROT`"]
pub type BKPRWDPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKPRWDPROT`"]
pub struct BKPRWDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRWDPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BKPWDPROT`"]
pub type BKPWDPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKPWDPROT`"]
pub struct BKPWDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPWDPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAMPDPROT`"]
pub type TAMPDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMPDPROT`"]
pub struct TAMPDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPDPROT_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Tamper protection"]
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W {
        BKPRWDPROT_W { w: self }
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W {
        BKPWDPROT_W { w: self }
    }
    #[doc = "Bit 31 - Tamper protection"]
    #[inline(always)]
    pub fn tampdprot(&mut self) -> TAMPDPROT_W {
        TAMPDPROT_W { w: self }
    }
}
