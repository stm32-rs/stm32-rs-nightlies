#[doc = "Reader of register DDRCTRL_DBGCMD"]
pub type R = crate::R<u32, super::DDRCTRL_DBGCMD>;
#[doc = "Writer for register DDRCTRL_DBGCMD"]
pub type W = crate::W<u32, super::DDRCTRL_DBGCMD>;
#[doc = "Register DDRCTRL_DBGCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DBGCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RANK0_REFRESH`"]
pub type RANK0_REFRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANK0_REFRESH`"]
pub struct RANK0_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RANK0_REFRESH_W<'a> {
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
#[doc = "Reader of field `ZQ_CALIB_SHORT`"]
pub type ZQ_CALIB_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZQ_CALIB_SHORT`"]
pub struct ZQ_CALIB_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQ_CALIB_SHORT_W<'a> {
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
#[doc = "Reader of field `CTRLUPD`"]
pub type CTRLUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRLUPD`"]
pub struct CTRLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLUPD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    pub fn rank0_refresh(&self) -> RANK0_REFRESH_R {
        RANK0_REFRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    pub fn zq_calib_short(&self) -> ZQ_CALIB_SHORT_R {
        ZQ_CALIB_SHORT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    pub fn ctrlupd(&self) -> CTRLUPD_R {
        CTRLUPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_REFRESH"]
    #[inline(always)]
    pub fn rank0_refresh(&mut self) -> RANK0_REFRESH_W {
        RANK0_REFRESH_W { w: self }
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT"]
    #[inline(always)]
    pub fn zq_calib_short(&mut self) -> ZQ_CALIB_SHORT_W {
        ZQ_CALIB_SHORT_W { w: self }
    }
    #[doc = "Bit 5 - CTRLUPD"]
    #[inline(always)]
    pub fn ctrlupd(&mut self) -> CTRLUPD_W {
        CTRLUPD_W { w: self }
    }
}
