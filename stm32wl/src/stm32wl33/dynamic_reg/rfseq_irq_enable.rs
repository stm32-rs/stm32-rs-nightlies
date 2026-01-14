///Register `RFSEQ_IRQ_ENABLE` reader
pub type R = crate::R<RFSEQ_IRQ_ENABLErs>;
///Register `RFSEQ_IRQ_ENABLE` writer
pub type W = crate::W<RFSEQ_IRQ_ENABLErs>;
///Field `TX_DONE_E` reader - Enable interrupt on TX_DONE_F flag
pub type TX_DONE_E_R = crate::BitReader;
///Field `TX_DONE_E` writer - Enable interrupt on TX_DONE_F flag
pub type TX_DONE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_OK_E` reader - Enable interrupt on RX_OK_F flag
pub type RX_OK_E_R = crate::BitReader;
///Field `RX_OK_E` writer - Enable interrupt on RX_OK_F flag
pub type RX_OK_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_TIMEOUT_E` reader - Enable interrupt on RX_TIMEOUT_F flag
pub type RX_TIMEOUT_E_R = crate::BitReader;
///Field `RX_TIMEOUT_E` writer - Enable interrupt on RX_TIMEOUT_F flag
pub type RX_TIMEOUT_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_CRC_ERROR_E` reader - Enable interrupt on RX_CRC_ERROR_F flag
pub type RX_CRC_ERROR_E_R = crate::BitReader;
///Field `RX_CRC_ERROR_E` writer - Enable interrupt on RX_CRC_ERROR_F flag
pub type RX_CRC_ERROR_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAST_RX_TERM_E` reader - Enable interrupt on FAST_RX_TERM_F flag
pub type FAST_RX_TERM_E_R = crate::BitReader;
///Field `FAST_RX_TERM_E` writer - Enable interrupt on FAST_RX_TERM_F flag
pub type FAST_RX_TERM_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTIMER_STOP_CDT_E` reader - Enable interrupt on RXTIMER_STOP_CDT_F flag
pub type RXTIMER_STOP_CDT_E_R = crate::BitReader;
///Field `RXTIMER_STOP_CDT_E` writer - Enable interrupt on RXTIMER_STOP_CDT_F flag
pub type RXTIMER_STOP_CDT_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SABORT_DONE_E` reader - Enable interrupt on SABORT command treated and done flag
pub type SABORT_DONE_E_R = crate::BitReader;
///Field `SABORT_DONE_E` writer - Enable interrupt on SABORT command treated and done flag
pub type SABORT_DONE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMMAND_REJECTED_E` reader - Enable interrupt on COMMAND_REJECTED flag
pub type COMMAND_REJECTED_E_R = crate::BitReader;
///Field `COMMAND_REJECTED_E` writer - Enable interrupt on COMMAND_REJECTED flag
pub type COMMAND_REJECTED_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_E` reader - Enable interrupt on CS_F flag
pub type CS_E_R = crate::BitReader;
///Field `CS_E` writer - Enable interrupt on CS_F flag
pub type CS_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PREAMBLE_VALID_E` reader - Enable interrupt on PREAMBLE_VALID_F flag
pub type PREAMBLE_VALID_E_R = crate::BitReader;
///Field `PREAMBLE_VALID_E` writer - Enable interrupt on PREAMBLE_VALID_F flag
pub type PREAMBLE_VALID_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_VALID_E` reader - Enable interrupt on SYNC_VALID_F flag
pub type SYNC_VALID_E_R = crate::BitReader;
///Field `SYNC_VALID_E` writer - Enable interrupt on SYNC_VALID_F flag
pub type SYNC_VALID_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATABUFFER0_USED_E` reader - Enable interrupt on DATABUFFER0_USED_F flag
pub type DATABUFFER0_USED_E_R = crate::BitReader;
///Field `DATABUFFER0_USED_E` writer - Enable interrupt on DATABUFFER0_USED_F flag
pub type DATABUFFER0_USED_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATABUFFER1_USED_E` reader - Enable interrupt on DATABUFFER1_USED_F flag
pub type DATABUFFER1_USED_E_R = crate::BitReader;
///Field `DATABUFFER1_USED_E` writer - Enable interrupt on DATABUFFER1_USED_F flag
pub type DATABUFFER1_USED_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ALMOST_FULL_0_E` reader - Enable interrupt on RX_ALMOST_FULL_0_F flag
pub type RX_ALMOST_FULL_0_E_R = crate::BitReader;
///Field `RX_ALMOST_FULL_0_E` writer - Enable interrupt on RX_ALMOST_FULL_0_F flag
pub type RX_ALMOST_FULL_0_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_ALMOST_FULL_1_E` reader - Enable interrupt on RX_ALMOST_FULL_1_F flag
pub type RX_ALMOST_FULL_1_E_R = crate::BitReader;
///Field `RX_ALMOST_FULL_1_E` writer - Enable interrupt on RX_ALMOST_FULL_1_F flag
pub type RX_ALMOST_FULL_1_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ALMOST_EMPTY_0_E` reader - Enable interrupt on TX_ALMOST_EMPTY_0_F flag
pub type TX_ALMOST_EMPTY_0_E_R = crate::BitReader;
///Field `TX_ALMOST_EMPTY_0_E` writer - Enable interrupt on TX_ALMOST_EMPTY_0_F flag
pub type TX_ALMOST_EMPTY_0_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ALMOST_EMPTY_1_E` reader - Enable interrupt on TX_ALMOST_EMPTY_1_F flag
pub type TX_ALMOST_EMPTY_1_E_R = crate::BitReader;
///Field `TX_ALMOST_EMPTY_1_E` writer - Enable interrupt on TX_ALMOST_EMPTY_1_F flag
pub type TX_ALMOST_EMPTY_1_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB_ACCESS_ERROR_E` reader - Enable interrupt on AHB_ACCESS_ERROR_F flag
pub type AHB_ACCESS_ERROR_E_R = crate::BitReader;
///Field `AHB_ACCESS_ERROR_E` writer - Enable interrupt on AHB_ACCESS_ERROR_F flag
pub type AHB_ACCESS_ERROR_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW_ANA_FAILURE_E` reader - Enable interrupt on HW_ANA_FAILURE_F flag
pub type HW_ANA_FAILURE_E_R = crate::BitReader;
///Field `HW_ANA_FAILURE_E` writer - Enable interrupt on HW_ANA_FAILURE_F flag
pub type HW_ANA_FAILURE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_E` reader - Enable interrupt on SEQ_F flag
pub type SEQ_E_R = crate::BitReader;
///Field `SEQ_E` writer - Enable interrupt on SEQ_F flag
pub type SEQ_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRM_CMD_START_E` reader - Enable interrupt on RRM_CMD_END_F flag
pub type RRM_CMD_START_E_R = crate::BitReader;
///Field `RRM_CMD_START_E` writer - Enable interrupt on RRM_CMD_END_F flag
pub type RRM_CMD_START_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRM_CMD_END_E` reader - Enable interrupt on RRM_CMD_END_F flag
pub type RRM_CMD_END_E_R = crate::BitReader;
///Field `RRM_CMD_END_E` writer - Enable interrupt on RRM_CMD_END_F flag
pub type RRM_CMD_END_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAFEASK_CALIB_DONE_E` reader - Enable interrupt on SAFEASK_CALIB_DONE_F flag
pub type SAFEASK_CALIB_DONE_E_R = crate::BitReader;
///Field `SAFEASK_CALIB_DONE_E` writer - Enable interrupt on SAFEASK_CALIB_DONE_F flag
pub type SAFEASK_CALIB_DONE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_CALIB_DONE_E` reader - Enable interrupt on AGC_CALIB_DONE_F flag
pub type AGC_CALIB_DONE_E_R = crate::BitReader;
///Field `AGC_CALIB_DONE_E` writer - Enable interrupt on AGC_CALIB_DONE_F flag
pub type AGC_CALIB_DONE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable interrupt on TX_DONE_F flag
    #[inline(always)]
    pub fn tx_done_e(&self) -> TX_DONE_E_R {
        TX_DONE_E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable interrupt on RX_OK_F flag
    #[inline(always)]
    pub fn rx_ok_e(&self) -> RX_OK_E_R {
        RX_OK_E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable interrupt on RX_TIMEOUT_F flag
    #[inline(always)]
    pub fn rx_timeout_e(&self) -> RX_TIMEOUT_E_R {
        RX_TIMEOUT_E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable interrupt on RX_CRC_ERROR_F flag
    #[inline(always)]
    pub fn rx_crc_error_e(&self) -> RX_CRC_ERROR_E_R {
        RX_CRC_ERROR_E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable interrupt on FAST_RX_TERM_F flag
    #[inline(always)]
    pub fn fast_rx_term_e(&self) -> FAST_RX_TERM_E_R {
        FAST_RX_TERM_E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag
    #[inline(always)]
    pub fn rxtimer_stop_cdt_e(&self) -> RXTIMER_STOP_CDT_E_R {
        RXTIMER_STOP_CDT_E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Enable interrupt on SABORT command treated and done flag
    #[inline(always)]
    pub fn sabort_done_e(&self) -> SABORT_DONE_E_R {
        SABORT_DONE_E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable interrupt on COMMAND_REJECTED flag
    #[inline(always)]
    pub fn command_rejected_e(&self) -> COMMAND_REJECTED_E_R {
        COMMAND_REJECTED_E_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Enable interrupt on CS_F flag
    #[inline(always)]
    pub fn cs_e(&self) -> CS_E_R {
        CS_E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable interrupt on PREAMBLE_VALID_F flag
    #[inline(always)]
    pub fn preamble_valid_e(&self) -> PREAMBLE_VALID_E_R {
        PREAMBLE_VALID_E_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable interrupt on SYNC_VALID_F flag
    #[inline(always)]
    pub fn sync_valid_e(&self) -> SYNC_VALID_E_R {
        SYNC_VALID_E_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Enable interrupt on DATABUFFER0_USED_F flag
    #[inline(always)]
    pub fn databuffer0_used_e(&self) -> DATABUFFER0_USED_E_R {
        DATABUFFER0_USED_E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Enable interrupt on DATABUFFER1_USED_F flag
    #[inline(always)]
    pub fn databuffer1_used_e(&self) -> DATABUFFER1_USED_E_R {
        DATABUFFER1_USED_E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Enable interrupt on RX_ALMOST_FULL_0_F flag
    #[inline(always)]
    pub fn rx_almost_full_0_e(&self) -> RX_ALMOST_FULL_0_E_R {
        RX_ALMOST_FULL_0_E_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Enable interrupt on RX_ALMOST_FULL_1_F flag
    #[inline(always)]
    pub fn rx_almost_full_1_e(&self) -> RX_ALMOST_FULL_1_E_R {
        RX_ALMOST_FULL_1_E_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Enable interrupt on TX_ALMOST_EMPTY_0_F flag
    #[inline(always)]
    pub fn tx_almost_empty_0_e(&self) -> TX_ALMOST_EMPTY_0_E_R {
        TX_ALMOST_EMPTY_0_E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Enable interrupt on TX_ALMOST_EMPTY_1_F flag
    #[inline(always)]
    pub fn tx_almost_empty_1_e(&self) -> TX_ALMOST_EMPTY_1_E_R {
        TX_ALMOST_EMPTY_1_E_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable interrupt on AHB_ACCESS_ERROR_F flag
    #[inline(always)]
    pub fn ahb_access_error_e(&self) -> AHB_ACCESS_ERROR_E_R {
        AHB_ACCESS_ERROR_E_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Enable interrupt on HW_ANA_FAILURE_F flag
    #[inline(always)]
    pub fn hw_ana_failure_e(&self) -> HW_ANA_FAILURE_E_R {
        HW_ANA_FAILURE_E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Enable interrupt on SEQ_F flag
    #[inline(always)]
    pub fn seq_e(&self) -> SEQ_E_R {
        SEQ_E_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Enable interrupt on RRM_CMD_END_F flag
    #[inline(always)]
    pub fn rrm_cmd_start_e(&self) -> RRM_CMD_START_E_R {
        RRM_CMD_START_E_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Enable interrupt on RRM_CMD_END_F flag
    #[inline(always)]
    pub fn rrm_cmd_end_e(&self) -> RRM_CMD_END_E_R {
        RRM_CMD_END_E_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Enable interrupt on SAFEASK_CALIB_DONE_F flag
    #[inline(always)]
    pub fn safeask_calib_done_e(&self) -> SAFEASK_CALIB_DONE_E_R {
        SAFEASK_CALIB_DONE_E_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Enable interrupt on AGC_CALIB_DONE_F flag
    #[inline(always)]
    pub fn agc_calib_done_e(&self) -> AGC_CALIB_DONE_E_R {
        AGC_CALIB_DONE_E_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSEQ_IRQ_ENABLE")
            .field("tx_done_e", &self.tx_done_e())
            .field("rx_ok_e", &self.rx_ok_e())
            .field("rx_timeout_e", &self.rx_timeout_e())
            .field("rx_crc_error_e", &self.rx_crc_error_e())
            .field("fast_rx_term_e", &self.fast_rx_term_e())
            .field("rxtimer_stop_cdt_e", &self.rxtimer_stop_cdt_e())
            .field("sabort_done_e", &self.sabort_done_e())
            .field("command_rejected_e", &self.command_rejected_e())
            .field("cs_e", &self.cs_e())
            .field("preamble_valid_e", &self.preamble_valid_e())
            .field("sync_valid_e", &self.sync_valid_e())
            .field("databuffer0_used_e", &self.databuffer0_used_e())
            .field("databuffer1_used_e", &self.databuffer1_used_e())
            .field("rx_almost_full_0_e", &self.rx_almost_full_0_e())
            .field("rx_almost_full_1_e", &self.rx_almost_full_1_e())
            .field("tx_almost_empty_0_e", &self.tx_almost_empty_0_e())
            .field("tx_almost_empty_1_e", &self.tx_almost_empty_1_e())
            .field("ahb_access_error_e", &self.ahb_access_error_e())
            .field("hw_ana_failure_e", &self.hw_ana_failure_e())
            .field("seq_e", &self.seq_e())
            .field("rrm_cmd_start_e", &self.rrm_cmd_start_e())
            .field("rrm_cmd_end_e", &self.rrm_cmd_end_e())
            .field("safeask_calib_done_e", &self.safeask_calib_done_e())
            .field("agc_calib_done_e", &self.agc_calib_done_e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable interrupt on TX_DONE_F flag
    #[inline(always)]
    pub fn tx_done_e(&mut self) -> TX_DONE_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        TX_DONE_E_W::new(self, 0)
    }
    ///Bit 1 - Enable interrupt on RX_OK_F flag
    #[inline(always)]
    pub fn rx_ok_e(&mut self) -> RX_OK_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RX_OK_E_W::new(self, 1)
    }
    ///Bit 2 - Enable interrupt on RX_TIMEOUT_F flag
    #[inline(always)]
    pub fn rx_timeout_e(&mut self) -> RX_TIMEOUT_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RX_TIMEOUT_E_W::new(self, 2)
    }
    ///Bit 3 - Enable interrupt on RX_CRC_ERROR_F flag
    #[inline(always)]
    pub fn rx_crc_error_e(&mut self) -> RX_CRC_ERROR_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RX_CRC_ERROR_E_W::new(self, 3)
    }
    ///Bit 4 - Enable interrupt on FAST_RX_TERM_F flag
    #[inline(always)]
    pub fn fast_rx_term_e(&mut self) -> FAST_RX_TERM_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        FAST_RX_TERM_E_W::new(self, 4)
    }
    ///Bit 7 - Enable interrupt on RXTIMER_STOP_CDT_F flag
    #[inline(always)]
    pub fn rxtimer_stop_cdt_e(&mut self) -> RXTIMER_STOP_CDT_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RXTIMER_STOP_CDT_E_W::new(self, 7)
    }
    ///Bit 8 - Enable interrupt on SABORT command treated and done flag
    #[inline(always)]
    pub fn sabort_done_e(&mut self) -> SABORT_DONE_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        SABORT_DONE_E_W::new(self, 8)
    }
    ///Bit 9 - Enable interrupt on COMMAND_REJECTED flag
    #[inline(always)]
    pub fn command_rejected_e(&mut self) -> COMMAND_REJECTED_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        COMMAND_REJECTED_E_W::new(self, 9)
    }
    ///Bit 12 - Enable interrupt on CS_F flag
    #[inline(always)]
    pub fn cs_e(&mut self) -> CS_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        CS_E_W::new(self, 12)
    }
    ///Bit 13 - Enable interrupt on PREAMBLE_VALID_F flag
    #[inline(always)]
    pub fn preamble_valid_e(&mut self) -> PREAMBLE_VALID_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        PREAMBLE_VALID_E_W::new(self, 13)
    }
    ///Bit 14 - Enable interrupt on SYNC_VALID_F flag
    #[inline(always)]
    pub fn sync_valid_e(&mut self) -> SYNC_VALID_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        SYNC_VALID_E_W::new(self, 14)
    }
    ///Bit 16 - Enable interrupt on DATABUFFER0_USED_F flag
    #[inline(always)]
    pub fn databuffer0_used_e(&mut self) -> DATABUFFER0_USED_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        DATABUFFER0_USED_E_W::new(self, 16)
    }
    ///Bit 17 - Enable interrupt on DATABUFFER1_USED_F flag
    #[inline(always)]
    pub fn databuffer1_used_e(&mut self) -> DATABUFFER1_USED_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        DATABUFFER1_USED_E_W::new(self, 17)
    }
    ///Bit 18 - Enable interrupt on RX_ALMOST_FULL_0_F flag
    #[inline(always)]
    pub fn rx_almost_full_0_e(&mut self) -> RX_ALMOST_FULL_0_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RX_ALMOST_FULL_0_E_W::new(self, 18)
    }
    ///Bit 19 - Enable interrupt on RX_ALMOST_FULL_1_F flag
    #[inline(always)]
    pub fn rx_almost_full_1_e(&mut self) -> RX_ALMOST_FULL_1_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RX_ALMOST_FULL_1_E_W::new(self, 19)
    }
    ///Bit 20 - Enable interrupt on TX_ALMOST_EMPTY_0_F flag
    #[inline(always)]
    pub fn tx_almost_empty_0_e(&mut self) -> TX_ALMOST_EMPTY_0_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        TX_ALMOST_EMPTY_0_E_W::new(self, 20)
    }
    ///Bit 21 - Enable interrupt on TX_ALMOST_EMPTY_1_F flag
    #[inline(always)]
    pub fn tx_almost_empty_1_e(&mut self) -> TX_ALMOST_EMPTY_1_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        TX_ALMOST_EMPTY_1_E_W::new(self, 21)
    }
    ///Bit 22 - Enable interrupt on AHB_ACCESS_ERROR_F flag
    #[inline(always)]
    pub fn ahb_access_error_e(&mut self) -> AHB_ACCESS_ERROR_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        AHB_ACCESS_ERROR_E_W::new(self, 22)
    }
    ///Bit 24 - Enable interrupt on HW_ANA_FAILURE_F flag
    #[inline(always)]
    pub fn hw_ana_failure_e(&mut self) -> HW_ANA_FAILURE_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        HW_ANA_FAILURE_E_W::new(self, 24)
    }
    ///Bit 26 - Enable interrupt on SEQ_F flag
    #[inline(always)]
    pub fn seq_e(&mut self) -> SEQ_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        SEQ_E_W::new(self, 26)
    }
    ///Bit 27 - Enable interrupt on RRM_CMD_END_F flag
    #[inline(always)]
    pub fn rrm_cmd_start_e(&mut self) -> RRM_CMD_START_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RRM_CMD_START_E_W::new(self, 27)
    }
    ///Bit 28 - Enable interrupt on RRM_CMD_END_F flag
    #[inline(always)]
    pub fn rrm_cmd_end_e(&mut self) -> RRM_CMD_END_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        RRM_CMD_END_E_W::new(self, 28)
    }
    ///Bit 30 - Enable interrupt on SAFEASK_CALIB_DONE_F flag
    #[inline(always)]
    pub fn safeask_calib_done_e(&mut self) -> SAFEASK_CALIB_DONE_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        SAFEASK_CALIB_DONE_E_W::new(self, 30)
    }
    ///Bit 31 - Enable interrupt on AGC_CALIB_DONE_F flag
    #[inline(always)]
    pub fn agc_calib_done_e(&mut self) -> AGC_CALIB_DONE_E_W<'_, RFSEQ_IRQ_ENABLErs> {
        AGC_CALIB_DONE_E_W::new(self, 31)
    }
}
/**RFSEQ_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`rfseq_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:RFSEQ_IRQ_ENABLE)*/
pub struct RFSEQ_IRQ_ENABLErs;
impl crate::RegisterSpec for RFSEQ_IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`rfseq_irq_enable::R`](R) reader structure
impl crate::Readable for RFSEQ_IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`rfseq_irq_enable::W`](W) writer structure
impl crate::Writable for RFSEQ_IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSEQ_IRQ_ENABLE to value 0
impl crate::Resettable for RFSEQ_IRQ_ENABLErs {}
