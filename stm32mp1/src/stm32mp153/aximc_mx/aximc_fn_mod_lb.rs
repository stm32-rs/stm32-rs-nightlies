#[doc = "Register `AXIMC_FN_MOD_LB` reader"]
pub type R = crate::R<AXIMC_FN_MOD_LBrs>;
#[doc = "Register `AXIMC_FN_MOD_LB` writer"]
pub type W = crate::W<AXIMC_FN_MOD_LBrs>;
#[doc = "Field `FN_MOD_LB` reader - FN_MOD_LB"]
pub type FN_MOD_LB_R = crate::BitReader;
#[doc = "Field `FN_MOD_LB` writer - FN_MOD_LB"]
pub type FN_MOD_LB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FN_MOD_LB"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FN_MOD_LB"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<AXIMC_FN_MOD_LBrs> {
        FN_MOD_LB_W::new(self, 0)
    }
}
#[doc = "AXIMC long burst capability inhibition register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_fn_mod_lb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_fn_mod_lb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_FN_MOD_LBrs;
impl crate::RegisterSpec for AXIMC_FN_MOD_LBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_fn_mod_lb::R`](R) reader structure"]
impl crate::Readable for AXIMC_FN_MOD_LBrs {}
#[doc = "`write(|w| ..)` method takes [`aximc_fn_mod_lb::W`](W) writer structure"]
impl crate::Writable for AXIMC_FN_MOD_LBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_FN_MOD_LB to value 0"]
impl crate::Resettable for AXIMC_FN_MOD_LBrs {
    const RESET_VALUE: u32 = 0;
}
