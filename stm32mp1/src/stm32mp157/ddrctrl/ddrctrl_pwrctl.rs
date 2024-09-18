///Register `DDRCTRL_PWRCTL` reader
pub type R = crate::R<DDRCTRL_PWRCTLrs>;
///Register `DDRCTRL_PWRCTL` writer
pub type W = crate::W<DDRCTRL_PWRCTLrs>;
///Field `SELFREF_EN` reader - SELFREF_EN
pub type SELFREF_EN_R = crate::BitReader;
///Field `SELFREF_EN` writer - SELFREF_EN
pub type SELFREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POWERDOWN_EN` reader - POWERDOWN_EN
pub type POWERDOWN_EN_R = crate::BitReader;
///Field `POWERDOWN_EN` writer - POWERDOWN_EN
pub type POWERDOWN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEEPPOWERDOWN_EN` reader - DEEPPOWERDOWN_EN
pub type DEEPPOWERDOWN_EN_R = crate::BitReader;
///Field `DEEPPOWERDOWN_EN` writer - DEEPPOWERDOWN_EN
pub type DEEPPOWERDOWN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_DFI_DRAM_CLK_DISABLE` reader - EN_DFI_DRAM_CLK_DISABLE
pub type EN_DFI_DRAM_CLK_DISABLE_R = crate::BitReader;
///Field `EN_DFI_DRAM_CLK_DISABLE` writer - EN_DFI_DRAM_CLK_DISABLE
pub type EN_DFI_DRAM_CLK_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SELFREF_SW` reader - SELFREF_SW
pub type SELFREF_SW_R = crate::BitReader;
///Field `SELFREF_SW` writer - SELFREF_SW
pub type SELFREF_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_CAM_DRAIN_SELFREF` reader - DIS_CAM_DRAIN_SELFREF
pub type DIS_CAM_DRAIN_SELFREF_R = crate::BitReader;
///Field `DIS_CAM_DRAIN_SELFREF` writer - DIS_CAM_DRAIN_SELFREF
pub type DIS_CAM_DRAIN_SELFREF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SELFREF_EN
    #[inline(always)]
    pub fn selfref_en(&self) -> SELFREF_EN_R {
        SELFREF_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - POWERDOWN_EN
    #[inline(always)]
    pub fn powerdown_en(&self) -> POWERDOWN_EN_R {
        POWERDOWN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DEEPPOWERDOWN_EN
    #[inline(always)]
    pub fn deeppowerdown_en(&self) -> DEEPPOWERDOWN_EN_R {
        DEEPPOWERDOWN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EN_DFI_DRAM_CLK_DISABLE
    #[inline(always)]
    pub fn en_dfi_dram_clk_disable(&self) -> EN_DFI_DRAM_CLK_DISABLE_R {
        EN_DFI_DRAM_CLK_DISABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SELFREF_SW
    #[inline(always)]
    pub fn selfref_sw(&self) -> SELFREF_SW_R {
        SELFREF_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - DIS_CAM_DRAIN_SELFREF
    #[inline(always)]
    pub fn dis_cam_drain_selfref(&self) -> DIS_CAM_DRAIN_SELFREF_R {
        DIS_CAM_DRAIN_SELFREF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_PWRCTL")
            .field("selfref_en", &self.selfref_en())
            .field("powerdown_en", &self.powerdown_en())
            .field("deeppowerdown_en", &self.deeppowerdown_en())
            .field("en_dfi_dram_clk_disable", &self.en_dfi_dram_clk_disable())
            .field("selfref_sw", &self.selfref_sw())
            .field("dis_cam_drain_selfref", &self.dis_cam_drain_selfref())
            .finish()
    }
}
impl W {
    ///Bit 0 - SELFREF_EN
    #[inline(always)]
    #[must_use]
    pub fn selfref_en(&mut self) -> SELFREF_EN_W<DDRCTRL_PWRCTLrs> {
        SELFREF_EN_W::new(self, 0)
    }
    ///Bit 1 - POWERDOWN_EN
    #[inline(always)]
    #[must_use]
    pub fn powerdown_en(&mut self) -> POWERDOWN_EN_W<DDRCTRL_PWRCTLrs> {
        POWERDOWN_EN_W::new(self, 1)
    }
    ///Bit 2 - DEEPPOWERDOWN_EN
    #[inline(always)]
    #[must_use]
    pub fn deeppowerdown_en(&mut self) -> DEEPPOWERDOWN_EN_W<DDRCTRL_PWRCTLrs> {
        DEEPPOWERDOWN_EN_W::new(self, 2)
    }
    ///Bit 3 - EN_DFI_DRAM_CLK_DISABLE
    #[inline(always)]
    #[must_use]
    pub fn en_dfi_dram_clk_disable(&mut self) -> EN_DFI_DRAM_CLK_DISABLE_W<DDRCTRL_PWRCTLrs> {
        EN_DFI_DRAM_CLK_DISABLE_W::new(self, 3)
    }
    ///Bit 5 - SELFREF_SW
    #[inline(always)]
    #[must_use]
    pub fn selfref_sw(&mut self) -> SELFREF_SW_W<DDRCTRL_PWRCTLrs> {
        SELFREF_SW_W::new(self, 5)
    }
    ///Bit 7 - DIS_CAM_DRAIN_SELFREF
    #[inline(always)]
    #[must_use]
    pub fn dis_cam_drain_selfref(&mut self) -> DIS_CAM_DRAIN_SELFREF_W<DDRCTRL_PWRCTLrs> {
        DIS_CAM_DRAIN_SELFREF_W::new(self, 7)
    }
}
/**DDRCTRL low power control register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_pwrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_pwrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_PWRCTL)*/
pub struct DDRCTRL_PWRCTLrs;
impl crate::RegisterSpec for DDRCTRL_PWRCTLrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_pwrctl::R`](R) reader structure
impl crate::Readable for DDRCTRL_PWRCTLrs {}
///`write(|w| ..)` method takes [`ddrctrl_pwrctl::W`](W) writer structure
impl crate::Writable for DDRCTRL_PWRCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_PWRCTL to value 0
impl crate::Resettable for DDRCTRL_PWRCTLrs {
    const RESET_VALUE: u32 = 0;
}
