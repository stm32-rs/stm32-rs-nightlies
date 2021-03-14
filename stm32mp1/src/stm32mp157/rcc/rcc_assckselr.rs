#[doc = "Reader of register RCC_ASSCKSELR"]
pub type R = crate::R<u32, super::RCC_ASSCKSELR>;
#[doc = "Writer for register RCC_ASSCKSELR"]
pub type W = crate::W<u32, super::RCC_ASSCKSELR>;
#[doc = "Register RCC_ASSCKSELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_ASSCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `AXISSRC`"]
pub type AXISSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AXISSRC`"]
pub struct AXISSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `AXISSRCRDY`"]
pub type AXISSRCRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXISSRCRDY"]
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&mut self) -> AXISSRC_W {
        AXISSRC_W { w: self }
    }
}
