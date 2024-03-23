#[doc = "Register `RCC_PLL1CSGR` reader"]
pub type R = crate::R<RCC_PLL1CSGRrs>;
#[doc = "Register `RCC_PLL1CSGR` writer"]
pub type W = crate::W<RCC_PLL1CSGRrs>;
#[doc = "Field `MOD_PER` reader - MOD_PER"]
pub type MOD_PER_R = crate::FieldReader<u16>;
#[doc = "Field `MOD_PER` writer - MOD_PER"]
pub type MOD_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `TPDFN_DIS` reader - TPDFN_DIS"]
pub type TPDFN_DIS_R = crate::BitReader;
#[doc = "Field `TPDFN_DIS` writer - TPDFN_DIS"]
pub type TPDFN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPDFN_DIS` reader - RPDFN_DIS"]
pub type RPDFN_DIS_R = crate::BitReader;
#[doc = "Field `RPDFN_DIS` writer - RPDFN_DIS"]
pub type RPDFN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCG_MODE` reader - SSCG_MODE"]
pub type SSCG_MODE_R = crate::BitReader;
#[doc = "Field `SSCG_MODE` writer - SSCG_MODE"]
pub type SSCG_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INC_STEP` reader - INC_STEP"]
pub type INC_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `INC_STEP` writer - INC_STEP"]
pub type INC_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&self) -> TPDFN_DIS_R {
        TPDFN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&self) -> RPDFN_DIS_R {
        RPDFN_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    #[must_use]
    pub fn mod_per(&mut self) -> MOD_PER_W<RCC_PLL1CSGRrs> {
        MOD_PER_W::new(self, 0)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn tpdfn_dis(&mut self) -> TPDFN_DIS_W<RCC_PLL1CSGRrs> {
        TPDFN_DIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn rpdfn_dis(&mut self) -> RPDFN_DIS_W<RCC_PLL1CSGRrs> {
        RPDFN_DIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W<RCC_PLL1CSGRrs> {
        SSCG_MODE_W::new(self, 15)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    #[must_use]
    pub fn inc_step(&mut self) -> INC_STEP_W<RCC_PLL1CSGRrs> {
        INC_STEP_W::new(self, 16)
    }
}
#[doc = "This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1csgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1csgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL1CSGRrs;
impl crate::RegisterSpec for RCC_PLL1CSGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll1csgr::R`](R) reader structure"]
impl crate::Readable for RCC_PLL1CSGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll1csgr::W`](W) writer structure"]
impl crate::Writable for RCC_PLL1CSGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL1CSGR to value 0"]
impl crate::Resettable for RCC_PLL1CSGRrs {
    const RESET_VALUE: u32 = 0;
}
