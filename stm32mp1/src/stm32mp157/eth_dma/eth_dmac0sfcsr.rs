#[doc = "Reader of register ETH_DMAC0SFCSR"]
pub type R = crate::R<u32, super::ETH_DMAC0SFCSR>;
#[doc = "Writer for register ETH_DMAC0SFCSR"]
pub type W = crate::W<u32, super::ETH_DMAC0SFCSR>;
#[doc = "Register ETH_DMAC0SFCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC0SFCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ESC`"]
pub type ESC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESC`"]
pub struct ESC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ASC`"]
pub type ASC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC`"]
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RSN`"]
pub type RSN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSN`"]
pub struct RSN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W {
        ESC_W { w: self }
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&mut self) -> RSN_W {
        RSN_W { w: self }
    }
}
