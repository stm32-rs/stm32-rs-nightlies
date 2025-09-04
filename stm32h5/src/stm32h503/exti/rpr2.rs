///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
/**configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF50R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<RPIF50R> for bool {
    #[inline(always)]
    fn from(variant: RPIF50R) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF50` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type RPIF50_R = crate::BitReader<RPIF50R>;
impl RPIF50_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPIF50R {
        match self.bits {
            false => RPIF50R::NotPending,
            true => RPIF50R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF50R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF50R::Pending
    }
}
/**configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF50W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<RPIF50W> for bool {
    #[inline(always)]
    fn from(variant: RPIF50W) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF50` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub type RPIF50_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF50W>;
impl<'a, REG> RPIF50_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF50W::Clear)
    }
}
///Field `RPIF53` reader - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use RPIF50_R as RPIF53_R;
///Field `RPIF53` writer - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use RPIF50_W as RPIF53_W;
impl R {
    ///Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn rpif50(&self) -> RPIF50_R {
        RPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn rpif53(&self) -> RPIF53_R {
        RPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif50", &self.rpif50())
            .field("rpif53", &self.rpif53())
            .finish()
    }
}
impl W {
    ///Bit 18 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn rpif50(&mut self) -> RPIF50_W<RPR2rs> {
        RPIF50_W::new(self, 18)
    }
    ///Bit 21 - configurable event inputs x rising edge pending bit When EXTI_PRIVCFGR.PRIVx is disabled, RPIFx can be accessed with unprivileged and privileged access. When EXTI_PRIVCFGR.PRIVx is enabled, RPIFx can only be accessed with privileged access. Unprivileged write to this RPIFx is discarded, unprivileged read returns 0. This bit is set when the rising edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn rpif53(&mut self) -> RPIF53_W<RPR2rs> {
        RPIF53_W::new(self, 21)
    }
}
/**EXTI rising edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_0000;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
