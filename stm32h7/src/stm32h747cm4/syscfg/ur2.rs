#[doc = "Reader of register UR2"]
pub type R = crate::R<u32, super::UR2>;
#[doc = "Writer for register UR2"]
pub type W = crate::W<u32, super::UR2>;
#[doc = "Register UR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::UR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BORH`"]
pub type BORH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BORH`"]
pub struct BORH_W<'a> {
    w: &'a mut W,
}
impl<'a> BORH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BOOT_ADD0`"]
pub type BOOT_ADD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_ADD0`"]
pub struct BOOT_ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&self) -> BORH_R {
        BORH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&mut self) -> BORH_W {
        BORH_W { w: self }
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W {
        BOOT_ADD0_W { w: self }
    }
}
