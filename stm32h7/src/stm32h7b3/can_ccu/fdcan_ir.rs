#[doc = "Register `FDCAN_IR` reader"]
pub type R = crate::R<FDCAN_IRrs>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub type RF0N_R = crate::BitReader;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Full"]
pub type RF0W_R = crate::BitReader;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub type RF0F_R = crate::BitReader;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub type RF1N_R = crate::BitReader;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub type RF1W_R = crate::BitReader;
#[doc = "Field `RF1F` reader - Rx FIFO 1 Watermark Reached"]
pub type RF1F_R = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HPM_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission Completed"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TEF` reader - Tx FIFO Empty"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TEFN_R = crate::BitReader;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TEFW_R = crate::BitReader;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TEFF_R = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TEFL_R = crate::BitReader;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TSW_R = crate::BitReader;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MRAF_R = crate::BitReader;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TOO_R = crate::BitReader;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub type DRX_R = crate::BitReader;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type ELO_R = crate::BitReader;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WDI_R = crate::BitReader;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
pub type PEA_R = crate::BitReader;
#[doc = "Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)"]
pub type PED_R = crate::BitReader;
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub type ARA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "FDCAN Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_IRrs;
impl crate::RegisterSpec for FDCAN_IRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ir::R`](R) reader structure"]
impl crate::Readable for FDCAN_IRrs {}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FDCAN_IRrs {
    const RESET_VALUE: u32 = 0;
}
