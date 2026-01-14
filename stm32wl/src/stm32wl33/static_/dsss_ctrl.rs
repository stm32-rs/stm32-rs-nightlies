///Register `DSSS_CTRL` reader
pub type R = crate::R<DSSS_CTRLrs>;
///Register `DSSS_CTRL` writer
pub type W = crate::W<DSSS_CTRLrs>;
///Field `ACQ_WINDOW` reader - DSSS acquisition window
pub type ACQ_WINDOW_R = crate::FieldReader;
///Field `ACQ_WINDOW` writer - DSSS acquisition window
pub type ACQ_WINDOW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SPREADING_EXP` reader - DSSS spreading exponent
pub type SPREADING_EXP_R = crate::FieldReader;
///Field `SPREADING_EXP` writer - DSSS spreading exponent
pub type SPREADING_EXP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DSSS_EN` reader - DSSS mode enable
pub type DSSS_EN_R = crate::BitReader;
///Field `DSSS_EN` writer - DSSS mode enable
pub type DSSS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQ_HITS` reader - DSSS acquisition hits
pub type ACQ_HITS_R = crate::FieldReader;
///Field `ACQ_HITS` writer - DSSS acquisition hits
pub type ACQ_HITS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ACQ_THR` reader - DSSS acquisition threshold
pub type ACQ_THR_R = crate::FieldReader;
///Field `ACQ_THR` writer - DSSS acquisition threshold
pub type ACQ_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:3 - DSSS acquisition window
    #[inline(always)]
    pub fn acq_window(&self) -> ACQ_WINDOW_R {
        ACQ_WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - DSSS spreading exponent
    #[inline(always)]
    pub fn spreading_exp(&self) -> SPREADING_EXP_R {
        SPREADING_EXP_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - DSSS mode enable
    #[inline(always)]
    pub fn dsss_en(&self) -> DSSS_EN_R {
        DSSS_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - DSSS acquisition hits
    #[inline(always)]
    pub fn acq_hits(&self) -> ACQ_HITS_R {
        ACQ_HITS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:15 - DSSS acquisition threshold
    #[inline(always)]
    pub fn acq_thr(&self) -> ACQ_THR_R {
        ACQ_THR_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSSS_CTRL")
            .field("acq_window", &self.acq_window())
            .field("spreading_exp", &self.spreading_exp())
            .field("dsss_en", &self.dsss_en())
            .field("acq_hits", &self.acq_hits())
            .field("acq_thr", &self.acq_thr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DSSS acquisition window
    #[inline(always)]
    pub fn acq_window(&mut self) -> ACQ_WINDOW_W<'_, DSSS_CTRLrs> {
        ACQ_WINDOW_W::new(self, 0)
    }
    ///Bits 4:6 - DSSS spreading exponent
    #[inline(always)]
    pub fn spreading_exp(&mut self) -> SPREADING_EXP_W<'_, DSSS_CTRLrs> {
        SPREADING_EXP_W::new(self, 4)
    }
    ///Bit 7 - DSSS mode enable
    #[inline(always)]
    pub fn dsss_en(&mut self) -> DSSS_EN_W<'_, DSSS_CTRLrs> {
        DSSS_EN_W::new(self, 7)
    }
    ///Bits 8:9 - DSSS acquisition hits
    #[inline(always)]
    pub fn acq_hits(&mut self) -> ACQ_HITS_W<'_, DSSS_CTRLrs> {
        ACQ_HITS_W::new(self, 8)
    }
    ///Bits 10:15 - DSSS acquisition threshold
    #[inline(always)]
    pub fn acq_thr(&mut self) -> ACQ_THR_W<'_, DSSS_CTRLrs> {
        ACQ_THR_W::new(self, 10)
    }
}
/**DSSS_CTRL register

You can [`read`](crate::Reg::read) this register and get [`dsss_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsss_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:DSSS_CTRL)*/
pub struct DSSS_CTRLrs;
impl crate::RegisterSpec for DSSS_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dsss_ctrl::R`](R) reader structure
impl crate::Readable for DSSS_CTRLrs {}
///`write(|w| ..)` method takes [`dsss_ctrl::W`](W) writer structure
impl crate::Writable for DSSS_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DSSS_CTRL to value 0
impl crate::Resettable for DSSS_CTRLrs {}
