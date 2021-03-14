#[doc = "Reader of register WKUPCR"]
pub type R = crate::R<u32, super::WKUPCR>;
#[doc = "Writer for register WKUPCR"]
pub type W = crate::W<u32, super::WKUPCR>;
#[doc = "Register WKUPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WKUPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WKUPC`"]
pub type WKUPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WKUPC`"]
pub struct WKUPC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&self) -> WKUPC_R {
        WKUPC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&mut self) -> WKUPC_W {
        WKUPC_W { w: self }
    }
}
