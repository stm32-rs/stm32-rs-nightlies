#[doc = "Reader of register ECCR"]
pub type R = crate::R<u32, super::ECCR>;
#[doc = "Writer for register ECCR"]
pub type W = crate::W<u32, super::ECCR>;
#[doc = "Register ECCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ECCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_ECC`"]
pub type ADDR_ECC_R = crate::R<u32, u32>;
#[doc = "Reader of field `BK_ECC`"]
pub type BK_ECC_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSF_ECC`"]
pub type SYSF_ECC_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCIE`"]
pub type ECCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCIE`"]
pub struct ECCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCIE_W<'a> {
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
#[doc = "Reader of field `ECCC2`"]
pub type ECCC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCC2`"]
pub struct ECCC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC2_W<'a> {
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
#[doc = "Reader of field `ECCD2`"]
pub type ECCD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCD2`"]
pub struct ECCD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD2_W<'a> {
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
#[doc = "Reader of field `ECCC`"]
pub type ECCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCC`"]
pub struct ECCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC_W<'a> {
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
#[doc = "Reader of field `ECCD`"]
pub type ECCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECCD`"]
pub struct ECCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD_W<'a> {
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
    #[doc = "Bits 0:18 - ECC fail address"]
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bit 21 - BK_ECC"]
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SYSF_ECC"]
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ECCIE"]
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ECC correction"]
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECCIE"]
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W {
        ECCIE_W { w: self }
    }
    #[doc = "Bit 28 - ECC correction"]
    #[inline(always)]
    pub fn eccc2(&mut self) -> ECCC2_W {
        ECCC2_W { w: self }
    }
    #[doc = "Bit 29 - ECC2 detection"]
    #[inline(always)]
    pub fn eccd2(&mut self) -> ECCD2_W {
        ECCD2_W { w: self }
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W {
        ECCC_W { w: self }
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W {
        ECCD_W { w: self }
    }
}
