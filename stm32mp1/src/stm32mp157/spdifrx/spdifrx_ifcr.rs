#[doc = "Reader of register SPDIFRX_IFCR"]
pub type R = crate::R<u32, super::SPDIFRX_IFCR>;
#[doc = "Writer for register SPDIFRX_IFCR"]
pub type W = crate::W<u32, super::SPDIFRX_IFCR>;
#[doc = "Register SPDIFRX_IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPDIFRX_IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PERRCF`"]
pub struct PERRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRCF_W<'a> {
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
#[doc = "Write proxy for field `OVRCF`"]
pub struct OVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRCF_W<'a> {
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
#[doc = "Write proxy for field `SBDCF`"]
pub struct SBDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SBDCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `SYNCDCF`"]
pub struct SYNCDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 2 - PERRCF"]
    #[inline(always)]
    pub fn perrcf(&mut self) -> PERRCF_W {
        PERRCF_W { w: self }
    }
    #[doc = "Bit 3 - OVRCF"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W {
        OVRCF_W { w: self }
    }
    #[doc = "Bit 4 - SBDCF"]
    #[inline(always)]
    pub fn sbdcf(&mut self) -> SBDCF_W {
        SBDCF_W { w: self }
    }
    #[doc = "Bit 5 - SYNCDCF"]
    #[inline(always)]
    pub fn syncdcf(&mut self) -> SYNCDCF_W {
        SYNCDCF_W { w: self }
    }
}
