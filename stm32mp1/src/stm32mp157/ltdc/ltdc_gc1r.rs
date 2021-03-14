#[doc = "Reader of register LTDC_GC1R"]
pub type R = crate::R<u32, super::LTDC_GC1R>;
#[doc = "Reader of field `WBCH`"]
pub type WBCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `WGCH`"]
pub type WGCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `WRCH`"]
pub type WRCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRBEN`"]
pub type PRBEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
#[doc = "Reader of field `GCT`"]
pub type GCT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SHREN`"]
pub type SHREN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BCP`"]
pub type BCP_R = crate::R<bool, bool>;
#[doc = "Reader of field `BBEN`"]
pub type BBEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `LNIP`"]
pub type LNIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TP`"]
pub type TP_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPP`"]
pub type IPP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPP`"]
pub type SPP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DWP`"]
pub type DWP_R = crate::R<bool, bool>;
#[doc = "Reader of field `STREN`"]
pub type STREN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BMEN`"]
pub type BMEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - WBCH"]
    #[inline(always)]
    pub fn wbch(&self) -> WBCH_R {
        WBCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - WGCH"]
    #[inline(always)]
    pub fn wgch(&self) -> WGCH_R {
        WGCH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WRCH"]
    #[inline(always)]
    pub fn wrch(&self) -> WRCH_R {
        WRCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - PRBEN"]
    #[inline(always)]
    pub fn prben(&self) -> PRBEN_R {
        PRBEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 17:19 - GCT"]
    #[inline(always)]
    pub fn gct(&self) -> GCT_R {
        GCT_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 21 - SHREN"]
    #[inline(always)]
    pub fn shren(&self) -> SHREN_R {
        SHREN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BCP"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - BBEN"]
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LNIP"]
    #[inline(always)]
    pub fn lnip(&self) -> LNIP_R {
        LNIP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TP"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IPP"]
    #[inline(always)]
    pub fn ipp(&self) -> IPP_R {
        IPP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SPP"]
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DWP"]
    #[inline(always)]
    pub fn dwp(&self) -> DWP_R {
        DWP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - STREN"]
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
