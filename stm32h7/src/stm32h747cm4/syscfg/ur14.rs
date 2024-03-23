#[doc = "Register `UR14` reader"]
pub type R = crate::R<UR14rs>;
#[doc = "Register `UR14` writer"]
pub type W = crate::W<UR14rs>;
#[doc = "Field `D1STPRST` reader - D1 Stop Reset"]
pub type D1STPRST_R = crate::BitReader;
#[doc = "Field `D1STPRST` writer - D1 Stop Reset"]
pub type D1STPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SBRST` reader - D2 Standby Reset"]
pub type D2SBRST_R = crate::BitReader;
#[doc = "Field `D2SBRST` writer - D2 Standby Reset"]
pub type D2SBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&self) -> D1STPRST_R {
        D1STPRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    pub fn d2sbrst(&self) -> D2SBRST_R {
        D2SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d1stprst(&mut self) -> D1STPRST_W<UR14rs> {
        D1STPRST_W::new(self, 0)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d2sbrst(&mut self) -> D2SBRST_W<UR14rs> {
        D2SBRST_W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR14rs;
impl crate::RegisterSpec for UR14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur14::R`](R) reader structure"]
impl crate::Readable for UR14rs {}
#[doc = "`write(|w| ..)` method takes [`ur14::W`](W) writer structure"]
impl crate::Writable for UR14rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UR14 to value 0"]
impl crate::Resettable for UR14rs {
    const RESET_VALUE: u32 = 0;
}
