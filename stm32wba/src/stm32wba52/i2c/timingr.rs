///Register `TIMINGR` reader
pub type R = crate::R<TIMINGRrs>;
///Register `TIMINGR` writer
pub type W = crate::W<TIMINGRrs>;
///Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tsubSCLL /sub= (SCLL+1) x tsubPRESC/sub Note: SCLL is also used to generate tsubBUF /suband tsubSU:STA /subtimings.
pub type SCLL_R = crate::FieldReader;
///Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tsubSCLL /sub= (SCLL+1) x tsubPRESC/sub Note: SCLL is also used to generate tsubBUF /suband tsubSU:STA /subtimings.
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tsubSCLH /sub= (SCLH+1) x tsubPRESC/sub Note: SCLH is also used to generate tsubSU:STO /suband tsubHD:STA /subtiming.
pub type SCLH_R = crate::FieldReader;
///Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tsubSCLH /sub= (SCLH+1) x tsubPRESC/sub Note: SCLH is also used to generate tsubSU:STO /suband tsubHD:STA /subtiming.
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `SDADEL` reader - Data hold time This field is used to generate the delay tsubSDADEL /subbetween SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSDADEL/sub. tsubSDADEL/sub= SDADEL x tsubPRESC/sub Note: SDADEL is used to generate tsubHD:DAT /subtiming.
pub type SDADEL_R = crate::FieldReader;
///Field `SDADEL` writer - Data hold time This field is used to generate the delay tsubSDADEL /subbetween SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSDADEL/sub. tsubSDADEL/sub= SDADEL x tsubPRESC/sub Note: SDADEL is used to generate tsubHD:DAT /subtiming.
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `SCLDEL` reader - Data setup time This field is used to generate a delay tsubSCLDEL /subbetween SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSCLDEL/sub. tsubSCLDEL /sub= (SCLDEL+1) x tsubPRESC/sub Note: tsubSCLDEL/sub is used to generate tsubSU:DAT /subtiming.
pub type SCLDEL_R = crate::FieldReader;
///Field `SCLDEL` writer - Data setup time This field is used to generate a delay tsubSCLDEL /subbetween SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSCLDEL/sub. tsubSCLDEL /sub= (SCLDEL+1) x tsubPRESC/sub Note: tsubSCLDEL/sub is used to generate tsubSU:DAT /subtiming.
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///Field `PRESC` reader - Timing prescaler This field is used to prescale i2c_ker_ck in order to generate the clock period tsubPRESC /subused for data setup and hold counters (refer to FMPI2C timings on page 1928) and for SCL high and low level counters (refer to FMPI2C master initialization on page 1951). tsubPRESC /sub= (PRESC+1) x tsubI2CCLK/sub
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Timing prescaler This field is used to prescale i2c_ker_ck in order to generate the clock period tsubPRESC /subused for data setup and hold counters (refer to FMPI2C timings on page 1928) and for SCL high and low level counters (refer to FMPI2C master initialization on page 1951). tsubPRESC /sub= (PRESC+1) x tsubI2CCLK/sub
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tsubSCLL /sub= (SCLL+1) x tsubPRESC/sub Note: SCLL is also used to generate tsubBUF /suband tsubSU:STA /subtimings.
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tsubSCLH /sub= (SCLH+1) x tsubPRESC/sub Note: SCLH is also used to generate tsubSU:STO /suband tsubHD:STA /subtiming.
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay tsubSDADEL /subbetween SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSDADEL/sub. tsubSDADEL/sub= SDADEL x tsubPRESC/sub Note: SDADEL is used to generate tsubHD:DAT /subtiming.
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tsubSCLDEL /subbetween SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSCLDEL/sub. tsubSCLDEL /sub= (SCLDEL+1) x tsubPRESC/sub Note: tsubSCLDEL/sub is used to generate tsubSU:DAT /subtiming.
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale i2c_ker_ck in order to generate the clock period tsubPRESC /subused for data setup and hold counters (refer to FMPI2C timings on page 1928) and for SCL high and low level counters (refer to FMPI2C master initialization on page 1951). tsubPRESC /sub= (PRESC+1) x tsubI2CCLK/sub
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
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. tsubSCLL /sub= (SCLL+1) x tsubPRESC/sub Note: SCLL is also used to generate tsubBUF /suband tsubSU:STA /subtimings.
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<'_, TIMINGRrs> {
        SCLL_W::new(self, 0)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. tsubSCLH /sub= (SCLH+1) x tsubPRESC/sub Note: SCLH is also used to generate tsubSU:STO /suband tsubHD:STA /subtiming.
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<'_, TIMINGRrs> {
        SCLH_W::new(self, 8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay tsubSDADEL /subbetween SCL falling edge and SDA edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSDADEL/sub. tsubSDADEL/sub= SDADEL x tsubPRESC/sub Note: SDADEL is used to generate tsubHD:DAT /subtiming.
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W<'_, TIMINGRrs> {
        SDADEL_W::new(self, 16)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay tsubSCLDEL /subbetween SDA edge and SCL rising edge. In master mode and in slave mode with NOSTRETCH = 0, the SCL line is stretched low during tsubSCLDEL/sub. tsubSCLDEL /sub= (SCLDEL+1) x tsubPRESC/sub Note: tsubSCLDEL/sub is used to generate tsubSU:DAT /subtiming.
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W<'_, TIMINGRrs> {
        SCLDEL_W::new(self, 20)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale i2c_ker_ck in order to generate the clock period tsubPRESC /subused for data setup and hold counters (refer to FMPI2C timings on page 1928) and for SCL high and low level counters (refer to FMPI2C master initialization on page 1951). tsubPRESC /sub= (PRESC+1) x tsubI2CCLK/sub
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, TIMINGRrs> {
        PRESC_W::new(self, 28)
    }
}
/**I2C timing register

You can [`read`](crate::Reg::read) this register and get [`timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#I2C:TIMINGR)*/
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
