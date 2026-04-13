///Register `WORD5` reader
pub type R = crate::R<WORD5rs>;
///Register `WORD5` writer
pub type W = crate::W<WORD5rs>;
///Field `AutoTxRxskipEn` reader - Automatic transfer (TX or RX) skip enable.
pub type AUTO_TX_RXSKIP_EN_R = crate::BitReader;
///Field `AutoTxRxskipEn` writer - Automatic transfer (TX or RX) skip enable.
pub type AUTO_TX_RXSKIP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ChkFlagAutoClearEna` reader - Active bit Auto Clear Enable.
pub type CHK_FLAG_AUTO_CLEAR_ENA_R = crate::BitReader;
///Field `ChkFlagAutoClearEna` writer - Active bit Auto Clear Enable.
pub type CHK_FLAG_AUTO_CLEAR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntAddPointError` reader - Address pointer error interrupt enable.
pub type INT_ADD_POINT_ERROR_R = crate::BitReader;
///Field `IntAddPointError` writer - Address pointer error interrupt enable.
pub type INT_ADD_POINT_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntAllTableReadyError` reader - All table ready error interrupt enable.
pub type INT_ALL_TABLE_READY_ERROR_R = crate::BitReader;
///Field `IntAllTableReadyError` writer - All table ready error interrupt enable.
pub type INT_ALL_TABLE_READY_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntTxDataReadyError` reader - Transmission data payload ready error interrupt enable.
pub type INT_TX_DATA_READY_ERROR_R = crate::BitReader;
///Field `IntTxDataReadyError` writer - Transmission data payload ready error interrupt enable.
pub type INT_TX_DATA_READY_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntNoActiveLError` reader - Active bit low value reading interrupt enable.
pub type INT_NO_ACTIVE_LERROR_R = crate::BitReader;
///Field `IntNoActiveLError` writer - Active bit low value reading interrupt enable.
pub type INT_NO_ACTIVE_LERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntRcvLengthError` reader - Too long received payload length interrupt enable.
pub type INT_RCV_LENGTH_ERROR_R = crate::BitReader;
///Field `IntRcvLengthError` writer - Too long received payload length interrupt enable.
pub type INT_RCV_LENGTH_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntSemaTimeoutError` reader - Semaphore timeout error interrupt enable.
pub type INT_SEMA_TIMEOUT_ERROR_R = crate::BitReader;
///Field `IntSemaTimeoutError` writer - Semaphore timeout error interrupt enable.
pub type INT_SEMA_TIMEOUT_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntSeqDone` reader - Sequencer end of task interrupt enable.
pub type INT_SEQ_DONE_R = crate::BitReader;
///Field `IntSeqDone` writer - Sequencer end of task interrupt enable.
pub type INT_SEQ_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `intTxRxSkip` reader - Transmission or reception skip interrupt enable.
pub type INT_TX_RX_SKIP_R = crate::BitReader;
///Field `intTxRxSkip` writer - Transmission or reception skip interrupt enable.
pub type INT_TX_RX_SKIP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntActive2Err` reader - not in ACTIVE2 information from Radio FSM received on time interrupt enable.
pub type INT_ACTIVE2ERR_R = crate::BitReader;
///Field `IntActive2Err` writer - not in ACTIVE2 information from Radio FSM received on time interrupt enable.
pub type INT_ACTIVE2ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IntConfigError` reader - Configuration error interrupt enable.
pub type INT_CONFIG_ERROR_R = crate::BitReader;
///Field `IntConfigError` writer - Configuration error interrupt enable.
pub type INT_CONFIG_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Automatic transfer (TX or RX) skip enable.
    #[inline(always)]
    pub fn auto_tx_rxskip_en(&self) -> AUTO_TX_RXSKIP_EN_R {
        AUTO_TX_RXSKIP_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Active bit Auto Clear Enable.
    #[inline(always)]
    pub fn chk_flag_auto_clear_ena(&self) -> CHK_FLAG_AUTO_CLEAR_ENA_R {
        CHK_FLAG_AUTO_CLEAR_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 20 - Address pointer error interrupt enable.
    #[inline(always)]
    pub fn int_add_point_error(&self) -> INT_ADD_POINT_ERROR_R {
        INT_ADD_POINT_ERROR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - All table ready error interrupt enable.
    #[inline(always)]
    pub fn int_all_table_ready_error(&self) -> INT_ALL_TABLE_READY_ERROR_R {
        INT_ALL_TABLE_READY_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Transmission data payload ready error interrupt enable.
    #[inline(always)]
    pub fn int_tx_data_ready_error(&self) -> INT_TX_DATA_READY_ERROR_R {
        INT_TX_DATA_READY_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Active bit low value reading interrupt enable.
    #[inline(always)]
    pub fn int_no_active_lerror(&self) -> INT_NO_ACTIVE_LERROR_R {
        INT_NO_ACTIVE_LERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Too long received payload length interrupt enable.
    #[inline(always)]
    pub fn int_rcv_length_error(&self) -> INT_RCV_LENGTH_ERROR_R {
        INT_RCV_LENGTH_ERROR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Semaphore timeout error interrupt enable.
    #[inline(always)]
    pub fn int_sema_timeout_error(&self) -> INT_SEMA_TIMEOUT_ERROR_R {
        INT_SEMA_TIMEOUT_ERROR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Sequencer end of task interrupt enable.
    #[inline(always)]
    pub fn int_seq_done(&self) -> INT_SEQ_DONE_R {
        INT_SEQ_DONE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Transmission or reception skip interrupt enable.
    #[inline(always)]
    pub fn int_tx_rx_skip(&self) -> INT_TX_RX_SKIP_R {
        INT_TX_RX_SKIP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - not in ACTIVE2 information from Radio FSM received on time interrupt enable.
    #[inline(always)]
    pub fn int_active2err(&self) -> INT_ACTIVE2ERR_R {
        INT_ACTIVE2ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Configuration error interrupt enable.
    #[inline(always)]
    pub fn int_config_error(&self) -> INT_CONFIG_ERROR_R {
        INT_CONFIG_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD5")
            .field("auto_tx_rxskip_en", &self.auto_tx_rxskip_en())
            .field("chk_flag_auto_clear_ena", &self.chk_flag_auto_clear_ena())
            .field("int_add_point_error", &self.int_add_point_error())
            .field(
                "int_all_table_ready_error",
                &self.int_all_table_ready_error(),
            )
            .field("int_tx_data_ready_error", &self.int_tx_data_ready_error())
            .field("int_no_active_lerror", &self.int_no_active_lerror())
            .field("int_rcv_length_error", &self.int_rcv_length_error())
            .field("int_sema_timeout_error", &self.int_sema_timeout_error())
            .field("int_seq_done", &self.int_seq_done())
            .field("int_tx_rx_skip", &self.int_tx_rx_skip())
            .field("int_active2err", &self.int_active2err())
            .field("int_config_error", &self.int_config_error())
            .finish()
    }
}
impl W {
    ///Bit 0 - Automatic transfer (TX or RX) skip enable.
    #[inline(always)]
    pub fn auto_tx_rxskip_en(&mut self) -> AUTO_TX_RXSKIP_EN_W<'_, WORD5rs> {
        AUTO_TX_RXSKIP_EN_W::new(self, 0)
    }
    ///Bit 2 - Active bit Auto Clear Enable.
    #[inline(always)]
    pub fn chk_flag_auto_clear_ena(&mut self) -> CHK_FLAG_AUTO_CLEAR_ENA_W<'_, WORD5rs> {
        CHK_FLAG_AUTO_CLEAR_ENA_W::new(self, 2)
    }
    ///Bit 20 - Address pointer error interrupt enable.
    #[inline(always)]
    pub fn int_add_point_error(&mut self) -> INT_ADD_POINT_ERROR_W<'_, WORD5rs> {
        INT_ADD_POINT_ERROR_W::new(self, 20)
    }
    ///Bit 21 - All table ready error interrupt enable.
    #[inline(always)]
    pub fn int_all_table_ready_error(&mut self) -> INT_ALL_TABLE_READY_ERROR_W<'_, WORD5rs> {
        INT_ALL_TABLE_READY_ERROR_W::new(self, 21)
    }
    ///Bit 22 - Transmission data payload ready error interrupt enable.
    #[inline(always)]
    pub fn int_tx_data_ready_error(&mut self) -> INT_TX_DATA_READY_ERROR_W<'_, WORD5rs> {
        INT_TX_DATA_READY_ERROR_W::new(self, 22)
    }
    ///Bit 23 - Active bit low value reading interrupt enable.
    #[inline(always)]
    pub fn int_no_active_lerror(&mut self) -> INT_NO_ACTIVE_LERROR_W<'_, WORD5rs> {
        INT_NO_ACTIVE_LERROR_W::new(self, 23)
    }
    ///Bit 25 - Too long received payload length interrupt enable.
    #[inline(always)]
    pub fn int_rcv_length_error(&mut self) -> INT_RCV_LENGTH_ERROR_W<'_, WORD5rs> {
        INT_RCV_LENGTH_ERROR_W::new(self, 25)
    }
    ///Bit 26 - Semaphore timeout error interrupt enable.
    #[inline(always)]
    pub fn int_sema_timeout_error(&mut self) -> INT_SEMA_TIMEOUT_ERROR_W<'_, WORD5rs> {
        INT_SEMA_TIMEOUT_ERROR_W::new(self, 26)
    }
    ///Bit 28 - Sequencer end of task interrupt enable.
    #[inline(always)]
    pub fn int_seq_done(&mut self) -> INT_SEQ_DONE_W<'_, WORD5rs> {
        INT_SEQ_DONE_W::new(self, 28)
    }
    ///Bit 29 - Transmission or reception skip interrupt enable.
    #[inline(always)]
    pub fn int_tx_rx_skip(&mut self) -> INT_TX_RX_SKIP_W<'_, WORD5rs> {
        INT_TX_RX_SKIP_W::new(self, 29)
    }
    ///Bit 30 - not in ACTIVE2 information from Radio FSM received on time interrupt enable.
    #[inline(always)]
    pub fn int_active2err(&mut self) -> INT_ACTIVE2ERR_W<'_, WORD5rs> {
        INT_ACTIVE2ERR_W::new(self, 30)
    }
    ///Bit 31 - Configuration error interrupt enable.
    #[inline(always)]
    pub fn int_config_error(&mut self) -> INT_CONFIG_ERROR_W<'_, WORD5rs> {
        INT_CONFIG_ERROR_W::new(self, 31)
    }
}
/**WORD5 register

You can [`read`](crate::Reg::read) this register and get [`word5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#GLOBALSTATMACH:WORD5)*/
pub struct WORD5rs;
impl crate::RegisterSpec for WORD5rs {
    type Ux = u32;
}
///`read()` method returns [`word5::R`](R) reader structure
impl crate::Readable for WORD5rs {}
///`write(|w| ..)` method takes [`word5::W`](W) writer structure
impl crate::Writable for WORD5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD5 to value 0
impl crate::Resettable for WORD5rs {}
