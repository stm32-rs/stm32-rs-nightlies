#[doc = "Reader of register TTIE"]
pub type R = crate::R<u32, super::TTIE>;
#[doc = "Writer for register TTIE"]
pub type W = crate::W<u32, super::TTIE>;
#[doc = "Register TTIE `reset()`'s with value 0"]
impl crate::ResetValue for super::TTIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBCE`"]
pub type SBCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBCE`"]
pub struct SBCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBCE_W<'a> {
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
#[doc = "Reader of field `SMCE`"]
pub type SMCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMCE`"]
pub struct SMCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCE_W<'a> {
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
#[doc = "Reader of field `CSME`"]
pub type CSME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSME`"]
pub struct CSME_W<'a> {
    w: &'a mut W,
}
impl<'a> CSME_W<'a> {
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
#[doc = "Reader of field `SOGE`"]
pub type SOGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOGE`"]
pub struct SOGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOGE_W<'a> {
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
#[doc = "Reader of field `RTMIE`"]
pub type RTMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMIE`"]
pub struct RTMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIE_W<'a> {
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
#[doc = "Reader of field `TTMIE`"]
pub type TTMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTMIE`"]
pub struct TTMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMIE_W<'a> {
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
#[doc = "Reader of field `SWEE`"]
pub type SWEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEE`"]
pub struct SWEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEE_W<'a> {
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
#[doc = "Reader of field `GTWE`"]
pub type GTWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTWE`"]
pub struct GTWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTWE_W<'a> {
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
#[doc = "Reader of field `GTDE`"]
pub type GTDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTDE`"]
pub struct GTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTDE_W<'a> {
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
#[doc = "Reader of field `GTEE`"]
pub type GTEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTEE`"]
pub struct GTEE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTEE_W<'a> {
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
#[doc = "Reader of field `TXUE`"]
pub type TXUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUE`"]
pub struct TXUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUE_W<'a> {
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
#[doc = "Reader of field `TXOE`"]
pub type TXOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOE`"]
pub struct TXOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOE_W<'a> {
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
#[doc = "Reader of field `SE1E`"]
pub type SE1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE1E`"]
pub struct SE1E_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1E_W<'a> {
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
#[doc = "Reader of field `SE2E`"]
pub type SE2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE2E`"]
pub struct SE2E_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ELCE`"]
pub type ELCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELCE`"]
pub struct ELCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ELCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `IWTGE`"]
pub type IWTGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWTGE`"]
pub struct IWTGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `WTE`"]
pub type WTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTE`"]
pub struct WTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AWE`"]
pub type AWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AWE`"]
pub struct AWE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CERE`"]
pub type CERE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERE`"]
pub struct CERE_W<'a> {
    w: &'a mut W,
}
impl<'a> CERE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&self) -> IWTGE_R {
        IWTGE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&mut self) -> SBCE_W {
        SBCE_W { w: self }
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&mut self) -> SMCE_W {
        SMCE_W { w: self }
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&mut self) -> CSME_W {
        CSME_W { w: self }
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&mut self) -> SOGE_W {
        SOGE_W { w: self }
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&mut self) -> RTMIE_W {
        RTMIE_W { w: self }
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&mut self) -> TTMIE_W {
        TTMIE_W { w: self }
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&mut self) -> SWEE_W {
        SWEE_W { w: self }
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&mut self) -> GTWE_W {
        GTWE_W { w: self }
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&mut self) -> GTDE_W {
        GTDE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&mut self) -> GTEE_W {
        GTEE_W { w: self }
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&mut self) -> TXUE_W {
        TXUE_W { w: self }
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&mut self) -> TXOE_W {
        TXOE_W { w: self }
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&mut self) -> SE1E_W {
        SE1E_W { w: self }
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&mut self) -> SE2E_W {
        SE2E_W { w: self }
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&mut self) -> ELCE_W {
        ELCE_W { w: self }
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&mut self) -> IWTGE_W {
        IWTGE_W { w: self }
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W {
        WTE_W { w: self }
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&mut self) -> AWE_W {
        AWE_W { w: self }
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&mut self) -> CERE_W {
        CERE_W { w: self }
    }
}
