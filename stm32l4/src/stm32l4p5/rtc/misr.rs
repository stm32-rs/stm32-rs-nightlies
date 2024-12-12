///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Register `MISR` writer
pub type W = crate::W<MISRrs>;
///Field `ALRAMF` reader - Alarm A masked flag
pub type ALRAMF_R = crate::BitReader;
///Field `ALRAMF` writer - Alarm A masked flag
pub type ALRAMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRBMF` reader - Alarm B masked flag
pub type ALRBMF_R = crate::BitReader;
///Field `ALRBMF` writer - Alarm B masked flag
pub type ALRBMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUTMF` reader - Wakeup timer masked flag
pub type WUTMF_R = crate::BitReader;
///Field `WUTMF` writer - Wakeup timer masked flag
pub type WUTMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSMF` reader - Timestamp masked flag
pub type TSMF_R = crate::BitReader;
///Field `TSMF` writer - Timestamp masked flag
pub type TSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSOVMF` reader - Timestamp overflow masked flag
pub type TSOVMF_R = crate::BitReader;
///Field `TSOVMF` writer - Timestamp overflow masked flag
pub type TSOVMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITSMF` reader - Internal timestamp masked flag
pub type ITSMF_R = crate::BitReader;
///Field `ITSMF` writer - Internal timestamp masked flag
pub type ITSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSRUMF` reader - SSR underflow masked flag
pub type SSRUMF_R = crate::BitReader;
///Field `SSRUMF` writer - SSR underflow masked flag
pub type SSRUMF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm A masked flag
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B masked flag
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer masked flag
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp masked flag
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow masked flag
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp masked flag
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow masked flag
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("alramf", &self.alramf())
            .field("alrbmf", &self.alrbmf())
            .field("wutmf", &self.wutmf())
            .field("tsmf", &self.tsmf())
            .field("tsovmf", &self.tsovmf())
            .field("itsmf", &self.itsmf())
            .field("ssrumf", &self.ssrumf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Alarm A masked flag
    #[inline(always)]
    pub fn alramf(&mut self) -> ALRAMF_W<MISRrs> {
        ALRAMF_W::new(self, 0)
    }
    ///Bit 1 - Alarm B masked flag
    #[inline(always)]
    pub fn alrbmf(&mut self) -> ALRBMF_W<MISRrs> {
        ALRBMF_W::new(self, 1)
    }
    ///Bit 2 - Wakeup timer masked flag
    #[inline(always)]
    pub fn wutmf(&mut self) -> WUTMF_W<MISRrs> {
        WUTMF_W::new(self, 2)
    }
    ///Bit 3 - Timestamp masked flag
    #[inline(always)]
    pub fn tsmf(&mut self) -> TSMF_W<MISRrs> {
        TSMF_W::new(self, 3)
    }
    ///Bit 4 - Timestamp overflow masked flag
    #[inline(always)]
    pub fn tsovmf(&mut self) -> TSOVMF_W<MISRrs> {
        TSOVMF_W::new(self, 4)
    }
    ///Bit 5 - Internal timestamp masked flag
    #[inline(always)]
    pub fn itsmf(&mut self) -> ITSMF_W<MISRrs> {
        ITSMF_W::new(self, 5)
    }
    ///Bit 6 - SSR underflow masked flag
    #[inline(always)]
    pub fn ssrumf(&mut self) -> SSRUMF_W<MISRrs> {
        SSRUMF_W::new(self, 6)
    }
}
/**RTC masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RTC:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`write(|w| ..)` method takes [`misr::W`](W) writer structure
impl crate::Writable for MISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
