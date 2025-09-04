///Register `IR` reader
pub type R = crate::R<IRrs>;
///Register `IR` writer
pub type W = crate::W<IRrs>;
///Field `RF0N` reader - Rx FIFO 0 new message
pub type RF0N_R = crate::BitReader;
///Field `RF0N` writer - Rx FIFO 0 new message
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0F` reader - Rx FIFO 0 full
pub type RF0F_R = crate::BitReader;
///Field `RF0F` writer - Rx FIFO 0 full
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0L` reader - Rx FIFO 0 message lost
pub type RF0L_R = crate::BitReader;
///Field `RF0L` writer - Rx FIFO 0 message lost
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1N` reader - Rx FIFO 1 new message
pub type RF1N_R = crate::BitReader;
///Field `RF1N` writer - Rx FIFO 1 new message
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1F` reader - Rx FIFO 1 full
pub type RF1F_R = crate::BitReader;
///Field `RF1F` writer - Rx FIFO 1 full
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1L` reader - Rx FIFO 1 message lost
pub type RF1L_R = crate::BitReader;
///Field `RF1L` writer - Rx FIFO 1 message lost
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPM` reader - High-priority message
pub type HPM_R = crate::BitReader;
///Field `HPM` writer - High-priority message
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - Transmission completed
pub type TC_R = crate::BitReader;
///Field `TC` writer - Transmission completed
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCF` reader - Transmission cancellation finished
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - Transmission cancellation finished
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFE` reader - Tx FIFO empty
pub type TFE_R = crate::BitReader;
///Field `TFE` writer - Tx FIFO empty
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFN` reader - Tx event FIFO New Entry
pub type TEFN_R = crate::BitReader;
///Field `TEFN` writer - Tx event FIFO New Entry
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFF` reader - Tx event FIFO full
pub type TEFF_R = crate::BitReader;
///Field `TEFF` writer - Tx event FIFO full
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFL` reader - Tx event FIFO element lost
pub type TEFL_R = crate::BitReader;
///Field `TEFL` writer - Tx event FIFO element lost
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSW` reader - Timestamp wraparound
pub type TSW_R = crate::BitReader;
///Field `TSW` writer - Timestamp wraparound
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.
pub type MRAF_R = crate::BitReader;
///Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOO` reader - Timeout occurred
pub type TOO_R = crate::BitReader;
///Field `TOO` writer - Timeout occurred
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELO` reader - Error logging overflow
pub type ELO_R = crate::BitReader;
///Field `ELO` writer - Error logging overflow
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EP` reader - Error passive
pub type EP_R = crate::BitReader;
///Field `EP` writer - Error passive
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EW` reader - Warning status
pub type EW_R = crate::BitReader;
///Field `EW` writer - Warning status
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BO` reader - Bus_Off status
pub type BO_R = crate::BitReader;
///Field `BO` writer - Bus_Off status
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDI` reader - Watchdog interrupt
pub type WDI_R = crate::BitReader;
///Field `WDI` writer - Watchdog interrupt
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)
pub type PEA_R = crate::BitReader;
///Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PED` reader - Protocol error in data phase (data bit time is used)
pub type PED_R = crate::BitReader;
///Field `PED` writer - Protocol error in data phase (data bit time is used)
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARA` reader - Access to reserved address
pub type ARA_R = crate::BitReader;
///Field `ARA` writer - Access to reserved address
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("rf0n", &self.rf0n())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
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
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<IRrs> {
        RF0N_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<IRrs> {
        RF0F_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<IRrs> {
        RF0L_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<IRrs> {
        RF1N_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<IRrs> {
        RF1F_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<IRrs> {
        RF1L_W::new(self, 5)
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<IRrs> {
        HPM_W::new(self, 6)
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<IRrs> {
        TC_W::new(self, 7)
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<IRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<IRrs> {
        TFE_W::new(self, 9)
    }
    ///Bit 10 - Tx event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<IRrs> {
        TEFN_W::new(self, 10)
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<IRrs> {
        TEFF_W::new(self, 11)
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<IRrs> {
        TEFL_W::new(self, 12)
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<IRrs> {
        TSW_W::new(self, 13)
    }
    ///Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted operation Mode (see Restricted operation mode). To leave Restricted operation Mode, the Host CPU has to reset CCCR.ASM.
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<IRrs> {
        MRAF_W::new(self, 14)
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<IRrs> {
        TOO_W::new(self, 15)
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<IRrs> {
        ELO_W::new(self, 16)
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<IRrs> {
        EP_W::new(self, 17)
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<IRrs> {
        EW_W::new(self, 18)
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<IRrs> {
        BO_W::new(self, 19)
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<IRrs> {
        WDI_W::new(self, 20)
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W<IRrs> {
        PEA_W::new(self, 21)
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W<IRrs> {
        PED_W::new(self, 22)
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W<IRrs> {
        ARA_W::new(self, 23)
    }
}
/**FDCAN interrupt register

You can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#FDCAN:IR)*/
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
