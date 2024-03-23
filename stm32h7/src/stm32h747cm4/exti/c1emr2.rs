#[doc = "Register `C1EMR2` reader"]
pub type R = crate::R<C1EMR2rs>;
#[doc = "Register `C1EMR2` writer"]
pub type W = crate::W<C1EMR2rs>;
#[doc = "CPU Interrupt Mask on Direct Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR32> for bool {
    #[inline(always)]
    fn from(variant: MR32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR32` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub type MR32_R = crate::BitReader<MR32>;
impl MR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR32 {
        match self.bits {
            false => MR32::Masked,
            true => MR32::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32::Unmasked
    }
}
#[doc = "Field `MR32` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub type MR32_W<'a, REG> = crate::BitWriter<'a, REG, MR32>;
impl<'a, REG> MR32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR32::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR32::Unmasked)
    }
}
#[doc = "Field `MR33` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR33_R;
#[doc = "Field `MR34` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR34_R;
#[doc = "Field `MR35` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR35_R;
#[doc = "Field `MR36` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR36_R;
#[doc = "Field `MR37` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR37_R;
#[doc = "Field `MR38` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR38_R;
#[doc = "Field `MR39` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR39_R;
#[doc = "Field `MR40` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR40_R;
#[doc = "Field `MR41` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR41_R;
#[doc = "Field `MR42` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR42_R;
#[doc = "Field `MR43` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR43_R;
#[doc = "Field `MR44` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR44_R;
#[doc = "Field `MR46` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR46_R;
#[doc = "Field `MR47` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR47_R;
#[doc = "Field `MR48` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR48_R;
#[doc = "Field `MR49` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR49_R;
#[doc = "Field `MR50` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR50_R;
#[doc = "Field `MR51` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR51_R;
#[doc = "Field `MR52` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR52_R;
#[doc = "Field `MR53` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR53_R;
#[doc = "Field `MR54` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR54_R;
#[doc = "Field `MR55` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR55_R;
#[doc = "Field `MR56` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR56_R;
#[doc = "Field `MR57` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR57_R;
#[doc = "Field `MR58` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR58_R;
#[doc = "Field `MR59` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR59_R;
#[doc = "Field `MR60` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR60_R;
#[doc = "Field `MR61` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR61_R;
#[doc = "Field `MR62` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR62_R;
#[doc = "Field `MR63` reader - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_R as MR63_R;
#[doc = "Field `MR33` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR33_W;
#[doc = "Field `MR34` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR34_W;
#[doc = "Field `MR35` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR35_W;
#[doc = "Field `MR36` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR36_W;
#[doc = "Field `MR37` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR37_W;
#[doc = "Field `MR38` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR38_W;
#[doc = "Field `MR39` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR39_W;
#[doc = "Field `MR40` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR40_W;
#[doc = "Field `MR41` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR41_W;
#[doc = "Field `MR42` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR42_W;
#[doc = "Field `MR43` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR43_W;
#[doc = "Field `MR44` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR44_W;
#[doc = "Field `MR46` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR46_W;
#[doc = "Field `MR47` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR47_W;
#[doc = "Field `MR48` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR48_W;
#[doc = "Field `MR49` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR49_W;
#[doc = "Field `MR50` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR50_W;
#[doc = "Field `MR51` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR51_W;
#[doc = "Field `MR52` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR52_W;
#[doc = "Field `MR53` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR53_W;
#[doc = "Field `MR54` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR54_W;
#[doc = "Field `MR55` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR55_W;
#[doc = "Field `MR56` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR56_W;
#[doc = "Field `MR57` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR57_W;
#[doc = "Field `MR58` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR58_W;
#[doc = "Field `MR59` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR59_W;
#[doc = "Field `MR60` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR60_W;
#[doc = "Field `MR61` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR61_W;
#[doc = "Field `MR62` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR62_W;
#[doc = "Field `MR63` writer - CPU Interrupt Mask on Direct Event input x+32"]
pub use MR32_W as MR63_W;
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<C1EMR2rs> {
        MR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<C1EMR2rs> {
        MR33_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<C1EMR2rs> {
        MR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<C1EMR2rs> {
        MR35_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr36(&mut self) -> MR36_W<C1EMR2rs> {
        MR36_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr37(&mut self) -> MR37_W<C1EMR2rs> {
        MR37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr38(&mut self) -> MR38_W<C1EMR2rs> {
        MR38_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr39(&mut self) -> MR39_W<C1EMR2rs> {
        MR39_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr40(&mut self) -> MR40_W<C1EMR2rs> {
        MR40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr41(&mut self) -> MR41_W<C1EMR2rs> {
        MR41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr42(&mut self) -> MR42_W<C1EMR2rs> {
        MR42_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr43(&mut self) -> MR43_W<C1EMR2rs> {
        MR43_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr44(&mut self) -> MR44_W<C1EMR2rs> {
        MR44_W::new(self, 12)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr46(&mut self) -> MR46_W<C1EMR2rs> {
        MR46_W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr47(&mut self) -> MR47_W<C1EMR2rs> {
        MR47_W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr48(&mut self) -> MR48_W<C1EMR2rs> {
        MR48_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr49(&mut self) -> MR49_W<C1EMR2rs> {
        MR49_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr50(&mut self) -> MR50_W<C1EMR2rs> {
        MR50_W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr51(&mut self) -> MR51_W<C1EMR2rs> {
        MR51_W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr52(&mut self) -> MR52_W<C1EMR2rs> {
        MR52_W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr53(&mut self) -> MR53_W<C1EMR2rs> {
        MR53_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr54(&mut self) -> MR54_W<C1EMR2rs> {
        MR54_W::new(self, 22)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr55(&mut self) -> MR55_W<C1EMR2rs> {
        MR55_W::new(self, 23)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr56(&mut self) -> MR56_W<C1EMR2rs> {
        MR56_W::new(self, 24)
    }
    #[doc = "Bit 25 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr57(&mut self) -> MR57_W<C1EMR2rs> {
        MR57_W::new(self, 25)
    }
    #[doc = "Bit 26 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr58(&mut self) -> MR58_W<C1EMR2rs> {
        MR58_W::new(self, 26)
    }
    #[doc = "Bit 27 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr59(&mut self) -> MR59_W<C1EMR2rs> {
        MR59_W::new(self, 27)
    }
    #[doc = "Bit 28 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr60(&mut self) -> MR60_W<C1EMR2rs> {
        MR60_W::new(self, 28)
    }
    #[doc = "Bit 29 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr61(&mut self) -> MR61_W<C1EMR2rs> {
        MR61_W::new(self, 29)
    }
    #[doc = "Bit 30 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr62(&mut self) -> MR62_W<C1EMR2rs> {
        MR62_W::new(self, 30)
    }
    #[doc = "Bit 31 - CPU Interrupt Mask on Direct Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr63(&mut self) -> MR63_W<C1EMR2rs> {
        MR63_W::new(self, 31)
    }
}
#[doc = "EXTI event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1EMR2rs;
impl crate::RegisterSpec for C1EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1emr2::R`](R) reader structure"]
impl crate::Readable for C1EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`c1emr2::W`](W) writer structure"]
impl crate::Writable for C1EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1EMR2 to value 0"]
impl crate::Resettable for C1EMR2rs {
    const RESET_VALUE: u32 = 0;
}
