///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
/**RCC functions privilege configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV {
    ///0: RCC functions can be modified by privileged or unprivileged access
    Any = 0,
    ///1: RCC functions can only be modified by privileged access
    PrivilegedOnly = 1,
}
impl From<PRIV> for bool {
    #[inline(always)]
    fn from(variant: PRIV) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV` reader - RCC functions privilege configuration
pub type PRIV_R = crate::BitReader<PRIV>;
impl PRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRIV {
        match self.bits {
            false => PRIV::Any,
            true => PRIV::PrivilegedOnly,
        }
    }
    ///RCC functions can be modified by privileged or unprivileged access
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == PRIV::Any
    }
    ///RCC functions can only be modified by privileged access
    #[inline(always)]
    pub fn is_privileged_only(&self) -> bool {
        *self == PRIV::PrivilegedOnly
    }
}
///Field `PRIV` writer - RCC functions privilege configuration
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RCC functions can be modified by privileged or unprivileged access
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Any)
    }
    ///RCC functions can only be modified by privileged access
    #[inline(always)]
    pub fn privileged_only(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::PrivilegedOnly)
    }
}
impl R {
    ///Bit 1 - RCC functions privilege configuration
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 1 - RCC functions privilege configuration
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 1)
    }
}
/**RCC privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
