#[doc = "Register `APB3SMENR` reader"]
pub type R = crate::R<APB3SMENRrs>;
#[doc = "Register `APB3SMENR` writer"]
pub type W = crate::W<APB3SMENRrs>;
#[doc = "Field `SUBGHZSPISMEN` reader - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
pub type SUBGHZSPISMEN_R = crate::BitReader;
#[doc = "Field `SUBGHZSPISMEN` writer - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
pub type SUBGHZSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspismen(&mut self) -> SUBGHZSPISMEN_W<APB3SMENRrs> {
        SUBGHZSPISMEN_W::new(self, 0)
    }
}
#[doc = "APB3 peripheral clock enable in Sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3SMENRrs;
impl crate::RegisterSpec for APB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3smenr::R`](R) reader structure"]
impl crate::Readable for APB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3smenr::W`](W) writer structure"]
impl crate::Writable for APB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3SMENR to value 0x01"]
impl crate::Resettable for APB3SMENRrs {
    const RESET_VALUE: u32 = 0x01;
}
