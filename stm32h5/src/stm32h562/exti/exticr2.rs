#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2rs>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2rs>;
#[doc = "Field `EXTI4` reader - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI5` reader - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI6` reader - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI7` reader - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_SECCFGR1.SEC4 is disabled, EXTI4 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC4 is enabled, EXTI4 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_SECCFGR1.SEC5 is disabled, EXTI5 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC5 is enabled, EXTI5 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_SECCFGR1.SEC6 is disabled, EXTI6 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC6 is enabled, EXTI6 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_SECCFGR1.SEC7 is disabled, EXTI7 can be accessed with non-secure and secure access. When EXTI_SECCFGR1.SEC7 is enabled, EXTI7 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR1.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR1.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2rs> {
        EXTI7_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2rs {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
