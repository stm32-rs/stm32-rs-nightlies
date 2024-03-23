#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCRrs>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CALRAF` reader - Clear alarm A flag"]
pub type CALRAF_R = crate::BitReader;
#[doc = "Field `CALRAF` writer - Clear alarm A flag"]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` reader - Clear alarm B flag"]
pub type CALRBF_R = crate::BitReader;
#[doc = "Field `CALRBF` writer - Clear alarm B flag"]
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` reader - Clear wakeup timer flag"]
pub type CWUTF_R = crate::BitReader;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag"]
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` reader - Clear timestamp flag"]
pub type CTSF_R = crate::BitReader;
#[doc = "Field `CTSF` writer - Clear timestamp flag"]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` reader - Clear timestamp overflow flag"]
pub type CTSOVF_R = crate::BitReader;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag"]
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` reader - Clear internal timestamp flag"]
pub type CITSF_R = crate::BitReader;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag"]
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSRUF` reader - Clear SSR underflow flag"]
pub type CSSRUF_R = crate::BitReader;
#[doc = "Field `CSSRUF` writer - Clear SSR underflow flag"]
pub type CSSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear alarm A flag"]
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear alarm B flag"]
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag"]
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear timestamp flag"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag"]
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag"]
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear SSR underflow flag"]
    #[inline(always)]
    pub fn cssruf(&self) -> CSSRUF_R {
        CSSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<SCRrs> {
        CALRBF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<SCRrs> {
        CWUTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear timestamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<SCRrs> {
        CITSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear SSR underflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn cssruf(&mut self) -> CSSRUF_W<SCRrs> {
        CSSRUF_W::new(self, 6)
    }
}
#[doc = "RTC status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCRrs {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
