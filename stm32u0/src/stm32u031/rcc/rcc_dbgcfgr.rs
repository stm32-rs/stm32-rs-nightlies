///Register `RCC_DBGCFGR` reader
pub type R = crate::R<RCC_DBGCFGRrs>;
///Register `RCC_DBGCFGR` writer
pub type W = crate::W<RCC_DBGCFGRrs>;
///Field `DBGEN` reader - Debug support clock enable Set and cleared by software.
pub type DBGEN_R = crate::BitReader;
///Field `DBGEN` writer - Debug support clock enable Set and cleared by software.
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGRST` reader - Debug support reset Set and cleared by software.
pub type DBGRST_R = crate::BitReader;
///Field `DBGRST` writer - Debug support reset Set and cleared by software.
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug support reset Set and cleared by software.
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DBGCFGR")
            .field("dbgen", &self.dbgen())
            .field("dbgrst", &self.dbgrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<RCC_DBGCFGRrs> {
        DBGEN_W::new(self, 0)
    }
    ///Bit 1 - Debug support reset Set and cleared by software.
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<RCC_DBGCFGRrs> {
        DBGRST_W::new(self, 1)
    }
}
/**Debug configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_dbgcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:RCC_DBGCFGR)*/
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
///`reset()` method sets RCC_DBGCFGR to value 0
impl crate::Resettable for RCC_DBGCFGRrs {
    const RESET_VALUE: u32 = 0;
}