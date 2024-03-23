#[doc = "Register `DTS_CFGR1` reader"]
pub type R = crate::R<DTS_CFGR1rs>;
#[doc = "Register `DTS_CFGR1` writer"]
pub type W = crate::W<DTS_CFGR1rs>;
#[doc = "Field `TS1_EN` reader - TS1_EN"]
pub type TS1_EN_R = crate::BitReader;
#[doc = "Field `TS1_EN` writer - TS1_EN"]
pub type TS1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_START` reader - TS1_START"]
pub type TS1_START_R = crate::BitReader;
#[doc = "Field `TS1_START` writer - TS1_START"]
pub type TS1_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_INTRIG_SEL` reader - TS1_INTRIG_SEL"]
pub type TS1_INTRIG_SEL_R = crate::FieldReader;
#[doc = "Field `TS1_INTRIG_SEL` writer - TS1_INTRIG_SEL"]
pub type TS1_INTRIG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS1_SMP_TIME` reader - TS1_SMP_TIME"]
pub type TS1_SMP_TIME_R = crate::FieldReader;
#[doc = "Field `TS1_SMP_TIME` writer - TS1_SMP_TIME"]
pub type TS1_SMP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REFCLK_SEL` reader - REFCLK_SEL"]
pub type REFCLK_SEL_R = crate::BitReader;
#[doc = "Field `REFCLK_SEL` writer - REFCLK_SEL"]
pub type REFCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_MEAS_opt` reader - Q_MEAS_opt"]
pub type Q_MEAS_OPT_R = crate::BitReader;
#[doc = "Field `Q_MEAS_opt` writer - Q_MEAS_opt"]
pub type Q_MEAS_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSREF_CLK_DIV` reader - HSREF_CLK_DIV"]
pub type HSREF_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `HSREF_CLK_DIV` writer - HSREF_CLK_DIV"]
pub type HSREF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en(&mut self) -> TS1_EN_W<DTS_CFGR1rs> {
        TS1_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_start(&mut self) -> TS1_START_W<DTS_CFGR1rs> {
        TS1_START_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W<DTS_CFGR1rs> {
        TS1_INTRIG_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W<DTS_CFGR1rs> {
        TS1_SMP_TIME_W::new(self, 16)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<DTS_CFGR1rs> {
        REFCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    #[must_use]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W<DTS_CFGR1rs> {
        Q_MEAS_OPT_W::new(self, 21)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W<DTS_CFGR1rs> {
        HSREF_CLK_DIV_W::new(self, 24)
    }
}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_CFGR1rs;
impl crate::RegisterSpec for DTS_CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_cfgr1::R`](R) reader structure"]
impl crate::Readable for DTS_CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`dts_cfgr1::W`](W) writer structure"]
impl crate::Writable for DTS_CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTS_CFGR1 to value 0"]
impl crate::Resettable for DTS_CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
