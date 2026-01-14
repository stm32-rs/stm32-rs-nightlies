///Register `RFSEQ_IRQ_STATUS` reader
pub type R = crate::R<RFSEQ_IRQ_STATUSrs>;
///Register `RFSEQ_IRQ_STATUS` writer
pub type W = crate::W<RFSEQ_IRQ_STATUSrs>;
///Field `TX_DONE_F` reader - Transmission done flag
pub type TX_DONE_F_R = crate::BitReader;
///Field `TX_DONE_F` writer - Transmission done flag
pub type TX_DONE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_OK_F` reader - Reception ended and OK flag
pub type RX_OK_F_R = crate::BitReader;
///Field `RX_OK_F` writer - Reception ended and OK flag
pub type RX_OK_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_TIMEOUT_F` reader - Reception timeout flag
pub type RX_TIMEOUT_F_R = crate::BitReader;
///Field `RX_TIMEOUT_F` writer - Reception timeout flag
pub type RX_TIMEOUT_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_CRC_FRROR_F` reader - Reception with CRC error flag
pub type RX_CRC_FRROR_F_R = crate::BitReader;
///Field `RX_CRC_FRROR_F` writer - Reception with CRC error flag
pub type RX_CRC_FRROR_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAST_RX_TERM_F` reader - Fast RX Termination flag
pub type FAST_RX_TERM_F_R = crate::BitReader;
///Field `FAST_RX_TERM_F` writer - Fast RX Termination flag
pub type FAST_RX_TERM_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTIMER_STOP_CDT_F` reader - Enable interrupt on RXTIMER_STOP_CDT_F flag
pub type RXTIMER_STOP_CDT_F_R = crate::BitReader;
///Field `RXTIMER_STOP_CDT_F` writer - Enable interrupt on RXTIMER_STOP_CDT_F flag
pub type RXTIMER_STOP_CDT_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SABORT_DONE_F` reader - SABORT command treated and done flag
pub type SABORT_DONE_F_R = crate::BitReader;
///Field `SABORT_DONE_F` writer - SABORT command treated and done flag
pub type SABORT_DONE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMMAND_REJECTED_F` reader - Command rejection flag.
pub type COMMAND_REJECTED_F_R = crate::BitReader;
///Field `COMMAND_REJECTED_F` writer - Command rejection flag.
pub type COMMAND_REJECTED_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_F` reader - Carrier Sense (RSSI over threshold) flag
pub type CS_F_R = crate::BitReader;
///Field `CS_F` writer - Carrier Sense (RSSI over threshold) flag
pub type CS_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREAMBLE_VALID_F` reader - Valid PREAMBLE detection flag.
pub type PREAMBLE_VALID_F_R = crate::BitReader;
///Field `PREAMBLE_VALID_F` writer - Valid PREAMBLE detection flag.
pub type PREAMBLE_VALID_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_VALID_F` reader - Valid SYNC word detection flag.
pub type SYNC_VALID_F_R = crate::BitReader;
///Field `SYNC_VALID_F` writer - Valid SYNC word detection flag.
pub type SYNC_VALID_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATABUFFER0_USED_F` reader - Data Buffer 0 fully read in TX or fully written in RX flag
pub type DATABUFFER0_USED_F_R = crate::BitReader;
///Field `DATABUFFER0_USED_F` writer - Data Buffer 0 fully read in TX or fully written in RX flag
pub type DATABUFFER0_USED_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATABUFFER1_USED_F` reader - Data Buffer 1 fully read in TX or fully written in RX flag
pub type DATABUFFER1_USED_F_R = crate::BitReader;
///Field `DATABUFFER1_USED_F` writer - Data Buffer 1 fully read in TX or fully written in RX flag
pub type DATABUFFER1_USED_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ALMOST_FULL_0_F` reader - Data Buffer0 used (written during a RX) up to programmed thresold flag
pub type RX_ALMOST_FULL_0_F_R = crate::BitReader;
///Field `RX_ALMOST_FULL_0_F` writer - Data Buffer0 used (written during a RX) up to programmed thresold flag
pub type RX_ALMOST_FULL_0_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ALMOST_FULL_1_F` reader - Data Buffer1 used (written during a RX) up to programmed thresold flag
pub type RX_ALMOST_FULL_1_F_R = crate::BitReader;
///Field `RX_ALMOST_FULL_1_F` writer - Data Buffer1 used (written during a RX) up to programmed thresold flag
pub type RX_ALMOST_FULL_1_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ALMOST_EMPTY_0_F` reader - Data Buffer0 used (read during a TX) up to programmed thresold flag
pub type TX_ALMOST_EMPTY_0_F_R = crate::BitReader;
///Field `TX_ALMOST_EMPTY_0_F` writer - Data Buffer0 used (read during a TX) up to programmed thresold flag
pub type TX_ALMOST_EMPTY_0_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ALMOST_EMPTY_1_F` reader - Data Buffer1 used (read during a TX) up to programmed thresold flag
pub type TX_ALMOST_EMPTY_1_F_R = crate::BitReader;
///Field `TX_ALMOST_EMPTY_1_F` writer - Data Buffer1 used (read during a TX) up to programmed thresold flag
pub type TX_ALMOST_EMPTY_1_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB_ACCESS_ERROR_F` reader - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer).
pub type AHB_ACCESS_ERROR_F_R = crate::BitReader;
///Field `AHB_ACCESS_ERROR_F` writer - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer).
pub type AHB_ACCESS_ERROR_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW_ANA_FAILURE_F` reader - Analog HW failure flag (PLL lock / unlock error, calibration error)
pub type HW_ANA_FAILURE_F_R = crate::BitReader;
///Field `HW_ANA_FAILURE_F` writer - Analog HW failure flag (PLL lock / unlock error, calibration error)
pub type HW_ANA_FAILURE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_F` reader - Sequencer completion flag.
pub type SEQ_F_R = crate::BitReader;
///Field `SEQ_F` writer - Sequencer completion flag.
pub type SEQ_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRM_CMD_START_F` reader - RRM-UDRA command list execution started flag.
pub type RRM_CMD_START_F_R = crate::BitReader;
///Field `RRM_CMD_START_F` writer - RRM-UDRA command list execution started flag.
pub type RRM_CMD_START_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRM_CMD_END_F` reader - RRM-UDRA command list execution ended flag.
pub type RRM_CMD_END_F_R = crate::BitReader;
///Field `RRM_CMD_END_F` writer - RRM-UDRA command list execution ended flag.
pub type RRM_CMD_END_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAFEASK_CALIB_DONE_F` reader - End of Safe-ASK PA calibration flag.
pub type SAFEASK_CALIB_DONE_F_R = crate::BitReader;
///Field `SAFEASK_CALIB_DONE_F` writer - End of Safe-ASK PA calibration flag.
pub type SAFEASK_CALIB_DONE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_CALIB_DONE_F` reader - Valid RSSI value available in the RSSI_RUNNING bit field flag.
pub type AGC_CALIB_DONE_F_R = crate::BitReader;
///Field `AGC_CALIB_DONE_F` writer - Valid RSSI value available in the RSSI_RUNNING bit field flag.
pub type AGC_CALIB_DONE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmission done flag
    #[inline(always)]
    pub fn tx_done_f(&self) -> TX_DONE_F_R {
        TX_DONE_F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reception ended and OK flag
    #[inline(always)]
    pub fn rx_ok_f(&self) -> RX_OK_F_R {
        RX_OK_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reception timeout flag
    #[inline(always)]
    pub fn rx_timeout_f(&self) -> RX_TIMEOUT_F_R {
        RX_TIMEOUT_F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reception with CRC error flag
    #[inline(always)]
    pub fn rx_crc_frror_f(&self) -> RX_CRC_FRROR_F_R {
        RX_CRC_FRROR_F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fast RX Termination flag
    #[inline(always)]
    pub fn fast_rx_term_f(&self) -> FAST_RX_TERM_F_R {
        FAST_RX_TERM_F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag
    #[inline(always)]
    pub fn rxtimer_stop_cdt_f(&self) -> RXTIMER_STOP_CDT_F_R {
        RXTIMER_STOP_CDT_F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SABORT command treated and done flag
    #[inline(always)]
    pub fn sabort_done_f(&self) -> SABORT_DONE_F_R {
        SABORT_DONE_F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Command rejection flag.
    #[inline(always)]
    pub fn command_rejected_f(&self) -> COMMAND_REJECTED_F_R {
        COMMAND_REJECTED_F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Carrier Sense (RSSI over threshold) flag
    #[inline(always)]
    pub fn cs_f(&self) -> CS_F_R {
        CS_F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Valid PREAMBLE detection flag.
    #[inline(always)]
    pub fn preamble_valid_f(&self) -> PREAMBLE_VALID_F_R {
        PREAMBLE_VALID_F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Valid SYNC word detection flag.
    #[inline(always)]
    pub fn sync_valid_f(&self) -> SYNC_VALID_F_R {
        SYNC_VALID_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Data Buffer 0 fully read in TX or fully written in RX flag
    #[inline(always)]
    pub fn databuffer0_used_f(&self) -> DATABUFFER0_USED_F_R {
        DATABUFFER0_USED_F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Data Buffer 1 fully read in TX or fully written in RX flag
    #[inline(always)]
    pub fn databuffer1_used_f(&self) -> DATABUFFER1_USED_F_R {
        DATABUFFER1_USED_F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Data Buffer0 used (written during a RX) up to programmed thresold flag
    #[inline(always)]
    pub fn rx_almost_full_0_f(&self) -> RX_ALMOST_FULL_0_F_R {
        RX_ALMOST_FULL_0_F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Data Buffer1 used (written during a RX) up to programmed thresold flag
    #[inline(always)]
    pub fn rx_almost_full_1_f(&self) -> RX_ALMOST_FULL_1_F_R {
        RX_ALMOST_FULL_1_F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Data Buffer0 used (read during a TX) up to programmed thresold flag
    #[inline(always)]
    pub fn tx_almost_empty_0_f(&self) -> TX_ALMOST_EMPTY_0_F_R {
        TX_ALMOST_EMPTY_0_F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Data Buffer1 used (read during a TX) up to programmed thresold flag
    #[inline(always)]
    pub fn tx_almost_empty_1_f(&self) -> TX_ALMOST_EMPTY_1_F_R {
        TX_ALMOST_EMPTY_1_F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer).
    #[inline(always)]
    pub fn ahb_access_error_f(&self) -> AHB_ACCESS_ERROR_F_R {
        AHB_ACCESS_ERROR_F_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Analog HW failure flag (PLL lock / unlock error, calibration error)
    #[inline(always)]
    pub fn hw_ana_failure_f(&self) -> HW_ANA_FAILURE_F_R {
        HW_ANA_FAILURE_F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Sequencer completion flag.
    #[inline(always)]
    pub fn seq_f(&self) -> SEQ_F_R {
        SEQ_F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - RRM-UDRA command list execution started flag.
    #[inline(always)]
    pub fn rrm_cmd_start_f(&self) -> RRM_CMD_START_F_R {
        RRM_CMD_START_F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RRM-UDRA command list execution ended flag.
    #[inline(always)]
    pub fn rrm_cmd_end_f(&self) -> RRM_CMD_END_F_R {
        RRM_CMD_END_F_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - End of Safe-ASK PA calibration flag.
    #[inline(always)]
    pub fn safeask_calib_done_f(&self) -> SAFEASK_CALIB_DONE_F_R {
        SAFEASK_CALIB_DONE_F_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Valid RSSI value available in the RSSI_RUNNING bit field flag.
    #[inline(always)]
    pub fn agc_calib_done_f(&self) -> AGC_CALIB_DONE_F_R {
        AGC_CALIB_DONE_F_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSEQ_IRQ_STATUS")
            .field("tx_done_f", &self.tx_done_f())
            .field("rx_ok_f", &self.rx_ok_f())
            .field("rx_timeout_f", &self.rx_timeout_f())
            .field("rx_crc_frror_f", &self.rx_crc_frror_f())
            .field("fast_rx_term_f", &self.fast_rx_term_f())
            .field("rxtimer_stop_cdt_f", &self.rxtimer_stop_cdt_f())
            .field("sabort_done_f", &self.sabort_done_f())
            .field("command_rejected_f", &self.command_rejected_f())
            .field("cs_f", &self.cs_f())
            .field("preamble_valid_f", &self.preamble_valid_f())
            .field("sync_valid_f", &self.sync_valid_f())
            .field("databuffer0_used_f", &self.databuffer0_used_f())
            .field("databuffer1_used_f", &self.databuffer1_used_f())
            .field("rx_almost_full_0_f", &self.rx_almost_full_0_f())
            .field("rx_almost_full_1_f", &self.rx_almost_full_1_f())
            .field("tx_almost_empty_0_f", &self.tx_almost_empty_0_f())
            .field("tx_almost_empty_1_f", &self.tx_almost_empty_1_f())
            .field("ahb_access_error_f", &self.ahb_access_error_f())
            .field("hw_ana_failure_f", &self.hw_ana_failure_f())
            .field("seq_f", &self.seq_f())
            .field("rrm_cmd_start_f", &self.rrm_cmd_start_f())
            .field("rrm_cmd_end_f", &self.rrm_cmd_end_f())
            .field("safeask_calib_done_f", &self.safeask_calib_done_f())
            .field("agc_calib_done_f", &self.agc_calib_done_f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmission done flag
    #[inline(always)]
    pub fn tx_done_f(&mut self) -> TX_DONE_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        TX_DONE_F_W::new(self, 0)
    }
    ///Bit 1 - Reception ended and OK flag
    #[inline(always)]
    pub fn rx_ok_f(&mut self) -> RX_OK_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RX_OK_F_W::new(self, 1)
    }
    ///Bit 2 - Reception timeout flag
    #[inline(always)]
    pub fn rx_timeout_f(&mut self) -> RX_TIMEOUT_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RX_TIMEOUT_F_W::new(self, 2)
    }
    ///Bit 3 - Reception with CRC error flag
    #[inline(always)]
    pub fn rx_crc_frror_f(&mut self) -> RX_CRC_FRROR_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RX_CRC_FRROR_F_W::new(self, 3)
    }
    ///Bit 4 - Fast RX Termination flag
    #[inline(always)]
    pub fn fast_rx_term_f(&mut self) -> FAST_RX_TERM_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        FAST_RX_TERM_F_W::new(self, 4)
    }
    ///Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag
    #[inline(always)]
    pub fn rxtimer_stop_cdt_f(&mut self) -> RXTIMER_STOP_CDT_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RXTIMER_STOP_CDT_F_W::new(self, 7)
    }
    ///Bit 8 - SABORT command treated and done flag
    #[inline(always)]
    pub fn sabort_done_f(&mut self) -> SABORT_DONE_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        SABORT_DONE_F_W::new(self, 8)
    }
    ///Bit 9 - Command rejection flag.
    #[inline(always)]
    pub fn command_rejected_f(&mut self) -> COMMAND_REJECTED_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        COMMAND_REJECTED_F_W::new(self, 9)
    }
    ///Bit 12 - Carrier Sense (RSSI over threshold) flag
    #[inline(always)]
    pub fn cs_f(&mut self) -> CS_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        CS_F_W::new(self, 12)
    }
    ///Bit 13 - Valid PREAMBLE detection flag.
    #[inline(always)]
    pub fn preamble_valid_f(&mut self) -> PREAMBLE_VALID_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        PREAMBLE_VALID_F_W::new(self, 13)
    }
    ///Bit 14 - Valid SYNC word detection flag.
    #[inline(always)]
    pub fn sync_valid_f(&mut self) -> SYNC_VALID_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        SYNC_VALID_F_W::new(self, 14)
    }
    ///Bit 16 - Data Buffer 0 fully read in TX or fully written in RX flag
    #[inline(always)]
    pub fn databuffer0_used_f(&mut self) -> DATABUFFER0_USED_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        DATABUFFER0_USED_F_W::new(self, 16)
    }
    ///Bit 17 - Data Buffer 1 fully read in TX or fully written in RX flag
    #[inline(always)]
    pub fn databuffer1_used_f(&mut self) -> DATABUFFER1_USED_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        DATABUFFER1_USED_F_W::new(self, 17)
    }
    ///Bit 18 - Data Buffer0 used (written during a RX) up to programmed thresold flag
    #[inline(always)]
    pub fn rx_almost_full_0_f(&mut self) -> RX_ALMOST_FULL_0_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RX_ALMOST_FULL_0_F_W::new(self, 18)
    }
    ///Bit 19 - Data Buffer1 used (written during a RX) up to programmed thresold flag
    #[inline(always)]
    pub fn rx_almost_full_1_f(&mut self) -> RX_ALMOST_FULL_1_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RX_ALMOST_FULL_1_F_W::new(self, 19)
    }
    ///Bit 20 - Data Buffer0 used (read during a TX) up to programmed thresold flag
    #[inline(always)]
    pub fn tx_almost_empty_0_f(&mut self) -> TX_ALMOST_EMPTY_0_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        TX_ALMOST_EMPTY_0_F_W::new(self, 20)
    }
    ///Bit 21 - Data Buffer1 used (read during a TX) up to programmed thresold flag
    #[inline(always)]
    pub fn tx_almost_empty_1_f(&mut self) -> TX_ALMOST_EMPTY_1_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        TX_ALMOST_EMPTY_1_F_W::new(self, 21)
    }
    ///Bit 22 - An AHB transfer issue occurred for one of the AHB masters (RRM, Data Buffer Manager, Sequencer).
    #[inline(always)]
    pub fn ahb_access_error_f(&mut self) -> AHB_ACCESS_ERROR_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        AHB_ACCESS_ERROR_F_W::new(self, 22)
    }
    ///Bit 24 - Analog HW failure flag (PLL lock / unlock error, calibration error)
    #[inline(always)]
    pub fn hw_ana_failure_f(&mut self) -> HW_ANA_FAILURE_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        HW_ANA_FAILURE_F_W::new(self, 24)
    }
    ///Bit 26 - Sequencer completion flag.
    #[inline(always)]
    pub fn seq_f(&mut self) -> SEQ_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        SEQ_F_W::new(self, 26)
    }
    ///Bit 27 - RRM-UDRA command list execution started flag.
    #[inline(always)]
    pub fn rrm_cmd_start_f(&mut self) -> RRM_CMD_START_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RRM_CMD_START_F_W::new(self, 27)
    }
    ///Bit 28 - RRM-UDRA command list execution ended flag.
    #[inline(always)]
    pub fn rrm_cmd_end_f(&mut self) -> RRM_CMD_END_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        RRM_CMD_END_F_W::new(self, 28)
    }
    ///Bit 30 - End of Safe-ASK PA calibration flag.
    #[inline(always)]
    pub fn safeask_calib_done_f(&mut self) -> SAFEASK_CALIB_DONE_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        SAFEASK_CALIB_DONE_F_W::new(self, 30)
    }
    ///Bit 31 - Valid RSSI value available in the RSSI_RUNNING bit field flag.
    #[inline(always)]
    pub fn agc_calib_done_f(&mut self) -> AGC_CALIB_DONE_F_W<'_, RFSEQ_IRQ_STATUSrs> {
        AGC_CALIB_DONE_F_W::new(self, 31)
    }
}
/**RFSEQ_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`rfseq_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RFSEQ_IRQ_STATUS)*/
pub struct RFSEQ_IRQ_STATUSrs;
impl crate::RegisterSpec for RFSEQ_IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`rfseq_irq_status::R`](R) reader structure
impl crate::Readable for RFSEQ_IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`rfseq_irq_status::W`](W) writer structure
impl crate::Writable for RFSEQ_IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSEQ_IRQ_STATUS to value 0
impl crate::Resettable for RFSEQ_IRQ_STATUSrs {}
