#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "Field `SPRIV` reader - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
pub type SPRIV_R = crate::BitReader;
#[doc = "Field `SPRIV` writer - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPRIV` reader - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_R = crate::BitReader;
#[doc = "Field `NSPRIV` writer - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "RCC privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
