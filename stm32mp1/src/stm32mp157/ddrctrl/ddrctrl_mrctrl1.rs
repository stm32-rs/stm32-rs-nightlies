#[doc = "Reader of register DDRCTRL_MRCTRL1"]
pub type R = crate::R<u32, super::DDRCTRL_MRCTRL1>;
#[doc = "Writer for register DDRCTRL_MRCTRL1"]
pub type W = crate::W<u32, super::DDRCTRL_MRCTRL1>;
#[doc = "Register DDRCTRL_MRCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_MRCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MR_DATA`"]
pub type MR_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MR_DATA`"]
pub struct MR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    pub fn mr_data(&self) -> MR_DATA_R {
        MR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    pub fn mr_data(&mut self) -> MR_DATA_W {
        MR_DATA_W { w: self }
    }
}
