///Register `_DCR` reader
pub type R = crate::R<_DCRrs>;
///Register `_DCR` writer
pub type W = crate::W<_DCRrs>;
///Field `DBA` reader - DBA
pub type DBA_R = crate::FieldReader;
///Field `DBA` writer - DBA
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DBL` reader - DBL
pub type DBL_R = crate::FieldReader;
///Field `DBL` writer - DBL
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DBA
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DBA
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<'_, _DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<'_, _DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM15 DMA control register

You can [`read`](crate::Reg::read) this register and get [`_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM15:_DCR)*/
pub struct _DCRrs;
impl crate::RegisterSpec for _DCRrs {
    type Ux = u16;
}
///`read()` method returns [`_dcr::R`](R) reader structure
impl crate::Readable for _DCRrs {}
///`write(|w| ..)` method takes [`_dcr::W`](W) writer structure
impl crate::Writable for _DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _DCR to value 0
impl crate::Resettable for _DCRrs {}
