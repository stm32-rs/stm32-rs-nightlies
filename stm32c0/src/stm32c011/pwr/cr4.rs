#[doc = "Register `CR4` reader"]
pub type R = crate::R<CR4rs>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<CR4rs>;
#[doc = "Field `WP1` reader - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP1_R = crate::BitReader;
#[doc = "Field `WP1` writer - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP2` reader - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP2_R = crate::BitReader;
#[doc = "Field `WP2` writer - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP3` reader - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP3_R = crate::BitReader;
#[doc = "Field `WP3` writer - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP4` reader - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP4_R = crate::BitReader;
#[doc = "Field `WP4` writer - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP6` reader - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP6_R = crate::BitReader;
#[doc = "Field `WP6` writer - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
pub type WP6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUP1 wakeup pin polarity WKUP1 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<CR4rs> {
        WP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - WKUP2 wakeup pin polarity WKUP2 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<CR4rs> {
        WP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - WKUP3 wakeup pin polarity WKUP3 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<CR4rs> {
        WP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - WKUP4 wakeup pin polarity WKUP4 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<CR4rs> {
        WP4_W::new(self, 3)
    }
    #[doc = "Bit 5 - WKUP6 wakeup pin polarity WKUP6 external wakeup signal polarity (level or edge) to generate wakeup condition:"]
    #[inline(always)]
    #[must_use]
    pub fn wp6(&mut self) -> WP6_W<CR4rs> {
        WP6_W::new(self, 5)
    }
}
#[doc = "PWR control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for CR4rs {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4rs {
    const RESET_VALUE: u32 = 0;
}
