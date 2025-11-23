///Register `CPUEMR2` reader
pub type R = crate::R<CPUEMR2rs>;
///Register `CPUEMR2` writer
pub type W = crate::W<CPUEMR2rs>;
/**CPU Interrupt Mask on Direct Event input x+32

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
///Field `MR32` reader - CPU Interrupt Mask on Direct Event input x+32
pub type MR32_R = crate::BitReader<EVENT_MASK>;
impl MR32_R {
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
///Field `MR32` writer - CPU Interrupt Mask on Direct Event input x+32
pub type MR32_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_MASK>;
impl<'a, REG> MR32_W<'a, REG>
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
///Field `MR33` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR33_R;
///Field `MR34` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR34_R;
///Field `MR35` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR35_R;
///Field `MR36` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR36_R;
///Field `MR37` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR37_R;
///Field `MR38` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR38_R;
///Field `MR39` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR39_R;
///Field `MR40` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR40_R;
///Field `MR41` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR41_R;
///Field `MR42` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR42_R;
///Field `MR43` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR43_R;
///Field `MR44` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR44_R;
///Field `MR46` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR46_R;
///Field `MR47` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR47_R;
///Field `MR48` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR48_R;
///Field `MR49` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR49_R;
///Field `MR50` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR50_R;
///Field `MR51` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR51_R;
///Field `MR52` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR52_R;
///Field `MR53` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR53_R;
///Field `MR54` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR54_R;
///Field `MR55` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR55_R;
///Field `MR56` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR56_R;
///Field `MR57` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR57_R;
///Field `MR58` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR58_R;
///Field `MR59` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR59_R;
///Field `MR60` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR60_R;
///Field `MR61` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR61_R;
///Field `MR62` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR62_R;
///Field `MR63` reader - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_R as MR63_R;
///Field `MR33` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR33_W;
///Field `MR34` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR34_W;
///Field `MR35` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR35_W;
///Field `MR36` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR36_W;
///Field `MR37` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR37_W;
///Field `MR38` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR38_W;
///Field `MR39` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR39_W;
///Field `MR40` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR40_W;
///Field `MR41` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR41_W;
///Field `MR42` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR42_W;
///Field `MR43` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR43_W;
///Field `MR44` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR44_W;
///Field `MR46` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR46_W;
///Field `MR47` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR47_W;
///Field `MR48` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR48_W;
///Field `MR49` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR49_W;
///Field `MR50` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR50_W;
///Field `MR51` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR51_W;
///Field `MR52` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR52_W;
///Field `MR53` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR53_W;
///Field `MR54` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR54_W;
///Field `MR55` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR55_W;
///Field `MR56` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR56_W;
///Field `MR57` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR57_W;
///Field `MR58` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR58_W;
///Field `MR59` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR59_W;
///Field `MR60` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR60_W;
///Field `MR61` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR61_W;
///Field `MR62` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR62_W;
///Field `MR63` writer - CPU Interrupt Mask on Direct Event input x+32
pub use MR32_W as MR63_W;
impl R {
    ///Bit 0 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUEMR2")
            .field("mr32", &self.mr32())
            .field("mr33", &self.mr33())
            .field("mr34", &self.mr34())
            .field("mr35", &self.mr35())
            .field("mr36", &self.mr36())
            .field("mr37", &self.mr37())
            .field("mr38", &self.mr38())
            .field("mr39", &self.mr39())
            .field("mr40", &self.mr40())
            .field("mr41", &self.mr41())
            .field("mr42", &self.mr42())
            .field("mr43", &self.mr43())
            .field("mr44", &self.mr44())
            .field("mr46", &self.mr46())
            .field("mr47", &self.mr47())
            .field("mr48", &self.mr48())
            .field("mr49", &self.mr49())
            .field("mr50", &self.mr50())
            .field("mr51", &self.mr51())
            .field("mr52", &self.mr52())
            .field("mr53", &self.mr53())
            .field("mr54", &self.mr54())
            .field("mr55", &self.mr55())
            .field("mr56", &self.mr56())
            .field("mr57", &self.mr57())
            .field("mr58", &self.mr58())
            .field("mr59", &self.mr59())
            .field("mr60", &self.mr60())
            .field("mr61", &self.mr61())
            .field("mr62", &self.mr62())
            .field("mr63", &self.mr63())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr32(&mut self) -> MR32_W<'_, CPUEMR2rs> {
        MR32_W::new(self, 0)
    }
    ///Bit 1 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr33(&mut self) -> MR33_W<'_, CPUEMR2rs> {
        MR33_W::new(self, 1)
    }
    ///Bit 2 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W<'_, CPUEMR2rs> {
        MR34_W::new(self, 2)
    }
    ///Bit 3 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W<'_, CPUEMR2rs> {
        MR35_W::new(self, 3)
    }
    ///Bit 4 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr36(&mut self) -> MR36_W<'_, CPUEMR2rs> {
        MR36_W::new(self, 4)
    }
    ///Bit 5 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr37(&mut self) -> MR37_W<'_, CPUEMR2rs> {
        MR37_W::new(self, 5)
    }
    ///Bit 6 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr38(&mut self) -> MR38_W<'_, CPUEMR2rs> {
        MR38_W::new(self, 6)
    }
    ///Bit 7 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr39(&mut self) -> MR39_W<'_, CPUEMR2rs> {
        MR39_W::new(self, 7)
    }
    ///Bit 8 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr40(&mut self) -> MR40_W<'_, CPUEMR2rs> {
        MR40_W::new(self, 8)
    }
    ///Bit 9 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr41(&mut self) -> MR41_W<'_, CPUEMR2rs> {
        MR41_W::new(self, 9)
    }
    ///Bit 10 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr42(&mut self) -> MR42_W<'_, CPUEMR2rs> {
        MR42_W::new(self, 10)
    }
    ///Bit 11 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr43(&mut self) -> MR43_W<'_, CPUEMR2rs> {
        MR43_W::new(self, 11)
    }
    ///Bit 12 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr44(&mut self) -> MR44_W<'_, CPUEMR2rs> {
        MR44_W::new(self, 12)
    }
    ///Bit 14 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr46(&mut self) -> MR46_W<'_, CPUEMR2rs> {
        MR46_W::new(self, 14)
    }
    ///Bit 15 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr47(&mut self) -> MR47_W<'_, CPUEMR2rs> {
        MR47_W::new(self, 15)
    }
    ///Bit 16 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr48(&mut self) -> MR48_W<'_, CPUEMR2rs> {
        MR48_W::new(self, 16)
    }
    ///Bit 17 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr49(&mut self) -> MR49_W<'_, CPUEMR2rs> {
        MR49_W::new(self, 17)
    }
    ///Bit 18 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr50(&mut self) -> MR50_W<'_, CPUEMR2rs> {
        MR50_W::new(self, 18)
    }
    ///Bit 19 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr51(&mut self) -> MR51_W<'_, CPUEMR2rs> {
        MR51_W::new(self, 19)
    }
    ///Bit 20 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr52(&mut self) -> MR52_W<'_, CPUEMR2rs> {
        MR52_W::new(self, 20)
    }
    ///Bit 21 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr53(&mut self) -> MR53_W<'_, CPUEMR2rs> {
        MR53_W::new(self, 21)
    }
    ///Bit 22 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr54(&mut self) -> MR54_W<'_, CPUEMR2rs> {
        MR54_W::new(self, 22)
    }
    ///Bit 23 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr55(&mut self) -> MR55_W<'_, CPUEMR2rs> {
        MR55_W::new(self, 23)
    }
    ///Bit 24 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr56(&mut self) -> MR56_W<'_, CPUEMR2rs> {
        MR56_W::new(self, 24)
    }
    ///Bit 25 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr57(&mut self) -> MR57_W<'_, CPUEMR2rs> {
        MR57_W::new(self, 25)
    }
    ///Bit 26 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr58(&mut self) -> MR58_W<'_, CPUEMR2rs> {
        MR58_W::new(self, 26)
    }
    ///Bit 27 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr59(&mut self) -> MR59_W<'_, CPUEMR2rs> {
        MR59_W::new(self, 27)
    }
    ///Bit 28 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr60(&mut self) -> MR60_W<'_, CPUEMR2rs> {
        MR60_W::new(self, 28)
    }
    ///Bit 29 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr61(&mut self) -> MR61_W<'_, CPUEMR2rs> {
        MR61_W::new(self, 29)
    }
    ///Bit 30 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr62(&mut self) -> MR62_W<'_, CPUEMR2rs> {
        MR62_W::new(self, 30)
    }
    ///Bit 31 - CPU Interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr63(&mut self) -> MR63_W<'_, CPUEMR2rs> {
        MR63_W::new(self, 31)
    }
}
/**EXTI event mask register

You can [`read`](crate::Reg::read) this register and get [`cpuemr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#EXTI:CPUEMR2)*/
pub struct CPUEMR2rs;
impl crate::RegisterSpec for CPUEMR2rs {
    type Ux = u32;
}
///`read()` method returns [`cpuemr2::R`](R) reader structure
impl crate::Readable for CPUEMR2rs {}
///`write(|w| ..)` method takes [`cpuemr2::W`](W) writer structure
impl crate::Writable for CPUEMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPUEMR2 to value 0
impl crate::Resettable for CPUEMR2rs {}
