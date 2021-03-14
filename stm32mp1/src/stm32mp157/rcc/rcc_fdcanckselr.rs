#[doc = "Reader of register RCC_FDCANCKSELR"]
pub type R = crate::R<u32, super::RCC_FDCANCKSELR>;
#[doc = "Writer for register RCC_FDCANCKSELR"]
pub type W = crate::W<u32, super::RCC_FDCANCKSELR>;
#[doc = "Register RCC_FDCANCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_FDCANCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDCANSRC`"]
pub type FDCANSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDCANSRC`"]
pub struct FDCANSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    pub fn fdcansrc(&self) -> FDCANSRC_R {
        FDCANSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    pub fn fdcansrc(&mut self) -> FDCANSRC_W {
        FDCANSRC_W { w: self }
    }
}
