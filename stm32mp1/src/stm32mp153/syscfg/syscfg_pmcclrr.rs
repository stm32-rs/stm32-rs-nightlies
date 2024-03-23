#[doc = "Register `SYSCFG_PMCCLRR` reader"]
pub type R = crate::R<SYSCFG_PMCCLRRrs>;
#[doc = "Register `SYSCFG_PMCCLRR` writer"]
pub type W = crate::W<SYSCFG_PMCCLRRrs>;
#[doc = "Field `I2C1_FMP` reader - I2C1_FMP"]
pub type I2C1_FMP_R = crate::BitReader;
#[doc = "Field `I2C1_FMP` writer - I2C1_FMP"]
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FMP` reader - I2C2_FMP"]
pub type I2C2_FMP_R = crate::BitReader;
#[doc = "Field `I2C2_FMP` writer - I2C2_FMP"]
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP"]
pub type I2C3_FMP_R = crate::BitReader;
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP"]
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4_FMP` reader - I2C4_FMP"]
pub type I2C4_FMP_R = crate::BitReader;
#[doc = "Field `I2C4_FMP` writer - I2C4_FMP"]
pub type I2C4_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C5_FMP` reader - I2C5_FMP"]
pub type I2C5_FMP_R = crate::BitReader;
#[doc = "Field `I2C5_FMP` writer - I2C5_FMP"]
pub type I2C5_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C6_FMP` reader - I2C6_FMP"]
pub type I2C6_FMP_R = crate::BitReader;
#[doc = "Field `I2C6_FMP` writer - I2C6_FMP"]
pub type I2C6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_BOOSTER` reader - EN_BOOSTER"]
pub type EN_BOOSTER_R = crate::BitReader;
#[doc = "Field `EN_BOOSTER` writer - EN_BOOSTER"]
pub type EN_BOOSTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANASWVDD` reader - ANASWVDD"]
pub type ANASWVDD_R = crate::BitReader;
#[doc = "Field `ANASWVDD` writer - ANASWVDD"]
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_CLK_SEL` reader - ETH_CLK_SEL"]
pub type ETH_CLK_SEL_R = crate::BitReader;
#[doc = "Field `ETH_CLK_SEL` writer - ETH_CLK_SEL"]
pub type ETH_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_REF_CLK_SEL` reader - ETH_REF_CLK_SEL"]
pub type ETH_REF_CLK_SEL_R = crate::BitReader;
#[doc = "Field `ETH_REF_CLK_SEL` writer - ETH_REF_CLK_SEL"]
pub type ETH_REF_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_SELMII` reader - ETH_SELMII"]
pub type ETH_SELMII_R = crate::BitReader;
#[doc = "Field `ETH_SELMII` writer - ETH_SELMII"]
pub type ETH_SELMII_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH_SEL` reader - ETH_SEL"]
pub type ETH_SEL_R = crate::FieldReader;
#[doc = "Field `ETH_SEL` writer - ETH_SEL"]
pub type ETH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ANA0_SEL` reader - ANA0_SEL"]
pub type ANA0_SEL_R = crate::BitReader;
#[doc = "Field `ANA0_SEL` writer - ANA0_SEL"]
pub type ANA0_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA1_SEL` reader - ANA1_SEL"]
pub type ANA1_SEL_R = crate::BitReader;
#[doc = "Field `ANA1_SEL` writer - ANA1_SEL"]
pub type ANA1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    pub fn i2c5_fmp(&self) -> I2C5_FMP_R {
        I2C5_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    pub fn i2c6_fmp(&self) -> I2C6_FMP_R {
        I2C6_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    pub fn en_booster(&self) -> EN_BOOSTER_R {
        EN_BOOSTER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    pub fn eth_clk_sel(&self) -> ETH_CLK_SEL_R {
        ETH_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    pub fn eth_ref_clk_sel(&self) -> ETH_REF_CLK_SEL_R {
        ETH_REF_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    pub fn eth_selmii(&self) -> ETH_SELMII_R {
        ETH_SELMII_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    pub fn eth_sel(&self) -> ETH_SEL_R {
        ETH_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    pub fn ana0_sel(&self) -> ANA0_SEL_R {
        ANA0_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    pub fn ana1_sel(&self) -> ANA1_SEL_R {
        ANA1_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C1_FMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C2_FMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C3_FMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C4_FMP_W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_fmp(&mut self) -> I2C5_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C5_FMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_fmp(&mut self) -> I2C6_FMP_W<SYSCFG_PMCCLRRrs> {
        I2C6_FMP_W::new(self, 5)
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    #[must_use]
    pub fn en_booster(&mut self) -> EN_BOOSTER_W<SYSCFG_PMCCLRRrs> {
        EN_BOOSTER_W::new(self, 8)
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    #[must_use]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<SYSCFG_PMCCLRRrs> {
        ANASWVDD_W::new(self, 9)
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_clk_sel(&mut self) -> ETH_CLK_SEL_W<SYSCFG_PMCCLRRrs> {
        ETH_CLK_SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_ref_clk_sel(&mut self) -> ETH_REF_CLK_SEL_W<SYSCFG_PMCCLRRrs> {
        ETH_REF_CLK_SEL_W::new(self, 17)
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    #[must_use]
    pub fn eth_selmii(&mut self) -> ETH_SELMII_W<SYSCFG_PMCCLRRrs> {
        ETH_SELMII_W::new(self, 20)
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_sel(&mut self) -> ETH_SEL_W<SYSCFG_PMCCLRRrs> {
        ETH_SEL_W::new(self, 21)
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ana0_sel(&mut self) -> ANA0_SEL_W<SYSCFG_PMCCLRRrs> {
        ANA0_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ana1_sel(&mut self) -> ANA1_SEL_W<SYSCFG_PMCCLRRrs> {
        ANA1_SEL_W::new(self, 25)
    }
}
#[doc = "SYSCFG peripheral mode configuration clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_pmcclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_pmcclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_PMCCLRRrs;
impl crate::RegisterSpec for SYSCFG_PMCCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_pmcclrr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_PMCCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_pmcclrr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_PMCCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_PMCCLRR to value 0"]
impl crate::Resettable for SYSCFG_PMCCLRRrs {
    const RESET_VALUE: u32 = 0;
}
