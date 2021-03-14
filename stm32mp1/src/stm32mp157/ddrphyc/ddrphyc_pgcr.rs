#[doc = "Reader of register DDRPHYC_PGCR"]
pub type R = crate::R<u32, super::DDRPHYC_PGCR>;
#[doc = "Writer for register DDRPHYC_PGCR"]
pub type W = crate::W<u32, super::DDRPHYC_PGCR>;
#[doc = "Register DDRPHYC_PGCR `reset()`'s with value 0x01bc_2e04"]
impl crate::ResetValue for super::DDRPHYC_PGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01bc_2e04
    }
}
#[doc = "Reader of field `ITMDMD`"]
pub type ITMDMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITMDMD`"]
pub struct ITMDMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMDMD_W<'a> {
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
#[doc = "Reader of field `DQSCFG`"]
pub type DQSCFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSCFG`"]
pub struct DQSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSCFG_W<'a> {
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
#[doc = "Reader of field `DFTCMP`"]
pub type DFTCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFTCMP`"]
pub struct DFTCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTCMP_W<'a> {
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
#[doc = "Reader of field `DFTLMT`"]
pub type DFTLMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFTLMT`"]
pub struct DFTLMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTLMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DTOSEL`"]
pub type DTOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTOSEL`"]
pub struct DTOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `CKEN`"]
pub type CKEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKEN`"]
pub struct CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `CKDV`"]
pub type CKDV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKDV`"]
pub struct CKDV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CKINV`"]
pub type CKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKINV`"]
pub struct CKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKINV_W<'a> {
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
#[doc = "Reader of field `IOLB`"]
pub type IOLB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOLB`"]
pub struct IOLB_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLB_W<'a> {
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
#[doc = "Reader of field `IODDRM`"]
pub type IODDRM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IODDRM`"]
pub struct IODDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IODDRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RANKEN`"]
pub type RANKEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RANKEN`"]
pub struct RANKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ZKSEL`"]
pub type ZKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ZKSEL`"]
pub struct ZKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PDDISDX`"]
pub type PDDISDX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDDISDX`"]
pub struct PDDISDX_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDISDX_W<'a> {
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
#[doc = "Reader of field `RFSHDT`"]
pub type RFSHDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFSHDT`"]
pub struct RFSHDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFSHDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "Reader of field `LBDQSS`"]
pub type LBDQSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBDQSS`"]
pub struct LBDQSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDQSS_W<'a> {
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
#[doc = "Reader of field `LBGDQS`"]
pub type LBGDQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBGDQS`"]
pub struct LBGDQS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBGDQS_W<'a> {
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
#[doc = "Reader of field `LBMODE`"]
pub type LBMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBMODE`"]
pub struct LBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBMODE_W<'a> {
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
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&self) -> ITMDMD_R {
        ITMDMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&self) -> DQSCFG_R {
        DQSCFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&self) -> DFTCMP_R {
        DFTCMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&self) -> DFTLMT_R {
        DFTLMT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&self) -> DTOSEL_R {
        DTOSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&self) -> CKDV_R {
        CKDV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&self) -> CKINV_R {
        CKINV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&self) -> IOLB_R {
        IOLB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&self) -> IODDRM_R {
        IODDRM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&self) -> RANKEN_R {
        RANKEN_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&self) -> ZKSEL_R {
        ZKSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&self) -> PDDISDX_R {
        PDDISDX_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&self) -> RFSHDT_R {
        RFSHDT_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&self) -> LBDQSS_R {
        LBDQSS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&self) -> LBGDQS_R {
        LBGDQS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&self) -> LBMODE_R {
        LBMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITMDMD"]
    #[inline(always)]
    pub fn itmdmd(&mut self) -> ITMDMD_W {
        ITMDMD_W { w: self }
    }
    #[doc = "Bit 1 - DQSCFG"]
    #[inline(always)]
    pub fn dqscfg(&mut self) -> DQSCFG_W {
        DQSCFG_W { w: self }
    }
    #[doc = "Bit 2 - DFTCMP"]
    #[inline(always)]
    pub fn dftcmp(&mut self) -> DFTCMP_W {
        DFTCMP_W { w: self }
    }
    #[doc = "Bits 3:4 - DFTLMT"]
    #[inline(always)]
    pub fn dftlmt(&mut self) -> DFTLMT_W {
        DFTLMT_W { w: self }
    }
    #[doc = "Bits 5:8 - DTOSEL"]
    #[inline(always)]
    pub fn dtosel(&mut self) -> DTOSEL_W {
        DTOSEL_W { w: self }
    }
    #[doc = "Bits 9:11 - CKEN"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W { w: self }
    }
    #[doc = "Bits 12:13 - CKDV"]
    #[inline(always)]
    pub fn ckdv(&mut self) -> CKDV_W {
        CKDV_W { w: self }
    }
    #[doc = "Bit 14 - CKINV"]
    #[inline(always)]
    pub fn ckinv(&mut self) -> CKINV_W {
        CKINV_W { w: self }
    }
    #[doc = "Bit 15 - IOLB"]
    #[inline(always)]
    pub fn iolb(&mut self) -> IOLB_W {
        IOLB_W { w: self }
    }
    #[doc = "Bits 16:17 - IODDRM"]
    #[inline(always)]
    pub fn ioddrm(&mut self) -> IODDRM_W {
        IODDRM_W { w: self }
    }
    #[doc = "Bits 18:21 - RANKEN"]
    #[inline(always)]
    pub fn ranken(&mut self) -> RANKEN_W {
        RANKEN_W { w: self }
    }
    #[doc = "Bits 22:23 - ZKSEL"]
    #[inline(always)]
    pub fn zksel(&mut self) -> ZKSEL_W {
        ZKSEL_W { w: self }
    }
    #[doc = "Bit 24 - PDDISDX"]
    #[inline(always)]
    pub fn pddisdx(&mut self) -> PDDISDX_W {
        PDDISDX_W { w: self }
    }
    #[doc = "Bits 25:28 - RFSHDT"]
    #[inline(always)]
    pub fn rfshdt(&mut self) -> RFSHDT_W {
        RFSHDT_W { w: self }
    }
    #[doc = "Bit 29 - LBDQSS"]
    #[inline(always)]
    pub fn lbdqss(&mut self) -> LBDQSS_W {
        LBDQSS_W { w: self }
    }
    #[doc = "Bit 30 - LBGDQS"]
    #[inline(always)]
    pub fn lbgdqs(&mut self) -> LBGDQS_W {
        LBGDQS_W { w: self }
    }
    #[doc = "Bit 31 - LBMODE"]
    #[inline(always)]
    pub fn lbmode(&mut self) -> LBMODE_W {
        LBMODE_W { w: self }
    }
}
