#[doc = "Register `D3PMR2` reader"]
pub type R = crate::R<D3PMR2rs>;
#[doc = "Register `D3PMR2` writer"]
pub type W = crate::W<D3PMR2rs>;
#[doc = "D3 Pending Mask on Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR34 {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<MR34> for bool {
    #[inline(always)]
    fn from(variant: MR34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR34` reader - D3 Pending Mask on Event input x+32"]
pub type MR34_R = crate::BitReader<MR34>;
impl MR34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR34 {
        match self.bits {
            false => MR34::Masked,
            true => MR34::Unmasked,
        }
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR34::Masked
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR34::Unmasked
    }
}
#[doc = "Field `MR34` writer - D3 Pending Mask on Event input x+32"]
pub type MR34_W<'a, REG> = crate::BitWriter<'a, REG, MR34>;
impl<'a, REG> MR34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(MR34::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(MR34::Unmasked)
    }
}
#[doc = "Field `MR35` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR35_R;
#[doc = "Field `MR41` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR41_R;
#[doc = "Field `MR48` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR48_R;
#[doc = "Field `MR49` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR49_R;
#[doc = "Field `MR50` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR50_R;
#[doc = "Field `MR51` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR51_R;
#[doc = "Field `MR52` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR52_R;
#[doc = "Field `MR53` reader - D3 Pending Mask on Event input x+32"]
pub use MR34_R as MR53_R;
#[doc = "Field `MR35` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR35_W;
#[doc = "Field `MR41` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR41_W;
#[doc = "Field `MR48` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR48_W;
#[doc = "Field `MR49` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR49_W;
#[doc = "Field `MR50` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR50_W;
#[doc = "Field `MR51` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR51_W;
#[doc = "Field `MR52` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR52_W;
#[doc = "Field `MR53` writer - D3 Pending Mask on Event input x+32"]
pub use MR34_W as MR53_W;
impl R {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<D3PMR2rs> {
        MR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<D3PMR2rs> {
        MR35_W::new(self, 3)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr41(&mut self) -> MR41_W<D3PMR2rs> {
        MR41_W::new(self, 9)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr48(&mut self) -> MR48_W<D3PMR2rs> {
        MR48_W::new(self, 16)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr49(&mut self) -> MR49_W<D3PMR2rs> {
        MR49_W::new(self, 17)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr50(&mut self) -> MR50_W<D3PMR2rs> {
        MR50_W::new(self, 18)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr51(&mut self) -> MR51_W<D3PMR2rs> {
        MR51_W::new(self, 19)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr52(&mut self) -> MR52_W<D3PMR2rs> {
        MR52_W::new(self, 20)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr53(&mut self) -> MR53_W<D3PMR2rs> {
        MR53_W::new(self, 21)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PMR2rs;
impl crate::RegisterSpec for D3PMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr2::R`](R) reader structure"]
impl crate::Readable for D3PMR2rs {}
#[doc = "`write(|w| ..)` method takes [`d3pmr2::W`](W) writer structure"]
impl crate::Writable for D3PMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PMR2 to value 0"]
impl crate::Resettable for D3PMR2rs {
    const RESET_VALUE: u32 = 0;
}
