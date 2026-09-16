///Register `MTLTXQ1SSCR` reader
pub type R = crate::R<MTLTXQ1SSCRrs>;
///Register `MTLTXQ1SSCR` writer
pub type W = crate::W<MTLTXQ1SSCRrs>;
///Field `SSC` reader - sendSlopeCredit Value
pub type SSC_R = crate::FieldReader<u16>;
///Field `SSC` writer - sendSlopeCredit Value
pub type SSC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - sendSlopeCredit Value
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1SSCR")
            .field("ssc", &self.ssc())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - sendSlopeCredit Value
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W<'_, MTLTXQ1SSCRrs> {
        SSC_W::new(self, 0)
    }
}
/**Tx queue 1 send slope credit Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1sscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1sscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MTLTXQ1SSCR)*/
pub struct MTLTXQ1SSCRrs;
impl crate::RegisterSpec for MTLTXQ1SSCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1sscr::R`](R) reader structure
impl crate::Readable for MTLTXQ1SSCRrs {}
///`write(|w| ..)` method takes [`mtltxq1sscr::W`](W) writer structure
impl crate::Writable for MTLTXQ1SSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1SSCR to value 0
impl crate::Resettable for MTLTXQ1SSCRrs {}
