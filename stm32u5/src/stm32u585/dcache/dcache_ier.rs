#[doc = "Register `DCACHE_IER` reader"]
pub type R = crate::R<DCACHE_IERrs>;
#[doc = "Register `DCACHE_IER` writer"]
pub type W = crate::W<DCACHE_IERrs>;
#[doc = "Field `BSYENDIE` reader - BSYENDIE"]
pub type BSYENDIE_R = crate::BitReader;
#[doc = "Field `BSYENDIE` writer - BSYENDIE"]
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDENDIE` reader - CMDENDIE"]
pub type CMDENDIE_R = crate::BitReader;
#[doc = "Field `CMDENDIE` writer - CMDENDIE"]
pub type CMDENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - BSYENDIE"]
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDENDIE"]
    #[inline(always)]
    pub fn cmdendie(&self) -> CMDENDIE_R {
        CMDENDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BSYENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<DCACHE_IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<DCACHE_IERrs> {
        ERRIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - CMDENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendie(&mut self) -> CMDENDIE_W<DCACHE_IERrs> {
        CMDENDIE_W::new(self, 4)
    }
}
#[doc = "DCACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_IERrs;
impl crate::RegisterSpec for DCACHE_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_ier::R`](R) reader structure"]
impl crate::Readable for DCACHE_IERrs {}
#[doc = "`write(|w| ..)` method takes [`dcache_ier::W`](W) writer structure"]
impl crate::Writable for DCACHE_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_IER to value 0"]
impl crate::Resettable for DCACHE_IERrs {
    const RESET_VALUE: u32 = 0;
}
