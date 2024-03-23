#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DBGSLEEP_D1` reader - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_D1_R = crate::BitReader;
#[doc = "Field `DBGSLEEP_D1` writer - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTOP_D1` reader - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_D1_R = crate::BitReader;
#[doc = "Field `DBGSTOP_D1` writer - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBY_D1` reader - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_D1_R = crate::BitReader;
#[doc = "Field `DBGSTBY_D1` writer - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TRACECLKEN_R = crate::BitReader;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1DBGCKEN` reader - D1 debug clock enable"]
pub type D1DBGCKEN_R = crate::BitReader;
#[doc = "Field `D1DBGCKEN` writer - D1 debug clock enable"]
pub type D1DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D3DBGCKEN` reader - D3 debug clock enable"]
pub type D3DBGCKEN_R = crate::BitReader;
#[doc = "Field `D3DBGCKEN` writer - D3 debug clock enable"]
pub type D3DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TRGOEN_R = crate::BitReader;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_d1(&self) -> DBGSLEEP_D1_R {
        DBGSLEEP_D1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_d1(&self) -> DBGSTOP_D1_R {
        DBGSTOP_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_d1(&self) -> DBGSTBY_D1_R {
        DBGSTBY_D1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
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
    pub fn dbgsleep_d1(&mut self) -> DBGSLEEP_D1_W<CRrs> {
        DBGSLEEP_D1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_d1(&mut self) -> DBGSTOP_D1_W<CRrs> {
        DBGSTOP_D1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstby_d1(&mut self) -> DBGSTBY_D1_W<CRrs> {
        DBGSTBY_D1_W::new(self, 2)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<CRrs> {
        TRACECLKEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W<CRrs> {
        D1DBGCKEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W<CRrs> {
        D3DBGCKEN_W::new(self, 22)
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
