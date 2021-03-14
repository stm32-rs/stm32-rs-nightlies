#[doc = "Reader of register PWRCR"]
pub type R = crate::R<u32, super::PWRCR>;
#[doc = "Writer for register PWRCR"]
pub type W = crate::W<u32, super::PWRCR>;
#[doc = "Register PWRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ODEN`"]
pub type ODEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODEN`"]
pub struct ODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W {
        ODEN_W { w: self }
    }
}
