#[doc = "Reader of register ISR1"]
pub type R = crate::R<u32, super::ISR1>;
#[doc = "Writer for register ISR1"]
pub type W = crate::W<u32, super::ISR1>;
#[doc = "Register ISR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPRXE`"]
pub type GPRXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPRXE`"]
pub struct GPRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `GPRDE`"]
pub type GPRDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPRDE`"]
pub struct GPRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `GPTXE`"]
pub type GPTXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTXE`"]
pub struct GPTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `GPWRE`"]
pub type GPWRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPWRE`"]
pub struct GPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `GCWRE`"]
pub type GCWRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCWRE`"]
pub struct GCWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPWRE`"]
pub type LPWRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPWRE`"]
pub struct LPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EOTPE`"]
pub type EOTPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTPE`"]
pub struct EOTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PSE`"]
pub type PSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSE`"]
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
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
#[doc = "Reader of field `CRCE`"]
pub type CRCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCE`"]
pub struct CRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCE_W<'a> {
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
#[doc = "Reader of field `ECCME`"]
pub type ECCME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCME`"]
pub struct ECCME_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCME_W<'a> {
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
#[doc = "Reader of field `ECCSE`"]
pub type ECCSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCSE`"]
pub struct ECCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSE_W<'a> {
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
#[doc = "Reader of field `TOLPRX`"]
pub type TOLPRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOLPRX`"]
pub struct TOLPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TOLPRX_W<'a> {
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
#[doc = "Reader of field `TOHSTX`"]
pub type TOHSTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOHSTX`"]
pub struct TOHSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> TOHSTX_W<'a> {
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
    #[doc = "Bit 12 - Generic payload receive error"]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic payload read error"]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic payload transmit error"]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic payload write error"]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic command write error"]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LTDC payload write error"]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EoTp error"]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet size error"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Generic payload receive error"]
    #[inline(always)]
    pub fn gprxe(&mut self) -> GPRXE_W {
        GPRXE_W { w: self }
    }
    #[doc = "Bit 11 - Generic payload read error"]
    #[inline(always)]
    pub fn gprde(&mut self) -> GPRDE_W {
        GPRDE_W { w: self }
    }
    #[doc = "Bit 10 - Generic payload transmit error"]
    #[inline(always)]
    pub fn gptxe(&mut self) -> GPTXE_W {
        GPTXE_W { w: self }
    }
    #[doc = "Bit 9 - Generic payload write error"]
    #[inline(always)]
    pub fn gpwre(&mut self) -> GPWRE_W {
        GPWRE_W { w: self }
    }
    #[doc = "Bit 8 - Generic command write error"]
    #[inline(always)]
    pub fn gcwre(&mut self) -> GCWRE_W {
        GCWRE_W { w: self }
    }
    #[doc = "Bit 7 - LTDC payload write error"]
    #[inline(always)]
    pub fn lpwre(&mut self) -> LPWRE_W {
        LPWRE_W { w: self }
    }
    #[doc = "Bit 6 - EoTp error"]
    #[inline(always)]
    pub fn eotpe(&mut self) -> EOTPE_W {
        EOTPE_W { w: self }
    }
    #[doc = "Bit 5 - Packet size error"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Bit 4 - CRC error"]
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W {
        CRCE_W { w: self }
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccme(&mut self) -> ECCME_W {
        ECCME_W { w: self }
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccse(&mut self) -> ECCSE_W {
        ECCSE_W { w: self }
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprx(&mut self) -> TOLPRX_W {
        TOLPRX_W { w: self }
    }
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstx(&mut self) -> TOHSTX_W {
        TOHSTX_W { w: self }
    }
}
