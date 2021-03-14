#[doc = "Reader of register ETH_MACTSICNR"]
pub type R = crate::R<u32, super::ETH_MACTSICNR>;
#[doc = "Writer for register ETH_MACTSICNR"]
pub type W = crate::W<u32, super::ETH_MACTSICNR>;
#[doc = "Register ETH_MACTSICNR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACTSICNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSIC`"]
pub type TSIC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSIC`"]
pub struct TSIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    pub fn tsic(&mut self) -> TSIC_W {
        TSIC_W { w: self }
    }
}
