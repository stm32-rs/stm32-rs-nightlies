#[doc = "Reader of register EXTSCR"]
pub type R = crate::R<u32, super::EXTSCR>;
#[doc = "Writer for register EXTSCR"]
pub type W = crate::W<u32, super::EXTSCR>;
#[doc = "Register EXTSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C2DS`"]
pub type C2DS_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1DS`"]
pub type C1DS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRPF`"]
pub type CRPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2STOPF`"]
pub type C2STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2SBF`"]
pub type C2SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1STOPF`"]
pub type C1STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1SBF`"]
pub type C1SBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCRPF`"]
pub struct CCRPF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRPF_W<'a> {
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
#[doc = "Write proxy for field `C2CSSF`"]
pub struct C2CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> C2CSSF_W<'a> {
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
#[doc = "Write proxy for field `C1CSSF`"]
pub struct C1CSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CSSF_W<'a> {
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
    #[doc = "Bit 15 - CPU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Critical Radio system phase"]
    #[inline(always)]
    pub fn crpf(&self) -> CRPF_R {
        CRPF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - System Stop flag for CPU2"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System Standby flag for CPU2"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - System Stop flag for CPU1"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - System Standby flag for CPU1"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Clear Critical Radio system phase"]
    #[inline(always)]
    pub fn ccrpf(&mut self) -> CCRPF_W {
        CCRPF_W { w: self }
    }
    #[doc = "Bit 1 - Clear CPU2 Stop Standby flags"]
    #[inline(always)]
    pub fn c2cssf(&mut self) -> C2CSSF_W {
        C2CSSF_W { w: self }
    }
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    pub fn c1cssf(&mut self) -> C1CSSF_W {
        C1CSSF_W { w: self }
    }
}
