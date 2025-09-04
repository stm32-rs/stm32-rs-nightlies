///Register `CLTCR` reader
pub type R = crate::R<CLTCRrs>;
///Register `CLTCR` writer
pub type W = crate::W<CLTCRrs>;
///Field `LP2HS_TIME` reader - Low-power to high-speed time
pub type LP2HS_TIME_R = crate::FieldReader<u16>;
///Field `LP2HS_TIME` writer - Low-power to high-speed time
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `HS2LP_TIME` reader - High-speed to low-power time
pub type HS2LP_TIME_R = crate::FieldReader<u16>;
///Field `HS2LP_TIME` writer - High-speed to low-power time
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Low-power to high-speed time
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - High-speed to low-power time
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLTCR")
            .field("lp2hs_time", &self.lp2hs_time())
            .field("hs2lp_time", &self.hs2lp_time())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Low-power to high-speed time
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<CLTCRrs> {
        LP2HS_TIME_W::new(self, 0)
    }
    ///Bits 16:25 - High-speed to low-power time
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<CLTCRrs> {
        HS2LP_TIME_W::new(self, 16)
    }
}
/**DSI Host clock lane timer configuration register

You can [`read`](crate::Reg::read) this register and get [`cltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CLTCR)*/
pub struct CLTCRrs;
impl crate::RegisterSpec for CLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`cltcr::R`](R) reader structure
impl crate::Readable for CLTCRrs {}
///`write(|w| ..)` method takes [`cltcr::W`](W) writer structure
impl crate::Writable for CLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLTCR to value 0
impl crate::Resettable for CLTCRrs {}
