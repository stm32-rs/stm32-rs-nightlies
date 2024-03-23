#[doc = "Register `CPUIMR3` reader"]
pub type R = crate::R<CPUIMR3rs>;
#[doc = "Register `CPUIMR3` writer"]
pub type W = crate::W<CPUIMR3rs>;
#[doc = "CPU Interrupt Mask on Direct Event input x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR64 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR64> for bool {
    #[inline(always)]
    fn from(variant: MR64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR64` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR64_R = crate::BitReader<MR64>;
impl MR64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR64 {
        match self.bits {
            false => MR64::Masked,
            true => MR64::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR64::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR64::Unmasked
    }
}
#[doc = "Field `MR64` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub type MR64_W<'a, REG> = crate::BitWriter<'a, REG, MR64>;
impl<'a, REG> MR64_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR64::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR64::Unmasked)
    }
}
#[doc = "Field `MR65` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR65_R;
#[doc = "Field `MR66` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR66_R;
#[doc = "Field `MR67` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR67_R;
#[doc = "Field `MR68` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR68_R;
#[doc = "Field `MR69` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR69_R;
#[doc = "Field `MR70` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR70_R;
#[doc = "Field `MR71` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR71_R;
#[doc = "Field `MR72` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR72_R;
#[doc = "Field `MR73` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR73_R;
#[doc = "Field `MR74` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR74_R;
#[doc = "Field `MR75` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR75_R;
#[doc = "Field `MR76` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR76_R;
#[doc = "Field `MR77` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR77_R;
#[doc = "Field `MR78` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR78_R;
#[doc = "Field `MR79` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR79_R;
#[doc = "Field `MR80` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR80_R;
#[doc = "Field `MR82` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR82_R;
#[doc = "Field `MR84` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR84_R;
#[doc = "Field `MR85` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR85_R;
#[doc = "Field `MR86` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR86_R;
#[doc = "Field `MR87` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR87_R;
#[doc = "Field `MR88` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_R as MR88_R;
#[doc = "Field `MR65` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR65_W;
#[doc = "Field `MR66` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR66_W;
#[doc = "Field `MR67` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR67_W;
#[doc = "Field `MR68` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR68_W;
#[doc = "Field `MR69` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR69_W;
#[doc = "Field `MR70` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR70_W;
#[doc = "Field `MR71` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR71_W;
#[doc = "Field `MR72` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR72_W;
#[doc = "Field `MR73` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR73_W;
#[doc = "Field `MR74` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR74_W;
#[doc = "Field `MR75` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR75_W;
#[doc = "Field `MR76` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR76_W;
#[doc = "Field `MR77` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR77_W;
#[doc = "Field `MR78` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR78_W;
#[doc = "Field `MR79` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR79_W;
#[doc = "Field `MR80` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR80_W;
#[doc = "Field `MR82` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR82_W;
#[doc = "Field `MR84` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR84_W;
#[doc = "Field `MR85` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR85_W;
#[doc = "Field `MR86` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR86_W;
#[doc = "Field `MR87` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR87_W;
#[doc = "Field `MR88` writer - CPU Interrupt Mask on Direct Event input x+64"]
pub use MR64_W as MR88_W;
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr64(&mut self) -> MR64_W<CPUIMR3rs> {
        MR64_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr65(&mut self) -> MR65_W<CPUIMR3rs> {
        MR65_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr66(&mut self) -> MR66_W<CPUIMR3rs> {
        MR66_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr67(&mut self) -> MR67_W<CPUIMR3rs> {
        MR67_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr68(&mut self) -> MR68_W<CPUIMR3rs> {
        MR68_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr69(&mut self) -> MR69_W<CPUIMR3rs> {
        MR69_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr70(&mut self) -> MR70_W<CPUIMR3rs> {
        MR70_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr71(&mut self) -> MR71_W<CPUIMR3rs> {
        MR71_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr72(&mut self) -> MR72_W<CPUIMR3rs> {
        MR72_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr73(&mut self) -> MR73_W<CPUIMR3rs> {
        MR73_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr74(&mut self) -> MR74_W<CPUIMR3rs> {
        MR74_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr75(&mut self) -> MR75_W<CPUIMR3rs> {
        MR75_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr76(&mut self) -> MR76_W<CPUIMR3rs> {
        MR76_W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr77(&mut self) -> MR77_W<CPUIMR3rs> {
        MR77_W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr78(&mut self) -> MR78_W<CPUIMR3rs> {
        MR78_W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr79(&mut self) -> MR79_W<CPUIMR3rs> {
        MR79_W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr80(&mut self) -> MR80_W<CPUIMR3rs> {
        MR80_W::new(self, 16)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr82(&mut self) -> MR82_W<CPUIMR3rs> {
        MR82_W::new(self, 18)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr84(&mut self) -> MR84_W<CPUIMR3rs> {
        MR84_W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr85(&mut self) -> MR85_W<CPUIMR3rs> {
        MR85_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr86(&mut self) -> MR86_W<CPUIMR3rs> {
        MR86_W::new(self, 22)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr87(&mut self) -> MR87_W<CPUIMR3rs> {
        MR87_W::new(self, 23)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<CPUIMR3rs> {
        MR88_W::new(self, 24)
    }
}
#[doc = "EXTI interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuimr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuimr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUIMR3rs;
impl crate::RegisterSpec for CPUIMR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuimr3::R`](R) reader structure"]
impl crate::Readable for CPUIMR3rs {}
#[doc = "`write(|w| ..)` method takes [`cpuimr3::W`](W) writer structure"]
impl crate::Writable for CPUIMR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIMR3 to value 0"]
impl crate::Resettable for CPUIMR3rs {
    const RESET_VALUE: u32 = 0;
}
