#[doc = "Reader of register DDRCTRL_INIT1"]
pub type R = crate::R<u32, super::DDRCTRL_INIT1>;
#[doc = "Writer for register DDRCTRL_INIT1"]
pub type W = crate::W<u32, super::DDRCTRL_INIT1>;
#[doc = "Register DDRCTRL_INIT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_INIT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE_OCD_X32`"]
pub type PRE_OCD_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_OCD_X32`"]
pub struct PRE_OCD_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_OCD_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DRAM_RSTN_X1024`"]
pub type DRAM_RSTN_X1024_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DRAM_RSTN_X1024`"]
pub struct DRAM_RSTN_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> DRAM_RSTN_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    pub fn pre_ocd_x32(&self) -> PRE_OCD_X32_R {
        PRE_OCD_X32_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    pub fn dram_rstn_x1024(&self) -> DRAM_RSTN_X1024_R {
        DRAM_RSTN_X1024_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    pub fn pre_ocd_x32(&mut self) -> PRE_OCD_X32_W {
        PRE_OCD_X32_W { w: self }
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    pub fn dram_rstn_x1024(&mut self) -> DRAM_RSTN_X1024_W {
        DRAM_RSTN_X1024_W { w: self }
    }
}
