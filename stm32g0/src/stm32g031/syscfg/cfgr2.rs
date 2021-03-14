#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKUP_LOCK`"]
pub type LOCKUP_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKUP_LOCK`"]
pub struct LOCKUP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_LOCK_W<'a> {
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
#[doc = "Reader of field `SRAM_PARITY_LOCK`"]
pub type SRAM_PARITY_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_PARITY_LOCK`"]
pub struct SRAM_PARITY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PARITY_LOCK_W<'a> {
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
#[doc = "Reader of field `PVD_LOCK`"]
pub type PVD_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVD_LOCK`"]
pub struct PVD_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_LOCK_W<'a> {
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
#[doc = "Reader of field `ECC_LOCK`"]
pub type ECC_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC_LOCK`"]
pub struct ECC_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_LOCK_W<'a> {
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
#[doc = "Reader of field `SRAM_PEF`"]
pub type SRAM_PEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_PEF`"]
pub struct SRAM_PEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PEF_W<'a> {
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
#[doc = "Reader of field `PA1_CDEN`"]
pub type PA1_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA1_CDEN`"]
pub struct PA1_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA1_CDEN_W<'a> {
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
#[doc = "Reader of field `PA3_CDEN`"]
pub type PA3_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA3_CDEN`"]
pub struct PA3_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA3_CDEN_W<'a> {
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
#[doc = "Reader of field `PA5_CDEN`"]
pub type PA5_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA5_CDEN`"]
pub struct PA5_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA5_CDEN_W<'a> {
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
#[doc = "Reader of field `PA6_CDEN`"]
pub type PA6_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA6_CDEN`"]
pub struct PA6_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA6_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PA13_CDEN`"]
pub type PA13_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA13_CDEN`"]
pub struct PA13_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA13_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PB0_CDEN`"]
pub type PB0_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB0_CDEN`"]
pub struct PB0_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB0_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PB1_CDEN`"]
pub type PB1_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB1_CDEN`"]
pub struct PB1_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB1_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PB2_CDEN`"]
pub type PB2_CDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB2_CDEN`"]
pub struct PB2_CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PB2_CDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    pub fn pa1_cden(&self) -> PA1_CDEN_R {
        PA1_CDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    pub fn pa3_cden(&self) -> PA3_CDEN_R {
        PA3_CDEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    pub fn pa5_cden(&self) -> PA5_CDEN_R {
        PA5_CDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    pub fn pa6_cden(&self) -> PA6_CDEN_R {
        PA6_CDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    pub fn pa13_cden(&self) -> PA13_CDEN_R {
        PA13_CDEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    pub fn pb0_cden(&self) -> PB0_CDEN_R {
        PB0_CDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    pub fn pb1_cden(&self) -> PB1_CDEN_R {
        PB1_CDEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    pub fn pb2_cden(&self) -> PB2_CDEN_R {
        PB2_CDEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W {
        LOCKUP_LOCK_W { w: self }
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W {
        SRAM_PARITY_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W {
        PVD_LOCK_W { w: self }
    }
    #[doc = "Bit 3 - ECC error lock bit"]
    #[inline(always)]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W {
        ECC_LOCK_W { w: self }
    }
    #[doc = "Bit 8 - SRAM parity error flag"]
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W {
        SRAM_PEF_W { w: self }
    }
    #[doc = "Bit 16 - PA1_CDEN"]
    #[inline(always)]
    pub fn pa1_cden(&mut self) -> PA1_CDEN_W {
        PA1_CDEN_W { w: self }
    }
    #[doc = "Bit 17 - PA3_CDEN"]
    #[inline(always)]
    pub fn pa3_cden(&mut self) -> PA3_CDEN_W {
        PA3_CDEN_W { w: self }
    }
    #[doc = "Bit 18 - PA5_CDEN"]
    #[inline(always)]
    pub fn pa5_cden(&mut self) -> PA5_CDEN_W {
        PA5_CDEN_W { w: self }
    }
    #[doc = "Bit 19 - PA6_CDEN"]
    #[inline(always)]
    pub fn pa6_cden(&mut self) -> PA6_CDEN_W {
        PA6_CDEN_W { w: self }
    }
    #[doc = "Bit 20 - PA13_CDEN"]
    #[inline(always)]
    pub fn pa13_cden(&mut self) -> PA13_CDEN_W {
        PA13_CDEN_W { w: self }
    }
    #[doc = "Bit 21 - PB0_CDEN"]
    #[inline(always)]
    pub fn pb0_cden(&mut self) -> PB0_CDEN_W {
        PB0_CDEN_W { w: self }
    }
    #[doc = "Bit 22 - PB1_CDEN"]
    #[inline(always)]
    pub fn pb1_cden(&mut self) -> PB1_CDEN_W {
        PB1_CDEN_W { w: self }
    }
    #[doc = "Bit 23 - PB2_CDEN"]
    #[inline(always)]
    pub fn pb2_cden(&mut self) -> PB2_CDEN_W {
        PB2_CDEN_W { w: self }
    }
}
