///Register `PRSR` reader
pub type R = crate::R<PRSRrs>;
///Field `ERRF` reader - Synchronization error raw interrupt status This bit is valid only in the embedded synchronization mode. It is cleared by writing a 1 to the CERRF bit in DCMIPP_PRFCR. This bit is available only in embedded synchronization mode.
pub type ERRF_R = crate::BitReader;
///Field `HSYNC` reader - This bit gives the state of the HSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit, and cleared otherwise. When embedded synchronization codes are used: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMIPP_CR is set.
pub type HSYNC_R = crate::BitReader;
///Field `VSYNC` reader - This bit gives the state of the VSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit, and cleared otherwise. When embedded synchronization codes are used: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMIPP_CR is set.
pub type VSYNC_R = crate::BitReader;
impl R {
    ///Bit 6 - Synchronization error raw interrupt status This bit is valid only in the embedded synchronization mode. It is cleared by writing a 1 to the CERRF bit in DCMIPP_PRFCR. This bit is available only in embedded synchronization mode.
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - This bit gives the state of the HSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit, and cleared otherwise. When embedded synchronization codes are used: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMIPP_CR is set.
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - This bit gives the state of the VSYNC pin with the correct programmed polarity if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit, and cleared otherwise. When embedded synchronization codes are used: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in DCMIPP_CR is set.
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSR")
            .field("errf", &self.errf())
            .field("hsync", &self.hsync())
            .field("vsync", &self.vsync())
            .finish()
    }
}
/**DCMIPP parallel interface status register

You can [`read`](crate::Reg::read) this register and get [`prsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:PRSR)*/
pub struct PRSRrs;
impl crate::RegisterSpec for PRSRrs {
    type Ux = u32;
}
///`read()` method returns [`prsr::R`](R) reader structure
impl crate::Readable for PRSRrs {}
///`reset()` method sets PRSR to value 0x0003_0000
impl crate::Resettable for PRSRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
