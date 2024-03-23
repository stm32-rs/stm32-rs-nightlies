#[doc = "Register `DDRCTRL_ZQCTL2` reader"]
pub type R = crate::R<DDRCTRL_ZQCTL2rs>;
#[doc = "Register `DDRCTRL_ZQCTL2` writer"]
pub type W = crate::W<DDRCTRL_ZQCTL2rs>;
#[doc = "Field `ZQ_RESET` reader - ZQ_RESET"]
pub type ZQ_RESET_R = crate::BitReader;
#[doc = "Field `ZQ_RESET` writer - ZQ_RESET"]
pub type ZQ_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&self) -> ZQ_RESET_R {
        ZQ_RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    #[must_use]
    pub fn zq_reset(&mut self) -> ZQ_RESET_W<DDRCTRL_ZQCTL2rs> {
        ZQ_RESET_W::new(self, 0)
    }
}
#[doc = "DDRCTRL ZQ control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ZQCTL2rs;
impl crate::RegisterSpec for DDRCTRL_ZQCTL2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_zqctl2::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_zqctl2::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL2 to value 0"]
impl crate::Resettable for DDRCTRL_ZQCTL2rs {
    const RESET_VALUE: u32 = 0;
}
