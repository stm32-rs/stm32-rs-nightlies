///Register `RX_INDICATOR` reader
pub type R = crate::R<RX_INDICATORrs>;
///Field `RSSI_LEVEL_ON_SYNC` reader - RSSI level captured at the end of the SYNC word detection of the received packet.
pub type RSSI_LEVEL_ON_SYNC_R = crate::FieldReader<u16>;
///Field `RSSI_LEVEL_RUN` reader - Continuous level of the output of the measured RSSI value
pub type RSSI_LEVEL_RUN_R = crate::FieldReader<u16>;
///Field `AGC_WORD` reader - AGC word of the received packet.
pub type AGC_WORD_R = crate::FieldReader;
///Field `ANT_SELECT` reader - Currently selected antenna
pub type ANT_SELECT_R = crate::BitReader;
impl R {
    ///Bits 0:8 - RSSI level captured at the end of the SYNC word detection of the received packet.
    #[inline(always)]
    pub fn rssi_level_on_sync(&self) -> RSSI_LEVEL_ON_SYNC_R {
        RSSI_LEVEL_ON_SYNC_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 12:20 - Continuous level of the output of the measured RSSI value
    #[inline(always)]
    pub fn rssi_level_run(&self) -> RSSI_LEVEL_RUN_R {
        RSSI_LEVEL_RUN_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    ///Bits 24:27 - AGC word of the received packet.
    #[inline(always)]
    pub fn agc_word(&self) -> AGC_WORD_R {
        AGC_WORD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - Currently selected antenna
    #[inline(always)]
    pub fn ant_select(&self) -> ANT_SELECT_R {
        ANT_SELECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_INDICATOR")
            .field("rssi_level_on_sync", &self.rssi_level_on_sync())
            .field("rssi_level_run", &self.rssi_level_run())
            .field("agc_word", &self.agc_word())
            .field("ant_select", &self.ant_select())
            .finish()
    }
}
/**RX_INDICATOR register

You can [`read`](crate::Reg::read) this register and get [`rx_indicator::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_INDICATOR)*/
pub struct RX_INDICATORrs;
impl crate::RegisterSpec for RX_INDICATORrs {
    type Ux = u32;
}
///`read()` method returns [`rx_indicator::R`](R) reader structure
impl crate::Readable for RX_INDICATORrs {}
///`reset()` method sets RX_INDICATOR to value 0
impl crate::Resettable for RX_INDICATORrs {}
