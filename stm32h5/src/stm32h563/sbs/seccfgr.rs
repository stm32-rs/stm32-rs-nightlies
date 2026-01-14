///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SBSSEC` reader - SBS clock control, memory-erase status register and compensation cell register security enable
pub type SBSSEC_R = crate::BitReader;
///Field `SBSSEC` writer - SBS clock control, memory-erase status register and compensation cell register security enable
pub type SBSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLASSBSEC` reader - ClassB security enable
pub type CLASSBSEC_R = crate::BitReader;
///Field `CLASSBSEC` writer - ClassB security enable
pub type CLASSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPUSEC` reader - FPU security enable Note: This bit can only be written through privilege transaction.
pub type FPUSEC_R = crate::BitReader;
///Field `FPUSEC` writer - FPU security enable Note: This bit can only be written through privilege transaction.
pub type FPUSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDCE_SEC_EN` reader - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
pub type SDCE_SEC_EN_R = crate::BitReader;
///Field `SDCE_SEC_EN` writer - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
pub type SDCE_SEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable
    #[inline(always)]
    pub fn sbssec(&self) -> SBSSEC_R {
        SBSSEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ClassB security enable
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction.
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
    #[inline(always)]
    pub fn sdce_sec_en(&self) -> SDCE_SEC_EN_R {
        SDCE_SEC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sbssec", &self.sbssec())
            .field("classbsec", &self.classbsec())
            .field("fpusec", &self.fpusec())
            .field("sdce_sec_en", &self.sdce_sec_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable
    #[inline(always)]
    pub fn sbssec(&mut self) -> SBSSEC_W<'_, SECCFGRrs> {
        SBSSEC_W::new(self, 0)
    }
    ///Bit 1 - ClassB security enable
    #[inline(always)]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<'_, SECCFGRrs> {
        CLASSBSEC_W::new(self, 1)
    }
    ///Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction.
    #[inline(always)]
    pub fn fpusec(&mut self) -> FPUSEC_W<'_, SECCFGRrs> {
        FPUSEC_W::new(self, 3)
    }
    ///Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR
    #[inline(always)]
    pub fn sdce_sec_en(&mut self) -> SDCE_SEC_EN_W<'_, SECCFGRrs> {
        SDCE_SEC_EN_W::new(self, 31)
    }
}
/**SBS security mode configuration control register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SBS:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
