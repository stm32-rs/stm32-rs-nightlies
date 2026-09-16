///Register `IMR1` reader
pub type R = crate::R<IMR1rs>;
///Register `IMR1` writer
pub type W = crate::W<IMR1rs>;
/**Peripheral TIM1 interrupt mask to CPU1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1IM {
    ///0: Peripheral interrupt forwarded to CPU1
    Unmasked = 0,
    ///1: Peripheral interrupt to CPU1 masked
    Masked = 1,
}
impl From<TIM1IM> for bool {
    #[inline(always)]
    fn from(variant: TIM1IM) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1IM` reader - Peripheral TIM1 interrupt mask to CPU1
pub type TIM1IM_R = crate::BitReader<TIM1IM>;
impl TIM1IM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1IM {
        match self.bits {
            false => TIM1IM::Unmasked,
            true => TIM1IM::Masked,
        }
    }
    ///Peripheral interrupt forwarded to CPU1
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TIM1IM::Unmasked
    }
    ///Peripheral interrupt to CPU1 masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TIM1IM::Masked
    }
}
///Field `TIM1IM` writer - Peripheral TIM1 interrupt mask to CPU1
pub type TIM1IM_W<'a, REG> = crate::BitWriter<'a, REG, TIM1IM>;
impl<'a, REG> TIM1IM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral interrupt forwarded to CPU1
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1IM::Unmasked)
    }
    ///Peripheral interrupt to CPU1 masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1IM::Masked)
    }
}
///Field `TIM16IM` reader - Peripheral TIM16 interrupt mask to CPU1
pub use TIM1IM_R as TIM16IM_R;
///Field `TIM17IM` reader - Peripheral TIM17 interrupt mask to CPU1
pub use TIM1IM_R as TIM17IM_R;
///Field `EXIT5IM` reader - Peripheral EXIT5 interrupt mask to CPU1
pub use TIM1IM_R as EXIT5IM_R;
///Field `EXIT6IM` reader - Peripheral EXIT6 interrupt mask to CPU1
pub use TIM1IM_R as EXIT6IM_R;
///Field `EXIT7IM` reader - Peripheral EXIT7 interrupt mask to CPU1
pub use TIM1IM_R as EXIT7IM_R;
///Field `EXIT8IM` reader - Peripheral EXIT8 interrupt mask to CPU1
pub use TIM1IM_R as EXIT8IM_R;
///Field `EXIT9IM` reader - Peripheral EXIT9 interrupt mask to CPU1
pub use TIM1IM_R as EXIT9IM_R;
///Field `EXIT10IM` reader - Peripheral EXIT10 interrupt mask to CPU1
pub use TIM1IM_R as EXIT10IM_R;
///Field `EXIT11IM` reader - Peripheral EXIT11 interrupt mask to CPU1
pub use TIM1IM_R as EXIT11IM_R;
///Field `EXIT12IM` reader - Peripheral EXIT12 interrupt mask to CPU1
pub use TIM1IM_R as EXIT12IM_R;
///Field `EXIT13IM` reader - Peripheral EXIT13 interrupt mask to CPU1
pub use TIM1IM_R as EXIT13IM_R;
///Field `EXIT14IM` reader - Peripheral EXIT14 interrupt mask to CPU1
pub use TIM1IM_R as EXIT14IM_R;
///Field `EXIT15IM` reader - Peripheral EXIT15 interrupt mask to CPU1
pub use TIM1IM_R as EXIT15IM_R;
///Field `TIM16IM` writer - Peripheral TIM16 interrupt mask to CPU1
pub use TIM1IM_W as TIM16IM_W;
///Field `TIM17IM` writer - Peripheral TIM17 interrupt mask to CPU1
pub use TIM1IM_W as TIM17IM_W;
///Field `EXIT5IM` writer - Peripheral EXIT5 interrupt mask to CPU1
pub use TIM1IM_W as EXIT5IM_W;
///Field `EXIT6IM` writer - Peripheral EXIT6 interrupt mask to CPU1
pub use TIM1IM_W as EXIT6IM_W;
///Field `EXIT7IM` writer - Peripheral EXIT7 interrupt mask to CPU1
pub use TIM1IM_W as EXIT7IM_W;
///Field `EXIT8IM` writer - Peripheral EXIT8 interrupt mask to CPU1
pub use TIM1IM_W as EXIT8IM_W;
///Field `EXIT9IM` writer - Peripheral EXIT9 interrupt mask to CPU1
pub use TIM1IM_W as EXIT9IM_W;
///Field `EXIT10IM` writer - Peripheral EXIT10 interrupt mask to CPU1
pub use TIM1IM_W as EXIT10IM_W;
///Field `EXIT11IM` writer - Peripheral EXIT11 interrupt mask to CPU1
pub use TIM1IM_W as EXIT11IM_W;
///Field `EXIT12IM` writer - Peripheral EXIT12 interrupt mask to CPU1
pub use TIM1IM_W as EXIT12IM_W;
///Field `EXIT13IM` writer - Peripheral EXIT13 interrupt mask to CPU1
pub use TIM1IM_W as EXIT13IM_W;
///Field `EXIT14IM` writer - Peripheral EXIT14 interrupt mask to CPU1
pub use TIM1IM_W as EXIT14IM_W;
///Field `EXIT15IM` writer - Peripheral EXIT15 interrupt mask to CPU1
pub use TIM1IM_W as EXIT15IM_W;
impl R {
    ///Bit 13 - Peripheral TIM1 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim1im(&self) -> TIM1IM_R {
        TIM1IM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Peripheral TIM16 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim16im(&self) -> TIM16IM_R {
        TIM16IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Peripheral TIM17 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim17im(&self) -> TIM17IM_R {
        TIM17IM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Peripheral EXIT5 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit5im(&self) -> EXIT5IM_R {
        EXIT5IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Peripheral EXIT6 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit6im(&self) -> EXIT6IM_R {
        EXIT6IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Peripheral EXIT7 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit7im(&self) -> EXIT7IM_R {
        EXIT7IM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Peripheral EXIT8 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit8im(&self) -> EXIT8IM_R {
        EXIT8IM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Peripheral EXIT9 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit9im(&self) -> EXIT9IM_R {
        EXIT9IM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Peripheral EXIT10 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit10im(&self) -> EXIT10IM_R {
        EXIT10IM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Peripheral EXIT11 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit11im(&self) -> EXIT11IM_R {
        EXIT11IM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Peripheral EXIT12 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit12im(&self) -> EXIT12IM_R {
        EXIT12IM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Peripheral EXIT13 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit13im(&self) -> EXIT13IM_R {
        EXIT13IM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Peripheral EXIT14 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit14im(&self) -> EXIT14IM_R {
        EXIT14IM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Peripheral EXIT15 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit15im(&self) -> EXIT15IM_R {
        EXIT15IM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR1")
            .field("tim1im", &self.tim1im())
            .field("tim16im", &self.tim16im())
            .field("tim17im", &self.tim17im())
            .field("exit5im", &self.exit5im())
            .field("exit6im", &self.exit6im())
            .field("exit7im", &self.exit7im())
            .field("exit8im", &self.exit8im())
            .field("exit9im", &self.exit9im())
            .field("exit10im", &self.exit10im())
            .field("exit11im", &self.exit11im())
            .field("exit12im", &self.exit12im())
            .field("exit13im", &self.exit13im())
            .field("exit14im", &self.exit14im())
            .field("exit15im", &self.exit15im())
            .finish()
    }
}
impl W {
    ///Bit 13 - Peripheral TIM1 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim1im(&mut self) -> TIM1IM_W<'_, IMR1rs> {
        TIM1IM_W::new(self, 13)
    }
    ///Bit 14 - Peripheral TIM16 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim16im(&mut self) -> TIM16IM_W<'_, IMR1rs> {
        TIM16IM_W::new(self, 14)
    }
    ///Bit 15 - Peripheral TIM17 interrupt mask to CPU1
    #[inline(always)]
    pub fn tim17im(&mut self) -> TIM17IM_W<'_, IMR1rs> {
        TIM17IM_W::new(self, 15)
    }
    ///Bit 21 - Peripheral EXIT5 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit5im(&mut self) -> EXIT5IM_W<'_, IMR1rs> {
        EXIT5IM_W::new(self, 21)
    }
    ///Bit 22 - Peripheral EXIT6 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit6im(&mut self) -> EXIT6IM_W<'_, IMR1rs> {
        EXIT6IM_W::new(self, 22)
    }
    ///Bit 23 - Peripheral EXIT7 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit7im(&mut self) -> EXIT7IM_W<'_, IMR1rs> {
        EXIT7IM_W::new(self, 23)
    }
    ///Bit 24 - Peripheral EXIT8 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit8im(&mut self) -> EXIT8IM_W<'_, IMR1rs> {
        EXIT8IM_W::new(self, 24)
    }
    ///Bit 25 - Peripheral EXIT9 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit9im(&mut self) -> EXIT9IM_W<'_, IMR1rs> {
        EXIT9IM_W::new(self, 25)
    }
    ///Bit 26 - Peripheral EXIT10 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit10im(&mut self) -> EXIT10IM_W<'_, IMR1rs> {
        EXIT10IM_W::new(self, 26)
    }
    ///Bit 27 - Peripheral EXIT11 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit11im(&mut self) -> EXIT11IM_W<'_, IMR1rs> {
        EXIT11IM_W::new(self, 27)
    }
    ///Bit 28 - Peripheral EXIT12 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit12im(&mut self) -> EXIT12IM_W<'_, IMR1rs> {
        EXIT12IM_W::new(self, 28)
    }
    ///Bit 29 - Peripheral EXIT13 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit13im(&mut self) -> EXIT13IM_W<'_, IMR1rs> {
        EXIT13IM_W::new(self, 29)
    }
    ///Bit 30 - Peripheral EXIT14 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit14im(&mut self) -> EXIT14IM_W<'_, IMR1rs> {
        EXIT14IM_W::new(self, 30)
    }
    ///Bit 31 - Peripheral EXIT15 interrupt mask to CPU1
    #[inline(always)]
    pub fn exit15im(&mut self) -> EXIT15IM_W<'_, IMR1rs> {
        EXIT15IM_W::new(self, 31)
    }
}
/**CPU1 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:IMR1)*/
pub struct IMR1rs;
impl crate::RegisterSpec for IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`imr1::R`](R) reader structure
impl crate::Readable for IMR1rs {}
///`write(|w| ..)` method takes [`imr1::W`](W) writer structure
impl crate::Writable for IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR1 to value 0
impl crate::Resettable for IMR1rs {}
