///Register `TSTR` reader
pub type R = crate::R<TSTRrs>;
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1TRG` reader - Active level for tamper 1
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader;
///Field `TAMP1INSEL` reader - TAMPER1 mapping
pub type TAMP1INSEL_R = crate::BitReader;
///Field `TSINSEL` reader - TIMESTAMP mapping
pub type TSINSEL_R = crate::BitReader;
///Field `ALARMOUTTYPE` reader - AFO_ALARM output type
pub type ALARMOUTTYPE_R = crate::BitReader;
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
        f.debug_struct("TSTR")
            .field("alarmouttype", &self.alarmouttype())
            .field("tsinsel", &self.tsinsel())
            .field("tamp1insel", &self.tamp1insel())
            .field("tampie", &self.tampie())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp1e", &self.tamp1e())
            .finish()
    }
}
/**time stamp time register

You can [`read`](crate::Reg::read) this register and get [`tstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RTC:TSTR)*/
pub struct TSTRrs;
impl crate::RegisterSpec for TSTRrs {
    type Ux = u32;
}
///`read()` method returns [`tstr::R`](R) reader structure
impl crate::Readable for TSTRrs {}
///`reset()` method sets TSTR to value 0
impl crate::Resettable for TSTRrs {}
