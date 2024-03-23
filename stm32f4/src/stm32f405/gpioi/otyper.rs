#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OTYPERrs>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OTYPERrs>;
#[doc = "Port x configuration pin %s\n\nValue on reset: 0"]
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
#[doc = "Field `OT(0-15)` reader - Port x configuration pin %s"]
pub type OT_R = crate::BitReader<OT0>;
impl OT_R {
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
#[doc = "Field `OT(0-15)` writer - Port x configuration pin %s"]
pub type OT_W<'a, REG> = crate::BitWriter<'a, REG, OT0>;
impl<'a, REG> OT_W<'a, REG>
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
impl R {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OT0` field"]
    #[inline(always)]
    pub fn ot(&self, n: u8) -> OT_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port x configuration pin (0-15)"]
    #[inline(always)]
    pub fn ot_iter(&self) -> impl Iterator<Item = OT_R> + '_ {
        (0..16).map(move |n| OT_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port x configuration pin 0"]
    #[inline(always)]
    pub fn ot0(&self) -> OT_R {
        OT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration pin 1"]
    #[inline(always)]
    pub fn ot1(&self) -> OT_R {
        OT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration pin 2"]
    #[inline(always)]
    pub fn ot2(&self) -> OT_R {
        OT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration pin 3"]
    #[inline(always)]
    pub fn ot3(&self) -> OT_R {
        OT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration pin 4"]
    #[inline(always)]
    pub fn ot4(&self) -> OT_R {
        OT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration pin 5"]
    #[inline(always)]
    pub fn ot5(&self) -> OT_R {
        OT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration pin 6"]
    #[inline(always)]
    pub fn ot6(&self) -> OT_R {
        OT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration pin 7"]
    #[inline(always)]
    pub fn ot7(&self) -> OT_R {
        OT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration pin 8"]
    #[inline(always)]
    pub fn ot8(&self) -> OT_R {
        OT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration pin 9"]
    #[inline(always)]
    pub fn ot9(&self) -> OT_R {
        OT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration pin 10"]
    #[inline(always)]
    pub fn ot10(&self) -> OT_R {
        OT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration pin 11"]
    #[inline(always)]
    pub fn ot11(&self) -> OT_R {
        OT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration pin 12"]
    #[inline(always)]
    pub fn ot12(&self) -> OT_R {
        OT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration pin 13"]
    #[inline(always)]
    pub fn ot13(&self) -> OT_R {
        OT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration pin 14"]
    #[inline(always)]
    pub fn ot14(&self) -> OT_R {
        OT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration pin 15"]
    #[inline(always)]
    pub fn ot15(&self) -> OT_R {
        OT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OT0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ot(&mut self, n: u8) -> OT_W<OTYPERrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OT_W::new(self, n)
    }
    #[doc = "Bit 0 - Port x configuration pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn ot7(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn ot8(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn ot9(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn ot10(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn ot11(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn ot12(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> OT_W<OTYPERrs> {
        OT_W::new(self, 15)
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
