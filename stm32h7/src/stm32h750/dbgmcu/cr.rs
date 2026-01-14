///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBGSLEEP_D1` reader - Allow D1 domain debug in Sleep mode
pub type DBGSLEEP_D1_R = crate::BitReader;
///Field `DBGSLEEP_D1` writer - Allow D1 domain debug in Sleep mode
pub type DBGSLEEP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTOP_D1` reader - Allow D1 domain debug in Stop mode
pub type DBGSTOP_D1_R = crate::BitReader;
///Field `DBGSTOP_D1` writer - Allow D1 domain debug in Stop mode
pub type DBGSTOP_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBY_D1` reader - Allow D1 domain debug in Standby mode
pub type DBGSTBY_D1_R = crate::BitReader;
///Field `DBGSTBY_D1` writer - Allow D1 domain debug in Standby mode
pub type DBGSTBY_D1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACECLKEN` reader - Trace port clock enable
pub type TRACECLKEN_R = crate::BitReader;
///Field `TRACECLKEN` writer - Trace port clock enable
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `D1DBGCKEN` reader - D1 debug clock enable
pub type D1DBGCKEN_R = crate::BitReader;
///Field `D1DBGCKEN` writer - D1 debug clock enable
pub type D1DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `D3DBGCKEN` reader - D3 debug clock enable
pub type D3DBGCKEN_R = crate::BitReader;
///Field `D3DBGCKEN` writer - D3 debug clock enable
pub type D3DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGOEN` reader - External trigger output enable
pub type TRGOEN_R = crate::BitReader;
///Field `TRGOEN` writer - External trigger output enable
pub type TRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Allow D1 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgsleep_d1(&self) -> DBGSLEEP_D1_R {
        DBGSLEEP_D1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstop_d1(&self) -> DBGSTOP_D1_R {
        DBGSTOP_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstby_d1(&self) -> DBGSTBY_D1_R {
        DBGSTBY_D1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 20 - Trace port clock enable
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - D1 debug clock enable
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - D3 debug clock enable
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
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
            .field("dbgsleep_d1", &self.dbgsleep_d1())
            .field("dbgstop_d1", &self.dbgstop_d1())
            .field("dbgstby_d1", &self.dbgstby_d1())
            .field("traceclken", &self.traceclken())
            .field("d1dbgcken", &self.d1dbgcken())
            .field("d3dbgcken", &self.d3dbgcken())
            .field("trgoen", &self.trgoen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Allow D1 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgsleep_d1(&mut self) -> DBGSLEEP_D1_W<'_, CRrs> {
        DBGSLEEP_D1_W::new(self, 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstop_d1(&mut self) -> DBGSTOP_D1_W<'_, CRrs> {
        DBGSTOP_D1_W::new(self, 1)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstby_d1(&mut self) -> DBGSTBY_D1_W<'_, CRrs> {
        DBGSTBY_D1_W::new(self, 2)
    }
    ///Bit 20 - Trace port clock enable
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<'_, CRrs> {
        TRACECLKEN_W::new(self, 20)
    }
    ///Bit 21 - D1 debug clock enable
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W<'_, CRrs> {
        D1DBGCKEN_W::new(self, 21)
    }
    ///Bit 22 - D3 debug clock enable
    #[inline(always)]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W<'_, CRrs> {
        D3DBGCKEN_W::new(self, 22)
    }
    ///Bit 28 - External trigger output enable
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W<'_, CRrs> {
        TRGOEN_W::new(self, 28)
    }
}
/**DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#DBGMCU:CR)*/
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
