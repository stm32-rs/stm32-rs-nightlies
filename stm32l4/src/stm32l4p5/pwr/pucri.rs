#[doc = "Register `PUCRI` reader"]
pub type R = crate::R<PUCRIrs>;
#[doc = "Register `PUCRI` writer"]
pub type W = crate::W<PUCRIrs>;
#[doc = "Port I pull-up bit y (y=0..11)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0 {
    #[doc = "0: Pull-Up on Pxx is disabled"]
    Disabled = 0,
    #[doc = "1: Pull-Up on Pxx is enabled"]
    Enabled = 1,
}
impl From<PU0> for bool {
    #[inline(always)]
    fn from(variant: PU0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU0` reader - Port I pull-up bit y (y=0..11)"]
pub type PU0_R = crate::BitReader<PU0>;
impl PU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PU0 {
        match self.bits {
            false => PU0::Disabled,
            true => PU0::Enabled,
        }
    }
    #[doc = "Pull-Up on Pxx is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0::Disabled
    }
    #[doc = "Pull-Up on Pxx is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0::Enabled
    }
}
#[doc = "Field `PU0` writer - Port I pull-up bit y (y=0..11)"]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG, PU0>;
impl<'a, REG> PU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-Up on Pxx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Disabled)
    }
    #[doc = "Pull-Up on Pxx is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PU0::Enabled)
    }
}
#[doc = "Field `PU1` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU1_R;
#[doc = "Field `PU2` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU2_R;
#[doc = "Field `PU3` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU3_R;
#[doc = "Field `PU4` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU4_R;
#[doc = "Field `PU5` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU5_R;
#[doc = "Field `PU6` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU6_R;
#[doc = "Field `PU7` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU7_R;
#[doc = "Field `PU8` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU8_R;
#[doc = "Field `PU9` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU9_R;
#[doc = "Field `PU10` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU10_R;
#[doc = "Field `PU11` reader - Port I pull-up bit y (y=0..11)"]
pub use PU0_R as PU11_R;
#[doc = "Field `PU1` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU1_W;
#[doc = "Field `PU2` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU2_W;
#[doc = "Field `PU3` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU3_W;
#[doc = "Field `PU4` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU4_W;
#[doc = "Field `PU5` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU5_W;
#[doc = "Field `PU6` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU6_W;
#[doc = "Field `PU7` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU7_W;
#[doc = "Field `PU8` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU8_W;
#[doc = "Field `PU9` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU9_W;
#[doc = "Field `PU10` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU10_W;
#[doc = "Field `PU11` writer - Port I pull-up bit y (y=0..11)"]
pub use PU0_W as PU11_W;
impl R {
    #[doc = "Bit 0 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PUCRIrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PUCRIrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PUCRIrs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PUCRIrs> {
        PU3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PUCRIrs> {
        PU4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<PUCRIrs> {
        PU5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<PUCRIrs> {
        PU6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<PUCRIrs> {
        PU7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<PUCRIrs> {
        PU8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<PUCRIrs> {
        PU9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<PUCRIrs> {
        PU10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port I pull-up bit y (y=0..11)"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<PUCRIrs> {
        PU11_W::new(self, 11)
    }
}
#[doc = "Power Port I pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRIrs;
impl crate::RegisterSpec for PUCRIrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucri::R`](R) reader structure"]
impl crate::Readable for PUCRIrs {}
#[doc = "`write(|w| ..)` method takes [`pucri::W`](W) writer structure"]
impl crate::Writable for PUCRIrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCRI to value 0"]
impl crate::Resettable for PUCRIrs {
    const RESET_VALUE: u32 = 0;
}
