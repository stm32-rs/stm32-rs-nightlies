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
#[doc = "Reader of field `C2STOPF`"]
pub type C2STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2STOP2F`"]
pub type C2STOP2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2SBF`"]
pub type C2SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1STOPF`"]
pub type C1STOPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1STOP2F`"]
pub type C1STOP2F_R = crate::R<bool, bool>;
#[doc = "Reader of field `C1SBF`"]
pub type C1SBF_R = crate::R<bool, bool>;
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
    #[doc = "Bit 15 - PU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ystem Stop0, 1 flag for CPU2. (All core states retained)"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ystem Stop2 flag for CPU2. (partial core states retained)"]
    #[inline(always)]
    pub fn c2stop2f(&self) -> C2STOP2F_R {
        C2STOP2F_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ystem Standby flag for CPU2. (no core states retained)"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System Stop0, 1 flag for CPU1. (All core states retained)"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - System Stop2 flag for CPU1. (partial core states retained)"]
    #[inline(always)]
    pub fn c1stop2f(&self) -> C1STOP2F_R {
        C1STOP2F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - System Standby flag for CPU1. (no core states retained)"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - lear CPU2 Stop Standby flags"]
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
