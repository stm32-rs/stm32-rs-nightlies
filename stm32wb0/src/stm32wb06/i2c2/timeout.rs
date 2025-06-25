///Register `TIMEOUT` reader
pub type R = crate::R<TIMEOUTrs>;
///Register `TIMEOUT` writer
pub type W = crate::W<TIMEOUTrs>;
///Field `TIMEOUTA` reader - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0.
pub type TIMEOUTA_R = crate::FieldReader<u16>;
///Field `TIMEOUTA` writer - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0.
pub type TIMEOUTA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TIDLE` reader - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0.
pub type TIDLE_R = crate::BitReader;
///Field `TIDLE` writer - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0.
pub type TIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEOUTEN` reader - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1).
pub type TIMEOUTEN_R = crate::BitReader;
///Field `TIMEOUTEN` writer - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1).
pub type TIMEOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0.
pub type TIMEOUTB_R = crate::FieldReader<u16>;
///Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0.
pub type TIMEOUTB_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `TEXTEN` reader - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1).
pub type TEXTEN_R = crate::BitReader;
///Field `TEXTEN` writer - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1).
pub type TEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0.
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0.
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1).
    #[inline(always)]
    pub fn timeouten(&self) -> TIMEOUTEN_R {
        TIMEOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0.
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1).
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("timeouta", &self.timeouta())
            .field("tidle", &self.tidle())
            .field("timeouten", &self.timeouten())
            .field("timeoutb", &self.timeoutb())
            .field("texten", &self.texten())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0.
    #[inline(always)]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<TIMEOUTrs> {
        TIMEOUTA_W::new(self, 0)
    }
    ///Bit 12 - Idle clock timeout detection - 0: TIMEOUTA is used to detect SCL low timeout - 1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition) Note: This bit can be written only when TIMOUTEN=0.
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W<TIMEOUTrs> {
        TIDLE_W::new(self, 12)
    }
    ///Bit 15 - Clock timeout enable - 0: SCL timeout detection is disabled - 1: SCL timeout detection is enabled: when SCL is low for more than tTIMEOUT (TIDLE=0) or high for more than tIDLE (TIDLE=1), a timeout error is detected (TIMEOUT=1).
    #[inline(always)]
    pub fn timeouten(&mut self) -> TIMEOUTEN_W<TIMEOUTrs> {
        TIMEOUTEN_W::new(self, 15)
    }
    ///Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0.
    #[inline(always)]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<TIMEOUTrs> {
        TIMEOUTB_W::new(self, 16)
    }
    ///Bit 31 - Extended clock timeout enable - 0: Extended clock timeout detection is disabled - 1: Extended clock timeout detection is enabled. When a cumulative SCL stretch for more than tLOW:EXT is done by the I2C interface, a timeout error is detected (TIMEOUT=1).
    #[inline(always)]
    pub fn texten(&mut self) -> TEXTEN_W<TIMEOUTrs> {
        TEXTEN_W::new(self, 31)
    }
}
/**I2C2_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#I2C2:TIMEOUT)*/
pub struct TIMEOUTrs;
impl crate::RegisterSpec for TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`timeout::R`](R) reader structure
impl crate::Readable for TIMEOUTrs {}
///`write(|w| ..)` method takes [`timeout::W`](W) writer structure
impl crate::Writable for TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMEOUT to value 0
impl crate::Resettable for TIMEOUTrs {}
