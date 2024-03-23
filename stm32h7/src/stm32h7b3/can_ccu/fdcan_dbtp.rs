#[doc = "Register `FDCAN_DBTP` reader"]
pub type R = crate::R<FDCAN_DBTPrs>;
#[doc = "Field `DSJW` reader - Synchronization Jump Width"]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG1` reader - Data time segment after sample point"]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DBRP` reader - Data BIt Rate Prescaler"]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `TDC` reader - Transceiver Delay Compensation"]
pub type TDC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_DBTPrs;
impl crate::RegisterSpec for FDCAN_DBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_dbtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_DBTPrs {}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
