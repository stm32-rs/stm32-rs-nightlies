///Register `TIM1_DCR` reader
pub type R = crate::R<TIM1_DCRrs>;
///Register `TIM1_DCR` writer
pub type W = crate::W<TIM1_DCRrs>;
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
        f.debug_struct("TIM1_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DBA
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<TIM1_DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<TIM1_DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM1 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim1_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM1:TIM1_DCR)*/
pub struct TIM1_DCRrs;
impl crate::RegisterSpec for TIM1_DCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim1_dcr::R`](R) reader structure
impl crate::Readable for TIM1_DCRrs {}
///`write(|w| ..)` method takes [`tim1_dcr::W`](W) writer structure
impl crate::Writable for TIM1_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM1_DCR to value 0
impl crate::Resettable for TIM1_DCRrs {
    const RESET_VALUE: u16 = 0;
}
