#[doc = "Reader of register OPTKEYR_"]
pub type R = crate::R<u32, super::OPTKEYR_>;
#[doc = "Writer for register OPTKEYR_"]
pub type W = crate::W<u32, super::OPTKEYR_>;
#[doc = "Register OPTKEYR_ `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTKEYR_ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPTKEYR`"]
pub type OPTKEYR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OPTKEYR`"]
pub struct OPTKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&self) -> OPTKEYR_R {
        OPTKEYR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W {
        OPTKEYR_W { w: self }
    }
}
