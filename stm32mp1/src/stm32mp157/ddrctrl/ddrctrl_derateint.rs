#[doc = "Reader of register DDRCTRL_DERATEINT"]
pub type R = crate::R<u32, super::DDRCTRL_DERATEINT>;
#[doc = "Writer for register DDRCTRL_DERATEINT"]
pub type W = crate::W<u32, super::DDRCTRL_DERATEINT>;
#[doc = "Register DDRCTRL_DERATEINT `reset()`'s with value 0x0080_0000"]
impl crate::ResetValue for super::DDRCTRL_DERATEINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0000
    }
}
#[doc = "Reader of field `MR4_READ_INTERVAL`"]
pub type MR4_READ_INTERVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MR4_READ_INTERVAL`"]
pub struct MR4_READ_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MR4_READ_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    pub fn mr4_read_interval(&self) -> MR4_READ_INTERVAL_R {
        MR4_READ_INTERVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MR4_READ_INTERVAL"]
    #[inline(always)]
    pub fn mr4_read_interval(&mut self) -> MR4_READ_INTERVAL_W {
        MR4_READ_INTERVAL_W { w: self }
    }
}
