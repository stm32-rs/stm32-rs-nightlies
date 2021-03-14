#[doc = "Reader of register DDRCTRL_DBG1"]
pub type R = crate::R<u32, super::DDRCTRL_DBG1>;
#[doc = "Writer for register DDRCTRL_DBG1"]
pub type W = crate::W<u32, super::DDRCTRL_DBG1>;
#[doc = "Register DDRCTRL_DBG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DBG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS_DQ`"]
pub type DIS_DQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_DQ`"]
pub struct DIS_DQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DQ_W<'a> {
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
#[doc = "Reader of field `DIS_HIF`"]
pub type DIS_HIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_HIF`"]
pub struct DIS_HIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_HIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DIS_DQ"]
    #[inline(always)]
    pub fn dis_dq(&self) -> DIS_DQ_R {
        DIS_DQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIS_HIF"]
    #[inline(always)]
    pub fn dis_hif(&self) -> DIS_HIF_R {
        DIS_HIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_DQ"]
    #[inline(always)]
    pub fn dis_dq(&mut self) -> DIS_DQ_W {
        DIS_DQ_W { w: self }
    }
    #[doc = "Bit 1 - DIS_HIF"]
    #[inline(always)]
    pub fn dis_hif(&mut self) -> DIS_HIF_W {
        DIS_HIF_W { w: self }
    }
}
