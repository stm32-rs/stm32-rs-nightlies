///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `ALRAWF` reader - Alarm A write flag
pub type ALRAWF_R = crate::BitReader;
///Field `ALRBWF` reader - Alarm B write flag
pub type ALRBWF_R = crate::BitReader;
///Field `WUTWF` reader - Wakeup timer write flag
pub type WUTWF_R = crate::BitReader;
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader;
///Field `SHPF` writer - Shift operation pending
pub type SHPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader;
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader;
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRAF` reader - Alarm A flag
pub type ALRAF_R = crate::BitReader;
///Field `ALRAF` writer - Alarm A flag
pub type ALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBF` reader - Alarm B flag
pub type ALRBF_R = crate::BitReader;
///Field `ALRBF` writer - Alarm B flag
pub type ALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTF` reader - Wakeup timer flag
pub type WUTF_R = crate::BitReader;
///Field `WUTF` writer - Wakeup timer flag
pub type WUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - Time-stamp flag
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - Time-stamp flag
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSOVF` reader - Time-stamp overflow flag
pub type TSOVF_R = crate::BitReader;
///Field `TSOVF` writer - Time-stamp overflow flag
pub type TSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1F` reader - Tamper detection flag
pub type TAMP1F_R = crate::BitReader;
///Field `TAMP1F` writer - Tamper detection flag
pub type TAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2F` reader - TAMPER2 detection flag
pub type TAMP2F_R = crate::BitReader;
///Field `TAMP2F` writer - TAMPER2 detection flag
pub type TAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A write flag
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B write flag
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Tamper detection flag
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TAMPER2 detection flag
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("alrawf", &self.alrawf())
            .field("alrbwf", &self.alrbwf())
            .field("wutwf", &self.wutwf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("recalpf", &self.recalpf())
            .finish()
    }
}
impl W {
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&mut self) -> SHPF_W<'_, ISRrs> {
        SHPF_W::new(self, 3)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ISRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ISRrs> {
        INIT_W::new(self, 7)
    }
    ///Bit 8 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRAF_W<'_, ISRrs> {
        ALRAF_W::new(self, 8)
    }
    ///Bit 9 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRBF_W<'_, ISRrs> {
        ALRBF_W::new(self, 9)
    }
    ///Bit 10 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W<'_, ISRrs> {
        WUTF_W::new(self, 10)
    }
    ///Bit 11 - Time-stamp flag
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, ISRrs> {
        TSF_W::new(self, 11)
    }
    ///Bit 12 - Time-stamp overflow flag
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W<'_, ISRrs> {
        TSOVF_W::new(self, 12)
    }
    ///Bit 13 - Tamper detection flag
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W<'_, ISRrs> {
        TAMP1F_W::new(self, 13)
    }
    ///Bit 14 - TAMPER2 detection flag
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W<'_, ISRrs> {
        TAMP2F_W::new(self, 14)
    }
}
/**initialization and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#RTC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0x07
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x07;
}
