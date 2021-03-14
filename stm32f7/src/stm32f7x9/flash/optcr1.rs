#[doc = "Reader of register OPTCR1"]
pub type R = crate::R<u32, super::OPTCR1>;
#[doc = "Writer for register OPTCR1"]
pub type W = crate::W<u32, super::OPTCR1>;
#[doc = "Register OPTCR1 `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::OPTCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BOOT_ADD1`"]
pub type BOOT_ADD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_ADD1`"]
pub struct BOOT_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Boot base address when Boot pin =0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot base address when Boot pin =1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Boot base address when Boot pin =0"]
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W {
        BOOT_ADD0_W { w: self }
    }
    #[doc = "Bits 16:31 - Boot base address when Boot pin =1"]
    #[inline(always)]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W {
        BOOT_ADD1_W { w: self }
    }
}
