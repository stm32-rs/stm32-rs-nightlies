#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PWRCRrs>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PWRCRrs>;
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub type ODEN_R = crate::BitReader;
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<PWRCRrs> {
        ODEN_W::new(self, 0)
    }
}
#[doc = "SYSCFG power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCRrs;
impl crate::RegisterSpec for PWRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PWRCRrs {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PWRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCRrs {
    const RESET_VALUE: u32 = 0;
}
