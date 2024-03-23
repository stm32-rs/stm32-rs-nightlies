#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CALRAF` writer - Clear alarm A flag"]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - Clear alarm B flag"]
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag"]
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - Clear timestamp flag"]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag"]
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag"]
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
#[doc = "RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
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
