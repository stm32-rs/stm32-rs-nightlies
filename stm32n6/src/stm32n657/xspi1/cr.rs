///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - Abort request
pub type ABORT_R = crate::BitReader;
///Field `ABORT` writer - Abort request
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA enable
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCEN` reader - Timeout counter enable
pub type TCEN_R = crate::BitReader;
///Field `TCEN` writer - Timeout counter enable
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMM` reader - Dual-memory configuration
pub type DMM_R = crate::BitReader;
///Field `DMM` writer - Dual-memory configuration
pub type DMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTHRES` reader - FIFO threshold level
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - FIFO threshold level
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTIE` reader - FIFO threshold interrupt enable
pub type FTIE_R = crate::BitReader;
///Field `FTIE` writer - FIFO threshold interrupt enable
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMIE` reader - Status match interrupt enable
pub type SMIE_R = crate::BitReader;
///Field `SMIE` writer - Status match interrupt enable
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOIE` reader - Timeout interrupt enable
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - Timeout interrupt enable
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APMS` reader - Automatic status-polling mode stop
pub type APMS_R = crate::BitReader;
///Field `APMS` writer - Automatic status-polling mode stop
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMM` reader - Polling match mode
pub type PMM_R = crate::BitReader;
///Field `PMM` writer - Polling match mode
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSEL` reader - chip select selection
pub type CSSEL_R = crate::BitReader;
///Field `CSSEL` writer - chip select selection
pub type CSSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOPREF` reader - no prefetch data
pub type NOPREF_R = crate::BitReader;
///Field `NOPREF` writer - no prefetch data
pub type NOPREF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOPREF_AXI` reader - no prefetch for signaled AXI transactions
pub type NOPREF_AXI_R = crate::BitReader;
///Field `NOPREF_AXI` writer - no prefetch for signaled AXI transactions
pub type NOPREF_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMODE` reader - Functional mode
pub type FMODE_R = crate::FieldReader;
///Field `FMODE` writer - Functional mode
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MSEL` reader - Flash select
pub type MSEL_R = crate::FieldReader;
///Field `MSEL` writer - Flash select
pub type MSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Dual-memory configuration
    #[inline(always)]
    pub fn dmm(&self) -> DMM_R {
        DMM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:13 - FIFO threshold level
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timeout interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic status-polling mode stop
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - chip select selection
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - no prefetch data
    #[inline(always)]
    pub fn nopref(&self) -> NOPREF_R {
        NOPREF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - no prefetch for signaled AXI transactions
    #[inline(always)]
    pub fn nopref_axi(&self) -> NOPREF_AXI_R {
        NOPREF_AXI_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Flash select
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("abort", &self.abort())
            .field("dmaen", &self.dmaen())
            .field("tcen", &self.tcen())
            .field("dmm", &self.dmm())
            .field("fthres", &self.fthres())
            .field("teie", &self.teie())
            .field("tcie", &self.tcie())
            .field("ftie", &self.ftie())
            .field("smie", &self.smie())
            .field("toie", &self.toie())
            .field("apms", &self.apms())
            .field("pmm", &self.pmm())
            .field("cssel", &self.cssel())
            .field("nopref", &self.nopref())
            .field("nopref_axi", &self.nopref_axi())
            .field("fmode", &self.fmode())
            .field("msel", &self.msel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<'_, CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 6 - Dual-memory configuration
    #[inline(always)]
    pub fn dmm(&mut self) -> DMM_W<'_, CRrs> {
        DMM_W::new(self, 6)
    }
    ///Bits 8:13 - FIFO threshold level
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<'_, CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<'_, CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<'_, CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - Timeout interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<'_, CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - Automatic status-polling mode stop
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<'_, CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<'_, CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bit 24 - chip select selection
    #[inline(always)]
    pub fn cssel(&mut self) -> CSSEL_W<'_, CRrs> {
        CSSEL_W::new(self, 24)
    }
    ///Bit 25 - no prefetch data
    #[inline(always)]
    pub fn nopref(&mut self) -> NOPREF_W<'_, CRrs> {
        NOPREF_W::new(self, 25)
    }
    ///Bit 26 - no prefetch for signaled AXI transactions
    #[inline(always)]
    pub fn nopref_axi(&mut self) -> NOPREF_AXI_W<'_, CRrs> {
        NOPREF_AXI_W::new(self, 26)
    }
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W<'_, CRrs> {
        FMODE_W::new(self, 28)
    }
    ///Bits 30:31 - Flash select
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<'_, CRrs> {
        MSEL_W::new(self, 30)
    }
}
/**XSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#XSPI1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
