///Register `WORD3` reader
pub type R = crate::R<WORD3rs>;
///Register `WORD3` writer
pub type W = crate::W<WORD3rs>;
///Field `ConfigEndDuration` reader - Duration for the Sequencer to execute the final configuration.
pub type CONFIG_END_DURATION_R = crate::FieldReader;
///Field `ConfigEndDuration` writer - Duration for the Sequencer to execute the final configuration.
pub type CONFIG_END_DURATION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TxdataReadyCheck` reader - Duration for the Sequencer to get the TxDataReady and DataPtr information in TxRxPack table.
pub type TXDATA_READY_CHECK_R = crate::FieldReader;
///Field `TxdataReadyCheck` writer - Duration for the Sequencer to get the TxDataReady and DataPtr information in TxRxPack table.
pub type TXDATA_READY_CHECK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TxdelayStart` reader - Delay added between the moment the Radio FSM is in TX mode (PA ramp up done and power present on the antenna) and the first bit transmission to the modulator.
pub type TXDELAY_START_R = crate::FieldReader;
///Field `TxdelayStart` writer - Delay added between the moment the Radio FSM is in TX mode (PA ramp up done and power present on the antenna) and the first bit transmission to the modulator.
pub type TXDELAY_START_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TxdelayEnd` reader - Delay added between the last bit transmission to the modulator and the end of transmission information for the Sequencer.
pub type TXDELAY_END_R = crate::FieldReader;
///Field `TxdelayEnd` writer - Delay added between the last bit transmission to the modulator and the end of transmission information for the Sequencer.
pub type TXDELAY_END_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TimeCaptureSel` reader - - 0: the captured time (absolute time) corresponds to the end of 1st INIT step in the sequence (InitDelay timeout event).
pub type TIME_CAPTURE_SEL_R = crate::BitReader;
///Field `TimeCaptureSel` writer - - 0: the captured time (absolute time) corresponds to the end of 1st INIT step in the sequence (InitDelay timeout event).
pub type TIME_CAPTURE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TimeCapture` reader - - 0: No capture is requested to monitor the Bluetooth LE sequence.
pub type TIME_CAPTURE_R = crate::BitReader;
///Field `TimeCapture` writer - - 0: No capture is requested to monitor the Bluetooth LE sequence.
pub type TIME_CAPTURE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Duration for the Sequencer to execute the final configuration.
    #[inline(always)]
    pub fn config_end_duration(&self) -> CONFIG_END_DURATION_R {
        CONFIG_END_DURATION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Duration for the Sequencer to get the TxDataReady and DataPtr information in TxRxPack table.
    #[inline(always)]
    pub fn txdata_ready_check(&self) -> TXDATA_READY_CHECK_R {
        TXDATA_READY_CHECK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay added between the moment the Radio FSM is in TX mode (PA ramp up done and power present on the antenna) and the first bit transmission to the modulator.
    #[inline(always)]
    pub fn txdelay_start(&self) -> TXDELAY_START_R {
        TXDELAY_START_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:29 - Delay added between the last bit transmission to the modulator and the end of transmission information for the Sequencer.
    #[inline(always)]
    pub fn txdelay_end(&self) -> TXDELAY_END_R {
        TXDELAY_END_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - - 0: the captured time (absolute time) corresponds to the end of 1st INIT step in the sequence (InitDelay timeout event).
    #[inline(always)]
    pub fn time_capture_sel(&self) -> TIME_CAPTURE_SEL_R {
        TIME_CAPTURE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - - 0: No capture is requested to monitor the Bluetooth LE sequence.
    #[inline(always)]
    pub fn time_capture(&self) -> TIME_CAPTURE_R {
        TIME_CAPTURE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD3")
            .field("config_end_duration", &self.config_end_duration())
            .field("txdata_ready_check", &self.txdata_ready_check())
            .field("txdelay_start", &self.txdelay_start())
            .field("txdelay_end", &self.txdelay_end())
            .field("time_capture_sel", &self.time_capture_sel())
            .field("time_capture", &self.time_capture())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Duration for the Sequencer to execute the final configuration.
    #[inline(always)]
    pub fn config_end_duration(&mut self) -> CONFIG_END_DURATION_W<'_, WORD3rs> {
        CONFIG_END_DURATION_W::new(self, 0)
    }
    ///Bits 8:15 - Duration for the Sequencer to get the TxDataReady and DataPtr information in TxRxPack table.
    #[inline(always)]
    pub fn txdata_ready_check(&mut self) -> TXDATA_READY_CHECK_W<'_, WORD3rs> {
        TXDATA_READY_CHECK_W::new(self, 8)
    }
    ///Bits 16:23 - Delay added between the moment the Radio FSM is in TX mode (PA ramp up done and power present on the antenna) and the first bit transmission to the modulator.
    #[inline(always)]
    pub fn txdelay_start(&mut self) -> TXDELAY_START_W<'_, WORD3rs> {
        TXDELAY_START_W::new(self, 16)
    }
    ///Bits 24:29 - Delay added between the last bit transmission to the modulator and the end of transmission information for the Sequencer.
    #[inline(always)]
    pub fn txdelay_end(&mut self) -> TXDELAY_END_W<'_, WORD3rs> {
        TXDELAY_END_W::new(self, 24)
    }
    ///Bit 30 - - 0: the captured time (absolute time) corresponds to the end of 1st INIT step in the sequence (InitDelay timeout event).
    #[inline(always)]
    pub fn time_capture_sel(&mut self) -> TIME_CAPTURE_SEL_W<'_, WORD3rs> {
        TIME_CAPTURE_SEL_W::new(self, 30)
    }
    ///Bit 31 - - 0: No capture is requested to monitor the Bluetooth LE sequence.
    #[inline(always)]
    pub fn time_capture(&mut self) -> TIME_CAPTURE_W<'_, WORD3rs> {
        TIME_CAPTURE_W::new(self, 31)
    }
}
/**WORD3 register

You can [`read`](crate::Reg::read) this register and get [`word3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#GLOBALSTATMACH:WORD3)*/
pub struct WORD3rs;
impl crate::RegisterSpec for WORD3rs {
    type Ux = u32;
}
///`read()` method returns [`word3::R`](R) reader structure
impl crate::Readable for WORD3rs {}
///`write(|w| ..)` method takes [`word3::W`](W) writer structure
impl crate::Writable for WORD3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD3 to value 0
impl crate::Resettable for WORD3rs {}
