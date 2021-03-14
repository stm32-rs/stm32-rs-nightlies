#[doc = "Reader of register M4FDRL"]
pub type R = crate::R<u32, super::M4FDRL>;
#[doc = "Writer for register M4FDRL"]
pub type W = crate::W<u32, super::M4FDRL>;
#[doc = "Register M4FDRL `reset()`'s with value 0"]
impl crate::ResetValue for super::M4FDRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDATAH`"]
pub type FDATAH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FDATAH`"]
pub struct FDATAH_W<'a> {
    w: &'a mut W,
}
impl<'a> FDATAH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&mut self) -> FDATAH_W {
        FDATAH_W { w: self }
    }
}
