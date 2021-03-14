#[doc = "Reader of register R3STARTADDR"]
pub type R = crate::R<u32, super::R3STARTADDR>;
#[doc = "Writer for register R3STARTADDR"]
pub type W = crate::W<u32, super::R3STARTADDR>;
#[doc = "Register R3STARTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::R3STARTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGx_START_ADDR`"]
pub type REGX_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGx_START_ADDR`"]
pub struct REGX_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Region AXI start address"]
    #[inline(always)]
    pub fn regx_start_addr(&self) -> REGX_START_ADDR_R {
        REGX_START_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AXI start address"]
    #[inline(always)]
    pub fn regx_start_addr(&mut self) -> REGX_START_ADDR_W {
        REGX_START_ADDR_W { w: self }
    }
}
