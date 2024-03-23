#[doc = "Register `ACR2` reader"]
pub type R = crate::R<ACR2rs>;
#[doc = "Register `ACR2` writer"]
pub type W = crate::W<ACR2rs>;
#[doc = "Field `PRIVMODE` reader - CFI privileged mode enable"]
pub type PRIVMODE_R = crate::BitReader;
#[doc = "Field `PRIVMODE` writer - CFI privileged mode enable"]
pub type PRIVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDPADIS` reader - Flash user hide protection area access disable"]
pub type HDPADIS_R = crate::BitReader;
#[doc = "Field `HDPADIS` writer - Flash user hide protection area access disable"]
pub type HDPADIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2SWDBGEN` reader - CPU2 Software debug enable"]
pub type C2SWDBGEN_R = crate::BitReader;
#[doc = "Field `C2SWDBGEN` writer - CPU2 Software debug enable"]
pub type C2SWDBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&self) -> PRIVMODE_R {
        PRIVMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&self) -> HDPADIS_R {
        HDPADIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&self) -> C2SWDBGEN_R {
        C2SWDBGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn privmode(&mut self) -> PRIVMODE_W<ACR2rs> {
        PRIVMODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    #[must_use]
    pub fn hdpadis(&mut self) -> HDPADIS_W<ACR2rs> {
        HDPADIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2swdbgen(&mut self) -> C2SWDBGEN_W<ACR2rs> {
        C2SWDBGEN_W::new(self, 2)
    }
}
#[doc = "Flash access control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR2rs;
impl crate::RegisterSpec for ACR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr2::R`](R) reader structure"]
impl crate::Readable for ACR2rs {}
#[doc = "`write(|w| ..)` method takes [`acr2::W`](W) writer structure"]
impl crate::Writable for ACR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR2 to value 0"]
impl crate::Resettable for ACR2rs {
    const RESET_VALUE: u32 = 0;
}
