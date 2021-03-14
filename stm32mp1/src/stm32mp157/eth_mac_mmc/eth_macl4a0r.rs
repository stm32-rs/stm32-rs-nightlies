#[doc = "Reader of register ETH_MACL4A0R"]
pub type R = crate::R<u32, super::ETH_MACL4A0R>;
#[doc = "Writer for register ETH_MACL4A0R"]
pub type W = crate::W<u32, super::ETH_MACL4A0R>;
#[doc = "Register ETH_MACL4A0R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACL4A0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L4SP0`"]
pub type L4SP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `L4SP0`"]
pub struct L4SP0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `L4DP0`"]
pub type L4DP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `L4DP0`"]
pub struct L4DP0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&mut self) -> L4SP0_W {
        L4SP0_W { w: self }
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&mut self) -> L4DP0_W {
        L4DP0_W { w: self }
    }
}
