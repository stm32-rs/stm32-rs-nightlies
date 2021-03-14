#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GECCDEBWIE`"]
pub type GECCDEBWIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GECCDEBWIE`"]
pub struct GECCDEBWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GECCDEBWIE_W<'a> {
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
#[doc = "Reader of field `GECCDEIE`"]
pub type GECCDEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GECCDEIE`"]
pub struct GECCDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GECCDEIE_W<'a> {
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
#[doc = "Reader of field `GECCSEIE`"]
pub type GECCSEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GECCSEIE`"]
pub struct GECCSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GECCSEIE_W<'a> {
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
#[doc = "Reader of field `GIE`"]
pub type GIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GIE`"]
pub struct GIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIE_W<'a> {
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
impl R {
    #[doc = "Bit 3 - Global ECC double error on byte write interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&self) -> GECCDEBWIE_R {
        GECCDEBWIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&self) -> GECCDEIE_R {
        GECCDEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie(&self) -> GECCSEIE_R {
        GECCSEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Global ECC double error on byte write interrupt enable"]
    #[inline(always)]
    pub fn geccdebwie(&mut self) -> GECCDEBWIE_W {
        GECCDEBWIE_W { w: self }
    }
    #[doc = "Bit 2 - Global ECC double error interrupt enable"]
    #[inline(always)]
    pub fn geccdeie(&mut self) -> GECCDEIE_W {
        GECCDEIE_W { w: self }
    }
    #[doc = "Bit 1 - Global ECC single error interrupt enable"]
    #[inline(always)]
    pub fn geccseie(&mut self) -> GECCSEIE_W {
        GECCSEIE_W { w: self }
    }
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W {
        GIE_W { w: self }
    }
}
