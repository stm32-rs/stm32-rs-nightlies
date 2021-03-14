#[doc = "Reader of register HWCFGR2"]
pub type R = crate::R<u32, super::HWCFGR2>;
#[doc = "Writer for register HWCFGR2"]
pub type W = crate::W<u32, super::HWCFGR2>;
#[doc = "Register HWCFGR2 `reset()`'s with value 0x0007_ffff"]
impl crate::ResetValue for super::HWCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_ffff
    }
}
#[doc = "Reader of field `EVENT_TRG`"]
pub type EVENT_TRG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EVENT_TRG`"]
pub struct EVENT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&mut self) -> EVENT_TRG_W {
        EVENT_TRG_W { w: self }
    }
}
