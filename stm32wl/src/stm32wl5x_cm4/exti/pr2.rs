///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
/**pending bit on event input 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF34R> for bool {
    #[inline(always)]
    fn from(variant: PIF34R) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF34` reader - pending bit on event input 34
pub type PIF34_R = crate::BitReader<PIF34R>;
impl PIF34_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIF34R {
        match self.bits {
            false => PIF34R::NotPending,
            true => PIF34R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF34R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF34R::Pending
    }
}
/**pending bit on event input 34

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF34W> for bool {
    #[inline(always)]
    fn from(variant: PIF34W) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF34` writer - pending bit on event input 34
pub type PIF34_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF34W>;
impl<'a, REG> PIF34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF34W::Clear)
    }
}
///Field `PIF40` reader - pending bit on event input 40
pub use PIF34_R as PIF40_R;
///Field `PIF41` reader - pending bit on event input 41
pub use PIF34_R as PIF41_R;
///Field `PIF45` reader - pending bit on event input 45 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use PIF34_R as PIF45_R;
///Field `PIF40` writer - pending bit on event input 40
pub use PIF34_W as PIF40_W;
///Field `PIF41` writer - pending bit on event input 41
pub use PIF34_W as PIF41_W;
///Field `PIF45` writer - pending bit on event input 45 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
pub use PIF34_W as PIF45_W;
impl R {
    ///Bit 2 - pending bit on event input 34
    #[inline(always)]
    pub fn pif34(&self) -> PIF34_R {
        PIF34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - pending bit on event input 40
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - pending bit on event input 41
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - pending bit on event input 45 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn pif45(&self) -> PIF45_R {
        PIF45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pif34", &self.pif34())
            .field("pif40", &self.pif40())
            .field("pif41", &self.pif41())
            .field("pif45", &self.pif45())
            .finish()
    }
}
impl W {
    ///Bit 2 - pending bit on event input 34
    #[inline(always)]
    pub fn pif34(&mut self) -> PIF34_W<'_, PR2rs> {
        PIF34_W::new(self, 2)
    }
    ///Bit 8 - pending bit on event input 40
    #[inline(always)]
    pub fn pif40(&mut self) -> PIF40_W<'_, PR2rs> {
        PIF40_W::new(self, 8)
    }
    ///Bit 9 - pending bit on event input 41
    #[inline(always)]
    pub fn pif41(&mut self) -> PIF41_W<'_, PR2rs> {
        PIF41_W::new(self, 9)
    }
    ///Bit 13 - pending bit on event input 45 These bits are set when the selected edge event or an EXTI_SWIER software trigger arrives on the configurable event line. This bit is cleared by writing 1 to it.
    #[inline(always)]
    pub fn pif45(&mut self) -> PIF45_W<'_, PR2rs> {
        PIF45_W::new(self, 13)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2304;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
