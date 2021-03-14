#[doc = "Reader of register FCCAN_CCU_CCFG"]
pub type R = crate::R<u32, super::FCCAN_CCU_CCFG>;
#[doc = "Writer for register FCCAN_CCU_CCFG"]
pub type W = crate::W<u32, super::FCCAN_CCU_CCFG>;
#[doc = "Register FCCAN_CCU_CCFG `reset()`'s with value 0x04"]
impl crate::ResetValue for super::FCCAN_CCU_CCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `TQBT`"]
pub type TQBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TQBT`"]
pub struct TQBT_W<'a> {
    w: &'a mut W,
}
impl<'a> TQBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `BCC`"]
pub type BCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCC`"]
pub struct BCC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCC_W<'a> {
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
#[doc = "Reader of field `CFL`"]
pub type CFL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFL`"]
pub struct CFL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFL_W<'a> {
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
#[doc = "Reader of field `OCPM`"]
pub type OCPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCPM`"]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CDIV`"]
pub type CDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CDIV`"]
pub struct CDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TQBT"]
    #[inline(always)]
    pub fn tqbt(&mut self) -> TQBT_W {
        TQBT_W { w: self }
    }
    #[doc = "Bit 6 - BCC"]
    #[inline(always)]
    pub fn bcc(&mut self) -> BCC_W {
        BCC_W { w: self }
    }
    #[doc = "Bit 7 - CFL"]
    #[inline(always)]
    pub fn cfl(&mut self) -> CFL_W {
        CFL_W { w: self }
    }
    #[doc = "Bits 8:15 - OCPM"]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bits 16:19 - CDIV"]
    #[inline(always)]
    pub fn cdiv(&mut self) -> CDIV_W {
        CDIV_W { w: self }
    }
    #[doc = "Bit 31 - SWR"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
}
