#[doc = "Reader of register ETH_MACTSEACR"]
pub type R = crate::R<u32, super::ETH_MACTSEACR>;
#[doc = "Writer for register ETH_MACTSEACR"]
pub type W = crate::W<u32, super::ETH_MACTSEACR>;
#[doc = "Register ETH_MACTSEACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACTSEACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSTEAC`"]
pub type OSTEAC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OSTEAC`"]
pub struct OSTEAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTEAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&mut self) -> OSTEAC_W {
        OSTEAC_W { w: self }
    }
}
