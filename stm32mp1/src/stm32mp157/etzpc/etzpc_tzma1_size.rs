#[doc = "Reader of register ETZPC_TZMA1_SIZE"]
pub type R = crate::R<u32, super::ETZPC_TZMA1_SIZE>;
#[doc = "Writer for register ETZPC_TZMA1_SIZE"]
pub type W = crate::W<u32, super::ETZPC_TZMA1_SIZE>;
#[doc = "Register ETZPC_TZMA1_SIZE `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::ETZPC_TZMA1_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `R0SIZE`"]
pub type R0SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `R0SIZE`"]
pub struct R0SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> R0SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&self) -> R0SIZE_R {
        R0SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&mut self) -> R0SIZE_W {
        R0SIZE_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
