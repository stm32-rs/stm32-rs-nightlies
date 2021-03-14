#[doc = "Reader of register DDRCTRL_CRCPARCTL0"]
pub type R = crate::R<u32, super::DDRCTRL_CRCPARCTL0>;
#[doc = "Writer for register DDRCTRL_CRCPARCTL0"]
pub type W = crate::W<u32, super::DDRCTRL_CRCPARCTL0>;
#[doc = "Register DDRCTRL_CRCPARCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_CRCPARCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DFI_ALERT_ERR_INT_EN`"]
pub type DFI_ALERT_ERR_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_ALERT_ERR_INT_EN`"]
pub struct DFI_ALERT_ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_INT_EN_W<'a> {
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
#[doc = "Reader of field `DFI_ALERT_ERR_INT_CLR`"]
pub type DFI_ALERT_ERR_INT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_ALERT_ERR_INT_CLR`"]
pub struct DFI_ALERT_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_INT_CLR_W<'a> {
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
#[doc = "Reader of field `DFI_ALERT_ERR_CNT_CLR`"]
pub type DFI_ALERT_ERR_CNT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_ALERT_ERR_CNT_CLR`"]
pub struct DFI_ALERT_ERR_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_ALERT_ERR_CNT_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&self) -> DFI_ALERT_ERR_INT_EN_R {
        DFI_ALERT_ERR_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&self) -> DFI_ALERT_ERR_INT_CLR_R {
        DFI_ALERT_ERR_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&self) -> DFI_ALERT_ERR_CNT_CLR_R {
        DFI_ALERT_ERR_CNT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_ALERT_ERR_INT_EN"]
    #[inline(always)]
    pub fn dfi_alert_err_int_en(&mut self) -> DFI_ALERT_ERR_INT_EN_W {
        DFI_ALERT_ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - DFI_ALERT_ERR_INT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_int_clr(&mut self) -> DFI_ALERT_ERR_INT_CLR_W {
        DFI_ALERT_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - DFI_ALERT_ERR_CNT_CLR"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt_clr(&mut self) -> DFI_ALERT_ERR_CNT_CLR_W {
        DFI_ALERT_ERR_CNT_CLR_W { w: self }
    }
}
