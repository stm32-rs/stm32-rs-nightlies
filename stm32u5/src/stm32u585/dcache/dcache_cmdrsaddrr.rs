#[doc = "Register `DCACHE_CMDRSADDRR` reader"]
pub type R = crate::R<DCACHE_CMDRSADDRRrs>;
#[doc = "Register `DCACHE_CMDRSADDRR` writer"]
pub type W = crate::W<DCACHE_CMDRSADDRRrs>;
#[doc = "Field `CMDSTARTADDR` reader - CMDSTARTADDR"]
pub type CMDSTARTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CMDSTARTADDR` writer - CMDSTARTADDR"]
pub type CMDSTARTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CMDSTARTADDR"]
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMDSTARTADDR"]
    #[inline(always)]
    #[must_use]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<DCACHE_CMDRSADDRRrs> {
        CMDSTARTADDR_W::new(self, 0)
    }
}
#[doc = "command range start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cmdrsaddrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cmdrsaddrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_CMDRSADDRRrs;
impl crate::RegisterSpec for DCACHE_CMDRSADDRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_cmdrsaddrr::R`](R) reader structure"]
impl crate::Readable for DCACHE_CMDRSADDRRrs {}
#[doc = "`write(|w| ..)` method takes [`dcache_cmdrsaddrr::W`](W) writer structure"]
impl crate::Writable for DCACHE_CMDRSADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_CMDRSADDRR to value 0"]
impl crate::Resettable for DCACHE_CMDRSADDRRrs {
    const RESET_VALUE: u32 = 0;
}
