#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - Enable Enable the QUADSPI."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable Enable the QUADSPI."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEN` reader - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\]
(QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\]
(QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
pub type TCEN_R = crate::BitReader;
#[doc = "Field `TCEN` writer - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\]
(QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\]
(QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSHIFT` reader - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
pub type SSHIFT_R = crate::BitReader;
#[doc = "Field `SSHIFT` writer - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFM` reader - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
pub type DFM_R = crate::BitReader;
#[doc = "Field `DFM` writer - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
pub type DFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEL` reader - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
pub type FSEL_R = crate::BitReader;
#[doc = "Field `FSEL` writer - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHRES` reader - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
pub type FTHRES_R = crate::FieldReader;
#[doc = "Field `FTHRES` writer - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FTIE_R = crate::BitReader;
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIE` reader - Status match interrupt enable This bit enables the status match interrupt."]
pub type SMIE_R = crate::BitReader;
#[doc = "Field `SMIE` writer - Status match interrupt enable This bit enables the status match interrupt."]
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIE` reader - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
pub type TOIE_R = crate::BitReader;
#[doc = "Field `TOIE` writer - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APMS` reader - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
pub type APMS_R = crate::BitReader;
#[doc = "Field `APMS` writer - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMM` reader - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
pub type PMM_R = crate::BitReader;
#[doc = "Field `PMM` writer - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALER` reader - clock prescaler"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - clock prescaler"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable Enable the QUADSPI."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\]
(QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\]
(QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Enable the QUADSPI."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\]
(QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\]
(QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<CRrs> {
        TCEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<CRrs> {
        SSHIFT_W::new(self, 4)
    }
    #[doc = "Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dfm(&mut self) -> DFM_W<CRrs> {
        DFM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<CRrs> {
        FSEL_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<CRrs> {
        FTHRES_W::new(self, 8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<CRrs> {
        FTIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<CRrs> {
        SMIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 20)
    }
    #[doc = "Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<CRrs> {
        APMS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<CRrs> {
        PMM_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CRrs> {
        PRESCALER_W::new(self, 24)
    }
}
#[doc = "QUADSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
