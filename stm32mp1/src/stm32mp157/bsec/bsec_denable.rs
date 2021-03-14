#[doc = "Reader of register BSEC_DENABLE"]
pub type R = crate::R<u32, super::BSEC_DENABLE>;
#[doc = "Writer for register BSEC_DENABLE"]
pub type W = crate::W<u32, super::BSEC_DENABLE>;
#[doc = "Register BSEC_DENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::BSEC_DENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DFTEN`"]
pub type DFTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFTEN`"]
pub struct DFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTEN_W<'a> {
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
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
#[doc = "Reader of field `NIDEN`"]
pub type NIDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIDEN`"]
pub struct NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_W<'a> {
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
#[doc = "Reader of field `DEVICEEN`"]
pub type DEVICEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVICEEN`"]
pub struct DEVICEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICEEN_W<'a> {
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
#[doc = "Reader of field `HDPEN`"]
pub type HDPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDPEN`"]
pub struct HDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPEN_W<'a> {
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
#[doc = "Reader of field `SPIDEN`"]
pub type SPIDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIDEN`"]
pub struct SPIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDEN_W<'a> {
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
#[doc = "Reader of field `SPNIDEN`"]
pub type SPNIDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPNIDEN`"]
pub struct SPNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPNIDEN_W<'a> {
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
#[doc = "Reader of field `CP15SDISABLE`"]
pub type CP15SDISABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CP15SDISABLE`"]
pub struct CP15SDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CP15SDISABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `CFGSDISABLE`"]
pub type CFGSDISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFGSDISABLE`"]
pub struct CFGSDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGSDISABLE_W<'a> {
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
#[doc = "Reader of field `DBGSWENABLE`"]
pub type DBGSWENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGSWENABLE`"]
pub struct DBGSWENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSWENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&self) -> DEVICEEN_R {
        DEVICEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&self) -> CP15SDISABLE_R {
        CP15SDISABLE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&self) -> CFGSDISABLE_R {
        CFGSDISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&self) -> DBGSWENABLE_R {
        DBGSWENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&mut self) -> DFTEN_W {
        DFTEN_W { w: self }
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&mut self) -> NIDEN_W {
        NIDEN_W { w: self }
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&mut self) -> DEVICEEN_W {
        DEVICEEN_W { w: self }
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W {
        HDPEN_W { w: self }
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SPIDEN_W {
        SPIDEN_W { w: self }
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SPNIDEN_W {
        SPNIDEN_W { w: self }
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&mut self) -> CP15SDISABLE_W {
        CP15SDISABLE_W { w: self }
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&mut self) -> CFGSDISABLE_W {
        CFGSDISABLE_W { w: self }
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&mut self) -> DBGSWENABLE_W {
        DBGSWENABLE_W { w: self }
    }
}
