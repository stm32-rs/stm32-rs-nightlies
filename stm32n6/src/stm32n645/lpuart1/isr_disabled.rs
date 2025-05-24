///Register `ISR_DISABLED` reader
pub type R = crate::R<ISR_DISABLEDrs>;
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader;
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader;
///Field `NE` reader - Start bit noise detection flag
pub type NE_R = crate::BitReader;
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader;
///Field `IDLE` reader - Idle line detected
pub type IDLE_R = crate::BitReader;
///Field `RXNE` reader - Read data register not empty
pub type RXNE_R = crate::BitReader;
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader;
///Field `TXE` reader - Transmit data register empty
pub type TXE_R = crate::BitReader;
///Field `CTSIF` reader - CTS interrupt flag
pub type CTSIF_R = crate::BitReader;
///Field `CTS` reader - CTS flag
pub type CTS_R = crate::BitReader;
///Field `BUSY` reader - Busy flag
pub type BUSY_R = crate::BitReader;
///Field `CMF` reader - Character match flag
pub type CMF_R = crate::BitReader;
///Field `SBKF` reader - Send break flag
pub type SBKF_R = crate::BitReader;
///Field `RWU` reader - Receiver wake-up from Mute mode
pub type RWU_R = crate::BitReader;
///Field `WUF` reader - Wake-up from low-power mode flag
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - Transmit enable acknowledge flag
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - Receive enable acknowledge flag
pub type REACK_R = crate::BitReader;
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
    ///Bit 2 - Start bit noise detection flag
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
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 19 - Receiver wake-up from Mute mode
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Wake-up from low-power mode flag
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR_DISABLED")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("ne", &self.ne())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxne", &self.rxne())
            .field("tc", &self.tc())
            .field("txe", &self.txe())
            .field("ctsif", &self.ctsif())
            .field("cts", &self.cts())
            .field("busy", &self.busy())
            .field("cmf", &self.cmf())
            .field("sbkf", &self.sbkf())
            .field("rwu", &self.rwu())
            .field("wuf", &self.wuf())
            .field("teack", &self.teack())
            .field("reack", &self.reack())
            .finish()
    }
}
/**LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr_disabled::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LPUART1:ISR_DISABLED)*/
pub struct ISR_DISABLEDrs;
impl crate::RegisterSpec for ISR_DISABLEDrs {
    type Ux = u32;
}
///`read()` method returns [`isr_disabled::R`](R) reader structure
impl crate::Readable for ISR_DISABLEDrs {}
///`reset()` method sets ISR_DISABLED to value 0x0080_00c0
impl crate::Resettable for ISR_DISABLEDrs {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
