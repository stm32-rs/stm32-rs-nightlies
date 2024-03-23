#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "Field `SPRIV` reader - PWR secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
pub type SPRIV_R = crate::BitReader;
#[doc = "Field `SPRIV` writer - PWR secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSPRIV {
    #[doc = "0: Read and write to PWR functions can be done by privileged or unprivileged access"]
    Unprivileged = 0,
    #[doc = "1: Read and write to PWR functions can be done by privileged access only"]
    Privileged = 1,
}
impl From<NSPRIV> for bool {
    #[inline(always)]
    fn from(variant: NSPRIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSPRIV` reader - PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_R = crate::BitReader<NSPRIV>;
impl NSPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NSPRIV {
        match self.bits {
            false => NSPRIV::Unprivileged,
            true => NSPRIV::Privileged,
        }
    }
    #[doc = "Read and write to PWR functions can be done by privileged or unprivileged access"]
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == NSPRIV::Unprivileged
    }
    #[doc = "Read and write to PWR functions can be done by privileged access only"]
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == NSPRIV::Privileged
    }
}
#[doc = "Field `NSPRIV` writer - PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, NSPRIV>;
impl<'a, REG> NSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read and write to PWR functions can be done by privileged or unprivileged access"]
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV::Unprivileged)
    }
    #[doc = "Read and write to PWR functions can be done by privileged access only"]
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV::Privileged)
    }
}
impl R {
    #[doc = "Bit 0 - PWR secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWR secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - PWR non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "PWR privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
