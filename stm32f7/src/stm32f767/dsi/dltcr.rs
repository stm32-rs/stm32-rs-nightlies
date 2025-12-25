///Register `DLTCR` reader
pub type R = crate::R<DLTCRrs>;
///Register `DLTCR` writer
pub type W = crate::W<DLTCRrs>;
///Field `MRD_TIME` reader - Maximum Read Time
pub type MRD_TIME_R = crate::FieldReader<u16>;
///Field `MRD_TIME` writer - Maximum Read Time
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `LP2HS_TIME` reader - Low-Power To High-Speed Time
pub type LP2HS_TIME_R = crate::FieldReader;
///Field `LP2HS_TIME` writer - Low-Power To High-Speed Time
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HS2LP_TIME` reader - High-Speed To Low-Power Time
pub type HS2LP_TIME_R = crate::FieldReader;
///Field `HS2LP_TIME` writer - High-Speed To Low-Power Time
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:14 - Maximum Read Time
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 16:23 - Low-Power To High-Speed Time
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - High-Speed To Low-Power Time
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLTCR")
            .field("mrd_time", &self.mrd_time())
            .field("lp2hs_time", &self.lp2hs_time())
            .field("hs2lp_time", &self.hs2lp_time())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Maximum Read Time
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<'_, DLTCRrs> {
        MRD_TIME_W::new(self, 0)
    }
    ///Bits 16:23 - Low-Power To High-Speed Time
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<'_, DLTCRrs> {
        LP2HS_TIME_W::new(self, 16)
    }
    ///Bits 24:31 - High-Speed To Low-Power Time
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<'_, DLTCRrs> {
        HS2LP_TIME_W::new(self, 24)
    }
}
/**DSI Host Data Lane Timer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:DLTCR)*/
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
