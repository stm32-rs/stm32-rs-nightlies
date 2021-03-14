#[doc = "Reader of register CFG2"]
pub type R = crate::R<u32, super::CFG2>;
#[doc = "Writer for register CFG2"]
pub type W = crate::W<u32, super::CFG2>;
#[doc = "Register CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFILTDIS`"]
pub type RXFILTDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFILTDIS`"]
pub struct RXFILTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILTDIS_W<'a> {
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
#[doc = "Reader of field `RXFILT2N3`"]
pub type RXFILT2N3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFILT2N3`"]
pub struct RXFILT2N3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILT2N3_W<'a> {
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
#[doc = "Reader of field `FORCECLK`"]
pub type FORCECLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCECLK`"]
pub struct FORCECLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCECLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WUPEN`"]
pub type WUPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPEN`"]
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFILTDIS"]
    #[inline(always)]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W {
        RXFILTDIS_W { w: self }
    }
    #[doc = "Bit 1 - RXFILT2N3"]
    #[inline(always)]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W {
        RXFILT2N3_W { w: self }
    }
    #[doc = "Bit 2 - FORCECLK"]
    #[inline(always)]
    pub fn forceclk(&mut self) -> FORCECLK_W {
        FORCECLK_W { w: self }
    }
    #[doc = "Bit 3 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
}
