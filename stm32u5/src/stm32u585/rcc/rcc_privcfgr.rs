#[doc = "Register `RCC_PRIVCFGR` reader"]
pub type R = crate::R<RCC_PRIVCFGRrs>;
#[doc = "Register `RCC_PRIVCFGR` writer"]
pub type W = crate::W<RCC_PRIVCFGRrs>;
#[doc = "Field `SPRIV` reader - RCC secure function privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
pub type SPRIV_R = crate::BitReader;
#[doc = "Field `SPRIV` writer - RCC secure function privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPRIV` reader - RCC non-secure function privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_R = crate::BitReader;
#[doc = "Field `NSPRIV` writer - RCC non-secure function privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RCC secure function privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RCC non-secure function privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCC secure function privilege configuration This bit is set and reset by software. It can be written only by a secure privileged access."]
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<RCC_PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - RCC non-secure function privilege configuration This bit is set and reset by software. It can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<RCC_PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "RCC privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PRIVCFGRrs;
impl crate::RegisterSpec for RCC_PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_privcfgr::R`](R) reader structure"]
impl crate::Readable for RCC_PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_privcfgr::W`](W) writer structure"]
impl crate::Writable for RCC_PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PRIVCFGR to value 0"]
impl crate::Resettable for RCC_PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
