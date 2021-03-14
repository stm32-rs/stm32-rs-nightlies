#[doc = "Reader of register BOOT7_PRGR"]
pub type R = crate::R<u32, super::BOOT7_PRGR>;
#[doc = "Writer for register BOOT7_PRGR"]
pub type W = crate::W<u32, super::BOOT7_PRGR>;
#[doc = "Register BOOT7_PRGR `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOT7_PRGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOOT_CM7_ADD1`"]
pub type BOOT_CM7_ADD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_CM7_ADD1`"]
pub struct BOOT_CM7_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM7_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BOOT_CM7_ADD0`"]
pub type BOOT_CM7_ADD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_CM7_ADD0`"]
pub struct BOOT_CM7_ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM7_ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Arm Cortex-M7 boot address 1 configuration"]
    #[inline(always)]
    pub fn boot_cm7_add1(&self) -> BOOT_CM7_ADD1_R {
        BOOT_CM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Arm Cortex-M7 boot address 0 configuration"]
    #[inline(always)]
    pub fn boot_cm7_add0(&self) -> BOOT_CM7_ADD0_R {
        BOOT_CM7_ADD0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Arm Cortex-M7 boot address 1 configuration"]
    #[inline(always)]
    pub fn boot_cm7_add1(&mut self) -> BOOT_CM7_ADD1_W {
        BOOT_CM7_ADD1_W { w: self }
    }
    #[doc = "Bits 0:15 - Arm Cortex-M7 boot address 0 configuration"]
    #[inline(always)]
    pub fn boot_cm7_add0(&mut self) -> BOOT_CM7_ADD0_W {
        BOOT_CM7_ADD0_W { w: self }
    }
}
