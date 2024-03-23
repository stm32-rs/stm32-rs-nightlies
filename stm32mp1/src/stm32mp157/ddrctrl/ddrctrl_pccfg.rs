#[doc = "Register `DDRCTRL_PCCFG` reader"]
pub type R = crate::R<DDRCTRL_PCCFGrs>;
#[doc = "Register `DDRCTRL_PCCFG` writer"]
pub type W = crate::W<DDRCTRL_PCCFGrs>;
#[doc = "Field `GO2CRITICAL_EN` reader - GO2CRITICAL_EN"]
pub type GO2CRITICAL_EN_R = crate::BitReader;
#[doc = "Field `GO2CRITICAL_EN` writer - GO2CRITICAL_EN"]
pub type GO2CRITICAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGEMATCH_LIMIT` reader - PAGEMATCH_LIMIT"]
pub type PAGEMATCH_LIMIT_R = crate::BitReader;
#[doc = "Field `PAGEMATCH_LIMIT` writer - PAGEMATCH_LIMIT"]
pub type PAGEMATCH_LIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL_EXP_MODE` reader - BL_EXP_MODE"]
pub type BL_EXP_MODE_R = crate::BitReader;
#[doc = "Field `BL_EXP_MODE` writer - BL_EXP_MODE"]
pub type BL_EXP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&self) -> GO2CRITICAL_EN_R {
        GO2CRITICAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&self) -> PAGEMATCH_LIMIT_R {
        PAGEMATCH_LIMIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&self) -> BL_EXP_MODE_R {
        BL_EXP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    #[must_use]
    pub fn go2critical_en(&mut self) -> GO2CRITICAL_EN_W<DDRCTRL_PCCFGrs> {
        GO2CRITICAL_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    #[must_use]
    pub fn pagematch_limit(&mut self) -> PAGEMATCH_LIMIT_W<DDRCTRL_PCCFGrs> {
        PAGEMATCH_LIMIT_W::new(self, 4)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn bl_exp_mode(&mut self) -> BL_EXP_MODE_W<DDRCTRL_PCCFGrs> {
        BL_EXP_MODE_W::new(self, 8)
    }
}
#[doc = "DDRCTRL port common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_PCCFGrs;
impl crate::RegisterSpec for DDRCTRL_PCCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_pccfg::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_PCCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_pccfg::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_PCCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCCFG to value 0"]
impl crate::Resettable for DDRCTRL_PCCFGrs {
    const RESET_VALUE: u32 = 0;
}
