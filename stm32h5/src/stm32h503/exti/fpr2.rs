///Register `FPR2` reader
pub type R = crate::R<FPR2rs>;
///Register `FPR2` writer
pub type W = crate::W<FPR2rs>;
/**configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF50R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF50R> for bool {
    #[inline(always)]
    fn from(variant: FPIF50R) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF50` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF50_R = crate::BitReader<FPIF50R>;
impl FPIF50_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPIF50R {
        match self.bits {
            false => FPIF50R::NotPending,
            true => FPIF50R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF50R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF50R::Pending
    }
}
/**configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF50W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF50W> for bool {
    #[inline(always)]
    fn from(variant: FPIF50W) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF50` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type FPIF50_W<'a, REG> = crate::BitWriter1C<'a, REG, FPIF50W>;
impl<'a, REG> FPIF50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FPIF50W::Clear)
    }
}
///Field `FPIF53` reader - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use FPIF50_R as FPIF53_R;
///Field `FPIF53` writer - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use FPIF50_W as FPIF53_W;
impl R {
    ///Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif50(&self) -> FPIF50_R {
        FPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif53(&self) -> FPIF53_R {
        FPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPR2")
            .field("fpif50", &self.fpif50())
            .field("fpif53", &self.fpif53())
            .finish()
    }
}
impl W {
    ///Bit 18 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif50(&mut self) -> FPIF50_W<'_, FPR2rs> {
        FPIF50_W::new(self, 18)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, FPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, FPIFx can only be accessed with privileged access. Unprivileged write to this FPIFx is discarded, unprivileged read returns 0. This bit is set when the falling edge event arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn fpif53(&mut self) -> FPIF53_W<'_, FPR2rs> {
        FPIF53_W::new(self, 21)
    }
}
/**EXTI falling edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`fpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#EXTI:FPR2)*/
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
///`read()` method returns [`fpr2::R`](R) reader structure
impl crate::Readable for FPR2rs {}
///`write(|w| ..)` method takes [`fpr2::W`](W) writer structure
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_0000;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2rs {}
