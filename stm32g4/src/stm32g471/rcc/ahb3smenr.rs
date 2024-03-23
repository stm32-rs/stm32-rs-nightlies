#[doc = "Register `AHB3SMENR` reader"]
pub type R = crate::R<AHB3SMENRrs>;
#[doc = "Register `AHB3SMENR` writer"]
pub type W = crate::W<AHB3SMENRrs>;
#[doc = "Field `FMCSMEN` reader - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_R = crate::BitReader;
#[doc = "Field `FMCSMEN` writer - Flexible memory controller clocks enable during Sleep and Stop modes"]
pub type FMCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPISMEN` reader - QUADSPI memory interface clock enable during Sleep and Stop modes"]
pub type QSPISMEN_R = crate::BitReader;
#[doc = "Field `QSPISMEN` writer - QUADSPI memory interface clock enable during Sleep and Stop modes"]
pub type QSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FMCSMEN_R {
        FMCSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsmen(&mut self) -> FMCSMEN_W<AHB3SMENRrs> {
        FMCSMEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn qspismen(&mut self) -> QSPISMEN_W<AHB3SMENRrs> {
        QSPISMEN_W::new(self, 8)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3smenr::R`](R) reader structure"]
impl crate::Readable for AHB3SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure"]
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0101"]
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0101;
}
