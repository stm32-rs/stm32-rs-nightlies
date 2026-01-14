///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` reader - ABORT
pub type ABORT_R = crate::BitReader;
///Field `ABORT` writer - ABORT
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCEN` reader - TCEN
pub type TCEN_R = crate::BitReader;
///Field `TCEN` writer - TCEN
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSHIFT` reader - SSHIFT
pub type SSHIFT_R = crate::BitReader;
///Field `SSHIFT` writer - SSHIFT
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFM` reader - DFM
pub type DFM_R = crate::BitReader;
///Field `DFM` writer - DFM
pub type DFM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSEL` reader - FSEL
pub type FSEL_R = crate::BitReader;
///Field `FSEL` writer - FSEL
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTHRES` reader - FTHRES
pub type FTHRES_R = crate::FieldReader;
///Field `FTHRES` writer - FTHRES
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TEIE` reader - TEIE
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - TEIE
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTIE` reader - FTIE
pub type FTIE_R = crate::BitReader;
///Field `FTIE` writer - FTIE
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMIE` reader - SMIE
pub type SMIE_R = crate::BitReader;
///Field `SMIE` writer - SMIE
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOIE` reader - TOIE
pub type TOIE_R = crate::BitReader;
///Field `TOIE` writer - TOIE
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APMS` reader - APMS
pub type APMS_R = crate::BitReader;
///Field `APMS` writer - APMS
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMM` reader - PMM
pub type PMM_R = crate::BitReader;
///Field `PMM` writer - PMM
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCALER` reader - PRESCALER
pub type PRESCALER_R = crate::FieldReader;
///Field `PRESCALER` writer - PRESCALER
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ABORT
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TCEN
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SSHIFT
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - DFM
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FSEL
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - FTHRES
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - TEIE
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FTIE
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SMIE
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TOIE
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - APMS
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PMM
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - PRESCALER
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("abort", &self.abort())
            .field("dmaen", &self.dmaen())
            .field("tcen", &self.tcen())
            .field("sshift", &self.sshift())
            .field("dfm", &self.dfm())
            .field("fsel", &self.fsel())
            .field("fthres", &self.fthres())
            .field("teie", &self.teie())
            .field("tcie", &self.tcie())
            .field("ftie", &self.ftie())
            .field("smie", &self.smie())
            .field("toie", &self.toie())
            .field("apms", &self.apms())
            .field("pmm", &self.pmm())
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - ABORT
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W<'_, CRrs> {
        ABORT_W::new(self, 1)
    }
    ///Bit 2 - DMAEN
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CRrs> {
        DMAEN_W::new(self, 2)
    }
    ///Bit 3 - TCEN
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W<'_, CRrs> {
        TCEN_W::new(self, 3)
    }
    ///Bit 4 - SSHIFT
    #[inline(always)]
    pub fn sshift(&mut self) -> SSHIFT_W<'_, CRrs> {
        SSHIFT_W::new(self, 4)
    }
    ///Bit 6 - DFM
    #[inline(always)]
    pub fn dfm(&mut self) -> DFM_W<'_, CRrs> {
        DFM_W::new(self, 6)
    }
    ///Bit 7 - FSEL
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W<'_, CRrs> {
        FSEL_W::new(self, 7)
    }
    ///Bits 8:11 - FTHRES
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W<'_, CRrs> {
        FTHRES_W::new(self, 8)
    }
    ///Bit 16 - TEIE
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 16)
    }
    ///Bit 17 - TCIE
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 17)
    }
    ///Bit 18 - FTIE
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W<'_, CRrs> {
        FTIE_W::new(self, 18)
    }
    ///Bit 19 - SMIE
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W<'_, CRrs> {
        SMIE_W::new(self, 19)
    }
    ///Bit 20 - TOIE
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<'_, CRrs> {
        TOIE_W::new(self, 20)
    }
    ///Bit 22 - APMS
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W<'_, CRrs> {
        APMS_W::new(self, 22)
    }
    ///Bit 23 - PMM
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W<'_, CRrs> {
        PMM_W::new(self, 23)
    }
    ///Bits 24:31 - PRESCALER
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<'_, CRrs> {
        PRESCALER_W::new(self, 24)
    }
}
/**QUADSPI control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#QUADSPI:CR)*/
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
