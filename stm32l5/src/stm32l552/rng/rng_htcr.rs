#[doc = "Reader of register RNG_HTCR"]
pub type R = crate::R<u32, super::RNG_HTCR>;
#[doc = "Writer for register RNG_HTCR"]
pub type W = crate::W<u32, super::RNG_HTCR>;
#[doc = "Register RNG_HTCR `reset()`'s with value 0x000c_aa74"]
impl crate::ResetValue for super::RNG_HTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000c_aa74
    }
}
#[doc = "Reader of field `HTCFG`"]
pub type HTCFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HTCFG`"]
pub struct HTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HTCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W {
        HTCFG_W { w: self }
    }
}
