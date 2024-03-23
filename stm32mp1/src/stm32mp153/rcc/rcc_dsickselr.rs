#[doc = "Register `RCC_DSICKSELR` reader"]
pub type R = crate::R<RCC_DSICKSELRrs>;
#[doc = "Register `RCC_DSICKSELR` writer"]
pub type W = crate::W<RCC_DSICKSELRrs>;
#[doc = "Field `DSISRC` reader - DSISRC"]
pub type DSISRC_R = crate::BitReader;
#[doc = "Field `DSISRC` writer - DSISRC"]
pub type DSISRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&self) -> DSISRC_R {
        DSISRC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    #[must_use]
    pub fn dsisrc(&mut self) -> DSISRC_W<RCC_DSICKSELRrs> {
        DSISRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the DSI block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dsickselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dsickselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_DSICKSELRrs;
impl crate::RegisterSpec for RCC_DSICKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_dsickselr::R`](R) reader structure"]
impl crate::Readable for RCC_DSICKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_dsickselr::W`](W) writer structure"]
impl crate::Writable for RCC_DSICKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_DSICKSELR to value 0"]
impl crate::Resettable for RCC_DSICKSELRrs {
    const RESET_VALUE: u32 = 0;
}
