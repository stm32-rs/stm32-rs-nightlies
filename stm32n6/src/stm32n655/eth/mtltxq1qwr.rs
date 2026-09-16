///Register `MTLTXQ1QWR` reader
pub type R = crate::R<MTLTXQ1QWRrs>;
///Register `MTLTXQ1QWR` writer
pub type W = crate::W<MTLTXQ1QWRrs>;
///Field `ISCQW` reader - IdleSlopeCredit or Weights
pub type ISCQW_R = crate::FieldReader<u16>;
///Field `ISCQW` writer - IdleSlopeCredit or Weights
pub type ISCQW_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - IdleSlopeCredit or Weights
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1QWR")
            .field("iscqw", &self.iscqw())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - IdleSlopeCredit or Weights
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W<'_, MTLTXQ1QWRrs> {
        ISCQW_W::new(self, 0)
    }
}
/**Tx queue 1 quantum weight register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1qwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1qwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1QWR)*/
pub struct MTLTXQ1QWRrs;
impl crate::RegisterSpec for MTLTXQ1QWRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1qwr::R`](R) reader structure
impl crate::Readable for MTLTXQ1QWRrs {}
///`write(|w| ..)` method takes [`mtltxq1qwr::W`](W) writer structure
impl crate::Writable for MTLTXQ1QWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1QWR to value 0
impl crate::Resettable for MTLTXQ1QWRrs {}
