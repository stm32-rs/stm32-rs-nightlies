#[doc = "Reader of register DDRPHYC_DCR"]
pub type R = crate::R<u32, super::DDRPHYC_DCR>;
#[doc = "Writer for register DDRPHYC_DCR"]
pub type W = crate::W<u32, super::DDRPHYC_DCR>;
#[doc = "Register DDRPHYC_DCR `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::DDRPHYC_DCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Reader of field `DDRMD`"]
pub type DDRMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDRMD`"]
pub struct DDRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DDR8BNK`"]
pub type DDR8BNK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR8BNK`"]
pub struct DDR8BNK_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR8BNK_W<'a> {
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
#[doc = "Reader of field `PDQ`"]
pub type PDQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDQ`"]
pub struct PDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PDQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MPRDQ`"]
pub type MPRDQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPRDQ`"]
pub struct MPRDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPRDQ_W<'a> {
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
#[doc = "Reader of field `DDRTYPE`"]
pub type DDRTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDRTYPE`"]
pub struct DDRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `NOSRA`"]
pub type NOSRA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOSRA`"]
pub struct NOSRA_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSRA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DDR2T`"]
pub type DDR2T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR2T`"]
pub struct DDR2T_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR2T_W<'a> {
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
#[doc = "Reader of field `UDIMM`"]
pub type UDIMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDIMM`"]
pub struct UDIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RDIMM`"]
pub type RDIMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDIMM`"]
pub struct RDIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIMM_W<'a> {
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
#[doc = "Reader of field `TPD`"]
pub type TPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPD`"]
pub struct TPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&self) -> DDRMD_R {
        DDRMD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&self) -> DDR8BNK_R {
        DDR8BNK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&self) -> PDQ_R {
        PDQ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&self) -> MPRDQ_R {
        MPRDQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&self) -> DDRTYPE_R {
        DDRTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&self) -> NOSRA_R {
        NOSRA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&self) -> DDR2T_R {
        DDR2T_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&self) -> UDIMM_R {
        UDIMM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&mut self) -> DDRMD_W {
        DDRMD_W { w: self }
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&mut self) -> DDR8BNK_W {
        DDR8BNK_W { w: self }
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&mut self) -> PDQ_W {
        PDQ_W { w: self }
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&mut self) -> MPRDQ_W {
        MPRDQ_W { w: self }
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&mut self) -> DDRTYPE_W {
        DDRTYPE_W { w: self }
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&mut self) -> NOSRA_W {
        NOSRA_W { w: self }
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&mut self) -> DDR2T_W {
        DDR2T_W { w: self }
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&mut self) -> UDIMM_W {
        UDIMM_W { w: self }
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&mut self) -> RDIMM_W {
        RDIMM_W { w: self }
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W {
        TPD_W { w: self }
    }
}
