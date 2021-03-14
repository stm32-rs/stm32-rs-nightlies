#[doc = "Reader of register FCCAN_CCU_CWD"]
pub type R = crate::R<u32, super::FCCAN_CCU_CWD>;
#[doc = "Writer for register FCCAN_CCU_CWD"]
pub type W = crate::W<u32, super::FCCAN_CCU_CWD>;
#[doc = "Register FCCAN_CCU_CWD `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCAN_CCU_CWD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDC`"]
pub type WDC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDC`"]
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WDV`"]
pub type WDV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
}
