///Register `RCC_DBGCFGR` reader
pub type R = crate::R<RCC_DBGCFGRrs>;
///Register `RCC_DBGCFGR` writer
pub type W = crate::W<RCC_DBGCFGRrs>;
///Field `TRACEDIV` reader - TRACEDIV
pub type TRACEDIV_R = crate::FieldReader;
///Field `TRACEDIV` writer - TRACEDIV
pub type TRACEDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBGCKEN` reader - DBGCKEN
pub type DBGCKEN_R = crate::BitReader;
///Field `DBGCKEN` writer - DBGCKEN
pub type DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACECKEN` reader - TRACECKEN
pub type TRACECKEN_R = crate::BitReader;
///Field `TRACECKEN` writer - TRACECKEN
pub type TRACECKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGRST` reader - DBGRST
pub type DBGRST_R = crate::BitReader;
///Field `DBGRST` writer - DBGRST
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - TRACEDIV
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - DBGCKEN
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TRACECKEN
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - DBGRST
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DBGCFGR")
            .field("tracediv", &self.tracediv())
            .field("dbgcken", &self.dbgcken())
            .field("tracecken", &self.tracecken())
            .field("dbgrst", &self.dbgrst())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - TRACEDIV
    #[inline(always)]
    #[must_use]
    pub fn tracediv(&mut self) -> TRACEDIV_W<RCC_DBGCFGRrs> {
        TRACEDIV_W::new(self, 0)
    }
    ///Bit 8 - DBGCKEN
    #[inline(always)]
    #[must_use]
    pub fn dbgcken(&mut self) -> DBGCKEN_W<RCC_DBGCFGRrs> {
        DBGCKEN_W::new(self, 8)
    }
    ///Bit 9 - TRACECKEN
    #[inline(always)]
    #[must_use]
    pub fn tracecken(&mut self) -> TRACECKEN_W<RCC_DBGCFGRrs> {
        TRACECKEN_W::new(self, 9)
    }
    ///Bit 12 - DBGRST
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<RCC_DBGCFGRrs> {
        DBGRST_W::new(self, 12)
    }
}
/**This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.

You can [`read`](crate::Reg::read) this register and get [`rcc_dbgcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_DBGCFGR)*/
pub struct RCC_DBGCFGRrs;
impl crate::RegisterSpec for RCC_DBGCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_dbgcfgr::R`](R) reader structure
impl crate::Readable for RCC_DBGCFGRrs {}
///`write(|w| ..)` method takes [`rcc_dbgcfgr::W`](W) writer structure
impl crate::Writable for RCC_DBGCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_DBGCFGR to value 0x01
impl crate::Resettable for RCC_DBGCFGRrs {
    const RESET_VALUE: u32 = 0x01;
}
