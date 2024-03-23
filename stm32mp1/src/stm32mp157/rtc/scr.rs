#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CALRAF` writer - CALRAF"]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - CALRBF"]
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - CWUTF"]
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - CTSF"]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - CTSOVF"]
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - CITSF"]
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<SCRrs> {
        CALRBF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<SCRrs> {
        CWUTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<SCRrs> {
        CITSF_W::new(self, 5)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
