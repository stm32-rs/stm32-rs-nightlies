#[doc = "Reader of register PWR_CR3"]
pub type R = crate::R<u32, super::PWR_CR3>;
#[doc = "Writer for register PWR_CR3"]
pub type W = crate::W<u32, super::PWR_CR3>;
#[doc = "Register PWR_CR3 `reset()`'s with value 0x5000_0000"]
impl crate::ResetValue for super::PWR_CR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000_0000
    }
}
#[doc = "Reader of field `VBE`"]
pub type VBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBE`"]
pub struct VBE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBE_W<'a> {
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
#[doc = "Reader of field `VBRS`"]
pub type VBRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBRS`"]
pub struct VBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBRS_W<'a> {
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
#[doc = "Reader of field `DDRSREN`"]
pub type DDRSREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRSREN`"]
pub struct DDRSREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSREN_W<'a> {
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
#[doc = "Reader of field `DDRSRDIS`"]
pub type DDRSRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRSRDIS`"]
pub struct DDRSRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRSRDIS_W<'a> {
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
#[doc = "Reader of field `DDRRETEN`"]
pub type DDRRETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRRETEN`"]
pub struct DDRRETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRRETEN_W<'a> {
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
#[doc = "Reader of field `POPL`"]
pub type POPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POPL`"]
pub struct POPL_W<'a> {
    w: &'a mut W,
}
impl<'a> POPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `USB33DEN`"]
pub type USB33DEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB33DEN`"]
pub struct USB33DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB33DEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USB33RDY`"]
pub type USB33RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `REG18EN`"]
pub type REG18EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG18EN`"]
pub struct REG18EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG18EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `REG18RDY`"]
pub type REG18RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `REG11EN`"]
pub type REG11EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG11EN`"]
pub struct REG11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG11EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `REG11RDY`"]
pub type REG11RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&self) -> DDRSREN_R {
        DDRSREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&self) -> DDRSRDIS_R {
        DDRSRDIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&self) -> DDRRETEN_R {
        DDRRETEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&self) -> POPL_R {
        POPL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB33RDY"]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&self) -> REG18EN_R {
        REG18EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - REG18RDY"]
    #[inline(always)]
    pub fn reg18rdy(&self) -> REG18RDY_R {
        REG18RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&self) -> REG11EN_R {
        REG11EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - REG11RDY"]
    #[inline(always)]
    pub fn reg11rdy(&self) -> REG11RDY_R {
        REG11RDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - VBE"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W {
        VBE_W { w: self }
    }
    #[doc = "Bit 9 - VBRS"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W {
        VBRS_W { w: self }
    }
    #[doc = "Bit 10 - DDRSREN"]
    #[inline(always)]
    pub fn ddrsren(&mut self) -> DDRSREN_W {
        DDRSREN_W { w: self }
    }
    #[doc = "Bit 11 - DDRSRDIS"]
    #[inline(always)]
    pub fn ddrsrdis(&mut self) -> DDRSRDIS_W {
        DDRSRDIS_W { w: self }
    }
    #[doc = "Bit 12 - DDRRETEN"]
    #[inline(always)]
    pub fn ddrreten(&mut self) -> DDRRETEN_W {
        DDRRETEN_W { w: self }
    }
    #[doc = "Bits 17:21 - POPL"]
    #[inline(always)]
    pub fn popl(&mut self) -> POPL_W {
        POPL_W { w: self }
    }
    #[doc = "Bit 24 - USB33DEN"]
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W {
        USB33DEN_W { w: self }
    }
    #[doc = "Bit 28 - REG18EN"]
    #[inline(always)]
    pub fn reg18en(&mut self) -> REG18EN_W {
        REG18EN_W { w: self }
    }
    #[doc = "Bit 30 - REG11EN"]
    #[inline(always)]
    pub fn reg11en(&mut self) -> REG11EN_W {
        REG11EN_W { w: self }
    }
}
