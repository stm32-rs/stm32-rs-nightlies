///Register `PLL2SSCGR` reader
pub type R = crate::R<PLL2SSCGRrs>;
///Register `PLL2SSCGR` writer
pub type W = crate::W<PLL2SSCGRrs>;
///Field `MOD_PER` reader - Modulation Period Adjustment for PLL2 Set and reset by software to adjust the modulation period of the clock spreading generator.
pub type MOD_PER_R = crate::FieldReader<u16>;
///Field `MOD_PER` writer - Modulation Period Adjustment for PLL2 Set and reset by software to adjust the modulation period of the clock spreading generator.
pub type MOD_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `TPDFN_DIS2` reader - Dithering TPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function.
pub type TPDFN_DIS2_R = crate::BitReader;
///Field `TPDFN_DIS2` writer - Dithering TPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function.
pub type TPDFN_DIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPDFN_DIS2` reader - Dithering RPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function.
pub type RPDFN_DIS2_R = crate::BitReader;
///Field `RPDFN_DIS2` writer - Dithering RPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function.
pub type RPDFN_DIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DWNSPREAD2` reader - Spread spectrum clock generator mode for PLL2 Set and reset by software to select the clock spreading mode.
pub type DWNSPREAD2_R = crate::BitReader;
///Field `DWNSPREAD2` writer - Spread spectrum clock generator mode for PLL2 Set and reset by software to select the clock spreading mode.
pub type DWNSPREAD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INC_STEP` reader - Modulation Depth Adjustment for PLL2 Set and reset by software to adjust the modulation depth of the clock spreading generator.
pub type INC_STEP_R = crate::FieldReader<u16>;
///Field `INC_STEP` writer - Modulation Depth Adjustment for PLL2 Set and reset by software to adjust the modulation depth of the clock spreading generator.
pub type INC_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:12 - Modulation Period Adjustment for PLL2 Set and reset by software to adjust the modulation period of the clock spreading generator.
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bit 13 - Dithering TPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function.
    #[inline(always)]
    pub fn tpdfn_dis2(&self) -> TPDFN_DIS2_R {
        TPDFN_DIS2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Dithering RPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function.
    #[inline(always)]
    pub fn rpdfn_dis2(&self) -> RPDFN_DIS2_R {
        RPDFN_DIS2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Spread spectrum clock generator mode for PLL2 Set and reset by software to select the clock spreading mode.
    #[inline(always)]
    pub fn dwnspread2(&self) -> DWNSPREAD2_R {
        DWNSPREAD2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:30 - Modulation Depth Adjustment for PLL2 Set and reset by software to adjust the modulation depth of the clock spreading generator.
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2SSCGR")
            .field("mod_per", &self.mod_per())
            .field("tpdfn_dis2", &self.tpdfn_dis2())
            .field("rpdfn_dis2", &self.rpdfn_dis2())
            .field("dwnspread2", &self.dwnspread2())
            .field("inc_step", &self.inc_step())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Modulation Period Adjustment for PLL2 Set and reset by software to adjust the modulation period of the clock spreading generator.
    #[inline(always)]
    #[must_use]
    pub fn mod_per(&mut self) -> MOD_PER_W<PLL2SSCGRrs> {
        MOD_PER_W::new(self, 0)
    }
    ///Bit 13 - Dithering TPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a triangular probability density function.
    #[inline(always)]
    #[must_use]
    pub fn tpdfn_dis2(&mut self) -> TPDFN_DIS2_W<PLL2SSCGRrs> {
        TPDFN_DIS2_W::new(self, 13)
    }
    ///Bit 14 - Dithering RPDF noise control for PLL2 Set and reset by software. This bit is used to enable or disable the injection of a dithering noise into the SSCG modulator. This dithering noise is generated using a rectangular probability density function.
    #[inline(always)]
    #[must_use]
    pub fn rpdfn_dis2(&mut self) -> RPDFN_DIS2_W<PLL2SSCGRrs> {
        RPDFN_DIS2_W::new(self, 14)
    }
    ///Bit 15 - Spread spectrum clock generator mode for PLL2 Set and reset by software to select the clock spreading mode.
    #[inline(always)]
    #[must_use]
    pub fn dwnspread2(&mut self) -> DWNSPREAD2_W<PLL2SSCGRrs> {
        DWNSPREAD2_W::new(self, 15)
    }
    ///Bits 16:30 - Modulation Depth Adjustment for PLL2 Set and reset by software to adjust the modulation depth of the clock spreading generator.
    #[inline(always)]
    #[must_use]
    pub fn inc_step(&mut self) -> INC_STEP_W<PLL2SSCGRrs> {
        INC_STEP_W::new(self, 16)
    }
}
/**RCC PLL2 Spread Spectrum Clock Generator register

You can [`read`](crate::Reg::read) this register and get [`pll2sscgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2sscgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:PLL2SSCGR)*/
pub struct PLL2SSCGRrs;
impl crate::RegisterSpec for PLL2SSCGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll2sscgr::R`](R) reader structure
impl crate::Readable for PLL2SSCGRrs {}
///`write(|w| ..)` method takes [`pll2sscgr::W`](W) writer structure
impl crate::Writable for PLL2SSCGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL2SSCGR to value 0
impl crate::Resettable for PLL2SSCGRrs {
    const RESET_VALUE: u32 = 0;
}
