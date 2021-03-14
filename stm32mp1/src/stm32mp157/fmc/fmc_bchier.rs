#[doc = "Reader of register FMC_BCHIER"]
pub type R = crate::R<u32, super::FMC_BCHIER>;
#[doc = "Writer for register FMC_BCHIER"]
pub type W = crate::W<u32, super::FMC_BCHIER>;
#[doc = "Register FMC_BCHIER `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_BCHIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUEIE`"]
pub type DUEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUEIE`"]
pub struct DUEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUEIE_W<'a> {
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
#[doc = "Reader of field `DERIE`"]
pub type DERIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DERIE`"]
pub struct DERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERIE_W<'a> {
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
#[doc = "Reader of field `DEFIE`"]
pub type DEFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEFIE`"]
pub struct DEFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFIE_W<'a> {
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
#[doc = "Reader of field `DSRIE`"]
pub type DSRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRIE`"]
pub struct DSRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRIE_W<'a> {
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
#[doc = "Reader of field `EPBRIE`"]
pub type EPBRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPBRIE`"]
pub struct EPBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBRIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    pub fn dueie(&self) -> DUEIE_R {
        DUEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    pub fn derie(&self) -> DERIE_R {
        DERIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    pub fn dsrie(&self) -> DSRIE_R {
        DSRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    pub fn epbrie(&self) -> EPBRIE_R {
        EPBRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    pub fn dueie(&mut self) -> DUEIE_W {
        DUEIE_W { w: self }
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    pub fn derie(&mut self) -> DERIE_W {
        DERIE_W { w: self }
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    pub fn defie(&mut self) -> DEFIE_W {
        DEFIE_W { w: self }
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    pub fn dsrie(&mut self) -> DSRIE_W {
        DSRIE_W { w: self }
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    pub fn epbrie(&mut self) -> EPBRIE_W {
        EPBRIE_W { w: self }
    }
}
