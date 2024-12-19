///Register `I2C_TIMEOUTR` reader
pub type R = crate::R<I2C_TIMEOUTRrs>;
///Register `I2C_TIMEOUTR` writer
pub type W = crate::W<I2C_TIMEOUTRrs>;
///Field `TIMEOUTA` reader - Bus timeout A This field is used to configure: The SCL low timeout condition t<sub>TIMEOUT</sub> when TIDLE = 0 t<sub>TIMEOUT</sub>= (TIMEOUTA + 1) x 2048 x t<sub>I2CCLK</sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t<sub>IDLE</sub>= (TIMEOUTA + 1) x 4 x t<sub>I2CCLK</sub> Note: These bits can be written only when TIMOUTEN = 0.
pub type TIMEOUTA_R = crate::FieldReader<u16>;
///Field `TIMEOUTA` writer - Bus timeout A This field is used to configure: The SCL low timeout condition t<sub>TIMEOUT</sub> when TIDLE = 0 t<sub>TIMEOUT</sub>= (TIMEOUTA + 1) x 2048 x t<sub>I2CCLK</sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t<sub>IDLE</sub>= (TIMEOUTA + 1) x 4 x t<sub>I2CCLK</sub> Note: These bits can be written only when TIMOUTEN = 0.
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TIDLE` reader - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0.
pub type TIDLE_R = crate::BitReader;
///Field `TIDLE` writer - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0.
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMOUTEN` reader - Clock timeout enable
pub type TIMOUTEN_R = crate::BitReader;
///Field `TIMOUTEN` writer - Clock timeout enable
pub type TIMOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t<sub>LOW:MEXT</sub>) is detected In slave mode, the slave cumulative clock low extend time (t<sub>LOW:SEXT</sub>) is detected t<sub>LOW:EXT </sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t<sub>I2CCLK</sub> Note: These bits can be written only when TEXTEN = 0.
pub type TIMEOUTB_R = crate::FieldReader<u16>;
///Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t<sub>LOW:MEXT</sub>) is detected In slave mode, the slave cumulative clock low extend time (t<sub>LOW:SEXT</sub>) is detected t<sub>LOW:EXT </sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t<sub>I2CCLK</sub> Note: These bits can be written only when TEXTEN = 0.
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TEXTEN` reader - Extended clock timeout enable
pub type TEXTEN_R = crate::BitReader;
///Field `TEXTEN` writer - Extended clock timeout enable
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bus timeout A This field is used to configure: The SCL low timeout condition t<sub>TIMEOUT</sub> when TIDLE = 0 t<sub>TIMEOUT</sub>= (TIMEOUTA + 1) x 2048 x t<sub>I2CCLK</sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t<sub>IDLE</sub>= (TIMEOUTA + 1) x 4 x t<sub>I2CCLK</sub> Note: These bits can be written only when TIMOUTEN = 0.
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0.
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t<sub>LOW:MEXT</sub>) is detected In slave mode, the slave cumulative clock low extend time (t<sub>LOW:SEXT</sub>) is detected t<sub>LOW:EXT </sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t<sub>I2CCLK</sub> Note: These bits can be written only when TEXTEN = 0.
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
        f.debug_struct("I2C_TIMEOUTR")
            .field("timeouta", &self.timeouta())
            .field("tidle", &self.tidle())
            .field("timouten", &self.timouten())
            .field("timeoutb", &self.timeoutb())
            .field("texten", &self.texten())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A This field is used to configure: The SCL low timeout condition t<sub>TIMEOUT</sub> when TIDLE = 0 t<sub>TIMEOUT</sub>= (TIMEOUTA + 1) x 2048 x t<sub>I2CCLK</sub> The bus idle condition (both SCL and SDA high) when TIDLE = 1 t<sub>IDLE</sub>= (TIMEOUTA + 1) x 4 x t<sub>I2CCLK</sub> Note: These bits can be written only when TIMOUTEN = 0.
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<I2C_TIMEOUTRrs> {
        TIMEOUTA_W::new(self, 0)
    }
    ///Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN = 0.
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W<I2C_TIMEOUTRrs> {
        TIDLE_W::new(self, 12)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&mut self) -> TIMOUTEN_W<I2C_TIMEOUTRrs> {
        TIMOUTEN_W::new(self, 15)
    }
    ///Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (t<sub>LOW:MEXT</sub>) is detected In slave mode, the slave cumulative clock low extend time (t<sub>LOW:SEXT</sub>) is detected t<sub>LOW:EXT </sub>= (TIMEOUTB + TIDLE = 01) x 2048 x t<sub>I2CCLK</sub> Note: These bits can be written only when TEXTEN = 0.
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<I2C_TIMEOUTRrs> {
        TIMEOUTB_W::new(self, 16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W<I2C_TIMEOUTRrs> {
        TEXTEN_W::new(self, 31)
    }
}
/**I2C timeout register

You can [`read`](crate::Reg::read) this register and get [`i2c_timeoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timeoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#I2C1:I2C_TIMEOUTR)*/
pub struct I2C_TIMEOUTRrs;
impl crate::RegisterSpec for I2C_TIMEOUTRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_timeoutr::R`](R) reader structure
impl crate::Readable for I2C_TIMEOUTRrs {}
///`write(|w| ..)` method takes [`i2c_timeoutr::W`](W) writer structure
impl crate::Writable for I2C_TIMEOUTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C_TIMEOUTR to value 0
impl crate::Resettable for I2C_TIMEOUTRrs {
    const RESET_VALUE: u32 = 0;
}
