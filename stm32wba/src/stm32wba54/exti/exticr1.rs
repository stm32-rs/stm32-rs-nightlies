///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
///Field `EXTI0` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI0_R = crate::FieldReader;
///Field `EXTI0` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI1` reader - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI1_R = crate::FieldReader;
///Field `EXTI1` writer - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI2` reader - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI2_R = crate::FieldReader;
///Field `EXTI2` writer - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI3` reader - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI3_R = crate::FieldReader;
///Field `EXTI3` writer - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR1")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_SECCFGR.SEC0 is disabled, EXTI0 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC0 is enabled, EXTI0 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_SECCFGR.SEC1 is disabled, EXTI1 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC1 is enabled, EXTI1 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_SECCFGR.SEC2 is disabled, EXTI2 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC2 is enabled, EXTI2 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_SECCFGR.SEC3 is disabled, EXTI3 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC3 is enabled, EXTI3 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#EXTI:EXTICR1)*/
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exticr1::R`](R) reader structure
impl crate::Readable for EXTICR1rs {}
///`write(|w| ..)` method takes [`exticr1::W`](W) writer structure
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1rs {}
