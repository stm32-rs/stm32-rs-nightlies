#[doc = "Reader of register FDCAN_TTOCN"]
pub type R = crate::R<u32, super::FDCAN_TTOCN>;
#[doc = "Writer for register FDCAN_TTOCN"]
pub type W = crate::W<u32, super::FDCAN_TTOCN>;
#[doc = "Register FDCAN_TTOCN `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTOCN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SGT`"]
pub type SGT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SGT`"]
pub struct SGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SGT_W<'a> {
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
#[doc = "Reader of field `ECS`"]
pub type ECS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECS`"]
pub struct ECS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECS_W<'a> {
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
#[doc = "Reader of field `SWP`"]
pub type SWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWP`"]
pub struct SWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_W<'a> {
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
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWS`"]
pub struct SWS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTIE`"]
pub type RTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIE`"]
pub struct RTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIE_W<'a> {
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
#[doc = "Reader of field `TMC`"]
pub type TMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMC`"]
pub struct TMC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TTIE`"]
pub type TTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTIE`"]
pub struct TTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTIE_W<'a> {
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
#[doc = "Reader of field `GCS`"]
pub type GCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCS`"]
pub struct GCS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCS_W<'a> {
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
#[doc = "Reader of field `FGP`"]
pub type FGP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FGP`"]
pub struct FGP_W<'a> {
    w: &'a mut W,
}
impl<'a> FGP_W<'a> {
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
#[doc = "Reader of field `TMG`"]
pub type TMG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TMG`"]
pub struct TMG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMG_W<'a> {
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
#[doc = "Reader of field `NIG`"]
pub type NIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIG`"]
pub struct NIG_W<'a> {
    w: &'a mut W,
}
impl<'a> NIG_W<'a> {
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
#[doc = "Reader of field `ESCN`"]
pub type ESCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESCN`"]
pub struct ESCN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCN_W<'a> {
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
#[doc = "Reader of field `LCKC`"]
pub type LCKC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCKC"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    pub fn sgt(&mut self) -> SGT_W {
        SGT_W { w: self }
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    pub fn swp(&mut self) -> SWP_W {
        SWP_W { w: self }
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W {
        SWS_W { w: self }
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    pub fn tmc(&mut self) -> TMC_W {
        TMC_W { w: self }
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TTIE_W {
        TTIE_W { w: self }
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W {
        GCS_W { w: self }
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    pub fn fgp(&mut self) -> FGP_W {
        FGP_W { w: self }
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    pub fn tmg(&mut self) -> TMG_W {
        TMG_W { w: self }
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    pub fn nig(&mut self) -> NIG_W {
        NIG_W { w: self }
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    pub fn escn(&mut self) -> ESCN_W {
        ESCN_W { w: self }
    }
}
