#[doc = "Register `I2C_TIMEOUTR` reader"]
pub type R = crate::R<I2C_TIMEOUTRrs>;
#[doc = "Register `I2C_TIMEOUTR` writer"]
pub type W = crate::W<I2C_TIMEOUTRrs>;
#[doc = "Field `TIMEOUTA` reader - TIMEOUTA"]
pub type TIMEOUTA_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTA` writer - TIMEOUTA"]
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIDLE` reader - TIDLE"]
pub type TIDLE_R = crate::BitReader;
#[doc = "Field `TIDLE` writer - TIDLE"]
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOUTEN` reader - TIMOUTEN"]
pub type TIMOUTEN_R = crate::BitReader;
#[doc = "Field `TIMOUTEN` writer - TIMOUTEN"]
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTB` reader - TIMEOUTB"]
pub type TIMEOUTB_R = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTB` writer - TIMEOUTB"]
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TEXTEN` reader - TEXTEN"]
pub type TEXTEN_R = crate::BitReader;
#[doc = "Field `TEXTEN` writer - TEXTEN"]
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIMEOUTA"]
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<I2C_TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    #[doc = "Bit 12 - TIDLE"]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<I2C_TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    #[doc = "Bit 15 - TIMOUTEN"]
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<I2C_TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    #[doc = "Bits 16:27 - TIMEOUTB"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<I2C_TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    #[doc = "Bit 31 - TEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<I2C_TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timeoutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timeoutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_TIMEOUTRrs;
impl crate::RegisterSpec for I2C_TIMEOUTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timeoutr::R`](R) reader structure"]
impl crate::Readable for I2C_TIMEOUTRrs {}
#[doc = "`write(|w| ..)` method takes [`i2c_timeoutr::W`](W) writer structure"]
impl crate::Writable for I2C_TIMEOUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TIMEOUTR to value 0"]
impl crate::Resettable for I2C_TIMEOUTRrs {
    const RESET_VALUE: u32 = 0;
}
