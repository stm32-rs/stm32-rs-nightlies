///Register `CPUEMR3` reader
pub type R = crate::R<CPUEMR3rs>;
///Register `CPUEMR3` writer
pub type W = crate::W<CPUEMR3rs>;
/**CPU Event mask on Event input x+64

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_MASK {
    ///0: Event request line is masked
    Masked = 0,
    ///1: Event request line is unmasked
    Unmasked = 1,
}
impl From<EVENT_MASK> for bool {
    #[inline(always)]
    fn from(variant: EVENT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `MR64` reader - CPU Event mask on Event input x+64
pub type MR64_R = crate::BitReader<EVENT_MASK>;
impl MR64_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_MASK {
        match self.bits {
            false => EVENT_MASK::Masked,
            true => EVENT_MASK::Unmasked,
        }
    }
    ///Event request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EVENT_MASK::Masked
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EVENT_MASK::Unmasked
    }
}
///Field `MR64` writer - CPU Event mask on Event input x+64
pub type MR64_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> MR64_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Masked)
    }
    ///Event request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_MASK::Unmasked)
    }
}
///Field `MR65` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR65_R;
///Field `MR66` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR66_R;
///Field `MR67` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR67_R;
///Field `MR68` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR68_R;
///Field `MR69` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR69_R;
///Field `MR70` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR70_R;
///Field `MR71` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR71_R;
///Field `MR72` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR72_R;
///Field `MR73` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR73_R;
///Field `MR74` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR74_R;
///Field `MR75` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR75_R;
///Field `MR76` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR76_R;
///Field `MR77` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR77_R;
///Field `MR78` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR78_R;
///Field `MR79` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR79_R;
///Field `MR80` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR80_R;
///Field `MR82` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR82_R;
///Field `MR84` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR84_R;
///Field `MR85` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR85_R;
///Field `MR86` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR86_R;
///Field `MR87` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR87_R;
///Field `MR88` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR88_R;
///Field `MR65` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR65_W;
///Field `MR66` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR66_W;
///Field `MR67` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR67_W;
///Field `MR68` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR68_W;
///Field `MR69` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR69_W;
///Field `MR70` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR70_W;
///Field `MR71` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR71_W;
///Field `MR72` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR72_W;
///Field `MR73` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR73_W;
///Field `MR74` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR74_W;
///Field `MR75` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR75_W;
///Field `MR76` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR76_W;
///Field `MR77` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR77_W;
///Field `MR78` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR78_W;
///Field `MR79` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR79_W;
///Field `MR80` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR80_W;
///Field `MR82` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR82_W;
///Field `MR84` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR84_W;
///Field `MR85` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR85_W;
///Field `MR86` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR86_W;
///Field `MR87` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR87_W;
///Field `MR88` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR88_W;
impl R {
    ///Bit 0 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUEMR3")
            .field("mr64", &self.mr64())
            .field("mr65", &self.mr65())
            .field("mr66", &self.mr66())
            .field("mr67", &self.mr67())
            .field("mr68", &self.mr68())
            .field("mr69", &self.mr69())
            .field("mr70", &self.mr70())
            .field("mr71", &self.mr71())
            .field("mr72", &self.mr72())
            .field("mr73", &self.mr73())
            .field("mr74", &self.mr74())
            .field("mr75", &self.mr75())
            .field("mr76", &self.mr76())
            .field("mr77", &self.mr77())
            .field("mr78", &self.mr78())
            .field("mr79", &self.mr79())
            .field("mr80", &self.mr80())
            .field("mr82", &self.mr82())
            .field("mr84", &self.mr84())
            .field("mr85", &self.mr85())
            .field("mr86", &self.mr86())
            .field("mr87", &self.mr87())
            .field("mr88", &self.mr88())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr64(&mut self) -> MR64_W<'_, CPUEMR3rs> {
        MR64_W::new(self, 0)
    }
    ///Bit 1 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr65(&mut self) -> MR65_W<'_, CPUEMR3rs> {
        MR65_W::new(self, 1)
    }
    ///Bit 2 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr66(&mut self) -> MR66_W<'_, CPUEMR3rs> {
        MR66_W::new(self, 2)
    }
    ///Bit 3 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr67(&mut self) -> MR67_W<'_, CPUEMR3rs> {
        MR67_W::new(self, 3)
    }
    ///Bit 4 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr68(&mut self) -> MR68_W<'_, CPUEMR3rs> {
        MR68_W::new(self, 4)
    }
    ///Bit 5 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr69(&mut self) -> MR69_W<'_, CPUEMR3rs> {
        MR69_W::new(self, 5)
    }
    ///Bit 6 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr70(&mut self) -> MR70_W<'_, CPUEMR3rs> {
        MR70_W::new(self, 6)
    }
    ///Bit 7 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr71(&mut self) -> MR71_W<'_, CPUEMR3rs> {
        MR71_W::new(self, 7)
    }
    ///Bit 8 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr72(&mut self) -> MR72_W<'_, CPUEMR3rs> {
        MR72_W::new(self, 8)
    }
    ///Bit 9 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr73(&mut self) -> MR73_W<'_, CPUEMR3rs> {
        MR73_W::new(self, 9)
    }
    ///Bit 10 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr74(&mut self) -> MR74_W<'_, CPUEMR3rs> {
        MR74_W::new(self, 10)
    }
    ///Bit 11 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr75(&mut self) -> MR75_W<'_, CPUEMR3rs> {
        MR75_W::new(self, 11)
    }
    ///Bit 12 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr76(&mut self) -> MR76_W<'_, CPUEMR3rs> {
        MR76_W::new(self, 12)
    }
    ///Bit 13 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr77(&mut self) -> MR77_W<'_, CPUEMR3rs> {
        MR77_W::new(self, 13)
    }
    ///Bit 14 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr78(&mut self) -> MR78_W<'_, CPUEMR3rs> {
        MR78_W::new(self, 14)
    }
    ///Bit 15 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr79(&mut self) -> MR79_W<'_, CPUEMR3rs> {
        MR79_W::new(self, 15)
    }
    ///Bit 16 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr80(&mut self) -> MR80_W<'_, CPUEMR3rs> {
        MR80_W::new(self, 16)
    }
    ///Bit 18 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr82(&mut self) -> MR82_W<'_, CPUEMR3rs> {
        MR82_W::new(self, 18)
    }
    ///Bit 20 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr84(&mut self) -> MR84_W<'_, CPUEMR3rs> {
        MR84_W::new(self, 20)
    }
    ///Bit 21 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr85(&mut self) -> MR85_W<'_, CPUEMR3rs> {
        MR85_W::new(self, 21)
    }
    ///Bit 22 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr86(&mut self) -> MR86_W<'_, CPUEMR3rs> {
        MR86_W::new(self, 22)
    }
    ///Bit 23 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr87(&mut self) -> MR87_W<'_, CPUEMR3rs> {
        MR87_W::new(self, 23)
    }
    ///Bit 24 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W<'_, CPUEMR3rs> {
        MR88_W::new(self, 24)
    }
}
/**EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`cpuemr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#EXTI:CPUEMR3)*/
pub struct CPUEMR3rs;
impl crate::RegisterSpec for CPUEMR3rs {
    type Ux = u32;
}
///`read()` method returns [`cpuemr3::R`](R) reader structure
impl crate::Readable for CPUEMR3rs {}
///`write(|w| ..)` method takes [`cpuemr3::W`](W) writer structure
impl crate::Writable for CPUEMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPUEMR3 to value 0
impl crate::Resettable for CPUEMR3rs {}
