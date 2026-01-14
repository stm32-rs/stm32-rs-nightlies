///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
/**PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSPRIV {
    ///0: Read and write to PWR functions can be done by privileged or unprivileged access
    Unprivileged = 0,
    ///1: Read and write to PWR functions can be done by privileged access only
    Privileged = 1,
}
impl From<NSPRIV> for bool {
    #[inline(always)]
    fn from(variant: NSPRIV) -> Self {
        variant as u8 != 0
    }
}
///Field `NSPRIV` reader - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.
pub type NSPRIV_R = crate::BitReader<NSPRIV>;
impl NSPRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NSPRIV {
        match self.bits {
            false => NSPRIV::Unprivileged,
            true => NSPRIV::Privileged,
        }
    }
    ///Read and write to PWR functions can be done by privileged or unprivileged access
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == NSPRIV::Unprivileged
    }
    ///Read and write to PWR functions can be done by privileged access only
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == NSPRIV::Privileged
    }
}
///Field `NSPRIV` writer - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG, NSPRIV>;
impl<'a, REG> NSPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read and write to PWR functions can be done by privileged or unprivileged access
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV::Unprivileged)
    }
    ///Read and write to PWR functions can be done by privileged access only
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(NSPRIV::Privileged)
    }
}
impl R {
    ///Bit 1 - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("nspriv", &self.nspriv())
            .finish()
    }
}
impl W {
    ///Bit 1 - PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access.
    #[inline(always)]
    pub fn nspriv(&mut self) -> NSPRIV_W<'_, PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
/**PWR privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#PWR:PRIVCFGR)*/
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
