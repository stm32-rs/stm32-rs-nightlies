#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Writer for register FIFO"]
pub type W = crate::W<u32, super::FIFO>;
#[doc = "Register FIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIF0Data`"]
pub type FIF0DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIF0Data`"]
pub struct FIF0DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIF0DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FIF0Data"]
    #[inline(always)]
    pub fn fif0data(&self) -> FIF0DATA_R {
        FIF0DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIF0Data"]
    #[inline(always)]
    pub fn fif0data(&mut self) -> FIF0DATA_W {
        FIF0DATA_W { w: self }
    }
}
