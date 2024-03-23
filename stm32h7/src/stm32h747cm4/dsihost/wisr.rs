#[doc = "Register `WISR` reader"]
pub type R = crate::R<WISRrs>;
#[doc = "Register `WISR` writer"]
pub type W = crate::W<WISRrs>;
#[doc = "Field `TEIF` reader - Tearing effect interrupt flag"]
pub type TEIF_R = crate::BitReader;
#[doc = "Field `TEIF` writer - Tearing effect interrupt flag"]
pub type TEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIF` reader - End of refresh interrupt flag"]
pub type ERIF_R = crate::BitReader;
#[doc = "Field `ERIF` writer - End of refresh interrupt flag"]
pub type ERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Busy flag"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLS` reader - PLL lock status"]
pub type PLLLS_R = crate::BitReader;
#[doc = "Field `PLLLS` writer - PLL lock status"]
pub type PLLLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLIF` reader - PLL lock interrupt flag"]
pub type PLLLIF_R = crate::BitReader;
#[doc = "Field `PLLLIF` writer - PLL lock interrupt flag"]
pub type PLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUIF` reader - PLL unlock interrupt flag"]
pub type PLLUIF_R = crate::BitReader;
#[doc = "Field `PLLUIF` writer - PLL unlock interrupt flag"]
pub type PLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRS` reader - Regulator ready status"]
pub type RRS_R = crate::BitReader;
#[doc = "Field `RRS` writer - Regulator ready status"]
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIF` reader - Regulator ready interrupt flag"]
pub type RRIF_R = crate::BitReader;
#[doc = "Field `RRIF` writer - Regulator ready interrupt flag"]
pub type RRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn teif(&mut self) -> TEIF_W<WISRrs> {
        TEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn erif(&mut self) -> ERIF_W<WISRrs> {
        ERIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<WISRrs> {
        BUSY_W::new(self, 2)
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    #[must_use]
    pub fn pllls(&mut self) -> PLLLS_W<WISRrs> {
        PLLLS_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn plllif(&mut self) -> PLLLIF_W<WISRrs> {
        PLLLIF_W::new(self, 9)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn plluif(&mut self) -> PLLUIF_W<WISRrs> {
        PLLUIF_W::new(self, 10)
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<WISRrs> {
        RRS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rrif(&mut self) -> RRIF_W<WISRrs> {
        RRIF_W::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wisr::R`](R) reader structure"]
impl crate::Readable for WISRrs {}
#[doc = "`write(|w| ..)` method takes [`wisr::W`](W) writer structure"]
impl crate::Writable for WISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISRrs {
    const RESET_VALUE: u32 = 0;
}
