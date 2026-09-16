///Register `MTLTXQ1LCR` reader
pub type R = crate::R<MTLTXQ1LCRrs>;
///Register `MTLTXQ1LCR` writer
pub type W = crate::W<MTLTXQ1LCRrs>;
///Field `LC` reader - loCredit Value
pub type LC_R = crate::FieldReader<u32>;
///Field `LC` writer - loCredit Value
pub type LC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - loCredit Value
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1LCR")
            .field("lc", &self.lc())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - loCredit Value
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W<'_, MTLTXQ1LCRrs> {
        LC_W::new(self, 0)
    }
}
/**Tx queue 1 loCredit register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLTXQ1LCR)*/
pub struct MTLTXQ1LCRrs;
impl crate::RegisterSpec for MTLTXQ1LCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1lcr::R`](R) reader structure
impl crate::Readable for MTLTXQ1LCRrs {}
///`write(|w| ..)` method takes [`mtltxq1lcr::W`](W) writer structure
impl crate::Writable for MTLTXQ1LCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1LCR to value 0
impl crate::Resettable for MTLTXQ1LCRrs {}
