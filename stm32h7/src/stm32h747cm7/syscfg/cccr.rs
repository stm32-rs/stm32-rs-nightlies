#[doc = "Reader of register CCCR"]
pub type R = crate::R<u32, super::CCCR>;
#[doc = "Writer for register CCCR"]
pub type W = crate::W<u32, super::CCCR>;
#[doc = "Register CCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NCC`"]
pub type NCC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NCC`"]
pub struct NCC_W<'a> {
    w: &'a mut W,
}
impl<'a> NCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PCC`"]
pub type PCC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCC`"]
pub struct PCC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&self) -> NCC_R {
        NCC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&self) -> PCC_R {
        PCC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - NMOS compensation code"]
    #[inline(always)]
    pub fn ncc(&mut self) -> NCC_W {
        NCC_W { w: self }
    }
    #[doc = "Bits 4:7 - PMOS compensation code"]
    #[inline(always)]
    pub fn pcc(&mut self) -> PCC_W {
        PCC_W { w: self }
    }
}
