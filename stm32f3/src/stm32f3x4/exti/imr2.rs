#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2rs>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2rs>;
#[doc = "Interrupt Mask on external/internal line 32\n\nValue on reset: 0"]
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
#[doc = "Field `MR32` reader - Interrupt Mask on external/internal line 32"]
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
#[doc = "Field `MR32` writer - Interrupt Mask on external/internal line 32"]
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
#[doc = "Field `MR33` reader - Interrupt Mask on external/internal line 33"]
pub use MR32_R as MR33_R;
#[doc = "Field `MR34` reader - Interrupt Mask on external/internal line 34"]
pub use MR32_R as MR34_R;
#[doc = "Field `MR35` reader - Interrupt Mask on external/internal line 35"]
pub use MR32_R as MR35_R;
#[doc = "Field `MR33` writer - Interrupt Mask on external/internal line 33"]
pub use MR32_W as MR33_W;
#[doc = "Field `MR34` writer - Interrupt Mask on external/internal line 34"]
pub use MR32_W as MR34_W;
#[doc = "Field `MR35` writer - Interrupt Mask on external/internal line 35"]
pub use MR32_W as MR35_W;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<IMR2rs> {
        MR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<IMR2rs> {
        MR33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<IMR2rs> {
        MR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<IMR2rs> {
        MR35_W::new(self, 3)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0xffff_fffc"]
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0xffff_fffc;
}
