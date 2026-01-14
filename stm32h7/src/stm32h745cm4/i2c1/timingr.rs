///Register `TIMINGR` reader
pub type R = crate::R<TIMINGRrs>;
///Register `TIMINGR` writer
pub type W = crate::W<TIMINGRrs>;
///Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
pub type SCLL_R = crate::FieldReader;
///Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tSCLL = (SCLL+1) x tPRESC Note: SCLL is also used to generate tBUF and tSU:STA timings.
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
pub type SCLH_R = crate::FieldReader;
///Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `SDADEL` reader - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
pub type SDADEL_R = crate::FieldReader;
///Field `SDADEL` writer - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `SCLDEL` reader - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
pub type SCLDEL_R = crate::FieldReader;
///Field `SCLDEL` writer - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `PRESC` reader - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
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
    ///Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMINGR")
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
    pub fn scll(&mut self) -> SCLL_W<'_, TIMINGRrs> {
        SCLL_W::new(self, 0)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tSCLH = (SCLH+1) x tPRESC Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<'_, TIMINGRrs> {
        SCLH_W::new(self, 8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay tSDADEL between SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSDADEL. tSDADEL= SDADEL x tPRESC Note: SDADEL is used to generate tHD:DAT timing.
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W<'_, TIMINGRrs> {
        SDADEL_W::new(self, 16)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tSCLDEL between SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tSCLDEL. tSCLDEL = (SCLDEL+1) x tPRESC Note: tSCLDEL is used to generate tSU:DAT timing.
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W<'_, TIMINGRrs> {
        SCLDEL_W::new(self, 20)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK in order to generate the clock period tPRESC used for data setup and hold counters (refer to I2C timings on page9) and for SCL high and low level counters (refer to I2C master initialization on page24). tPRESC = (PRESC+1) x tI2CCLK
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, TIMINGRrs> {
        PRESC_W::new(self, 28)
    }
}
/**Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#I2C1:TIMINGR)*/
pub struct TIMINGRrs;
impl crate::RegisterSpec for TIMINGRrs {
    type Ux = u32;
}
///`read()` method returns [`timingr::R`](R) reader structure
impl crate::Readable for TIMINGRrs {}
///`write(|w| ..)` method takes [`timingr::W`](W) writer structure
impl crate::Writable for TIMINGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMINGR to value 0
impl crate::Resettable for TIMINGRrs {}
