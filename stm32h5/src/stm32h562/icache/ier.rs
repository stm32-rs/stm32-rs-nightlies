#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `BSYENDIE` reader - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_R = crate::BitReader;
#[doc = "Field `BSYENDIE` writer - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - interrupt enable on busy end Set by software to enable an interrupt generation at the end of a cache invalidate operation."]
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - interrupt enable on cache error Set by software to enable an interrupt generation in case of cache functional error (cacheable write access)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<IERrs> {
        ERRIE_W::new(self, 2)
    }
}
#[doc = "ICACHE interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
