#[doc = "Register `RCC_PLL3CFGR2` reader"]
pub type R = crate::R<RCC_PLL3CFGR2rs>;
#[doc = "Register `RCC_PLL3CFGR2` writer"]
pub type W = crate::W<RCC_PLL3CFGR2rs>;
#[doc = "Field `DIVP` reader - DIVP"]
pub type DIVP_R = crate::FieldReader;
#[doc = "Field `DIVP` writer - DIVP"]
pub type DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVQ` reader - DIVQ"]
pub type DIVQ_R = crate::FieldReader;
#[doc = "Field `DIVQ` writer - DIVQ"]
pub type DIVQ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVR` reader - DIVR"]
pub type DIVR_R = crate::FieldReader;
#[doc = "Field `DIVR` writer - DIVR"]
pub type DIVR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    #[must_use]
    pub fn divp(&mut self) -> DIVP_W<RCC_PLL3CFGR2rs> {
        DIVP_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    #[must_use]
    pub fn divq(&mut self) -> DIVQ_W<RCC_PLL3CFGR2rs> {
        DIVQ_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DIVR_W<RCC_PLL3CFGR2rs> {
        DIVR_W::new(self, 16)
    }
}
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_PLL3CFGR2rs;
impl crate::RegisterSpec for RCC_PLL3CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_pll3cfgr2::R`](R) reader structure"]
impl crate::Readable for RCC_PLL3CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`rcc_pll3cfgr2::W`](W) writer structure"]
impl crate::Writable for RCC_PLL3CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_PLL3CFGR2 to value 0x0001_0101"]
impl crate::Resettable for RCC_PLL3CFGR2rs {
    const RESET_VALUE: u32 = 0x0001_0101;
}
