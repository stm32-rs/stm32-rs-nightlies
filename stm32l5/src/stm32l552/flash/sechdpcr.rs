#[doc = "Register `SECHDPCR` reader"]
pub type R = crate::R<SECHDPCRrs>;
#[doc = "Register `SECHDPCR` writer"]
pub type W = crate::W<SECHDPCRrs>;
#[doc = "Field `HDP1_ACCDIS` reader - HDP1_ACCDIS"]
pub type HDP1_ACCDIS_R = crate::BitReader;
#[doc = "Field `HDP1_ACCDIS` writer - HDP1_ACCDIS"]
pub type HDP1_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP2_ACCDIS` reader - HDP2_ACCDIS"]
pub type HDP2_ACCDIS_R = crate::BitReader;
#[doc = "Field `HDP2_ACCDIS` writer - HDP2_ACCDIS"]
pub type HDP2_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDP1_ACCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W<SECHDPCRrs> {
        HDP1_ACCDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - HDP2_ACCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W<SECHDPCRrs> {
        HDP2_ACCDIS_W::new(self, 1)
    }
}
#[doc = "FLASH secure HDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sechdpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sechdpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECHDPCRrs;
impl crate::RegisterSpec for SECHDPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sechdpcr::R`](R) reader structure"]
impl crate::Readable for SECHDPCRrs {}
#[doc = "`write(|w| ..)` method takes [`sechdpcr::W`](W) writer structure"]
impl crate::Writable for SECHDPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECHDPCR to value 0"]
impl crate::Resettable for SECHDPCRrs {
    const RESET_VALUE: u32 = 0;
}
