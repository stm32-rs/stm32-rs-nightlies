#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DBGSLEEP_CD` reader - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_CD_R = crate::BitReader;
#[doc = "Field `DBGSLEEP_CD` writer - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTOP_CD` reader - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_CD_R = crate::BitReader;
#[doc = "Field `DBGSTOP_CD` writer - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBY_CD` reader - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_CD_R = crate::BitReader;
#[doc = "Field `DBGSTBY_CD` writer - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTOP_SRD` reader - debug in SmartRun domain Stop mode"]
pub type DBGSTOP_SRD_R = crate::BitReader;
#[doc = "Field `DBGSTOP_SRD` writer - debug in SmartRun domain Stop mode"]
pub type DBGSTOP_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBY_SRD` reader - debug in SmartRun domain Standby mode"]
pub type DBGSTBY_SRD_R = crate::BitReader;
#[doc = "Field `DBGSTBY_SRD` writer - debug in SmartRun domain Standby mode"]
pub type DBGSTBY_SRD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TRACECLKEN_R = crate::BitReader;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDDBGCKEN` reader - CPU domain debug clock enable"]
pub type CDDBGCKEN_R = crate::BitReader;
#[doc = "Field `CDDBGCKEN` writer - CPU domain debug clock enable"]
pub type CDDBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRDDBGCKEN` reader - SmartRun domain debug clock enable"]
pub type SRDDBGCKEN_R = crate::BitReader;
#[doc = "Field `SRDDBGCKEN` writer - SmartRun domain debug clock enable"]
pub type SRDDBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TRGOEN_R = crate::BitReader;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_cd(&self) -> DBGSLEEP_CD_R {
        DBGSLEEP_CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_cd(&self) -> DBGSTOP_CD_R {
        DBGSTOP_CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_cd(&self) -> DBGSTBY_CD_R {
        DBGSTBY_CD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    pub fn dbgstop_srd(&self) -> DBGSTOP_SRD_R {
        DBGSTOP_SRD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    pub fn dbgstby_srd(&self) -> DBGSTBY_SRD_R {
        DBGSTBY_SRD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    pub fn cddbgcken(&self) -> CDDBGCKEN_R {
        CDDBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    pub fn srddbgcken(&self) -> SRDDBGCKEN_R {
        SRDDBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsleep_cd(&mut self) -> DBGSLEEP_CD_W<CRrs> {
        DBGSLEEP_CD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_cd(&mut self) -> DBGSTOP_CD_W<CRrs> {
        DBGSTOP_CD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstby_cd(&mut self) -> DBGSTBY_CD_W<CRrs> {
        DBGSTBY_CD_W::new(self, 2)
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_srd(&mut self) -> DBGSTOP_SRD_W<CRrs> {
        DBGSTOP_SRD_W::new(self, 7)
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstby_srd(&mut self) -> DBGSTBY_SRD_W<CRrs> {
        DBGSTBY_SRD_W::new(self, 8)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<CRrs> {
        TRACECLKEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cddbgcken(&mut self) -> CDDBGCKEN_W<CRrs> {
        CDDBGCKEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn srddbgcken(&mut self) -> SRDDBGCKEN_W<CRrs> {
        SRDDBGCKEN_W::new(self, 22)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgoen(&mut self) -> TRGOEN_W<CRrs> {
        TRGOEN_W::new(self, 28)
    }
}
#[doc = "DBGMCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
