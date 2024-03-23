#[doc = "Register `CLTCR` reader"]
pub type R = crate::R<CLTCRrs>;
#[doc = "Register `CLTCR` writer"]
pub type W = crate::W<CLTCRrs>;
#[doc = "Field `LP2HS_TIME` reader - Low-power to high-speed time"]
pub type LP2HS_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `LP2HS_TIME` writer - Low-power to high-speed time"]
pub type LP2HS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HS2LP_TIME` reader - High-speed to low-power time"]
pub type HS2LP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `HS2LP_TIME` writer - High-speed to low-power time"]
pub type HS2LP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Low-power to high-speed time"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - High-speed to low-power time"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low-power to high-speed time"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<CLTCRrs> {
        LP2HS_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - High-speed to low-power time"]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<CLTCRrs> {
        HS2LP_TIME_W::new(self, 16)
    }
}
#[doc = "DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLTCRrs;
impl crate::RegisterSpec for CLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cltcr::R`](R) reader structure"]
impl crate::Readable for CLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`cltcr::W`](W) writer structure"]
impl crate::Writable for CLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLTCR to value 0"]
impl crate::Resettable for CLTCRrs {
    const RESET_VALUE: u32 = 0;
}
