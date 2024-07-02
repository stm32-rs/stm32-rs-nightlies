///Register `TIM4_DCR` reader
pub type R = crate::R<TIM4_DCRrs>;
///Register `TIM4_DCR` writer
pub type W = crate::W<TIM4_DCRrs>;
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
        f.debug_struct("TIM4_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DBA
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<TIM4_DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<TIM4_DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM4 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim4_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim4_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM4:TIM4_DCR)*/
pub struct TIM4_DCRrs;
impl crate::RegisterSpec for TIM4_DCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim4_dcr::R`](R) reader structure
impl crate::Readable for TIM4_DCRrs {}
///`write(|w| ..)` method takes [`tim4_dcr::W`](W) writer structure
impl crate::Writable for TIM4_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM4_DCR to value 0
impl crate::Resettable for TIM4_DCRrs {
    const RESET_VALUE: u16 = 0;
}
