#[doc = "Reader of register ETH_MACL4A1R"]
pub type R = crate::R<u32, super::ETH_MACL4A1R>;
#[doc = "Writer for register ETH_MACL4A1R"]
pub type W = crate::W<u32, super::ETH_MACL4A1R>;
#[doc = "Register ETH_MACL4A1R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACL4A1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L4SP1`"]
pub type L4SP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `L4SP1`"]
pub struct L4SP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `L4DP1`"]
pub type L4DP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `L4DP1`"]
pub struct L4DP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&mut self) -> L4SP1_W {
        L4SP1_W { w: self }
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&mut self) -> L4DP1_W {
        L4DP1_W { w: self }
    }
}
