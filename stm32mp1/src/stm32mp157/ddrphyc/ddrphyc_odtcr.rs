#[doc = "Reader of register DDRPHYC_ODTCR"]
pub type R = crate::R<u32, super::DDRPHYC_ODTCR>;
#[doc = "Writer for register DDRPHYC_ODTCR"]
pub type W = crate::W<u32, super::DDRPHYC_ODTCR>;
#[doc = "Register DDRPHYC_ODTCR `reset()`'s with value 0x8421_0000"]
impl crate::ResetValue for super::DDRPHYC_ODTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8421_0000
    }
}
#[doc = "Reader of field `RDODT`"]
pub type RDODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDODT`"]
pub struct RDODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT_W<'a> {
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
#[doc = "Reader of field `WRODT`"]
pub type WRODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRODT`"]
pub struct WRODT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&self) -> RDODT_R {
        RDODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&self) -> WRODT_R {
        WRODT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&mut self) -> RDODT_W {
        RDODT_W { w: self }
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&mut self) -> WRODT_W {
        WRODT_W { w: self }
    }
}
