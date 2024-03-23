#[doc = "Register `D3PMR1` reader"]
pub type R = crate::R<D3PMR1rs>;
#[doc = "Register `D3PMR1` writer"]
pub type W = crate::W<D3PMR1rs>;
#[doc = "Rising trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR0> for bool {
    #[inline(always)]
    fn from(variant: MR0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type MR0_R = crate::BitReader<MR0>;
impl MR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR0 {
        match self.bits {
            false => MR0::Masked,
            true => MR0::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR0::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR0::Unmasked
    }
}
#[doc = "Field `MR0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type MR0_W<'a, REG> = crate::BitWriter<'a, REG, MR0>;
impl<'a, REG> MR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR0::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR0::Unmasked)
    }
}
#[doc = "Field `MR1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR1_R;
#[doc = "Field `MR2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR2_R;
#[doc = "Field `MR3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR3_R;
#[doc = "Field `MR4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR4_R;
#[doc = "Field `MR5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR5_R;
#[doc = "Field `MR6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR6_R;
#[doc = "Field `MR7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR7_R;
#[doc = "Field `MR8` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR8_R;
#[doc = "Field `MR9` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR9_R;
#[doc = "Field `MR10` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR10_R;
#[doc = "Field `MR11` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR11_R;
#[doc = "Field `MR12` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR12_R;
#[doc = "Field `MR13` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR13_R;
#[doc = "Field `MR14` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR14_R;
#[doc = "Field `MR15` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR15_R;
#[doc = "Field `MR19` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR19_R;
#[doc = "Field `MR20` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR20_R;
#[doc = "Field `MR21` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR21_R;
#[doc = "Field `MR25` reader - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_R as MR25_R;
#[doc = "Field `MR1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR1_W;
#[doc = "Field `MR2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR2_W;
#[doc = "Field `MR3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR3_W;
#[doc = "Field `MR4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR4_W;
#[doc = "Field `MR5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR5_W;
#[doc = "Field `MR6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR6_W;
#[doc = "Field `MR7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR7_W;
#[doc = "Field `MR8` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR8_W;
#[doc = "Field `MR9` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR9_W;
#[doc = "Field `MR10` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR10_W;
#[doc = "Field `MR11` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR11_W;
#[doc = "Field `MR12` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR12_W;
#[doc = "Field `MR13` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR13_W;
#[doc = "Field `MR14` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR14_W;
#[doc = "Field `MR15` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR15_W;
#[doc = "Field `MR19` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR19_W;
#[doc = "Field `MR20` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR20_W;
#[doc = "Field `MR21` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR21_W;
#[doc = "Field `MR25` writer - Rising trigger event configuration bit of Configurable Event input"]
pub use MR0_W as MR25_W;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<D3PMR1rs> {
        MR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<D3PMR1rs> {
        MR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<D3PMR1rs> {
        MR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<D3PMR1rs> {
        MR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<D3PMR1rs> {
        MR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<D3PMR1rs> {
        MR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<D3PMR1rs> {
        MR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<D3PMR1rs> {
        MR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<D3PMR1rs> {
        MR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<D3PMR1rs> {
        MR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<D3PMR1rs> {
        MR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<D3PMR1rs> {
        MR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<D3PMR1rs> {
        MR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<D3PMR1rs> {
        MR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<D3PMR1rs> {
        MR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<D3PMR1rs> {
        MR15_W::new(self, 15)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr19(&mut self) -> MR19_W<D3PMR1rs> {
        MR19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr20(&mut self) -> MR20_W<D3PMR1rs> {
        MR20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr21(&mut self) -> MR21_W<D3PMR1rs> {
        MR21_W::new(self, 21)
    }
    #[doc = "Bit 25 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn mr25(&mut self) -> MR25_W<D3PMR1rs> {
        MR25_W::new(self, 25)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PMR1rs;
impl crate::RegisterSpec for D3PMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr1::R`](R) reader structure"]
impl crate::Readable for D3PMR1rs {}
#[doc = "`write(|w| ..)` method takes [`d3pmr1::W`](W) writer structure"]
impl crate::Writable for D3PMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PMR1 to value 0"]
impl crate::Resettable for D3PMR1rs {
    const RESET_VALUE: u32 = 0;
}
