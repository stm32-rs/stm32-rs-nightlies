#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type ALRAWF_R = crate::BitReader;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub type ALRBWF_R = crate::BitReader;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader;
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader;
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type SHPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader;
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader;
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader;
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type ALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader;
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type ALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader;
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TSOVF_R = crate::BitReader;
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub type TAMP1F_R = crate::BitReader;
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub type TAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2F` reader - RTC_TAMP2 detection flag"]
pub type TAMP2F_R = crate::BitReader;
#[doc = "Field `TAMP2F` writer - RTC_TAMP2 detection flag"]
pub type TAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3F` reader - RTC_TAMP3 detection flag"]
pub type TAMP3F_R = crate::BitReader;
#[doc = "Field `TAMP3F` writer - RTC_TAMP3 detection flag"]
pub type TAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RECALPF_R = crate::BitReader;
#[doc = "Field `ITSF` reader - INTERNAL TIME-STAMP FLAG"]
pub type ITSF_R = crate::BitReader;
#[doc = "Field `ITSF` writer - INTERNAL TIME-STAMP FLAG"]
pub type ITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - INTERNAL TIME-STAMP FLAG"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    #[must_use]
    pub fn shpf(&mut self) -> SHPF_W<ISRrs> {
        SHPF_W::new(self, 3)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<ISRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<ISRrs> {
        INIT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<ISRrs> {
        ALRAF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbf(&mut self) -> ALRBF_W<ISRrs> {
        ALRBF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutf(&mut self) -> WUTF_W<ISRrs> {
        WUTF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<ISRrs> {
        TSF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<ISRrs> {
        TSOVF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1f(&mut self) -> TAMP1F_W<ISRrs> {
        TAMP1F_W::new(self, 13)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2f(&mut self) -> TAMP2F_W<ISRrs> {
        TAMP2F_W::new(self, 14)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3f(&mut self) -> TAMP3F_W<ISRrs> {
        TAMP3F_W::new(self, 15)
    }
    #[doc = "Bit 17 - INTERNAL TIME-STAMP FLAG"]
    #[inline(always)]
    #[must_use]
    pub fn itsf(&mut self) -> ITSF_W<ISRrs> {
        ITSF_W::new(self, 17)
    }
}
#[doc = "initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0x07"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x07;
}
