#[doc = "Reader of register GICH_APR0"]
pub type R = crate::R<u32, super::GICH_APR0>;
#[doc = "Writer for register GICH_APR0"]
pub type W = crate::W<u32, super::GICH_APR0>;
#[doc = "Register GICH_APR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICH_APR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APR0`"]
pub type APR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APR0`"]
pub struct APR0_W<'a> {
    w: &'a mut W,
}
impl<'a> APR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&mut self) -> APR0_W {
        APR0_W { w: self }
    }
}
