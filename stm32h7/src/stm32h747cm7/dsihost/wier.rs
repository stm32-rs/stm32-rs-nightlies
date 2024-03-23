#[doc = "Register `WIER` reader"]
pub type R = crate::R<WIERrs>;
#[doc = "Register `WIER` writer"]
pub type W = crate::W<WIERrs>;
#[doc = "Field `TEIE` reader - Tearing effect interrupt enable"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Tearing effect interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - End of refresh interrupt enable"]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - End of refresh interrupt enable"]
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLIE` reader - PLL lock interrupt enable"]
pub type PLLLIE_R = crate::BitReader;
#[doc = "Field `PLLLIE` writer - PLL lock interrupt enable"]
pub type PLLLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUIE` reader - PLL unlock interrupt enable"]
pub type PLLUIE_R = crate::BitReader;
#[doc = "Field `PLLUIE` writer - PLL unlock interrupt enable"]
pub type PLLUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIE` reader - Regulator ready interrupt enable"]
pub type RRIE_R = crate::BitReader;
#[doc = "Field `RRIE` writer - Regulator ready interrupt enable"]
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tearing effect interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt enable"]
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt enable"]
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Regulator ready interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing effect interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<WIERrs> {
        TEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<WIERrs> {
        ERIE_W::new(self, 1)
    }
    #[doc = "Bit 9 - PLL lock interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn plllie(&mut self) -> PLLLIE_W<WIERrs> {
        PLLLIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - PLL unlock interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn plluie(&mut self) -> PLLUIE_W<WIERrs> {
        PLLUIE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Regulator ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<WIERrs> {
        RRIE_W::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIERrs;
impl crate::RegisterSpec for WIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wier::R`](R) reader structure"]
impl crate::Readable for WIERrs {}
#[doc = "`write(|w| ..)` method takes [`wier::W`](W) writer structure"]
impl crate::Writable for WIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIER to value 0"]
impl crate::Resettable for WIERrs {
    const RESET_VALUE: u32 = 0;
}
