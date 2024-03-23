#[doc = "Register `ICACHE_IER` reader"]
pub type R = crate::R<ICACHE_IERrs>;
#[doc = "Register `ICACHE_IER` writer"]
pub type W = crate::W<ICACHE_IERrs>;
#[doc = "Field `BSYENDIE` reader - BSYENDIE"]
pub type BSYENDIE_R = crate::BitReader;
#[doc = "Field `BSYENDIE` writer - BSYENDIE"]
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 1 - BSYENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<ICACHE_IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<ICACHE_IERrs> {
        ERRIE_W::new(self, 2)
    }
}
#[doc = "ICACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_IERrs;
impl crate::RegisterSpec for ICACHE_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_ier::R`](R) reader structure"]
impl crate::Readable for ICACHE_IERrs {}
#[doc = "`write(|w| ..)` method takes [`icache_ier::W`](W) writer structure"]
impl crate::Writable for ICACHE_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHE_IER to value 0"]
impl crate::Resettable for ICACHE_IERrs {
    const RESET_VALUE: u32 = 0;
}
