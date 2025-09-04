///Register `TIMEOUTR` reader
pub type R = crate::R<TIMEOUTRrs>;
///Register `TIMEOUTR` writer
pub type W = crate::W<TIMEOUTRrs>;
///Field `TIMEOUTA` reader - Bus timeout A
pub type TIMEOUTA_R = crate::FieldReader<u16>;
///Field `TIMEOUTA` writer - Bus timeout A
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TIDLE` reader - Idle clock timeout detection
pub type TIDLE_R = crate::BitReader;
///Field `TIDLE` writer - Idle clock timeout detection
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMOUTEN` reader - Clock timeout enable
pub type TIMOUTEN_R = crate::BitReader;
///Field `TIMOUTEN` writer - Clock timeout enable
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEOUTB` reader - Bus timeout B
pub type TIMEOUTB_R = crate::FieldReader<u16>;
///Field `TIMEOUTB` writer - Bus timeout B
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TEXTEN` reader - Extended clock timeout enable
pub type TEXTEN_R = crate::BitReader;
///Field `TEXTEN` writer - Extended clock timeout enable
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUTR")
            .field("timeouta", &self.timeouta())
            .field("tidle", &self.tidle())
            .field("timouten", &self.timouten())
            .field("timeoutb", &self.timeoutb())
            .field("texten", &self.texten())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W<TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&mut self) -> TIMOUTEN_W<TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W<TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
/**Status register 1

You can [`read`](crate::Reg::read) this register and get [`timeoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#I2C1:TIMEOUTR)*/
pub struct TIMEOUTRrs;
impl crate::RegisterSpec for TIMEOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`timeoutr::R`](R) reader structure
impl crate::Readable for TIMEOUTRrs {}
///`write(|w| ..)` method takes [`timeoutr::W`](W) writer structure
impl crate::Writable for TIMEOUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMEOUTR to value 0
impl crate::Resettable for TIMEOUTRrs {}
