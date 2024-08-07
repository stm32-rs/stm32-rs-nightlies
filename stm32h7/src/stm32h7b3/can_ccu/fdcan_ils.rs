///Register `FDCAN_ILS` reader
pub type R = crate::R<FDCAN_ILSrs>;
///Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line
pub type RF0NL_R = crate::BitReader;
///Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line
pub type RF0WL_R = crate::BitReader;
///Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line
pub type RF0FL_R = crate::BitReader;
///Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line
pub type RF0LL_R = crate::BitReader;
///Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line
pub type RF1NL_R = crate::BitReader;
///Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line
pub type RF1WL_R = crate::BitReader;
///Field `RF1FL` reader - Rx FIFO 1 Full Interrupt Line
pub type RF1FL_R = crate::BitReader;
///Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line
pub type RF1LL_R = crate::BitReader;
///Field `HPML` reader - High Priority Message Interrupt Line
pub type HPML_R = crate::BitReader;
///Field `TCL` reader - Transmission Completed Interrupt Line
pub type TCL_R = crate::BitReader;
///Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line
pub type TCFL_R = crate::BitReader;
///Field `TEFL` reader - Tx FIFO Empty Interrupt Line
pub type TEFL_R = crate::BitReader;
///Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line
pub type TEFNL_R = crate::BitReader;
///Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line
pub type TEFWL_R = crate::BitReader;
///Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line
pub type TEFFL_R = crate::BitReader;
///Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line
pub type TEFLL_R = crate::BitReader;
///Field `TSWL` reader - Timestamp Wraparound Interrupt Line
pub type TSWL_R = crate::BitReader;
///Field `MRAFL` reader - Message RAM Access Failure Interrupt Line
pub type MRAFL_R = crate::BitReader;
///Field `TOOL` reader - Timeout Occurred Interrupt Line
pub type TOOL_R = crate::BitReader;
///Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line
pub type DRXL_R = crate::BitReader;
///Field `BECL` reader - Bit Error Corrected Interrupt Line
pub type BECL_R = crate::BitReader;
///Field `BEUL` reader - Bit Error Uncorrected Interrupt Line
pub type BEUL_R = crate::BitReader;
///Field `ELOL` reader - Error Logging Overflow Interrupt Line
pub type ELOL_R = crate::BitReader;
///Field `EPL` reader - Error Passive Interrupt Line
pub type EPL_R = crate::BitReader;
///Field `EWL` reader - Warning Status Interrupt Line
pub type EWL_R = crate::BitReader;
///Field `BOL` reader - Bus_Off Status
pub type BOL_R = crate::BitReader;
///Field `WDIL` reader - Watchdog Interrupt Line
pub type WDIL_R = crate::BitReader;
///Field `PEAL` reader - Protocol Error in Arbitration Phase Line
pub type PEAL_R = crate::BitReader;
///Field `PEDL` reader - Protocol Error in Data Phase Line
pub type PEDL_R = crate::BitReader;
///Field `ARAL` reader - Access to Reserved Address Line
pub type ARAL_R = crate::BitReader;
impl R {
    ///Bit 0 - Rx FIFO 0 New Message Interrupt Line
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 Full Interrupt Line
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 0 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 New Message Interrupt Line
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx FIFO 1 Full Interrupt Line
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rx FIFO 1 Message Lost Interrupt Line
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - High Priority Message Interrupt Line
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission Completed Interrupt Line
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission Cancellation Finished Interrupt Line
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx FIFO Empty Interrupt Line
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx Event FIFO New Entry Interrupt Line
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx Event FIFO Full Interrupt Line
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Tx Event FIFO Element Lost Interrupt Line
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Timestamp Wraparound Interrupt Line
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Message RAM Access Failure Interrupt Line
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timeout Occurred Interrupt Line
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bit Error Corrected Interrupt Line
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Bit Error Uncorrected Interrupt Line
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Error Logging Overflow Interrupt Line
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Error Passive Interrupt Line
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Warning Status Interrupt Line
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bus_Off Status
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Watchdog Interrupt Line
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Protocol Error in Arbitration Phase Line
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Protocol Error in Data Phase Line
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Access to Reserved Address Line
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILS")
            .field("rf0nl", &self.rf0nl())
            .field("rf0wl", &self.rf0wl())
            .field("rf0fl", &self.rf0fl())
            .field("rf0ll", &self.rf0ll())
            .field("rf1nl", &self.rf1nl())
            .field("rf1wl", &self.rf1wl())
            .field("rf1fl", &self.rf1fl())
            .field("rf1ll", &self.rf1ll())
            .field("hpml", &self.hpml())
            .field("tcl", &self.tcl())
            .field("tcfl", &self.tcfl())
            .field("tefl", &self.tefl())
            .field("tefnl", &self.tefnl())
            .field("tefwl", &self.tefwl())
            .field("teffl", &self.teffl())
            .field("tefll", &self.tefll())
            .field("tswl", &self.tswl())
            .field("mrafl", &self.mrafl())
            .field("tool", &self.tool())
            .field("drxl", &self.drxl())
            .field("becl", &self.becl())
            .field("beul", &self.beul())
            .field("elol", &self.elol())
            .field("epl", &self.epl())
            .field("ewl", &self.ewl())
            .field("bol", &self.bol())
            .field("wdil", &self.wdil())
            .field("peal", &self.peal())
            .field("pedl", &self.pedl())
            .field("aral", &self.aral())
            .finish()
    }
}
/**FDCAN Interrupt Line Select Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_ILS)*/
pub struct FDCAN_ILSrs;
impl crate::RegisterSpec for FDCAN_ILSrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ils::R`](R) reader structure
impl crate::Readable for FDCAN_ILSrs {}
///`reset()` method sets FDCAN_ILS to value 0
impl crate::Resettable for FDCAN_ILSrs {
    const RESET_VALUE: u32 = 0;
}
