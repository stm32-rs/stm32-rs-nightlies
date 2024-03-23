#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "Field `PRIV` reader - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
pub type PRIV_R = crate::BitReader;
#[doc = "Field `PRIV` writer - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged access protection. Unprivileged read accesses to registers return zeros Unprivileged write accesses to registers are ignored. Note: This bit can only be written in privileged mode. There is no limitations on reads."]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 0)
    }
}
#[doc = "OTFDEC_PRIVCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
