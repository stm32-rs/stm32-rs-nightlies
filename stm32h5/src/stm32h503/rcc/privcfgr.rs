#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGRrs>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGRrs>;
#[doc = "RCC functions privilege configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV {
    #[doc = "0: RCC functions can be modified by privileged or unprivileged access"]
    Any = 0,
    #[doc = "1: RCC functions can only be modified by privileged access"]
    PrivilegedOnly = 1,
}
impl From<PRIV> for bool {
    #[inline(always)]
    fn from(variant: PRIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - RCC functions privilege configuration"]
pub type PRIV_R = crate::BitReader<PRIV>;
impl PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV {
        match self.bits {
            false => PRIV::Any,
            true => PRIV::PrivilegedOnly,
        }
    }
    #[doc = "RCC functions can be modified by privileged or unprivileged access"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == PRIV::Any
    }
    #[doc = "RCC functions can only be modified by privileged access"]
    #[inline(always)]
    pub fn is_privileged_only(&self) -> bool {
        *self == PRIV::PrivilegedOnly
    }
}
#[doc = "Field `PRIV` writer - RCC functions privilege configuration"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RCC functions can be modified by privileged or unprivileged access"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Any)
    }
    #[doc = "RCC functions can only be modified by privileged access"]
    #[inline(always)]
    pub fn privileged_only(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::PrivilegedOnly)
    }
}
impl R {
    #[doc = "Bit 1 - RCC functions privilege configuration"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RCC functions privilege configuration"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<PRIVCFGRrs> {
        PRIV_W::new(self, 1)
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
