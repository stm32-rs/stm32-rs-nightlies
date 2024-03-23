#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OTYPERrs>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OTYPERrs>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT0 {
    #[doc = "0: Output push-pull (reset state)"]
    PushPull = 0,
    #[doc = "1: Output open-drain"]
    OpenDrain = 1,
}
impl From<OT0> for bool {
    #[inline(always)]
    fn from(variant: OT0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT0` reader - Port x configuration bits (y = 0..15)"]
pub type OT0_R = crate::BitReader<OT0>;
impl OT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT0 {
        match self.bits {
            false => OT0::PushPull,
            true => OT0::OpenDrain,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OT0::PushPull
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OT0::OpenDrain
    }
}
#[doc = "Field `OT0` writer - Port x configuration bits (y = 0..15)"]
pub type OT0_W<'a, REG> = crate::BitWriter<'a, REG, OT0>;
impl<'a, REG> OT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OT0::PushPull)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OT0::OpenDrain)
    }
}
#[doc = "Field `OT1` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT1_R;
#[doc = "Field `OT2` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT2_R;
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT3_R;
#[doc = "Field `OT4` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT4_R;
#[doc = "Field `OT5` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT5_R;
#[doc = "Field `OT6` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT6_R;
#[doc = "Field `OT7` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT7_R;
#[doc = "Field `OT8` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT8_R;
#[doc = "Field `OT9` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT9_R;
#[doc = "Field `OT10` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT10_R;
#[doc = "Field `OT11` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT11_R;
#[doc = "Field `OT12` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT12_R;
#[doc = "Field `OT13` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT13_R;
#[doc = "Field `OT14` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT14_R;
#[doc = "Field `OT15` reader - Port x configuration bits (y = 0..15)"]
pub use OT0_R as OT15_R;
#[doc = "Field `OT1` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT1_W;
#[doc = "Field `OT2` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT2_W;
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT3_W;
#[doc = "Field `OT4` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT4_W;
#[doc = "Field `OT5` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT5_W;
#[doc = "Field `OT6` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT6_W;
#[doc = "Field `OT7` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT7_W;
#[doc = "Field `OT8` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT8_W;
#[doc = "Field `OT9` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT9_W;
#[doc = "Field `OT10` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT10_W;
#[doc = "Field `OT11` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT11_W;
#[doc = "Field `OT12` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT12_W;
#[doc = "Field `OT13` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT13_W;
#[doc = "Field `OT14` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT14_W;
#[doc = "Field `OT15` writer - Port x configuration bits (y = 0..15)"]
pub use OT0_W as OT15_W;
impl R {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&self) -> OT7_R {
        OT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&self) -> OT8_R {
        OT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot9(&self) -> OT9_R {
        OT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot10(&self) -> OT10_R {
        OT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot11(&self) -> OT11_R {
        OT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot12(&self) -> OT12_R {
        OT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT0_W<OTYPERrs> {
        OT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT1_W<OTYPERrs> {
        OT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> OT2_W<OTYPERrs> {
        OT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<OTYPERrs> {
        OT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> OT4_W<OTYPERrs> {
        OT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> OT5_W<OTYPERrs> {
        OT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> OT6_W<OTYPERrs> {
        OT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot7(&mut self) -> OT7_W<OTYPERrs> {
        OT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot8(&mut self) -> OT8_W<OTYPERrs> {
        OT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot9(&mut self) -> OT9_W<OTYPERrs> {
        OT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot10(&mut self) -> OT10_W<OTYPERrs> {
        OT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot11(&mut self) -> OT11_W<OTYPERrs> {
        OT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot12(&mut self) -> OT12_W<OTYPERrs> {
        OT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> OT13_W<OTYPERrs> {
        OT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> OT14_W<OTYPERrs> {
        OT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> OT15_W<OTYPERrs> {
        OT15_W::new(self, 15)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OTYPERrs {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPERrs {
    const RESET_VALUE: u32 = 0;
}
