#[doc = "Reader of register SECBOOTADD0R"]
pub type R = crate::R<u32, super::SECBOOTADD0R>;
#[doc = "Writer for register SECBOOTADD0R"]
pub type W = crate::W<u32, super::SECBOOTADD0R>;
#[doc = "Register SECBOOTADD0R `reset()`'s with value 0"]
impl crate::ResetValue for super::SECBOOTADD0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `SECBOOTADD0`"]
pub struct SECBOOTADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBOOTADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    #[doc = "Bits 7:31 - SECBOOTADD0"]
    #[inline(always)]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W {
        SECBOOTADD0_W { w: self }
    }
}
