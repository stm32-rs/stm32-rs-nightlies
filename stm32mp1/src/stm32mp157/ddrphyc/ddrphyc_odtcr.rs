#[doc = "Register `DDRPHYC_ODTCR` reader"]
pub type R = crate::R<DDRPHYC_ODTCRrs>;
#[doc = "Register `DDRPHYC_ODTCR` writer"]
pub type W = crate::W<DDRPHYC_ODTCRrs>;
#[doc = "Field `RDODT` reader - RDODT"]
pub type RDODT_R = crate::BitReader;
#[doc = "Field `RDODT` writer - RDODT"]
pub type RDODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRODT` reader - WRODT"]
pub type WRODT_R = crate::BitReader;
#[doc = "Field `WRODT` writer - WRODT"]
pub type WRODT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&self) -> RDODT_R {
        RDODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&self) -> WRODT_R {
        WRODT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    #[must_use]
    pub fn rdodt(&mut self) -> RDODT_W<DDRPHYC_ODTCRrs> {
        RDODT_W::new(self, 0)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    #[must_use]
    pub fn wrodt(&mut self) -> WRODT_W<DDRPHYC_ODTCRrs> {
        WRODT_W::new(self, 16)
    }
}
#[doc = "DDRPHYC ODTC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_odtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_odtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ODTCRrs;
impl crate::RegisterSpec for DDRPHYC_ODTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_odtcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ODTCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_odtcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_ODTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ODTCR to value 0x8421_0000"]
impl crate::Resettable for DDRPHYC_ODTCRrs {
    const RESET_VALUE: u32 = 0x8421_0000;
}
