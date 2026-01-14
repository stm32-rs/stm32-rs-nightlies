///Register `IE` reader
pub type R = crate::R<IErs>;
///Register `IE` writer
pub type W = crate::W<IErs>;
///Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable
pub type RF0NE_R = crate::BitReader;
///Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0WE` reader - Rx FIFO 0 watermark reached interrupt enable
pub type RF0WE_R = crate::BitReader;
///Field `RF0WE` writer - Rx FIFO 0 watermark reached interrupt enable
pub type RF0WE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0FE` reader - Rx FIFO 0 full interrupt enable
pub type RF0FE_R = crate::BitReader;
///Field `RF0FE` writer - Rx FIFO 0 full interrupt enable
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable
pub type RF0LE_R = crate::BitReader;
///Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable
pub type RF1NE_R = crate::BitReader;
///Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1WE` reader - Rx FIFO 1 watermark reached interrupt enable
pub type RF1WE_R = crate::BitReader;
///Field `RF1WE` writer - Rx FIFO 1 watermark reached interrupt enable
pub type RF1WE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1FE` reader - Rx FIFO 1 full interrupt enable
pub type RF1FE_R = crate::BitReader;
///Field `RF1FE` writer - Rx FIFO 1 full interrupt enable
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable
pub type RF1LE_R = crate::BitReader;
///Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPME` reader - High priority message interrupt enable
pub type HPME_R = crate::BitReader;
///Field `HPME` writer - High priority message interrupt enable
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCE` reader - Transmission completed interrupt enable
pub type TCE_R = crate::BitReader;
///Field `TCE` writer - Transmission completed interrupt enable
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCFE` reader - Transmission cancellation finished interrupt enable
pub type TCFE_R = crate::BitReader;
///Field `TCFE` writer - Transmission cancellation finished interrupt enable
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFEE` reader - Tx FIFO empty interrupt enable
pub type TFEE_R = crate::BitReader;
///Field `TFEE` writer - Tx FIFO empty interrupt enable
pub type TFEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFNE` reader - Tx event FIFO new entry interrupt enable
pub type TEFNE_R = crate::BitReader;
///Field `TEFNE` writer - Tx event FIFO new entry interrupt enable
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFWE` reader - Tx event FIFO watermark reached interrupt enable
pub type TEFWE_R = crate::BitReader;
///Field `TEFWE` writer - Tx event FIFO watermark reached interrupt enable
pub type TEFWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFFE` reader - Tx event FIFO full interrupt enable
pub type TEFFE_R = crate::BitReader;
///Field `TEFFE` writer - Tx event FIFO full interrupt enable
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEFLE` reader - Tx event FIFO element lost interrupt enable
pub type TEFLE_R = crate::BitReader;
///Field `TEFLE` writer - Tx event FIFO element lost interrupt enable
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSWE` reader - Timestamp wraparound interrupt enable
pub type TSWE_R = crate::BitReader;
///Field `TSWE` writer - Timestamp wraparound interrupt enable
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MRAFE` reader - Message RAM access failure interrupt enable
pub type MRAFE_R = crate::BitReader;
///Field `MRAFE` writer - Message RAM access failure interrupt enable
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOOE` reader - Timeout occurred interrupt enable
pub type TOOE_R = crate::BitReader;
///Field `TOOE` writer - Timeout occurred interrupt enable
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRXE` reader - Message stored to dedicated Rx buffer interrupt enable
pub type DRXE_R = crate::BitReader;
///Field `DRXE` writer - Message stored to dedicated Rx buffer interrupt enable
pub type DRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELOE` reader - Error logging overflow interrupt enable
pub type ELOE_R = crate::BitReader;
///Field `ELOE` writer - Error logging overflow interrupt enable
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPE` reader - Error passive interrupt enable
pub type EPE_R = crate::BitReader;
///Field `EPE` writer - Error passive interrupt enable
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWE` reader - Warning status interrupt enable
pub type EWE_R = crate::BitReader;
///Field `EWE` writer - Warning status interrupt enable
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOE` reader - Bus_Off status
pub type BOE_R = crate::BitReader;
///Field `BOE` writer - Bus_Off status
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDIE` reader - Watchdog interrupt enable
pub type WDIE_R = crate::BitReader;
///Field `WDIE` writer - Watchdog interrupt enable
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEAE` reader - Protocol error in Arbitration phase enable
pub type PEAE_R = crate::BitReader;
///Field `PEAE` writer - Protocol error in Arbitration phase enable
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEDE` reader - Protocol error in data phase enable
pub type PEDE_R = crate::BitReader;
///Field `PEDE` writer - Protocol error in data phase enable
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAE` reader - Access to Reserved address enable
pub type ARAE_R = crate::BitReader;
///Field `ARAE` writer - Access to Reserved address enable
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 watermark reached interrupt enable
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 watermark reached interrupt enable
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx event FIFO watermark reached interrupt enable
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to dedicated Rx buffer interrupt enable
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off status
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol error in Arbitration phase enable
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved address enable
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0we", &self.rf0we())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1we", &self.rf1we())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("tefwe", &self.tefwe())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("drxe", &self.drxe())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W<'_, IErs> {
        RF0NE_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 watermark reached interrupt enable
    #[inline(always)]
    pub fn rf0we(&mut self) -> RF0WE_W<'_, IErs> {
        RF0WE_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W<'_, IErs> {
        RF0FE_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W<'_, IErs> {
        RF0LE_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W<'_, IErs> {
        RF1NE_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 watermark reached interrupt enable
    #[inline(always)]
    pub fn rf1we(&mut self) -> RF1WE_W<'_, IErs> {
        RF1WE_W::new(self, 5)
    }
    ///Bit 6 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W<'_, IErs> {
        RF1FE_W::new(self, 6)
    }
    ///Bit 7 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W<'_, IErs> {
        RF1LE_W::new(self, 7)
    }
    ///Bit 8 - High priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W<'_, IErs> {
        HPME_W::new(self, 8)
    }
    ///Bit 9 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W<'_, IErs> {
        TCE_W::new(self, 9)
    }
    ///Bit 10 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W<'_, IErs> {
        TCFE_W::new(self, 10)
    }
    ///Bit 11 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W<'_, IErs> {
        TFEE_W::new(self, 11)
    }
    ///Bit 12 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W<'_, IErs> {
        TEFNE_W::new(self, 12)
    }
    ///Bit 13 - Tx event FIFO watermark reached interrupt enable
    #[inline(always)]
    pub fn tefwe(&mut self) -> TEFWE_W<'_, IErs> {
        TEFWE_W::new(self, 13)
    }
    ///Bit 14 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W<'_, IErs> {
        TEFFE_W::new(self, 14)
    }
    ///Bit 15 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W<'_, IErs> {
        TEFLE_W::new(self, 15)
    }
    ///Bit 16 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W<'_, IErs> {
        TSWE_W::new(self, 16)
    }
    ///Bit 17 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W<'_, IErs> {
        MRAFE_W::new(self, 17)
    }
    ///Bit 18 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W<'_, IErs> {
        TOOE_W::new(self, 18)
    }
    ///Bit 19 - Message stored to dedicated Rx buffer interrupt enable
    #[inline(always)]
    pub fn drxe(&mut self) -> DRXE_W<'_, IErs> {
        DRXE_W::new(self, 19)
    }
    ///Bit 22 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W<'_, IErs> {
        ELOE_W::new(self, 22)
    }
    ///Bit 23 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W<'_, IErs> {
        EPE_W::new(self, 23)
    }
    ///Bit 24 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W<'_, IErs> {
        EWE_W::new(self, 24)
    }
    ///Bit 25 - Bus_Off status
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W<'_, IErs> {
        BOE_W::new(self, 25)
    }
    ///Bit 26 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W<'_, IErs> {
        WDIE_W::new(self, 26)
    }
    ///Bit 27 - Protocol error in Arbitration phase enable
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W<'_, IErs> {
        PEAE_W::new(self, 27)
    }
    ///Bit 28 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W<'_, IErs> {
        PEDE_W::new(self, 28)
    }
    ///Bit 29 - Access to Reserved address enable
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W<'_, IErs> {
        ARAE_W::new(self, 29)
    }
}
/**FDCAN interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FDCAN1:IE)*/
pub struct IErs;
impl crate::RegisterSpec for IErs {
    type Ux = u32;
}
///`read()` method returns [`ie::R`](R) reader structure
impl crate::Readable for IErs {}
///`write(|w| ..)` method takes [`ie::W`](W) writer structure
impl crate::Writable for IErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IErs {}
