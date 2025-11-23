///Register `IR` reader
pub type R = crate::R<IRrs>;
///Register `IR` writer
pub type W = crate::W<IRrs>;
///Field `RF0N` reader - Rx FIFO 0 New Message
pub type RF0N_R = crate::BitReader;
///Field `RF0N` writer - Rx FIFO 0 New Message
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0W` reader - Rx FIFO 0 Full
pub type RF0W_R = crate::BitReader;
///Field `RF0W` writer - Rx FIFO 0 Full
pub type RF0W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0F` reader - Rx FIFO 0 Full
pub type RF0F_R = crate::BitReader;
///Field `RF0F` writer - Rx FIFO 0 Full
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0L` reader - Rx FIFO 0 Message Lost
pub type RF0L_R = crate::BitReader;
///Field `RF0L` writer - Rx FIFO 0 Message Lost
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1N` reader - Rx FIFO 1 New Message
pub type RF1N_R = crate::BitReader;
///Field `RF1N` writer - Rx FIFO 1 New Message
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1W` reader - Rx FIFO 1 Watermark Reached
pub type RF1W_R = crate::BitReader;
///Field `RF1W` writer - Rx FIFO 1 Watermark Reached
pub type RF1W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1F` reader - Rx FIFO 1 Watermark Reached
pub type RF1F_R = crate::BitReader;
///Field `RF1F` writer - Rx FIFO 1 Watermark Reached
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1L` reader - Rx FIFO 1 Message Lost
pub type RF1L_R = crate::BitReader;
///Field `RF1L` writer - Rx FIFO 1 Message Lost
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPM` reader - High Priority Message
pub type HPM_R = crate::BitReader;
///Field `HPM` writer - High Priority Message
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - Transmission Completed
pub type TC_R = crate::BitReader;
///Field `TC` writer - Transmission Completed
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCF` reader - Transmission Cancellation Finished
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - Transmission Cancellation Finished
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEF` reader - Tx FIFO Empty
pub type TEF_R = crate::BitReader;
///Field `TEF` writer - Tx FIFO Empty
pub type TEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFN` reader - Tx Event FIFO New Entry
pub type TEFN_R = crate::BitReader;
///Field `TEFN` writer - Tx Event FIFO New Entry
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFW` reader - Tx Event FIFO Watermark Reached
pub type TEFW_R = crate::BitReader;
///Field `TEFW` writer - Tx Event FIFO Watermark Reached
pub type TEFW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFF` reader - Tx Event FIFO Full
pub type TEFF_R = crate::BitReader;
///Field `TEFF` writer - Tx Event FIFO Full
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFL` reader - Tx Event FIFO Element Lost
pub type TEFL_R = crate::BitReader;
///Field `TEFL` writer - Tx Event FIFO Element Lost
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSW` reader - Timestamp Wraparound
pub type TSW_R = crate::BitReader;
///Field `TSW` writer - Timestamp Wraparound
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAF` reader - Message RAM Access Failure
pub type MRAF_R = crate::BitReader;
///Field `MRAF` writer - Message RAM Access Failure
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOO` reader - Timeout Occurred
pub type TOO_R = crate::BitReader;
///Field `TOO` writer - Timeout Occurred
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRX` reader - Message stored to Dedicated Rx Buffer
pub type DRX_R = crate::BitReader;
///Field `DRX` writer - Message stored to Dedicated Rx Buffer
pub type DRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELO` reader - Error Logging Overflow
pub type ELO_R = crate::BitReader;
///Field `ELO` writer - Error Logging Overflow
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP` reader - Error Passive
pub type EP_R = crate::BitReader;
///Field `EP` writer - Error Passive
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EW` reader - Warning Status
pub type EW_R = crate::BitReader;
///Field `EW` writer - Warning Status
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BO` reader - Bus_Off Status
pub type BO_R = crate::BitReader;
///Field `BO` writer - Bus_Off Status
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDI` reader - Watchdog Interrupt
pub type WDI_R = crate::BitReader;
///Field `WDI` writer - Watchdog Interrupt
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub type PEA_R = crate::BitReader;
///Field `PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)
pub type PED_R = crate::BitReader;
///Field `PED` writer - Protocol Error in Data Phase (Data Bit Time is used)
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARA` reader - Access to Reserved Address
pub type ARA_R = crate::BitReader;
///Field `ARA` writer - Access to Reserved Address
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("rf0n", &self.rf0n())
            .field("rf0w", &self.rf0w())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1w", &self.rf1w())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tef", &self.tef())
            .field("tefn", &self.tefn())
            .field("tefw", &self.tefw())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("drx", &self.drx())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 New Message
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<'_, IRrs> {
        RF0N_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W<'_, IRrs> {
        RF0W_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 Full
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<'_, IRrs> {
        RF0F_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<'_, IRrs> {
        RF0L_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 New Message
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<'_, IRrs> {
        RF1N_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W<'_, IRrs> {
        RF1W_W::new(self, 5)
    }
    ///Bit 6 - Rx FIFO 1 Watermark Reached
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<'_, IRrs> {
        RF1F_W::new(self, 6)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<'_, IRrs> {
        RF1L_W::new(self, 7)
    }
    ///Bit 8 - High Priority Message
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<'_, IRrs> {
        HPM_W::new(self, 8)
    }
    ///Bit 9 - Transmission Completed
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, IRrs> {
        TC_W::new(self, 9)
    }
    ///Bit 10 - Transmission Cancellation Finished
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<'_, IRrs> {
        TCF_W::new(self, 10)
    }
    ///Bit 11 - Tx FIFO Empty
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W<'_, IRrs> {
        TEF_W::new(self, 11)
    }
    ///Bit 12 - Tx Event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<'_, IRrs> {
        TEFN_W::new(self, 12)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W<'_, IRrs> {
        TEFW_W::new(self, 13)
    }
    ///Bit 14 - Tx Event FIFO Full
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<'_, IRrs> {
        TEFF_W::new(self, 14)
    }
    ///Bit 15 - Tx Event FIFO Element Lost
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<'_, IRrs> {
        TEFL_W::new(self, 15)
    }
    ///Bit 16 - Timestamp Wraparound
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<'_, IRrs> {
        TSW_W::new(self, 16)
    }
    ///Bit 17 - Message RAM Access Failure
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<'_, IRrs> {
        MRAF_W::new(self, 17)
    }
    ///Bit 18 - Timeout Occurred
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<'_, IRrs> {
        TOO_W::new(self, 18)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W<'_, IRrs> {
        DRX_W::new(self, 19)
    }
    ///Bit 22 - Error Logging Overflow
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<'_, IRrs> {
        ELO_W::new(self, 22)
    }
    ///Bit 23 - Error Passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<'_, IRrs> {
        EP_W::new(self, 23)
    }
    ///Bit 24 - Warning Status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<'_, IRrs> {
        EW_W::new(self, 24)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<'_, IRrs> {
        BO_W::new(self, 25)
    }
    ///Bit 26 - Watchdog Interrupt
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<'_, IRrs> {
        WDI_W::new(self, 26)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W<'_, IRrs> {
        PEA_W::new(self, 27)
    }
    ///Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W<'_, IRrs> {
        PED_W::new(self, 28)
    }
    ///Bit 29 - Access to Reserved Address
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W<'_, IRrs> {
        ARA_W::new(self, 29)
    }
}
/**FDCAN Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#FDCAN1:IR)*/
pub struct IRrs;
impl crate::RegisterSpec for IRrs {
    type Ux = u32;
}
///`read()` method returns [`ir::R`](R) reader structure
impl crate::Readable for IRrs {}
///`write(|w| ..)` method takes [`ir::W`](W) writer structure
impl crate::Writable for IRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IRrs {}
