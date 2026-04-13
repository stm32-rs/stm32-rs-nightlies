///Register `TAFCR` reader
pub type R = crate::R<TAFCRrs>;
///Register `TAFCR` writer
pub type W = crate::W<TAFCRrs>;
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - Tamper 1 detection enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for tamper 1
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for tamper 1
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1INSEL` reader - TAMPER1 mapping
pub type TAMP1INSEL_R = crate::BitReader;
///Field `TAMP1INSEL` writer - TAMPER1 mapping
pub type TAMP1INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSINSEL` reader - TIMESTAMP mapping
pub type TSINSEL_R = crate::BitReader;
///Field `TSINSEL` writer - TIMESTAMP mapping
pub type TSINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALARMOUTTYPE` reader - AFO_ALARM output type
pub type ALARMOUTTYPE_R = crate::BitReader;
///Field `ALARMOUTTYPE` writer - AFO_ALARM output type
pub type ALARMOUTTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - TAMPER1 mapping
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIMESTAMP mapping
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AFO_ALARM output type
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAFCR")
            .field("alarmouttype", &self.alarmouttype())
            .field("tsinsel", &self.tsinsel())
            .field("tamp1insel", &self.tamp1insel())
            .field("tampie", &self.tampie())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp1e", &self.tamp1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, TAFCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, TAFCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, TAFCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 16 - TAMPER1 mapping
    #[inline(always)]
    pub fn tamp1insel(&mut self) -> TAMP1INSEL_W<'_, TAFCRrs> {
        TAMP1INSEL_W::new(self, 16)
    }
    ///Bit 17 - TIMESTAMP mapping
    #[inline(always)]
    pub fn tsinsel(&mut self) -> TSINSEL_W<'_, TAFCRrs> {
        TSINSEL_W::new(self, 17)
    }
    ///Bit 18 - AFO_ALARM output type
    #[inline(always)]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<'_, TAFCRrs> {
        ALARMOUTTYPE_W::new(self, 18)
    }
}
/**tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#RTC:TAFCR)*/
pub struct TAFCRrs;
impl crate::RegisterSpec for TAFCRrs {
    type Ux = u32;
}
///`read()` method returns [`tafcr::R`](R) reader structure
impl crate::Readable for TAFCRrs {}
///`write(|w| ..)` method takes [`tafcr::W`](W) writer structure
impl crate::Writable for TAFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TAFCR to value 0
impl crate::Resettable for TAFCRrs {}
