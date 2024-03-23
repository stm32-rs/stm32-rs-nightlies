#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDRrs>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDRrs>;
#[doc = "Port x configuration pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPDR0 {
    #[doc = "0: No pull-up, pull-down"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PUPDR0> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPDR0 {
    type Ux = u8;
}
#[doc = "Field `PUPDR(0-15)` reader - Port x configuration pin %s"]
pub type PUPDR_R = crate::FieldReader<PUPDR0>;
impl PUPDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPDR0> {
        match self.bits {
            0 => Some(PUPDR0::Floating),
            1 => Some(PUPDR0::PullUp),
            2 => Some(PUPDR0::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPDR0::Floating
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPDR0::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPDR0::PullDown
    }
}
#[doc = "Field `PUPDR(0-15)` writer - Port x configuration pin %s"]
pub type PUPDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPDR0>;
impl<'a, REG> PUPDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR0::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR0::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR0::PullDown)
    }
}
impl R {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `PUPDR0` field"]
    #[inline(always)]
    pub fn pupdr(&self, n: u8) -> PUPDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port x configuration pin (0-15)"]
    #[inline(always)]
    pub fn pupdr_iter(&self) -> impl Iterator<Item = PUPDR_R> + '_ {
        (0..16).map(move |n| PUPDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR_R {
        PUPDR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR_R {
        PUPDR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `PUPDR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr(&mut self, n: u8) -> PUPDR_W<PUPDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PUPDR_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr8(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr9(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr10(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr11(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr12(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr13(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr14(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr15(&mut self) -> PUPDR_W<PUPDRrs> {
        PUPDR_W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0x1210_0000"]
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0x1210_0000;
}
