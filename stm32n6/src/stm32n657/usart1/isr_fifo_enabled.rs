///Register `ISR_FIFO_ENABLED` reader
pub type R = crate::R<ISR_FIFO_ENABLEDrs>;
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader;
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader;
///Field `NE` reader - Noise detection flag
pub type NE_R = crate::BitReader;
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader;
///Field `IDLE` reader - Idle line detected
pub type IDLE_R = crate::BitReader;
///Field `RXFNE` reader - RXFIFO not empty
pub type RXFNE_R = crate::BitReader;
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader;
///Field `TXFNF` reader - TXFIFO not full
pub type TXFNF_R = crate::BitReader;
///Field `LBDF` reader - LIN break detection flag
pub type LBDF_R = crate::BitReader;
///Field `CTSIF` reader - CTS interrupt flag
pub type CTSIF_R = crate::BitReader;
///Field `CTS` reader - CTS flag
pub type CTS_R = crate::BitReader;
///Field `RTOF` reader - Receiver timeout
pub type RTOF_R = crate::BitReader;
///Field `EOBF` reader - End of block flag
pub type EOBF_R = crate::BitReader;
///Field `UDR` reader - SPI slave underrun error flag
pub type UDR_R = crate::BitReader;
///Field `ABRE` reader - Auto baud rate error
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - Auto baud rate flag
pub type ABRF_R = crate::BitReader;
///Field `BUSY` reader - Busy flag
pub type BUSY_R = crate::BitReader;
///Field `CMF` reader - Character match flag
pub type CMF_R = crate::BitReader;
///Field `SBKF` reader - Send break flag
pub type SBKF_R = crate::BitReader;
///Field `RWU` reader - Receiver wakeup from Mute mode
pub type RWU_R = crate::BitReader;
///Field `WUF` reader - Wakeup from low-power mode flag
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - Transmit enable acknowledge flag
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - Receive enable acknowledge flag
pub type REACK_R = crate::BitReader;
///Field `TXFE` reader - TXFIFO Empty
pub type TXFE_R = crate::BitReader;
///Field `RXFF` reader - RXFIFO Full
pub type RXFF_R = crate::BitReader;
///Field `TCBGT` reader - Transmission complete before guard time flag
pub type TCBGT_R = crate::BitReader;
///Field `RXFT` reader - RXFIFO threshold flag
pub type RXFT_R = crate::BitReader;
///Field `TXFT` reader - TXFIFO threshold flag
pub type TXFT_R = crate::BitReader;
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detection flag
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not full
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS interrupt flag
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS flag
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver timeout
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - End of block flag
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI slave underrun error flag
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Auto baud rate error
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto baud rate flag
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Character match flag
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Send break flag
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receiver wakeup from Mute mode
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Wakeup from low-power mode flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transmit enable acknowledge flag
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Receive enable acknowledge flag
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO Empty
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFIFO Full
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Transmission complete before guard time flag
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RXFIFO threshold flag
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFIFO threshold flag
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR_FIFO_ENABLED")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("ne", &self.ne())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxfne", &self.rxfne())
            .field("tc", &self.tc())
            .field("txfnf", &self.txfnf())
            .field("lbdf", &self.lbdf())
            .field("ctsif", &self.ctsif())
            .field("cts", &self.cts())
            .field("rtof", &self.rtof())
            .field("eobf", &self.eobf())
            .field("udr", &self.udr())
            .field("abre", &self.abre())
            .field("abrf", &self.abrf())
            .field("busy", &self.busy())
            .field("cmf", &self.cmf())
            .field("sbkf", &self.sbkf())
            .field("rwu", &self.rwu())
            .field("wuf", &self.wuf())
            .field("teack", &self.teack())
            .field("reack", &self.reack())
            .field("txfe", &self.txfe())
            .field("rxff", &self.rxff())
            .field("tcbgt", &self.tcbgt())
            .field("rxft", &self.rxft())
            .field("txft", &self.txft())
            .finish()
    }
}
/**USART interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_fifo_enabled::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#USART1:ISR_FIFO_ENABLED)*/
pub struct ISR_FIFO_ENABLEDrs;
impl crate::RegisterSpec for ISR_FIFO_ENABLEDrs {
    type Ux = u32;
}
///`read()` method returns [`isr_fifo_enabled::R`](R) reader structure
impl crate::Readable for ISR_FIFO_ENABLEDrs {}
///`reset()` method sets ISR_FIFO_ENABLED to value 0xc0
impl crate::Resettable for ISR_FIFO_ENABLEDrs {
    const RESET_VALUE: u32 = 0xc0;
}
