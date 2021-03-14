#[doc = "Reader of register RCC_ETHCKSELR"]
pub type R = crate::R<u32, super::RCC_ETHCKSELR>;
#[doc = "Writer for register RCC_ETHCKSELR"]
pub type W = crate::W<u32, super::RCC_ETHCKSELR>;
#[doc = "Register RCC_ETHCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_ETHCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETHSRC`"]
pub type ETHSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETHSRC`"]
pub struct ETHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ETHPTPDIV`"]
pub type ETHPTPDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETHPTPDIV`"]
pub struct ETHPTPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHPTPDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&self) -> ETHSRC_R {
        ETHSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&self) -> ETHPTPDIV_R {
        ETHPTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&mut self) -> ETHSRC_W {
        ETHSRC_W { w: self }
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&mut self) -> ETHPTPDIV_W {
        ETHPTPDIV_W { w: self }
    }
}
