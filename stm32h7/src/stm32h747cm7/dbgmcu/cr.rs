///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBGSLPD1` reader - Allow D1 domain debug in Sleep mode
pub type DBGSLPD1_R = crate::BitReader;
///Field `DBGSLPD1` writer - Allow D1 domain debug in Sleep mode
pub type DBGSLPD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTPD1` reader - Allow D1 domain debug in Stop mode
pub type DBGSTPD1_R = crate::BitReader;
///Field `DBGSTPD1` writer - Allow D1 domain debug in Stop mode
pub type DBGSTPD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBD1` reader - Allow D1 domain debug in Standby mode
pub type DBGSTBD1_R = crate::BitReader;
///Field `DBGSTBD1` writer - Allow D1 domain debug in Standby mode
pub type DBGSTBD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSLPD2` reader - Allow D2 domain debug in Sleep mode
pub type DBGSLPD2_R = crate::BitReader;
///Field `DBGSLPD2` writer - Allow D2 domain debug in Sleep mode
pub type DBGSLPD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTPD2` reader - Allow D2 domain debug in Stop mode
pub type DBGSTPD2_R = crate::BitReader;
///Field `DBGSTPD2` writer - Allow D2 domain debug in Stop mode
pub type DBGSTPD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBD2` reader - Allow D2 domain debug in Standby mode
pub type DBGSTBD2_R = crate::BitReader;
///Field `DBGSTBD2` writer - Allow D2 domain debug in Standby mode
pub type DBGSTBD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTPD3` reader - Allow debug in D3 Stop mode
pub type DBGSTPD3_R = crate::BitReader;
///Field `DBGSTPD3` writer - Allow debug in D3 Stop mode
pub type DBGSTPD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBD3` reader - Allow debug in D3 Standby mode
pub type DBGSTBD3_R = crate::BitReader;
///Field `DBGSTBD3` writer - Allow debug in D3 Standby mode
pub type DBGSTBD3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn dbgslpd1(&self) -> DBGSLPD1_R {
        DBGSLPD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstpd1(&self) -> DBGSTPD1_R {
        DBGSTPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstbd1(&self) -> DBGSTBD1_R {
        DBGSTBD1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Allow D2 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgslpd2(&self) -> DBGSLPD2_R {
        DBGSLPD2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Allow D2 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstpd2(&self) -> DBGSTPD2_R {
        DBGSTPD2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Allow D2 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstbd2(&self) -> DBGSTBD2_R {
        DBGSTBD2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Allow debug in D3 Stop mode
    #[inline(always)]
    pub fn dbgstpd3(&self) -> DBGSTPD3_R {
        DBGSTPD3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Allow debug in D3 Standby mode
    #[inline(always)]
    pub fn dbgstbd3(&self) -> DBGSTBD3_R {
        DBGSTBD3_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("dbgslpd1", &self.dbgslpd1())
            .field("dbgstpd1", &self.dbgstpd1())
            .field("dbgstbd1", &self.dbgstbd1())
            .field("dbgslpd2", &self.dbgslpd2())
            .field("dbgstpd2", &self.dbgstpd2())
            .field("dbgstbd2", &self.dbgstbd2())
            .field("dbgstpd3", &self.dbgstpd3())
            .field("dbgstbd3", &self.dbgstbd3())
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
    pub fn dbgslpd1(&mut self) -> DBGSLPD1_W<'_, CRrs> {
        DBGSLPD1_W::new(self, 0)
    }
    ///Bit 1 - Allow D1 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstpd1(&mut self) -> DBGSTPD1_W<'_, CRrs> {
        DBGSTPD1_W::new(self, 1)
    }
    ///Bit 2 - Allow D1 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstbd1(&mut self) -> DBGSTBD1_W<'_, CRrs> {
        DBGSTBD1_W::new(self, 2)
    }
    ///Bit 3 - Allow D2 domain debug in Sleep mode
    #[inline(always)]
    pub fn dbgslpd2(&mut self) -> DBGSLPD2_W<'_, CRrs> {
        DBGSLPD2_W::new(self, 3)
    }
    ///Bit 4 - Allow D2 domain debug in Stop mode
    #[inline(always)]
    pub fn dbgstpd2(&mut self) -> DBGSTPD2_W<'_, CRrs> {
        DBGSTPD2_W::new(self, 4)
    }
    ///Bit 5 - Allow D2 domain debug in Standby mode
    #[inline(always)]
    pub fn dbgstbd2(&mut self) -> DBGSTBD2_W<'_, CRrs> {
        DBGSTBD2_W::new(self, 5)
    }
    ///Bit 7 - Allow debug in D3 Stop mode
    #[inline(always)]
    pub fn dbgstpd3(&mut self) -> DBGSTPD3_W<'_, CRrs> {
        DBGSTPD3_W::new(self, 7)
    }
    ///Bit 8 - Allow debug in D3 Standby mode
    #[inline(always)]
    pub fn dbgstbd3(&mut self) -> DBGSTBD3_W<'_, CRrs> {
        DBGSTBD3_W::new(self, 8)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#DBGMCU:CR)*/
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
