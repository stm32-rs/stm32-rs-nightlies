///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
/**configurable event inputs x rising edge pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF46R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<RPIF46R> for bool {
    #[inline(always)]
    fn from(variant: RPIF46R) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF46` reader - configurable event inputs x rising edge pending bit
pub type RPIF46_R = crate::BitReader<RPIF46R>;
impl RPIF46_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPIF46R {
        match self.bits {
            false => RPIF46R::NotPending,
            true => RPIF46R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF46R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF46R::Pending
    }
}
/**configurable event inputs x rising edge pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPIF46W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<RPIF46W> for bool {
    #[inline(always)]
    fn from(variant: RPIF46W) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF46` writer - configurable event inputs x rising edge pending bit
pub type RPIF46_W<'a, REG> = crate::BitWriter1C<'a, REG, RPIF46W>;
impl<'a, REG> RPIF46_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RPIF46W::Clear)
    }
}
///Field `RPIF50` reader - configurable event inputs x rising edge pending bit
pub use RPIF46_R as RPIF50_R;
///Field `RPIF53` reader - configurable event inputs x rising edge pending bit
pub use RPIF46_R as RPIF53_R;
///Field `RPIF50` writer - configurable event inputs x rising edge pending bit
pub use RPIF46_W as RPIF50_W;
///Field `RPIF53` writer - configurable event inputs x rising edge pending bit
pub use RPIF46_W as RPIF53_W;
impl R {
    ///Bit 14 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif46(&self) -> RPIF46_R {
        RPIF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif50(&self) -> RPIF50_R {
        RPIF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif53(&self) -> RPIF53_R {
        RPIF53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif46", &self.rpif46())
            .field("rpif50", &self.rpif50())
            .field("rpif53", &self.rpif53())
            .finish()
    }
}
impl W {
    ///Bit 14 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif46(&mut self) -> RPIF46_W<'_, RPR2rs> {
        RPIF46_W::new(self, 14)
    }
    ///Bit 18 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif50(&mut self) -> RPIF50_W<'_, RPR2rs> {
        RPIF50_W::new(self, 18)
    }
    ///Bit 21 - configurable event inputs x rising edge pending bit
    #[inline(always)]
    pub fn rpif53(&mut self) -> RPIF53_W<'_, RPR2rs> {
        RPIF53_W::new(self, 21)
    }
}
/**EXTI rising edge pending register 2

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0024_4000;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
