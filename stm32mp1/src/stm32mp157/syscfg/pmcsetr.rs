///Register `PMCSETR` reader
pub type R = crate::R<PMCSETRrs>;
///Register `PMCSETR` writer
pub type W = crate::W<PMCSETRrs>;
///Field `I2C1_FMP` reader - I2C1_FMP
pub type I2C1_FMP_R = crate::BitReader;
///Field `I2C1_FMP` writer - I2C1_FMP
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2_FMP` reader - I2C2_FMP
pub type I2C2_FMP_R = crate::BitReader;
///Field `I2C2_FMP` writer - I2C2_FMP
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3_FMP` reader - I2C3_FMP
pub type I2C3_FMP_R = crate::BitReader;
///Field `I2C3_FMP` writer - I2C3_FMP
pub type I2C3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4_FMP` reader - I2C4_FMP
pub type I2C4_FMP_R = crate::BitReader;
///Field `I2C4_FMP` writer - I2C4_FMP
pub type I2C4_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C5_FMP` reader - I2C5_FMP
pub type I2C5_FMP_R = crate::BitReader;
///Field `I2C5_FMP` writer - I2C5_FMP
pub type I2C5_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6_FMP` reader - I2C6_FMP
pub type I2C6_FMP_R = crate::BitReader;
///Field `I2C6_FMP` writer - I2C6_FMP
pub type I2C6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_BOOSTER` reader - EN_BOOSTER
pub type EN_BOOSTER_R = crate::BitReader;
///Field `EN_BOOSTER` writer - EN_BOOSTER
pub type EN_BOOSTER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASWVDD` reader - ANASWVDD
pub type ANASWVDD_R = crate::BitReader;
///Field `ANASWVDD` writer - ANASWVDD
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_CLK_SEL` reader - ETH_CLK_SEL
pub type ETH_CLK_SEL_R = crate::BitReader;
///Field `ETH_CLK_SEL` writer - ETH_CLK_SEL
pub type ETH_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_REF_CLK_SEL` reader - ETH_REF_CLK_SEL
pub type ETH_REF_CLK_SEL_R = crate::BitReader;
///Field `ETH_REF_CLK_SEL` writer - ETH_REF_CLK_SEL
pub type ETH_REF_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_SELMII` reader - ETH_SELMII
pub type ETH_SELMII_R = crate::BitReader;
///Field `ETH_SELMII` writer - ETH_SELMII
pub type ETH_SELMII_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH_SEL` reader - ETH_SEL
pub type ETH_SEL_R = crate::FieldReader;
///Field `ETH_SEL` writer - ETH_SEL
pub type ETH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANA0_SEL` reader - ANA0_SEL
pub type ANA0_SEL_R = crate::BitReader;
///Field `ANA0_SEL` writer - ANA0_SEL
pub type ANA0_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANA1_SEL` reader - ANA1_SEL
pub type ANA1_SEL_R = crate::BitReader;
///Field `ANA1_SEL` writer - ANA1_SEL
pub type ANA1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2C1_FMP
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C2_FMP
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2C3_FMP
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C4_FMP
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I2C5_FMP
    #[inline(always)]
    pub fn i2c5_fmp(&self) -> I2C5_FMP_R {
        I2C5_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I2C6_FMP
    #[inline(always)]
    pub fn i2c6_fmp(&self) -> I2C6_FMP_R {
        I2C6_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - EN_BOOSTER
    #[inline(always)]
    pub fn en_booster(&self) -> EN_BOOSTER_R {
        EN_BOOSTER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ANASWVDD
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - ETH_CLK_SEL
    #[inline(always)]
    pub fn eth_clk_sel(&self) -> ETH_CLK_SEL_R {
        ETH_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ETH_REF_CLK_SEL
    #[inline(always)]
    pub fn eth_ref_clk_sel(&self) -> ETH_REF_CLK_SEL_R {
        ETH_REF_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - ETH_SELMII
    #[inline(always)]
    pub fn eth_selmii(&self) -> ETH_SELMII_R {
        ETH_SELMII_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:23 - ETH_SEL
    #[inline(always)]
    pub fn eth_sel(&self) -> ETH_SEL_R {
        ETH_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - ANA0_SEL
    #[inline(always)]
    pub fn ana0_sel(&self) -> ANA0_SEL_R {
        ANA0_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ANA1_SEL
    #[inline(always)]
    pub fn ana1_sel(&self) -> ANA1_SEL_R {
        ANA1_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCSETR")
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c3_fmp", &self.i2c3_fmp())
            .field("i2c4_fmp", &self.i2c4_fmp())
            .field("i2c5_fmp", &self.i2c5_fmp())
            .field("i2c6_fmp", &self.i2c6_fmp())
            .field("en_booster", &self.en_booster())
            .field("anaswvdd", &self.anaswvdd())
            .field("eth_clk_sel", &self.eth_clk_sel())
            .field("eth_ref_clk_sel", &self.eth_ref_clk_sel())
            .field("eth_selmii", &self.eth_selmii())
            .field("eth_sel", &self.eth_sel())
            .field("ana0_sel", &self.ana0_sel())
            .field("ana1_sel", &self.ana1_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C1_FMP
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, PMCSETRrs> {
        I2C1_FMP_W::new(self, 0)
    }
    ///Bit 1 - I2C2_FMP
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, PMCSETRrs> {
        I2C2_FMP_W::new(self, 1)
    }
    ///Bit 2 - I2C3_FMP
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<'_, PMCSETRrs> {
        I2C3_FMP_W::new(self, 2)
    }
    ///Bit 3 - I2C4_FMP
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<'_, PMCSETRrs> {
        I2C4_FMP_W::new(self, 3)
    }
    ///Bit 4 - I2C5_FMP
    #[inline(always)]
    pub fn i2c5_fmp(&mut self) -> I2C5_FMP_W<'_, PMCSETRrs> {
        I2C5_FMP_W::new(self, 4)
    }
    ///Bit 5 - I2C6_FMP
    #[inline(always)]
    pub fn i2c6_fmp(&mut self) -> I2C6_FMP_W<'_, PMCSETRrs> {
        I2C6_FMP_W::new(self, 5)
    }
    ///Bit 8 - EN_BOOSTER
    #[inline(always)]
    pub fn en_booster(&mut self) -> EN_BOOSTER_W<'_, PMCSETRrs> {
        EN_BOOSTER_W::new(self, 8)
    }
    ///Bit 9 - ANASWVDD
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<'_, PMCSETRrs> {
        ANASWVDD_W::new(self, 9)
    }
    ///Bit 16 - ETH_CLK_SEL
    #[inline(always)]
    pub fn eth_clk_sel(&mut self) -> ETH_CLK_SEL_W<'_, PMCSETRrs> {
        ETH_CLK_SEL_W::new(self, 16)
    }
    ///Bit 17 - ETH_REF_CLK_SEL
    #[inline(always)]
    pub fn eth_ref_clk_sel(&mut self) -> ETH_REF_CLK_SEL_W<'_, PMCSETRrs> {
        ETH_REF_CLK_SEL_W::new(self, 17)
    }
    ///Bit 20 - ETH_SELMII
    #[inline(always)]
    pub fn eth_selmii(&mut self) -> ETH_SELMII_W<'_, PMCSETRrs> {
        ETH_SELMII_W::new(self, 20)
    }
    ///Bits 21:23 - ETH_SEL
    #[inline(always)]
    pub fn eth_sel(&mut self) -> ETH_SEL_W<'_, PMCSETRrs> {
        ETH_SEL_W::new(self, 21)
    }
    ///Bit 24 - ANA0_SEL
    #[inline(always)]
    pub fn ana0_sel(&mut self) -> ANA0_SEL_W<'_, PMCSETRrs> {
        ANA0_SEL_W::new(self, 24)
    }
    ///Bit 25 - ANA1_SEL
    #[inline(always)]
    pub fn ana1_sel(&mut self) -> ANA1_SEL_W<'_, PMCSETRrs> {
        ANA1_SEL_W::new(self, 25)
    }
}
/**SYSCFG peripheral mode configuration set register

You can [`read`](crate::Reg::read) this register and get [`pmcsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:PMCSETR)*/
pub struct PMCSETRrs;
impl crate::RegisterSpec for PMCSETRrs {
    type Ux = u32;
}
///`read()` method returns [`pmcsetr::R`](R) reader structure
impl crate::Readable for PMCSETRrs {}
///`write(|w| ..)` method takes [`pmcsetr::W`](W) writer structure
impl crate::Writable for PMCSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCSETR to value 0
impl crate::Resettable for PMCSETRrs {}
