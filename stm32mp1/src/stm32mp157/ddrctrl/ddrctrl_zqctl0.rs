#[doc = "Reader of register DDRCTRL_ZQCTL0"]
pub type R = crate::R<u32, super::DDRCTRL_ZQCTL0>;
#[doc = "Writer for register DDRCTRL_ZQCTL0"]
pub type W = crate::W<u32, super::DDRCTRL_ZQCTL0>;
#[doc = "Register DDRCTRL_ZQCTL0 `reset()`'s with value 0x0200_0040"]
impl crate::ResetValue for super::DDRCTRL_ZQCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0040
    }
}
#[doc = "Reader of field `T_ZQ_SHORT_NOP`"]
pub type T_ZQ_SHORT_NOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_ZQ_SHORT_NOP`"]
pub struct T_ZQ_SHORT_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_SHORT_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `T_ZQ_LONG_NOP`"]
pub type T_ZQ_LONG_NOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_ZQ_LONG_NOP`"]
pub struct T_ZQ_LONG_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_LONG_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ZQ_RESISTOR_SHARED`"]
pub type ZQ_RESISTOR_SHARED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZQ_RESISTOR_SHARED`"]
pub struct ZQ_RESISTOR_SHARED_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQ_RESISTOR_SHARED_W<'a> {
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
#[doc = "Reader of field `DIS_SRX_ZQCL`"]
pub type DIS_SRX_ZQCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_SRX_ZQCL`"]
pub struct DIS_SRX_ZQCL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_SRX_ZQCL_W<'a> {
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
#[doc = "Reader of field `DIS_AUTO_ZQ`"]
pub type DIS_AUTO_ZQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_AUTO_ZQ`"]
pub struct DIS_AUTO_ZQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_ZQ_W<'a> {
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
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&self) -> T_ZQ_SHORT_NOP_R {
        T_ZQ_SHORT_NOP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&self) -> T_ZQ_LONG_NOP_R {
        T_ZQ_LONG_NOP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&self) -> ZQ_RESISTOR_SHARED_R {
        ZQ_RESISTOR_SHARED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&self) -> DIS_SRX_ZQCL_R {
        DIS_SRX_ZQCL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&self) -> DIS_AUTO_ZQ_R {
        DIS_AUTO_ZQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&mut self) -> T_ZQ_SHORT_NOP_W {
        T_ZQ_SHORT_NOP_W { w: self }
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&mut self) -> T_ZQ_LONG_NOP_W {
        T_ZQ_LONG_NOP_W { w: self }
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&mut self) -> ZQ_RESISTOR_SHARED_W {
        ZQ_RESISTOR_SHARED_W { w: self }
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&mut self) -> DIS_SRX_ZQCL_W {
        DIS_SRX_ZQCL_W { w: self }
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&mut self) -> DIS_AUTO_ZQ_W {
        DIS_AUTO_ZQ_W { w: self }
    }
}
