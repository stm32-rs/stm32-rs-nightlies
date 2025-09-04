///Register `TAFCR` reader
pub type R = crate::R<TAFCRrs>;
///Register `TAFCR` writer
pub type W = crate::W<TAFCRrs>;
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - Tamper 1 detection enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1ETRG` reader - Active level for tamper 1
pub type TAMP1ETRG_R = crate::BitReader;
///Field `TAMP1ETRG` writer - Active level for tamper 1
pub type TAMP1ETRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2E` reader - Tamper 2 detection enable
pub type TAMP2E_R = crate::BitReader;
///Field `TAMP2E` writer - Tamper 2 detection enable
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2TRG` reader - Active level for tamper 2
pub type TAMP2TRG_R = crate::BitReader;
///Field `TAMP2TRG` writer - Active level for tamper 2
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3E` reader - TIMESTAMP mapping
pub type TAMP3E_R = crate::BitReader;
///Field `TAMP3E` writer - TIMESTAMP mapping
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3TRG` reader - TAMPER1 mapping
pub type TAMP3TRG_R = crate::BitReader;
///Field `TAMP3TRG` writer - TAMPER1 mapping
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader;
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader;
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAMPFLT` reader - Tamper filter count
pub type TAMPFLT_R = crate::FieldReader;
///Field `TAMPFLT` writer - Tamper filter count
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPRCH` reader - Tamper precharge duration
pub type TAMPPRCH_R = crate::FieldReader;
///Field `TAMPPRCH` writer - Tamper precharge duration
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPUDIS` reader - TAMPER pull-up disable
pub type TAMPPUDIS_R = crate::BitReader;
///Field `TAMPPUDIS` writer - TAMPER pull-up disable
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn tamp1etrg(&self) -> TAMP1ETRG_R {
        TAMP1ETRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIMESTAMP mapping
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TAMPER1 mapping
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("tamppudis", &self.tamppudis())
            .field("tampprch", &self.tampprch())
            .field("tampflt", &self.tampflt())
            .field("tampfreq", &self.tampfreq())
            .field("tampts", &self.tampts())
            .field("tamp3trg", &self.tamp3trg())
            .field("tamp3e", &self.tamp3e())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp2e", &self.tamp2e())
            .field("tampie", &self.tampie())
            .field("tamp1etrg", &self.tamp1etrg())
            .field("tamp1e", &self.tamp1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<TAFCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1etrg(&mut self) -> TAMP1ETRG_W<TAFCRrs> {
        TAMP1ETRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<TAFCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<TAFCRrs> {
        TAMP2E_W::new(self, 3)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<TAFCRrs> {
        TAMP2TRG_W::new(self, 4)
    }
    ///Bit 5 - TIMESTAMP mapping
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<TAFCRrs> {
        TAMP3E_W::new(self, 5)
    }
    ///Bit 6 - TAMPER1 mapping
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<TAFCRrs> {
        TAMP3TRG_W::new(self, 6)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<TAFCRrs> {
        TAMPTS_W::new(self, 7)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<TAFCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<TAFCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<TAFCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<TAFCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    ///Bit 18 - AFO_ALARM output type
    #[inline(always)]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W<TAFCRrs> {
        ALARMOUTTYPE_W::new(self, 18)
    }
}
/**tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RTC:TAFCR)*/
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
