#[doc = "Reader of register MACPPSTTSR"]
pub type R = crate::R<u32, super::MACPPSTTSR>;
#[doc = "Writer for register MACPPSTTSR"]
pub type W = crate::W<u32, super::MACPPSTTSR>;
#[doc = "Register MACPPSTTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPPSTTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSTRH0`"]
pub type TSTRH0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSTRH0`"]
pub struct TSTRH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstrh0(&mut self) -> TSTRH0_W {
        TSTRH0_W { w: self }
    }
}
