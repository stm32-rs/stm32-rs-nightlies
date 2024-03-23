#[doc = "Register `DLTRC` reader"]
pub type R = crate::R<DLTRCrs>;
#[doc = "Register `DLTRC` writer"]
pub type W = crate::W<DLTRCrs>;
#[doc = "Field `MRD_TIME` reader - Maximum Read Time"]
pub type MRD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MRD_TIME` writer - Maximum Read Time"]
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LP2HS_TIME` reader - Low-Power To High-Speed Time"]
pub type LP2HS_TIME_R = crate::FieldReader;
#[doc = "Field `LP2HS_TIME` writer - Low-Power To High-Speed Time"]
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HS2LP_TIME` reader - High-Speed To Low-Power Time"]
pub type HS2LP_TIME_R = crate::FieldReader;
#[doc = "Field `HS2LP_TIME` writer - High-Speed To Low-Power Time"]
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:14 - Maximum Read Time"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - Low-Power To High-Speed Time"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High-Speed To Low-Power Time"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Maximum Read Time"]
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DLTRCrs> {
        MRD_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Low-Power To High-Speed Time"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<DLTRCrs> {
        LP2HS_TIME_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - High-Speed To Low-Power Time"]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<DLTRCrs> {
        HS2LP_TIME_W::new(self, 24)
    }
}
#[doc = "DSI Host Data Lane Timer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dltrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dltrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLTRCrs;
impl crate::RegisterSpec for DLTRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dltrc::R`](R) reader structure"]
impl crate::Readable for DLTRCrs {}
#[doc = "`write(|w| ..)` method takes [`dltrc::W`](W) writer structure"]
impl crate::Writable for DLTRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLTRC to value 0"]
impl crate::Resettable for DLTRCrs {
    const RESET_VALUE: u32 = 0;
}
