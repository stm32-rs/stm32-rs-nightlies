///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RXNE` reader - Read data register not empty This bit is set by hardware when a valid data is available into SPDIFRX_FMTx_DR register. This flag is cleared by reading the SPDIFRX_FMTx_DR register. An interrupt is generated if RXNEIE=1 in the SPDIFRX_IMR register.
pub type RXNE_R = crate::BitReader;
///Field `CSRNE` reader - Control buffer register not empty This bit is set by hardware when a valid control information is ready. This flag is cleared when reading SPDIFRX_CSR register. An interrupt is generated if CBRDYIE = 1 in the SPDIFRX_IMR register.
pub type CSRNE_R = crate::BitReader;
///Field `PERR` reader - Parity error This bit is set by hardware when the data and status bits of the sub-frame received contain an odd number of 0 and 1. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if PIE = 1 in the SPDIFRX_IMR register.
pub type PERR_R = crate::BitReader;
///Field `OVR` reader - Overrun error This bit is set by hardware when a received data is ready to be transferred in the SPDIFRX_FMTx_DR register while RXNE = 1 and both SPDIFRX_FMTx_DR and RX_BUF are full. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if OVRIE=1 in the SPDIFRX_IMR register. Note: When this bit is set, the SPDIFRX_FMTx_DR register content is not lost but the last data received are.
pub type OVR_R = crate::BitReader;
///Field `SBD` reader - Synchronization block detected This bit is set by hardware when a B preamble is detected. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if SBLKIE = 1 in the SPDIFRX_IMR register.
pub type SBD_R = crate::BitReader;
///Field `SYNCD` reader - Synchronization done This bit is set by hardware when the initial synchronization phase is properly completed. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if SYNCDIE = 1 in the SPDIFRX_IMR register.
pub type SYNCD_R = crate::BitReader;
///Field `FERR` reader - Framing error This bit is set by hardware when an error occurs during data reception: such as preamble not at the expected place, short transition not grouped by pairs. This is set by the hardware only if the synchronization is completed (SYNCD = 1). This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE=1 in the SPDIFRX_IMR register.
pub type FERR_R = crate::BitReader;
///Field `SERR` reader - Synchronization error This bit is set by hardware when the synchronization fails due to amount of re-tries for NBTR. This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE = 1 in the SPDIFRX_IMR register.
pub type SERR_R = crate::BitReader;
///Field `TERR` reader - Time-out error This bit is set by hardware when the counter TRCNT reaches its max value. It indicates that the time interval between two transitions is too long. It generally indicates that there is no valid signal on SPDIFRX_IN input. This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE=1 in the SPDIFRX_IMR register.
pub type TERR_R = crate::BitReader;
///Field `WIDTH5` reader - duration of 5 symbols counted with spdifrx_ker_ck This value represents the amount of spdifrx_ker_ck clock periods contained on a length of 5 consecutive symbols. This value can be used to estimate the S/PDIF symbol rate. Its accuracy is limited by the frequency of spdifrx_ker_ck. For example if the spdifrx_ker_ck is fixed to 84 MHz, and WIDTH5 = 147d. The estimated sampling rate of the S/PDIF stream is: Fs = 5 x F<sub>spdifrx_ker_ck</sub> / (WIDTH5 x 64) ~ 44.6 kHz, so the closest standard sampling rate is 44.1 kHz. Note that WIDTH5 is updated by the hardware when SYNCD goes high, and then every frame.
pub type WIDTH5_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - Read data register not empty This bit is set by hardware when a valid data is available into SPDIFRX_FMTx_DR register. This flag is cleared by reading the SPDIFRX_FMTx_DR register. An interrupt is generated if RXNEIE=1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control buffer register not empty This bit is set by hardware when a valid control information is ready. This flag is cleared when reading SPDIFRX_CSR register. An interrupt is generated if CBRDYIE = 1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Parity error This bit is set by hardware when the data and status bits of the sub-frame received contain an odd number of 0 and 1. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if PIE = 1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error This bit is set by hardware when a received data is ready to be transferred in the SPDIFRX_FMTx_DR register while RXNE = 1 and both SPDIFRX_FMTx_DR and RX_BUF are full. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if OVRIE=1 in the SPDIFRX_IMR register. Note: When this bit is set, the SPDIFRX_FMTx_DR register content is not lost but the last data received are.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization block detected This bit is set by hardware when a B preamble is detected. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if SBLKIE = 1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization done This bit is set by hardware when the initial synchronization phase is properly completed. This flag is cleared by writing a 1 to its corresponding bit on SPDIFRX_IFCR register. An interrupt is generated if SYNCDIE = 1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Framing error This bit is set by hardware when an error occurs during data reception: such as preamble not at the expected place, short transition not grouped by pairs. This is set by the hardware only if the synchronization is completed (SYNCD = 1). This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE=1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization error This bit is set by hardware when the synchronization fails due to amount of re-tries for NBTR. This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE = 1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Time-out error This bit is set by hardware when the counter TRCNT reaches its max value. It indicates that the time interval between two transitions is too long. It generally indicates that there is no valid signal on SPDIFRX_IN input. This flag is cleared by writing SPDIFRXEN to 0. An interrupt is generated if IFEIE=1 in the SPDIFRX_IMR register.
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:30 - duration of 5 symbols counted with spdifrx_ker_ck This value represents the amount of spdifrx_ker_ck clock periods contained on a length of 5 consecutive symbols. This value can be used to estimate the S/PDIF symbol rate. Its accuracy is limited by the frequency of spdifrx_ker_ck. For example if the spdifrx_ker_ck is fixed to 84 MHz, and WIDTH5 = 147d. The estimated sampling rate of the S/PDIF stream is: Fs = 5 x F<sub>spdifrx_ker_ck</sub> / (WIDTH5 x 64) ~ 44.6 kHz, so the closest standard sampling rate is 44.1 kHz. Note that WIDTH5 is updated by the hardware when SYNCD goes high, and then every frame.
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rxne", &self.rxne())
            .field("csrne", &self.csrne())
            .field("perr", &self.perr())
            .field("ovr", &self.ovr())
            .field("sbd", &self.sbd())
            .field("syncd", &self.syncd())
            .field("ferr", &self.ferr())
            .field("serr", &self.serr())
            .field("terr", &self.terr())
            .field("width5", &self.width5())
            .finish()
    }
}
/**SPDIFRX status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SPDIFRX:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
