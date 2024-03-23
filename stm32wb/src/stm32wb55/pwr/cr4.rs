#[doc = "Register `CR4` reader"]
pub type R = crate::R<CR4rs>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<CR4rs>;
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity"]
pub type WP1_R = crate::BitReader;
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity"]
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub type WP2_R = crate::BitReader;
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub type WP3_R = crate::BitReader;
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP4` reader - Wakeup pin WKUP4 polarity"]
pub type WP4_R = crate::BitReader;
#[doc = "Field `WP4` writer - Wakeup pin WKUP4 polarity"]
pub type WP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP5` reader - Wakeup pin WKUP5 polarity"]
pub type WP5_R = crate::BitReader;
#[doc = "Field `WP5` writer - Wakeup pin WKUP5 polarity"]
pub type WP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub type VBE_R = crate::BitReader;
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub type VBRS_R = crate::BitReader;
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2BOOT` reader - BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
pub type C2BOOT_R = crate::BitReader;
#[doc = "Field `C2BOOT` writer - BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
pub type C2BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
    #[inline(always)]
    pub fn c2boot(&self) -> C2BOOT_R {
        C2BOOT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<CR4rs> {
        WP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<CR4rs> {
        WP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<CR4rs> {
        WP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<CR4rs> {
        WP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> WP5_W<CR4rs> {
        WP5_W::new(self, 4)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<CR4rs> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<CR4rs> {
        VBRS_W::new(self, 9)
    }
    #[doc = "Bit 15 - BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
    #[inline(always)]
    #[must_use]
    pub fn c2boot(&mut self) -> C2BOOT_W<CR4rs> {
        C2BOOT_W::new(self, 15)
    }
}
#[doc = "Power control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
