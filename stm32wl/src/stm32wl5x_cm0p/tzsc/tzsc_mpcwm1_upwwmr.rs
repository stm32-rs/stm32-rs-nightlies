#[doc = "Reader of register TZSC_MPCWM1_UPWWMR"]
pub type R = crate::R<u32, super::TZSC_MPCWM1_UPWWMR>;
#[doc = "Writer for register TZSC_MPCWM1_UPWWMR"]
pub type W = crate::W<u32, super::TZSC_MPCWM1_UPWWMR>;
#[doc = "Register TZSC_MPCWM1_UPWWMR `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::TZSC_MPCWM1_UPWWMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `LGTH`"]
pub type LGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LGTH`"]
pub struct LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2"]
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2"]
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W {
        LGTH_W { w: self }
    }
}
