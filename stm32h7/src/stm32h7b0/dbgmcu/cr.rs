///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBGSLEEP_CD` reader - Allow D1 domain debug in Sleep mode
pub type DBGSLEEP_CD_R = crate::BitReader;
///Field `DBGSLEEP_CD` writer - Allow D1 domain debug in Sleep mode
pub type DBGSLEEP_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTOP_CD` reader - Allow D1 domain debug in Stop mode
pub type DBGSTOP_CD_R = crate::BitReader;
///Field `DBGSTOP_CD` writer - Allow D1 domain debug in Stop mode
pub type DBGSTOP_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBY_CD` reader - Allow D1 domain debug in Standby mode
pub type DBGSTBY_CD_R = crate::BitReader;
///Field `DBGSTBY_CD` writer - Allow D1 domain debug in Standby mode
pub type DBGSTBY_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTOP_SRD` reader - debug in SmartRun domain Stop mode
pub type DBGSTOP_SRD_R = crate::BitReader;
///Field `DBGSTOP_SRD` writer - debug in SmartRun domain Stop mode
pub type DBGSTOP_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBY_SRD` reader - debug in SmartRun domain Standby mode
pub type DBGSTBY_SRD_R = crate::BitReader;
///Field `DBGSTBY_SRD` writer - debug in SmartRun domain Standby mode
pub type DBGSTBY_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACECLKEN` reader - Trace port clock enable
pub type TRACECLKEN_R = crate::BitReader;
///Field `TRACECLKEN` writer - Trace port clock enable
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDDBGCKEN` reader - CPU domain debug clock enable
pub type CDDBGCKEN_R = crate::BitReader;
///Field `CDDBGCKEN` writer - CPU domain debug clock enable
pub type CDDBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRDDBGCKEN` reader - SmartRun domain debug clock enable
pub type SRDDBGCKEN_R = crate::BitReader;
///Field `SRDDBGCKEN` writer - SmartRun domain debug clock enable
pub type SRDDBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGOEN` reader - External trigger output enable
pub type TRGOEN_R = crate::BitReader;
///Field `TRGOEN` writer - External trigger output enable
pub type TRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Allow D1 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgsleep_cd(&self) -> DBGSLEEP_CD_R {
        DBGSLEEP_CD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstop_cd(&self) -> DBGSTOP_CD_R {
        DBGSTOP_CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstby_cd(&self) -> DBGSTBY_CD_R {
        DBGSTBY_CD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - debug in SmartRun domain Stop mode
    #[inline(always)]
    pub fn dbgstop_srd(&self) -> DBGSTOP_SRD_R {
        DBGSTOP_SRD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - debug in SmartRun domain Standby mode
    #[inline(always)]
    pub fn dbgstby_srd(&self) -> DBGSTBY_SRD_R {
        DBGSTBY_SRD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 20 - Trace port clock enable
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU domain debug clock enable
    #[inline(always)]
    pub fn cddbgcken(&self) -> CDDBGCKEN_R {
        CDDBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SmartRun domain debug clock enable
    #[inline(always)]
    pub fn srddbgcken(&self) -> SRDDBGCKEN_R {
        SRDDBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 28 - External trigger output enable
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbgsleep_cd", &self.dbgsleep_cd())
            .field("dbgstop_cd", &self.dbgstop_cd())
            .field("dbgstby_cd", &self.dbgstby_cd())
            .field("dbgstop_srd", &self.dbgstop_srd())
            .field("dbgstby_srd", &self.dbgstby_srd())
            .field("traceclken", &self.traceclken())
            .field("cddbgcken", &self.cddbgcken())
            .field("srddbgcken", &self.srddbgcken())
            .field("trgoen", &self.trgoen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Allow D1 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgsleep_cd(&mut self) -> DBGSLEEP_CD_W<'_, CRrs> {
        DBGSLEEP_CD_W::new(self, 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstop_cd(&mut self) -> DBGSTOP_CD_W<'_, CRrs> {
        DBGSTOP_CD_W::new(self, 1)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstby_cd(&mut self) -> DBGSTBY_CD_W<'_, CRrs> {
        DBGSTBY_CD_W::new(self, 2)
    }
    ///Bit 7 - debug in SmartRun domain Stop mode
    #[inline(always)]
    pub fn dbgstop_srd(&mut self) -> DBGSTOP_SRD_W<'_, CRrs> {
        DBGSTOP_SRD_W::new(self, 7)
    }
    ///Bit 8 - debug in SmartRun domain Standby mode
    #[inline(always)]
    pub fn dbgstby_srd(&mut self) -> DBGSTBY_SRD_W<'_, CRrs> {
        DBGSTBY_SRD_W::new(self, 8)
    }
    ///Bit 20 - Trace port clock enable
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<'_, CRrs> {
        TRACECLKEN_W::new(self, 20)
    }
    ///Bit 21 - CPU domain debug clock enable
    #[inline(always)]
    pub fn cddbgcken(&mut self) -> CDDBGCKEN_W<'_, CRrs> {
        CDDBGCKEN_W::new(self, 21)
    }
    ///Bit 22 - SmartRun domain debug clock enable
    #[inline(always)]
    pub fn srddbgcken(&mut self) -> SRDDBGCKEN_W<'_, CRrs> {
        SRDDBGCKEN_W::new(self, 22)
    }
    ///Bit 28 - External trigger output enable
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W<'_, CRrs> {
        TRGOEN_W::new(self, 28)
    }
}
/**DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#DBGMCU:CR)*/
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
