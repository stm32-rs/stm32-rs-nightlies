#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Clear alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRAF {
    #[doc = "1: Clear interrupt flag by writing 1"]
    Clear = 1,
}
impl From<CALRAF> for bool {
    #[inline(always)]
    fn from(variant: CALRAF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRAF` writer - Clear alarm A flag"]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG, CALRAF>;
impl<'a, REG> CALRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt flag by writing 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CALRAF::Clear)
    }
}
#[doc = "Field `CALRBF` writer - Clear alarm B flag"]
pub use CALRAF_W as CALRBF_W;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag"]
pub use CALRAF_W as CWUTF_W;
#[doc = "Field `CTSF` writer - Clear timestamp flag"]
pub use CALRAF_W as CTSF_W;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag"]
pub use CALRAF_W as CTSOVF_W;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag"]
pub use CALRAF_W as CITSF_W;
#[doc = "Field `CSSRUF` writer - Clear SSR underflow flag"]
pub use CALRAF_W as CSSRUF_W;
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
#[doc = "Status clear register (interrupts)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
