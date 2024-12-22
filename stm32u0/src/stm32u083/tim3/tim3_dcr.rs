///Register `TIM3_DCR` reader
pub type R = crate::R<TIM3_DCRrs>;
///Register `TIM3_DCR` writer
pub type W = crate::W<TIM3_DCRrs>;
///Field `DBA` reader - DMA base address
pub type DBA_R = crate::FieldReader;
///Field `DBA` writer - DMA base address
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DBL` reader - DMA burst length
pub type DBL_R = crate::FieldReader;
///Field `DBL` writer - DMA burst length
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<TIM3_DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<TIM3_DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM3 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim3_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM3:TIM3_DCR)*/
pub struct TIM3_DCRrs;
impl crate::RegisterSpec for TIM3_DCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim3_dcr::R`](R) reader structure
impl crate::Readable for TIM3_DCRrs {}
///`write(|w| ..)` method takes [`tim3_dcr::W`](W) writer structure
impl crate::Writable for TIM3_DCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM3_DCR to value 0
impl crate::Resettable for TIM3_DCRrs {
    const RESET_VALUE: u16 = 0;
}
