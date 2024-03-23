#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::BitReader;
#[doc = "Field `FE` reader - Framing error"]
pub type FE_R = crate::BitReader;
#[doc = "Field `NF` reader - Noise detected flag"]
pub type NF_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error"]
pub type ORE_R = crate::BitReader;
#[doc = "Field `IDLE` reader - Idle line detected"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTS interrupt flag"]
pub type CTSIF_R = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `RTOF` reader - Receiver timeout"]
pub type RTOF_R = crate::BitReader;
#[doc = "Field `EOBF` reader - End of block flag"]
pub type EOBF_R = crate::BitReader;
#[doc = "Field `ABRE` reader - Auto baud rate error"]
pub type ABRE_R = crate::BitReader;
#[doc = "Field `ABRF` reader - Auto baud rate flag"]
pub type ABRF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `CMF` reader - character match flag"]
pub type CMF_R = crate::BitReader;
#[doc = "Field `SBKF` reader - Send break flag"]
pub type SBKF_R = crate::BitReader;
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `WUF` reader - Wakeup from Stop mode flag"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag"]
pub type TEACK_R = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag"]
pub type REACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag"]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout"]
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error"]
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag"]
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - character match flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag"]
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag"]
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag"]
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Interrupt &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0xc0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0xc0;
}
