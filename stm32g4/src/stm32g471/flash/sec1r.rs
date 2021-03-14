#[doc = "Reader of register SEC1R"]
pub type R = crate::R<u32, super::SEC1R>;
#[doc = "Writer for register SEC1R"]
pub type W = crate::W<u32, super::SEC1R>;
#[doc = "Register SEC1R `reset()`'s with value 0xff00_ff00"]
impl crate::ResetValue for super::SEC1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_ff00
    }
}
#[doc = "Reader of field `BOOT_LOCK`"]
pub type BOOT_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_LOCK`"]
pub struct BOOT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_LOCK_W<'a> {
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
#[doc = "Reader of field `SEC_SIZE1`"]
pub type SEC_SIZE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC_SIZE1`"]
pub struct SEC_SIZE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_SIZE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&self) -> SEC_SIZE1_R {
        SEC_SIZE1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    #[doc = "Bits 0:7 - SEC_SIZE1"]
    #[inline(always)]
    pub fn sec_size1(&mut self) -> SEC_SIZE1_W {
        SEC_SIZE1_W { w: self }
    }
}
