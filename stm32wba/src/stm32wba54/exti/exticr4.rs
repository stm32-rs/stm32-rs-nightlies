///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
///Field `EXTI12` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_SECCFGR.SEC12 is disabled, EXTI12 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC12 is enabled, EXTI12 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI12_R = crate::FieldReader;
///Field `EXTI12` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_SECCFGR.SEC12 is disabled, EXTI12 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC12 is enabled, EXTI12 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI13` reader - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_SECCFGR.SEC13 is disabled, EXTI13 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC13 is enabled, EXTI13 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI13_R = crate::FieldReader;
///Field `EXTI13` writer - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_SECCFGR.SEC13 is disabled, EXTI13 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC13 is enabled, EXTI13 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI14` reader - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_SECCFGR.SEC14 is disabled, EXTI14 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC14 is enabled, EXTI14 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI14_R = crate::FieldReader;
///Field `EXTI14` writer - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_SECCFGR.SEC14 is disabled, EXTI14 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC14 is enabled, EXTI14 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI15` reader - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_SECCFGR.SEC15 is disabled, EXTI15 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC15 is enabled, EXTI15 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI15_R = crate::FieldReader;
///Field `EXTI15` writer - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_SECCFGR.SEC15 is disabled, EXTI15 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC15 is enabled, EXTI15 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_SECCFGR.SEC12 is disabled, EXTI12 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC12 is enabled, EXTI12 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_SECCFGR.SEC13 is disabled, EXTI13 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC13 is enabled, EXTI13 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_SECCFGR.SEC14 is disabled, EXTI14 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC14 is enabled, EXTI14 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_SECCFGR.SEC15 is disabled, EXTI15 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC15 is enabled, EXTI15 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("exti12", &self.exti12())
            .field("exti13", &self.exti13())
            .field("exti14", &self.exti14())
            .field("exti15", &self.exti15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_SECCFGR.SEC12 is disabled, EXTI12 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC12 is enabled, EXTI12 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_SECCFGR.SEC13 is disabled, EXTI13 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC13 is enabled, EXTI13 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4rs> {
        EXTI13_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_SECCFGR.SEC14 is disabled, EXTI14 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC14 is enabled, EXTI14 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4rs> {
        EXTI14_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_SECCFGR.SEC15 is disabled, EXTI15 can be accessed with non-secure and secure access. When EXTI_SECCFGR.SEC15 is enabled, EXTI15 can only be accessed with secure access. Non-secure write is discarded and non-secure read returns 0. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4rs> {
        EXTI15_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#EXTI:EXTICR4)*/
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exticr4::R`](R) reader structure
impl crate::Readable for EXTICR4rs {}
///`write(|w| ..)` method takes [`exticr4::W`](W) writer structure
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4rs {}
