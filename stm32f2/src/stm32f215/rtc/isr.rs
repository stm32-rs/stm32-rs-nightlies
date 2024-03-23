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
}
impl W {
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
