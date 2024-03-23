#[doc = "Register `AXIMC_M6_FN_MOD_AHB` reader"]
pub type R = crate::R<AXIMC_M6_FN_MOD_AHBrs>;
#[doc = "Register `AXIMC_M6_FN_MOD_AHB` writer"]
pub type W = crate::W<AXIMC_M6_FN_MOD_AHBrs>;
#[doc = "Field `RD_INC_OVERRIDE` reader - RD_INC_OVERRIDE"]
pub type RD_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `RD_INC_OVERRIDE` writer - RD_INC_OVERRIDE"]
pub type RD_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_INC_OVERRIDE` reader - WR_INC_OVERRIDE"]
pub type WR_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WR_INC_OVERRIDE` writer - WR_INC_OVERRIDE"]
pub type WR_INC_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RD_INC_OVERRIDE"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WR_INC_OVERRIDE"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RD_INC_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<AXIMC_M6_FN_MOD_AHBrs> {
        RD_INC_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - WR_INC_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<AXIMC_M6_FN_MOD_AHBrs> {
        WR_INC_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXIMC master 6 AHB conversion override functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m6_fn_mod_ahb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m6_fn_mod_ahb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_M6_FN_MOD_AHBrs;
impl crate::RegisterSpec for AXIMC_M6_FN_MOD_AHBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_m6_fn_mod_ahb::R`](R) reader structure"]
impl crate::Readable for AXIMC_M6_FN_MOD_AHBrs {}
#[doc = "`write(|w| ..)` method takes [`aximc_m6_fn_mod_ahb::W`](W) writer structure"]
impl crate::Writable for AXIMC_M6_FN_MOD_AHBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_M6_FN_MOD_AHB to value 0"]
impl crate::Resettable for AXIMC_M6_FN_MOD_AHBrs {
    const RESET_VALUE: u32 = 0;
}
