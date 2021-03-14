#[doc = "Reader of register ARR"]
pub type R = crate::R<u32, super::ARR>;
#[doc = "Writer for register ARR"]
pub type W = crate::W<u32, super::ARR>;
#[doc = "Register ARR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::ARR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ARR_bit0`"]
pub type ARR_BIT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR_bit0`"]
pub struct ARR_BIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_BIT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ARR_bit0"]
    #[inline(always)]
    pub fn arr_bit0(&self) -> ARR_BIT0_R {
        ARR_BIT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR_bit0"]
    #[inline(always)]
    pub fn arr_bit0(&mut self) -> ARR_BIT0_W {
        ARR_BIT0_W { w: self }
    }
}
