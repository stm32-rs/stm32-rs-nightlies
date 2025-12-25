///Register `DLTCR` reader
pub type R = crate::R<DLTCRrs>;
///Register `DLTCR` writer
pub type W = crate::W<DLTCRrs>;
///Field `LP2HS_TIME` reader - LP2HS_TIME
pub type LP2HS_TIME_R = crate::FieldReader<u16>;
///Field `LP2HS_TIME` writer - LP2HS_TIME
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `HS2LP_TIME` reader - HS2LP_TIME
pub type HS2LP_TIME_R = crate::FieldReader<u16>;
///Field `HS2LP_TIME` writer - HS2LP_TIME
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - LP2HS_TIME
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - HS2LP_TIME
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLTCR")
            .field("lp2hs_time", &self.lp2hs_time())
            .field("hs2lp_time", &self.hs2lp_time())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - LP2HS_TIME
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<'_, DLTCRrs> {
        LP2HS_TIME_W::new(self, 0)
    }
    ///Bits 16:25 - HS2LP_TIME
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<'_, DLTCRrs> {
        HS2LP_TIME_W::new(self, 16)
    }
}
/**DSI Host data lane timer configuration register

You can [`read`](crate::Reg::read) this register and get [`dltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:DLTCR)*/
pub struct DLTCRrs;
impl crate::RegisterSpec for DLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`dltcr::R`](R) reader structure
impl crate::Readable for DLTCRrs {}
///`write(|w| ..)` method takes [`dltcr::W`](W) writer structure
impl crate::Writable for DLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLTCR to value 0
impl crate::Resettable for DLTCRrs {}
