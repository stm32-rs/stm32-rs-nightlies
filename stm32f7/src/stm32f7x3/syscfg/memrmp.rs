#[doc = "Reader of register MEMRMP"]
pub type R = crate::R<u32, super::MEMRMP>;
#[doc = "Writer for register MEMRMP"]
pub type W = crate::W<u32, super::MEMRMP>;
#[doc = "Register MEMRMP `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMRMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BOOT`"]
pub type MEM_BOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_BOOT`"]
pub struct MEM_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BOOT_W<'a> {
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
#[doc = "Reader of field `SWP_FMC`"]
pub type SWP_FMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWP_FMC`"]
pub struct SWP_FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_FMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Memory boot mapping"]
    #[inline(always)]
    pub fn mem_boot(&self) -> MEM_BOOT_R {
        MEM_BOOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Memory boot mapping"]
    #[inline(always)]
    pub fn mem_boot(&mut self) -> MEM_BOOT_W {
        MEM_BOOT_W { w: self }
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W {
        SWP_FMC_W { w: self }
    }
}
