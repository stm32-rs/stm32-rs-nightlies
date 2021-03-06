#[doc = "Reader of register BOOT4_CURR"]
pub type R = crate::R<u32, super::BOOT4_CURR>;
#[doc = "Writer for register BOOT4_CURR"]
pub type W = crate::W<u32, super::BOOT4_CURR>;
#[doc = "Register BOOT4_CURR `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOT4_CURR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOOT_CM4_ADD1`"]
pub type BOOT_CM4_ADD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_CM4_ADD1`"]
pub struct BOOT_CM4_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM4_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BOOT_CM4_ADD0`"]
pub type BOOT_CM4_ADD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_CM4_ADD0`"]
pub struct BOOT_CM4_ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM4_ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Arm Cortex-M4 boot address 1"]
    #[inline(always)]
    pub fn boot_cm4_add1(&self) -> BOOT_CM4_ADD1_R {
        BOOT_CM4_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Arm Cortex-M4 boot address 0"]
    #[inline(always)]
    pub fn boot_cm4_add0(&self) -> BOOT_CM4_ADD0_R {
        BOOT_CM4_ADD0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Arm Cortex-M4 boot address 1"]
    #[inline(always)]
    pub fn boot_cm4_add1(&mut self) -> BOOT_CM4_ADD1_W {
        BOOT_CM4_ADD1_W { w: self }
    }
    #[doc = "Bits 0:15 - Arm Cortex-M4 boot address 0"]
    #[inline(always)]
    pub fn boot_cm4_add0(&mut self) -> BOOT_CM4_ADD0_W {
        BOOT_CM4_ADD0_W { w: self }
    }
}
