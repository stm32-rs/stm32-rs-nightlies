///Register `I2C_TIMING` reader
pub type R = crate::R<I2C_TIMINGrs>;
///Register `I2C_TIMING` writer
pub type W = crate::W<I2C_TIMINGrs>;
///Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
pub type SCLL_R = crate::FieldReader;
///Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
pub type SCLH_R = crate::FieldReader;
///Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SDADEL` reader - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
pub type SDADEL_R = crate::FieldReader;
///Field `SDADEL` writer - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCLDEL` reader - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
pub type SCLDEL_R = crate::FieldReader;
///Field `SCLDEL` writer - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRESC` reader - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TIMING")
            .field("scll", &self.scll())
            .field("sclh", &self.sclh())
            .field("sdadel", &self.sdadel())
            .field("scldel", &self.scldel())
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<'_, I2C_TIMINGrs> {
        SCLL_W::new(self, 0)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<'_, I2C_TIMINGrs> {
        SCLH_W::new(self, 8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge SDA edge in transmission mode. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W<'_, I2C_TIMINGrs> {
        SDADEL_W::new(self, 16)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge in transmission mode. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W<'_, I2C_TIMINGrs> {
        SCLDEL_W::new(self, 20)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters and for SCL high and low level counters tPRESC = (PRESC+1) x tI2CCLK
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, I2C_TIMINGrs> {
        PRESC_W::new(self, 28)
    }
}
/**I2C_TIMING register

You can [`read`](crate::Reg::read) this register and get [`i2c_timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C2:I2C_TIMING)*/
pub struct I2C_TIMINGrs;
impl crate::RegisterSpec for I2C_TIMINGrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_timing::R`](R) reader structure
impl crate::Readable for I2C_TIMINGrs {}
///`write(|w| ..)` method takes [`i2c_timing::W`](W) writer structure
impl crate::Writable for I2C_TIMINGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_TIMING to value 0
impl crate::Resettable for I2C_TIMINGrs {}
