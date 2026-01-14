///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `TS1_EN` reader - TS1_EN
pub type TS1_EN_R = crate::BitReader;
///Field `TS1_EN` writer - TS1_EN
pub type TS1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_START` reader - TS1_START
pub type TS1_START_R = crate::BitReader;
///Field `TS1_START` writer - TS1_START
pub type TS1_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_INTRIG_SEL` reader - TS1_INTRIG_SEL
pub type TS1_INTRIG_SEL_R = crate::FieldReader;
///Field `TS1_INTRIG_SEL` writer - TS1_INTRIG_SEL
pub type TS1_INTRIG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TS1_SMP_TIME` reader - TS1_SMP_TIME
pub type TS1_SMP_TIME_R = crate::FieldReader;
///Field `TS1_SMP_TIME` writer - TS1_SMP_TIME
pub type TS1_SMP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REFCLK_SEL` reader - REFCLK_SEL
pub type REFCLK_SEL_R = crate::BitReader;
///Field `REFCLK_SEL` writer - REFCLK_SEL
pub type REFCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Q_MEAS_opt` reader - Q_MEAS_opt
pub type Q_MEAS_OPT_R = crate::BitReader;
///Field `Q_MEAS_opt` writer - Q_MEAS_opt
pub type Q_MEAS_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSREF_CLK_DIV` reader - HSREF_CLK_DIV
pub type HSREF_CLK_DIV_R = crate::FieldReader;
///Field `HSREF_CLK_DIV` writer - HSREF_CLK_DIV
pub type HSREF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - TS1_EN
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - TS1_START
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - TS1_INTRIG_SEL
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - TS1_SMP_TIME
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - REFCLK_SEL
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Q_MEAS_opt
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:30 - HSREF_CLK_DIV
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("ts1_en", &self.ts1_en())
            .field("ts1_start", &self.ts1_start())
            .field("ts1_intrig_sel", &self.ts1_intrig_sel())
            .field("ts1_smp_time", &self.ts1_smp_time())
            .field("refclk_sel", &self.refclk_sel())
            .field("q_meas_opt", &self.q_meas_opt())
            .field("hsref_clk_div", &self.hsref_clk_div())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS1_EN
    #[inline(always)]
    pub fn ts1_en(&mut self) -> TS1_EN_W<'_, CFGR1rs> {
        TS1_EN_W::new(self, 0)
    }
    ///Bit 4 - TS1_START
    #[inline(always)]
    pub fn ts1_start(&mut self) -> TS1_START_W<'_, CFGR1rs> {
        TS1_START_W::new(self, 4)
    }
    ///Bits 8:11 - TS1_INTRIG_SEL
    #[inline(always)]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W<'_, CFGR1rs> {
        TS1_INTRIG_SEL_W::new(self, 8)
    }
    ///Bits 16:19 - TS1_SMP_TIME
    #[inline(always)]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W<'_, CFGR1rs> {
        TS1_SMP_TIME_W::new(self, 16)
    }
    ///Bit 20 - REFCLK_SEL
    #[inline(always)]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<'_, CFGR1rs> {
        REFCLK_SEL_W::new(self, 20)
    }
    ///Bit 21 - Q_MEAS_opt
    #[inline(always)]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W<'_, CFGR1rs> {
        Q_MEAS_OPT_W::new(self, 21)
    }
    ///Bits 24:30 - HSREF_CLK_DIV
    #[inline(always)]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W<'_, CFGR1rs> {
        HSREF_CLK_DIV_W::new(self, 24)
    }
}
/**DTS_CFGR1 is the configuration register for temperature sensor 1.

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DTS:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
