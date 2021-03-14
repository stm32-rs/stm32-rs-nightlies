#[doc = "Reader of register DDRPHYC_DLLGCR"]
pub type R = crate::R<u32, super::DDRPHYC_DLLGCR>;
#[doc = "Writer for register DDRPHYC_DLLGCR"]
pub type W = crate::W<u32, super::DDRPHYC_DLLGCR>;
#[doc = "Register DDRPHYC_DLLGCR `reset()`'s with value 0x0373_7000"]
impl crate::ResetValue for super::DDRPHYC_DLLGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0373_7000
    }
}
#[doc = "Reader of field `DRES`"]
pub type DRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRES`"]
pub struct DRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IPUMP`"]
pub type IPUMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPUMP`"]
pub struct IPUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> IPUMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `TESTEN`"]
pub type TESTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTEN`"]
pub struct TESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTEN_W<'a> {
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
#[doc = "Reader of field `DTC`"]
pub type DTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTC`"]
pub struct DTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `ATC`"]
pub type ATC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATC`"]
pub struct ATC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `TESTSW`"]
pub type TESTSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTSW`"]
pub struct TESTSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTSW_W<'a> {
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
#[doc = "Reader of field `MBIAS`"]
pub type MBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MBIAS`"]
pub struct MBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> MBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SBIAS2_0`"]
pub type SBIAS2_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBIAS2_0`"]
pub struct SBIAS2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIAS2_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `BPS200`"]
pub type BPS200_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BPS200`"]
pub struct BPS200_W<'a> {
    w: &'a mut W,
}
impl<'a> BPS200_W<'a> {
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
#[doc = "Reader of field `SBIAS5_3`"]
pub type SBIAS5_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBIAS5_3`"]
pub struct SBIAS5_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIAS5_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `FDTRMSL`"]
pub type FDTRMSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDTRMSL`"]
pub struct FDTRMSL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDTRMSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `LOCKDET`"]
pub type LOCKDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKDET`"]
pub struct LOCKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKDET_W<'a> {
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
#[doc = "Reader of field `DLLRSVD2`"]
pub type DLLRSVD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLLRSVD2`"]
pub struct DLLRSVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRSVD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&self) -> IPUMP_R {
        IPUMP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&self) -> ATC_R {
        ATC_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&self) -> TESTSW_R {
        TESTSW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&self) -> MBIAS_R {
        MBIAS_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&self) -> SBIAS2_0_R {
        SBIAS2_0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&self) -> BPS200_R {
        BPS200_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&self) -> SBIAS5_3_R {
        SBIAS5_3_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&self) -> FDTRMSL_R {
        FDTRMSL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&self) -> LOCKDET_R {
        LOCKDET_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&self) -> DLLRSVD2_R {
        DLLRSVD2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRES"]
    #[inline(always)]
    pub fn dres(&mut self) -> DRES_W {
        DRES_W { w: self }
    }
    #[doc = "Bits 2:4 - IPUMP"]
    #[inline(always)]
    pub fn ipump(&mut self) -> IPUMP_W {
        IPUMP_W { w: self }
    }
    #[doc = "Bit 5 - TESTEN"]
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W {
        TESTEN_W { w: self }
    }
    #[doc = "Bits 6:8 - DTC"]
    #[inline(always)]
    pub fn dtc(&mut self) -> DTC_W {
        DTC_W { w: self }
    }
    #[doc = "Bits 9:10 - ATC"]
    #[inline(always)]
    pub fn atc(&mut self) -> ATC_W {
        ATC_W { w: self }
    }
    #[doc = "Bit 11 - TESTSW"]
    #[inline(always)]
    pub fn testsw(&mut self) -> TESTSW_W {
        TESTSW_W { w: self }
    }
    #[doc = "Bits 12:19 - MBIAS"]
    #[inline(always)]
    pub fn mbias(&mut self) -> MBIAS_W {
        MBIAS_W { w: self }
    }
    #[doc = "Bits 20:22 - SBIAS2_0"]
    #[inline(always)]
    pub fn sbias2_0(&mut self) -> SBIAS2_0_W {
        SBIAS2_0_W { w: self }
    }
    #[doc = "Bit 23 - BPS200"]
    #[inline(always)]
    pub fn bps200(&mut self) -> BPS200_W {
        BPS200_W { w: self }
    }
    #[doc = "Bits 24:26 - SBIAS5_3"]
    #[inline(always)]
    pub fn sbias5_3(&mut self) -> SBIAS5_3_W {
        SBIAS5_3_W { w: self }
    }
    #[doc = "Bits 27:28 - FDTRMSL"]
    #[inline(always)]
    pub fn fdtrmsl(&mut self) -> FDTRMSL_W {
        FDTRMSL_W { w: self }
    }
    #[doc = "Bit 29 - LOCKDET"]
    #[inline(always)]
    pub fn lockdet(&mut self) -> LOCKDET_W {
        LOCKDET_W { w: self }
    }
    #[doc = "Bits 30:31 - DLLRSVD2"]
    #[inline(always)]
    pub fn dllrsvd2(&mut self) -> DLLRSVD2_W {
        DLLRSVD2_W { w: self }
    }
}
