#[doc = "Reader of register IWDG_PR"]
pub type R = crate::R<u32, super::IWDG_PR>;
#[doc = "Writer for register IWDG_PR"]
pub type W = crate::W<u32, super::IWDG_PR>;
#[doc = "Register IWDG_PR `reset()`'s with value 0x07"]
impl crate::ResetValue for super::IWDG_PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PR"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
