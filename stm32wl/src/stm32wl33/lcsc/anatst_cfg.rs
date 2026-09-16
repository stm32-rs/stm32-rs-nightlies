///Register `ANATST_CFG` reader
pub type R = crate::R<ANATST_CFGrs>;
///Register `ANATST_CFG` writer
pub type W = crate::W<ANATST_CFGrs>;
///Field `VCMBUFF_ENOUT_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC
pub type VCMBUFF_ENOUT_SEL_R = crate::BitReader;
///Field `VCMBUFF_ENOUT_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC
pub type VCMBUFF_ENOUT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMBUFF_ENOUT` reader - VCMBUFFER output buffer enable pin
pub type VCMBUFF_ENOUT_R = crate::BitReader;
///Field `VCMBUFF_ENOUT` writer - VCMBUFFER output buffer enable pin
pub type VCMBUFF_ENOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMBUFF_PWDN_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC
pub type VCMBUFF_PWDN_SEL_R = crate::BitReader;
///Field `VCMBUFF_PWDN_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC
pub type VCMBUFF_PWDN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMBUFF_PWDN` reader - VCMBUFF power-down pin
pub type VCMBUFF_PWDN_R = crate::BitReader;
///Field `VCMBUFF_PWDN` writer - VCMBUFF power-down pin
pub type VCMBUFF_PWDN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_PWDN_SEL` reader - Selection of the signal to be used to supply the COMP in the LCSC Analog part
pub type COMP_PWDN_SEL_R = crate::BitReader;
///Field `COMP_PWDN_SEL` writer - Selection of the signal to be used to supply the COMP in the LCSC Analog part
pub type COMP_PWDN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_PWDN` reader - COMP power-down pin
pub type COMP_PWDN_R = crate::BitReader;
///Field `COMP_PWDN` writer - COMP power-down pin
pub type COMP_PWDN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_PWDN_SEL` reader - Selection of the signal to be used to supply the DAC in the LCSC Analog part
pub type DAC_PWDN_SEL_R = crate::BitReader;
///Field `DAC_PWDN_SEL` writer - Selection of the signal to be used to supply the DAC in the LCSC Analog part
pub type DAC_PWDN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_PWDN` reader - DAC power-down pin
pub type DAC_PWDN_R = crate::BitReader;
///Field `DAC_PWDN` writer - DAC power-down pin
pub type DAC_PWDN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Selection of the signal to be used to supply the DAC in the LCSC
    #[inline(always)]
    pub fn vcmbuff_enout_sel(&self) -> VCMBUFF_ENOUT_SEL_R {
        VCMBUFF_ENOUT_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VCMBUFFER output buffer enable pin
    #[inline(always)]
    pub fn vcmbuff_enout(&self) -> VCMBUFF_ENOUT_R {
        VCMBUFF_ENOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Selection of the signal to be used to supply the DAC in the LCSC
    #[inline(always)]
    pub fn vcmbuff_pwdn_sel(&self) -> VCMBUFF_PWDN_SEL_R {
        VCMBUFF_PWDN_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VCMBUFF power-down pin
    #[inline(always)]
    pub fn vcmbuff_pwdn(&self) -> VCMBUFF_PWDN_R {
        VCMBUFF_PWDN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Selection of the signal to be used to supply the COMP in the LCSC Analog part
    #[inline(always)]
    pub fn comp_pwdn_sel(&self) -> COMP_PWDN_SEL_R {
        COMP_PWDN_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COMP power-down pin
    #[inline(always)]
    pub fn comp_pwdn(&self) -> COMP_PWDN_R {
        COMP_PWDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Selection of the signal to be used to supply the DAC in the LCSC Analog part
    #[inline(always)]
    pub fn dac_pwdn_sel(&self) -> DAC_PWDN_SEL_R {
        DAC_PWDN_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DAC power-down pin
    #[inline(always)]
    pub fn dac_pwdn(&self) -> DAC_PWDN_R {
        DAC_PWDN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANATST_CFG")
            .field("vcmbuff_enout_sel", &self.vcmbuff_enout_sel())
            .field("vcmbuff_enout", &self.vcmbuff_enout())
            .field("vcmbuff_pwdn_sel", &self.vcmbuff_pwdn_sel())
            .field("vcmbuff_pwdn", &self.vcmbuff_pwdn())
            .field("comp_pwdn_sel", &self.comp_pwdn_sel())
            .field("comp_pwdn", &self.comp_pwdn())
            .field("dac_pwdn_sel", &self.dac_pwdn_sel())
            .field("dac_pwdn", &self.dac_pwdn())
            .finish()
    }
}
impl W {
    ///Bit 0 - Selection of the signal to be used to supply the DAC in the LCSC
    #[inline(always)]
    pub fn vcmbuff_enout_sel(&mut self) -> VCMBUFF_ENOUT_SEL_W<'_, ANATST_CFGrs> {
        VCMBUFF_ENOUT_SEL_W::new(self, 0)
    }
    ///Bit 1 - VCMBUFFER output buffer enable pin
    #[inline(always)]
    pub fn vcmbuff_enout(&mut self) -> VCMBUFF_ENOUT_W<'_, ANATST_CFGrs> {
        VCMBUFF_ENOUT_W::new(self, 1)
    }
    ///Bit 2 - Selection of the signal to be used to supply the DAC in the LCSC
    #[inline(always)]
    pub fn vcmbuff_pwdn_sel(&mut self) -> VCMBUFF_PWDN_SEL_W<'_, ANATST_CFGrs> {
        VCMBUFF_PWDN_SEL_W::new(self, 2)
    }
    ///Bit 3 - VCMBUFF power-down pin
    #[inline(always)]
    pub fn vcmbuff_pwdn(&mut self) -> VCMBUFF_PWDN_W<'_, ANATST_CFGrs> {
        VCMBUFF_PWDN_W::new(self, 3)
    }
    ///Bit 4 - Selection of the signal to be used to supply the COMP in the LCSC Analog part
    #[inline(always)]
    pub fn comp_pwdn_sel(&mut self) -> COMP_PWDN_SEL_W<'_, ANATST_CFGrs> {
        COMP_PWDN_SEL_W::new(self, 4)
    }
    ///Bit 5 - COMP power-down pin
    #[inline(always)]
    pub fn comp_pwdn(&mut self) -> COMP_PWDN_W<'_, ANATST_CFGrs> {
        COMP_PWDN_W::new(self, 5)
    }
    ///Bit 6 - Selection of the signal to be used to supply the DAC in the LCSC Analog part
    #[inline(always)]
    pub fn dac_pwdn_sel(&mut self) -> DAC_PWDN_SEL_W<'_, ANATST_CFGrs> {
        DAC_PWDN_SEL_W::new(self, 6)
    }
    ///Bit 7 - DAC power-down pin
    #[inline(always)]
    pub fn dac_pwdn(&mut self) -> DAC_PWDN_W<'_, ANATST_CFGrs> {
        DAC_PWDN_W::new(self, 7)
    }
}
/**LCSC ANA Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`anatst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anatst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ANATST_CFG)*/
pub struct ANATST_CFGrs;
impl crate::RegisterSpec for ANATST_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`anatst_cfg::R`](R) reader structure
impl crate::Readable for ANATST_CFGrs {}
///`write(|w| ..)` method takes [`anatst_cfg::W`](W) writer structure
impl crate::Writable for ANATST_CFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ANATST_CFG to value 0
impl crate::Resettable for ANATST_CFGrs {}
