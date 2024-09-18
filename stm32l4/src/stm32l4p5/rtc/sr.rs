///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
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
///Field `TSF` reader - Timestamp flag
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - Timestamp flag
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSOVF` reader - Timestamp overflow flag
pub type TSOVF_R = crate::BitReader;
///Field `TSOVF` writer - Timestamp overflow flag
pub type TSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITSF` reader - Internal timestamp flag
pub type ITSF_R = crate::BitReader;
///Field `ITSF` writer - Internal timestamp flag
pub type ITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSRUF` reader - SSR underflow flag
pub type SSRUF_R = crate::BitReader;
///Field `SSRUF` writer - SSR underflow flag
pub type SSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm A flag
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B flag
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer flag
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp flag
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow flag
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp flag
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow flag
    #[inline(always)]
    pub fn ssruf(&self) -> SSRUF_R {
        SSRUF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("alraf", &self.alraf())
            .field("alrbf", &self.alrbf())
            .field("wutf", &self.wutf())
            .field("tsf", &self.tsf())
            .field("tsovf", &self.tsovf())
            .field("itsf", &self.itsf())
            .field("ssruf", &self.ssruf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alarm A flag
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<SRrs> {
        ALRAF_W::new(self, 0)
    }
    ///Bit 1 - Alarm B flag
    #[inline(always)]
    #[must_use]
    pub fn alrbf(&mut self) -> ALRBF_W<SRrs> {
        ALRBF_W::new(self, 1)
    }
    ///Bit 2 - Wakeup timer flag
    #[inline(always)]
    #[must_use]
    pub fn wutf(&mut self) -> WUTF_W<SRrs> {
        WUTF_W::new(self, 2)
    }
    ///Bit 3 - Timestamp flag
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<SRrs> {
        TSF_W::new(self, 3)
    }
    ///Bit 4 - Timestamp overflow flag
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<SRrs> {
        TSOVF_W::new(self, 4)
    }
    ///Bit 5 - Internal timestamp flag
    #[inline(always)]
    #[must_use]
    pub fn itsf(&mut self) -> ITSF_W<SRrs> {
        ITSF_W::new(self, 5)
    }
    ///Bit 6 - SSR underflow flag
    #[inline(always)]
    #[must_use]
    pub fn ssruf(&mut self) -> SSRUF_W<SRrs> {
        SSRUF_W::new(self, 6)
    }
}
/**RTC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RTC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
