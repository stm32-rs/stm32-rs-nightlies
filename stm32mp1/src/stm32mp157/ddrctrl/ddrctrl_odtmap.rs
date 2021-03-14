#[doc = "Reader of register DDRCTRL_ODTMAP"]
pub type R = crate::R<u32, super::DDRCTRL_ODTMAP>;
#[doc = "Writer for register DDRCTRL_ODTMAP"]
pub type W = crate::W<u32, super::DDRCTRL_ODTMAP>;
#[doc = "Register DDRCTRL_ODTMAP `reset()`'s with value 0x11"]
impl crate::ResetValue for super::DDRCTRL_ODTMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x11
    }
}
#[doc = "Reader of field `RANK0_WR_ODT`"]
pub type RANK0_WR_ODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANK0_WR_ODT`"]
pub struct RANK0_WR_ODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANK0_WR_ODT_W<'a> {
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
#[doc = "Reader of field `RANK0_RD_ODT`"]
pub type RANK0_RD_ODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANK0_RD_ODT`"]
pub struct RANK0_RD_ODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANK0_RD_ODT_W<'a> {
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
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    pub fn rank0_wr_odt(&self) -> RANK0_WR_ODT_R {
        RANK0_WR_ODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    pub fn rank0_rd_odt(&self) -> RANK0_RD_ODT_R {
        RANK0_RD_ODT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    pub fn rank0_wr_odt(&mut self) -> RANK0_WR_ODT_W {
        RANK0_WR_ODT_W { w: self }
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    pub fn rank0_rd_odt(&mut self) -> RANK0_RD_ODT_W {
        RANK0_RD_ODT_W { w: self }
    }
}
