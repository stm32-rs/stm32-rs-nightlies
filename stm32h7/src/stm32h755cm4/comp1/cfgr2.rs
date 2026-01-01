///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `EN` reader - COMP channel 1 enable bit
pub type EN_R = crate::BitReader;
///Field `EN` writer - COMP channel 1 enable bit
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRGEN` reader - Scaler bridge enable
pub type BRGEN_R = crate::BitReader;
///Field `BRGEN` writer - Scaler bridge enable
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALEN` reader - Voltage scaler enable bit
pub type SCALEN_R = crate::BitReader;
///Field `SCALEN` writer - Voltage scaler enable bit
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POLARITY` reader - COMP channel 1 polarity selection bit
pub type POLARITY_R = crate::BitReader;
///Field `POLARITY` writer - COMP channel 1 polarity selection bit
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WINMODE` reader - Window comparator mode selection bit
pub type WINMODE_R = crate::BitReader;
///Field `WINMODE` writer - Window comparator mode selection bit
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITEN` reader - COMP channel 1 interrupt enable
pub type ITEN_R = crate::BitReader;
///Field `ITEN` writer - COMP channel 1 interrupt enable
pub type ITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HYST` reader - COMP channel 1 hysteresis selection bits
pub type HYST_R = crate::FieldReader;
///Field `HYST` writer - COMP channel 1 hysteresis selection bits
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PWRMODE` reader - Power Mode of the COMP channel 1
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - Power Mode of the COMP channel 1
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INMSEL` reader - COMP channel 1 inverting input selection field
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - COMP channel 1 inverting input selection field
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INPSEL` reader - COMP channel 1 non-inverting input selection bit
pub type INPSEL_R = crate::BitReader;
///Field `INPSEL` writer - COMP channel 1 non-inverting input selection bit
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLANKING` reader - COMP channel 1 blanking source selection bits
pub type BLANKING_R = crate::FieldReader;
///Field `BLANKING` writer - COMP channel 1 blanking source selection bits
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOCK` reader - Lock bit
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - COMP channel 1 polarity selection bit
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Window comparator mode selection bit
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - COMP channel 1 interrupt enable
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - COMP channel 1 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Power Mode of the COMP channel 1
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - COMP channel 1 inverting input selection field
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - COMP channel 1 non-inverting input selection bit
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:27 - COMP channel 1 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - Lock bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("en", &self.en())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("polarity", &self.polarity())
            .field("winmode", &self.winmode())
            .field("iten", &self.iten())
            .field("hyst", &self.hyst())
            .field("pwrmode", &self.pwrmode())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("blanking", &self.blanking())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CFGR2rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, CFGR2rs> {
        BRGEN_W::new(self, 1)
    }
    ///Bit 2 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, CFGR2rs> {
        SCALEN_W::new(self, 2)
    }
    ///Bit 3 - COMP channel 1 polarity selection bit
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<'_, CFGR2rs> {
        POLARITY_W::new(self, 3)
    }
    ///Bit 4 - Window comparator mode selection bit
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<'_, CFGR2rs> {
        WINMODE_W::new(self, 4)
    }
    ///Bit 6 - COMP channel 1 interrupt enable
    #[inline(always)]
    pub fn iten(&mut self) -> ITEN_W<'_, CFGR2rs> {
        ITEN_W::new(self, 6)
    }
    ///Bits 8:9 - COMP channel 1 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, CFGR2rs> {
        HYST_W::new(self, 8)
    }
    ///Bits 12:13 - Power Mode of the COMP channel 1
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<'_, CFGR2rs> {
        PWRMODE_W::new(self, 12)
    }
    ///Bits 16:18 - COMP channel 1 inverting input selection field
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, CFGR2rs> {
        INMSEL_W::new(self, 16)
    }
    ///Bit 20 - COMP channel 1 non-inverting input selection bit
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<'_, CFGR2rs> {
        INPSEL_W::new(self, 20)
    }
    ///Bits 24:27 - COMP channel 1 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<'_, CFGR2rs> {
        BLANKING_W::new(self, 24)
    }
    ///Bit 31 - Lock bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CFGR2rs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#COMP1:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
