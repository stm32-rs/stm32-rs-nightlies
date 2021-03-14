#[doc = "Reader of register ETH_MACL3A10R"]
pub type R = crate::R<u32, super::ETH_MACL3A10R>;
#[doc = "Writer for register ETH_MACL3A10R"]
pub type W = crate::W<u32, super::ETH_MACL3A10R>;
#[doc = "Register ETH_MACL3A10R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACL3A10R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3A10`"]
pub type L3A10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `L3A10`"]
pub struct L3A10_W<'a> {
    w: &'a mut W,
}
impl<'a> L3A10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    pub fn l3a10(&self) -> L3A10_R {
        L3A10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    pub fn l3a10(&mut self) -> L3A10_W {
        L3A10_W { w: self }
    }
}
