///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `ROVSE` reader - Regular oversampling enable
pub type ROVSE_R = crate::BitReader;
///Field `ROVSE` writer - Regular oversampling enable
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVSE` reader - Injected oversampling enable
pub type JOVSE_R = crate::BitReader;
///Field `JOVSE` writer - Injected oversampling enable
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSS` reader - Oversampling shift
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TROVS` reader - Triggered regular oversampling
pub type TROVS_R = crate::BitReader;
///Field `TROVS` writer - Triggered regular oversampling
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVSM` reader - Regular oversampling mode
pub type ROVSM_R = crate::BitReader;
///Field `ROVSM` writer - Regular oversampling mode
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BULB` reader - Bulb sampling mode
pub type BULB_R = crate::BitReader;
///Field `BULB` writer - Bulb sampling mode
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_R = crate::BitReader;
///Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPTRIG` reader - Sampling time control trigger mode
pub type SMPTRIG_R = crate::BitReader;
///Field `SMPTRIG` writer - Sampling time control trigger mode
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSR` reader - Oversampling ratio
pub type OSR_R = crate::FieldReader<u16>;
///Field `OSR` writer - Oversampling ratio
pub type OSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LSHIFT` reader - Left shift factor
pub type LSHIFT_R = crate::FieldReader;
///Field `LSHIFT` writer - Left shift factor
pub type LSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Regular oversampling enable
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected oversampling enable
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered regular oversampling
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular oversampling mode
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Bulb sampling mode
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Sampling time control trigger mode
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25 - Oversampling ratio
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 28:31 - Left shift factor
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("rovse", &self.rovse())
            .field("jovse", &self.jovse())
            .field("ovss", &self.ovss())
            .field("trovs", &self.trovs())
            .field("rovsm", &self.rovsm())
            .field("bulb", &self.bulb())
            .field("swtrig", &self.swtrig())
            .field("smptrig", &self.smptrig())
            .field("osr", &self.osr())
            .field("lshift", &self.lshift())
            .finish()
    }
}
impl W {
    ///Bit 0 - Regular oversampling enable
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W<'_, CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - Injected oversampling enable
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W<'_, CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered regular oversampling
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W<'_, CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - Regular oversampling mode
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W<'_, CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Bit 13 - Bulb sampling mode
    #[inline(always)]
    pub fn bulb(&mut self) -> BULB_W<'_, CFGR2rs> {
        BULB_W::new(self, 13)
    }
    ///Bit 14 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W<'_, CFGR2rs> {
        SWTRIG_W::new(self, 14)
    }
    ///Bit 15 - Sampling time control trigger mode
    #[inline(always)]
    pub fn smptrig(&mut self) -> SMPTRIG_W<'_, CFGR2rs> {
        SMPTRIG_W::new(self, 15)
    }
    ///Bits 16:25 - Oversampling ratio
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W<'_, CFGR2rs> {
        OSR_W::new(self, 16)
    }
    ///Bits 28:31 - Left shift factor
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W<'_, CFGR2rs> {
        LSHIFT_W::new(self, 28)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CFGR2)*/
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
