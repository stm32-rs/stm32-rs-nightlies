#[doc = "Reader of register MDMA_C30BNDTR"]
pub type R = crate::R<u32, super::MDMA_C30BNDTR>;
#[doc = "Writer for register MDMA_C30BNDTR"]
pub type W = crate::W<u32, super::MDMA_C30BNDTR>;
#[doc = "Register MDMA_C30BNDTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C30BNDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BNDT`"]
pub type BNDT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BNDT`"]
pub struct BNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> BNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `BRSUM`"]
pub type BRSUM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRSUM`"]
pub struct BRSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSUM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `BRDUM`"]
pub type BRDUM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRDUM`"]
pub struct BRDUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDUM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `BRC`"]
pub type BRC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRC`"]
pub struct BRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W {
        BNDT_W { w: self }
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&mut self) -> BRSUM_W {
        BRSUM_W { w: self }
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&mut self) -> BRDUM_W {
        BRDUM_W { w: self }
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W {
        BRC_W { w: self }
    }
}
