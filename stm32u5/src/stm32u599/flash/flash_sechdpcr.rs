#[doc = "Register `FLASH_SECHDPCR` reader"]
pub type R = crate::R<FLASH_SECHDPCRrs>;
#[doc = "Register `FLASH_SECHDPCR` writer"]
pub type W = crate::W<FLASH_SECHDPCRrs>;
#[doc = "Field `HDP1_ACCDIS` reader - HDP1 area access disable When set, this bit is only cleared by a system reset."]
pub type HDP1_ACCDIS_R = crate::BitReader;
#[doc = "Field `HDP1_ACCDIS` writer - HDP1 area access disable When set, this bit is only cleared by a system reset."]
pub type HDP1_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDP2_ACCDIS` reader - HDP2 area access disable When set, this bit is only cleared by a system reset."]
pub type HDP2_ACCDIS_R = crate::BitReader;
#[doc = "Field `HDP2_ACCDIS` writer - HDP2 area access disable When set, this bit is only cleared by a system reset."]
pub type HDP2_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset."]
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset."]
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W<FLASH_SECHDPCRrs> {
        HDP1_ACCDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W<FLASH_SECHDPCRrs> {
        HDP2_ACCDIS_W::new(self, 1)
    }
}
#[doc = "FLASH secure HDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_sechdpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sechdpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SECHDPCRrs;
impl crate::RegisterSpec for FLASH_SECHDPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sechdpcr::R`](R) reader structure"]
impl crate::Readable for FLASH_SECHDPCRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_sechdpcr::W`](W) writer structure"]
impl crate::Writable for FLASH_SECHDPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECHDPCR to value 0"]
impl crate::Resettable for FLASH_SECHDPCRrs {
    const RESET_VALUE: u32 = 0;
}
