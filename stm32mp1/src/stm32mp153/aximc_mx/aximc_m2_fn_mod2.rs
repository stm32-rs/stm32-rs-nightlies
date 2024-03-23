#[doc = "Register `AXIMC_M2_FN_MOD2` reader"]
pub type R = crate::R<AXIMC_M2_FN_MOD2rs>;
#[doc = "Register `AXIMC_M2_FN_MOD2` writer"]
pub type W = crate::W<AXIMC_M2_FN_MOD2rs>;
#[doc = "Field `BYPASS_MERGE` reader - BYPASS_MERGE"]
pub type BYPASS_MERGE_R = crate::BitReader;
#[doc = "Field `BYPASS_MERGE` writer - BYPASS_MERGE"]
pub type BYPASS_MERGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BYPASS_MERGE"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BYPASS_MERGE"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<AXIMC_M2_FN_MOD2rs> {
        BYPASS_MERGE_W::new(self, 0)
    }
}
#[doc = "AXIMC master 2 packing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m2_fn_mod2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m2_fn_mod2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_M2_FN_MOD2rs;
impl crate::RegisterSpec for AXIMC_M2_FN_MOD2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_m2_fn_mod2::R`](R) reader structure"]
impl crate::Readable for AXIMC_M2_FN_MOD2rs {}
#[doc = "`write(|w| ..)` method takes [`aximc_m2_fn_mod2::W`](W) writer structure"]
impl crate::Writable for AXIMC_M2_FN_MOD2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_M2_FN_MOD2 to value 0"]
impl crate::Resettable for AXIMC_M2_FN_MOD2rs {
    const RESET_VALUE: u32 = 0;
}
