///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular
pub type ROVSE_R = crate::BitReader;
///Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected
pub type JOVSE_R = crate::BitReader;
///Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSR` reader - ADC oversampling ratio
pub type OVSR_R = crate::FieldReader;
///Field `OVSR` writer - ADC oversampling ratio
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OVSS` reader - ADC oversampling shift
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - ADC oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TOVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
pub type TOVS_R = crate::BitReader;
///Field `TOVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVSM` reader - ADC oversampling mode managing interlaced conversions of ADC group regular and group injected
pub type ROVSM_R = crate::BitReader;
///Field `ROVSM` writer - ADC oversampling mode managing interlaced conversions of ADC group regular and group injected
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ADC oversampler enable on scope ADC group regular
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC oversampler enable on scope ADC group injected
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - ADC oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - ADC oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC oversampling mode managing interlaced conversions of ADC group regular and group injected
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("rovsm", &self.rovsm())
            .field("tovs", &self.tovs())
            .field("ovss", &self.ovss())
            .field("ovsr", &self.ovsr())
            .field("jovse", &self.jovse())
            .field("rovse", &self.rovse())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC oversampler enable on scope ADC group regular
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - ADC oversampler enable on scope ADC group injected
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 2:4 - ADC oversampling ratio
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - ADC oversampling shift
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 10 - ADC oversampling mode managing interlaced conversions of ADC group regular and group injected
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC1:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
