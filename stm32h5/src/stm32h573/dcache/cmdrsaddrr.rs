#[doc = "Register `CMDRSADDRR` reader"]
pub type R = crate::R<CMDRSADDRRrs>;
#[doc = "Register `CMDRSADDRR` writer"]
pub type W = crate::W<CMDRSADDRRrs>;
#[doc = "Field `CMDSTARTADDR` reader - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. ."]
pub type CMDSTARTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CMDSTARTADDR` writer - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. ."]
pub type CMDSTARTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. ."]
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - start address of range to which the cache maintenance command specified in DCACHE_CR.CACHECMD field applies This register must be set before DCACHE_CR.CACHECMD is written. ."]
    #[inline(always)]
    #[must_use]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<CMDRSADDRRrs> {
        CMDSTARTADDR_W::new(self, 4)
    }
}
#[doc = "DCACHE command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdrsaddrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdrsaddrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDRSADDRRrs;
impl crate::RegisterSpec for CMDRSADDRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdrsaddrr::R`](R) reader structure"]
impl crate::Readable for CMDRSADDRRrs {}
#[doc = "`write(|w| ..)` method takes [`cmdrsaddrr::W`](W) writer structure"]
impl crate::Writable for CMDRSADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDRSADDRR to value 0"]
impl crate::Resettable for CMDRSADDRRrs {
    const RESET_VALUE: u32 = 0;
}
