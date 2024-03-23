#[doc = "Register `EXTICR3` reader"]
pub type R = crate::R<EXTICR3rs>;
#[doc = "Register `EXTICR3` writer"]
pub type W = crate::W<EXTICR3rs>;
#[doc = "Field `EXTI8` reader - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI8_R = crate::FieldReader;
#[doc = "Field `EXTI8` writer - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI9` reader - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI9_R = crate::FieldReader;
#[doc = "Field `EXTI9` writer - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI10` reader - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI10_R = crate::FieldReader;
#[doc = "Field `EXTI10` writer - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI11` reader - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI11_R = crate::FieldReader;
#[doc = "Field `EXTI11` writer - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<EXTICR3rs> {
        EXTI9_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<EXTICR3rs> {
        EXTI10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<EXTICR3rs> {
        EXTI11_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR3rs;
impl crate::RegisterSpec for EXTICR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr3::R`](R) reader structure"]
impl crate::Readable for EXTICR3rs {}
#[doc = "`write(|w| ..)` method takes [`exticr3::W`](W) writer structure"]
impl crate::Writable for EXTICR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3rs {
    const RESET_VALUE: u32 = 0;
}
