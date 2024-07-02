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
///Field `SSHIFT` reader - Sample shift
pub type SSHIFT_R = crate::BitReader;
///Field `SSHIFT` writer - Sample shift
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFM` reader - Dual-flash mode
pub type DFM_R = crate::BitReader;
///Field `DFM` writer - Dual-flash mode
pub type DFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSEL` reader - FLASH memory selection
pub type FSEL_R = crate::BitReader;
///Field `FSEL` writer - FLASH memory selection
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTHRES` reader - IFO threshold level
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - IFO threshold level
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
///Field `TOIE` reader - TimeOut interrupt enable
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - TimeOut interrupt enable
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APMS` reader - Automatic poll mode stop
pub type APMS_R = crate::BitReader;
///Field `APMS` writer - Automatic poll mode stop
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMM` reader - Polling match mode
pub type PMM_R = crate::BitReader;
///Field `PMM` writer - Polling match mode
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCALER` reader - Clock prescaler
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - Clock prescaler
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    ///Bit 4 - Sample shift
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Dual-flash mode
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FLASH memory selection
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:12 - IFO threshold level
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
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
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("prescaler", &self.prescaler())
            .field("pmm", &self.pmm())
            .field("apms", &self.apms())
            .field("toie", &self.toie())
            .field("smie", &self.smie())
            .field("ftie", &self.ftie())
            .field("tcie", &self.tcie())
            .field("teie", &self.teie())
            .field("fthres", &self.fthres())
            .field("fsel", &self.fsel())
            .field("dfm", &self.dfm())
            .field("sshift", &self.sshift())
            .field("tcen", &self.tcen())
            .field("dmaen", &self.dmaen())
            .field("abort", &self.abort())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 4 - Sample shift
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<CRrs> {
        SSHIFT_W::new(self, 4)
    }
    ///Bit 6 - Dual-flash mode
    #[inline(always)]
    #[must_use]
    pub fn dfm(&mut self) -> DFM_W<CRrs> {
        DFM_W::new(self, 6)
    }
    ///Bit 7 - FLASH memory selection
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<CRrs> {
        FSEL_W::new(self, 7)
    }
    ///Bits 8:12 - IFO threshold level
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bits 24:31 - Clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CRrs> {
        PRESCALER_W::new(self, 24)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#QUADSPI:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
