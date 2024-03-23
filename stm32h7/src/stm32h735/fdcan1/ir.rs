#[doc = "Register `IR` reader"]
pub type R = crate::R<IRrs>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IRrs>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub type RF0N_R = crate::BitReader;
#[doc = "Field `RF0N` writer - Rx FIFO 0 New Message"]
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Full"]
pub type RF0W_R = crate::BitReader;
#[doc = "Field `RF0W` writer - Rx FIFO 0 Full"]
pub type RF0W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub type RF0F_R = crate::BitReader;
#[doc = "Field `RF0F` writer - Rx FIFO 0 Full"]
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub type RF1N_R = crate::BitReader;
#[doc = "Field `RF1N` writer - Rx FIFO 1 New Message"]
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub type RF1W_R = crate::BitReader;
#[doc = "Field `RF1W` writer - Rx FIFO 1 Watermark Reached"]
pub type RF1W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 Watermark Reached"]
pub type RF1F_R = crate::BitReader;
#[doc = "Field `RF1F` writer - Rx FIFO 1 Watermark Reached"]
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader;
#[doc = "Field `RF1L` writer - Rx FIFO 1 Message Lost"]
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HPM_R = crate::BitReader;
#[doc = "Field `HPM` writer - High Priority Message"]
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmission Completed"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Transmission Completed"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TCF` writer - Transmission Cancellation Finished"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEF` reader - Tx FIFO Empty"]
pub type TEF_R = crate::BitReader;
#[doc = "Field `TEF` writer - Tx FIFO Empty"]
pub type TEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TEFN_R = crate::BitReader;
#[doc = "Field `TEFN` writer - Tx Event FIFO New Entry"]
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TEFW_R = crate::BitReader;
#[doc = "Field `TEFW` writer - Tx Event FIFO Watermark Reached"]
pub type TEFW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TEFF_R = crate::BitReader;
#[doc = "Field `TEFF` writer - Tx Event FIFO Full"]
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TEFL_R = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost"]
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TSW_R = crate::BitReader;
#[doc = "Field `TSW` writer - Timestamp Wraparound"]
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MRAF_R = crate::BitReader;
#[doc = "Field `MRAF` writer - Message RAM Access Failure"]
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TOO_R = crate::BitReader;
#[doc = "Field `TOO` writer - Timeout Occurred"]
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub type DRX_R = crate::BitReader;
#[doc = "Field `DRX` writer - Message stored to Dedicated Rx Buffer"]
pub type DRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type ELO_R = crate::BitReader;
#[doc = "Field `ELO` writer - Error Logging Overflow"]
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EP` writer - Error Passive"]
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `EW` writer - Warning Status"]
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WDI_R = crate::BitReader;
#[doc = "Field `WDI` writer - Watchdog Interrupt"]
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
pub type PEA_R = crate::BitReader;
#[doc = "Field `PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)"]
pub type PED_R = crate::BitReader;
#[doc = "Field `PED` writer - Protocol Error in Data Phase (Data Bit Time is used)"]
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub type ARA_R = crate::BitReader;
#[doc = "Field `ARA` writer - Access to Reserved Address"]
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<IRrs> {
        RF0N_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> RF0W_W<IRrs> {
        RF0W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<IRrs> {
        RF0F_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<IRrs> {
        RF0L_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<IRrs> {
        RF1N_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> RF1W_W<IRrs> {
        RF1W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<IRrs> {
        RF1F_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> RF1L_W<IRrs> {
        RF1L_W::new(self, 7)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<IRrs> {
        HPM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<IRrs> {
        TC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<IRrs> {
        TCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<IRrs> {
        TEF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<IRrs> {
        TEFN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TEFW_W<IRrs> {
        TEFW_W::new(self, 13)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<IRrs> {
        TEFF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<IRrs> {
        TEFL_W::new(self, 15)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<IRrs> {
        TSW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<IRrs> {
        MRAF_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<IRrs> {
        TOO_W::new(self, 18)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DRX_W<IRrs> {
        DRX_W::new(self, 19)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<IRrs> {
        ELO_W::new(self, 22)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<IRrs> {
        EP_W::new(self, 23)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<IRrs> {
        EW_W::new(self, 24)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<IRrs> {
        BO_W::new(self, 25)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<IRrs> {
        WDI_W::new(self, 26)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<IRrs> {
        PEA_W::new(self, 27)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<IRrs> {
        PED_W::new(self, 28)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<IRrs> {
        ARA_W::new(self, 29)
    }
}
#[doc = "FDCAN Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IRrs {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IRrs {
    const RESET_VALUE: u32 = 0;
}
