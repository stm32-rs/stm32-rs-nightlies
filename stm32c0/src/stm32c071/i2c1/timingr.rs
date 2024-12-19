///Register `TIMINGR` reader
pub type R = crate::R<TIMINGRrs>;
///Register `TIMINGR` writer
pub type W = crate::W<TIMINGRrs>;
///Field `SCLL` reader - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t<sub>SCLL </sub>= (SCLL + 1) x t<sub>PRESC</sub> Note: SCLL is also used to generate t<sub>BUF </sub>and t<sub>SU:STA </sub>timings.
pub type SCLL_R = crate::FieldReader;
///Field `SCLL` writer - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t<sub>SCLL </sub>= (SCLL + 1) x t<sub>PRESC</sub> Note: SCLL is also used to generate t<sub>BUF </sub>and t<sub>SU:STA </sub>timings.
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLH` reader - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t<sub>SCLH </sub>= (SCLH + 1) x t<sub>PRESC</sub> Note: SCLH is also used to generate t<sub>SU:STO </sub>and t<sub>HD:STA </sub>timing.
pub type SCLH_R = crate::FieldReader;
///Field `SCLH` writer - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t<sub>SCLH </sub>= (SCLH + 1) x t<sub>PRESC</sub> Note: SCLH is also used to generate t<sub>SU:STO </sub>and t<sub>HD:STA </sub>timing.
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SDADEL` reader - Data hold time This field is used to generate the delay t<sub>SDADEL </sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SDADEL</sub>. t<sub>SDADEL</sub>= SDADEL x t<sub>PRESC</sub> Note: SDADEL is used to generate t<sub>HD:DAT </sub>timing.
pub type SDADEL_R = crate::FieldReader;
///Field `SDADEL` writer - Data hold time This field is used to generate the delay t<sub>SDADEL </sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SDADEL</sub>. t<sub>SDADEL</sub>= SDADEL x t<sub>PRESC</sub> Note: SDADEL is used to generate t<sub>HD:DAT </sub>timing.
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCLDEL` reader - Data setup time This field is used to generate a delay t<sub>SCLDEL</sub> = (SCLDEL + 1) x t<sub>PRESC</sub> between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SCLDEL</sub>. Note: t<sub>SCLDEL</sub> is used to generate t<sub>SU:DAT </sub>timing.
pub type SCLDEL_R = crate::FieldReader;
///Field `SCLDEL` writer - Data setup time This field is used to generate a delay t<sub>SCLDEL</sub> = (SCLDEL + 1) x t<sub>PRESC</sub> between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SCLDEL</sub>. Note: t<sub>SCLDEL</sub> is used to generate t<sub>SU:DAT </sub>timing.
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRESC` reader - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t<sub>PRESC </sub>used for data setup and hold counters (refer to I2C timings), and for SCL high and low level counters (refer to I2C master initialization). t<sub>PRESC </sub>= (PRESC + 1) x t<sub>I2CCLK</sub>
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t<sub>PRESC </sub>used for data setup and hold counters (refer to I2C timings), and for SCL high and low level counters (refer to I2C master initialization). t<sub>PRESC </sub>= (PRESC + 1) x t<sub>I2CCLK</sub>
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t<sub>SCLL </sub>= (SCLL + 1) x t<sub>PRESC</sub> Note: SCLL is also used to generate t<sub>BUF </sub>and t<sub>SU:STA </sub>timings.
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t<sub>SCLH </sub>= (SCLH + 1) x t<sub>PRESC</sub> Note: SCLH is also used to generate t<sub>SU:STO </sub>and t<sub>HD:STA </sub>timing.
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay t<sub>SDADEL </sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SDADEL</sub>. t<sub>SDADEL</sub>= SDADEL x t<sub>PRESC</sub> Note: SDADEL is used to generate t<sub>HD:DAT </sub>timing.
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay t<sub>SCLDEL</sub> = (SCLDEL + 1) x t<sub>PRESC</sub> between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SCLDEL</sub>. Note: t<sub>SCLDEL</sub> is used to generate t<sub>SU:DAT </sub>timing.
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t<sub>PRESC </sub>used for data setup and hold counters (refer to I2C timings), and for SCL high and low level counters (refer to I2C master initialization). t<sub>PRESC </sub>= (PRESC + 1) x t<sub>I2CCLK</sub>
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
    ///Bits 0:7 - SCL low period (master mode) This field is used to generate the SCL low period in master mode. t<sub>SCLL </sub>= (SCLL + 1) x t<sub>PRESC</sub> Note: SCLL is also used to generate t<sub>BUF </sub>and t<sub>SU:STA </sub>timings.
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<TIMINGRrs> {
        SCLL_W::new(self, 0)
    }
    ///Bits 8:15 - SCL high period (master mode) This field is used to generate the SCL high period in master mode. t<sub>SCLH </sub>= (SCLH + 1) x t<sub>PRESC</sub> Note: SCLH is also used to generate t<sub>SU:STO </sub>and t<sub>HD:STA </sub>timing.
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<TIMINGRrs> {
        SCLH_W::new(self, 8)
    }
    ///Bits 16:19 - Data hold time This field is used to generate the delay t<sub>SDADEL </sub>between SCL falling edge and SDA edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SDADEL</sub>. t<sub>SDADEL</sub>= SDADEL x t<sub>PRESC</sub> Note: SDADEL is used to generate t<sub>HD:DAT </sub>timing.
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W<TIMINGRrs> {
        SDADEL_W::new(self, 16)
    }
    ///Bits 20:23 - Data setup time This field is used to generate a delay t<sub>SCLDEL</sub> = (SCLDEL + 1) x t<sub>PRESC</sub> between SDA edge and SCL rising edge. In master and in slave modes with NOSTRETCH = 0, the SCL line is stretched low during t<sub>SCLDEL</sub>. Note: t<sub>SCLDEL</sub> is used to generate t<sub>SU:DAT </sub>timing.
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W<TIMINGRrs> {
        SCLDEL_W::new(self, 20)
    }
    ///Bits 28:31 - Timing prescaler This field is used to prescale I2CCLK to generate the clock period t<sub>PRESC </sub>used for data setup and hold counters (refer to I2C timings), and for SCL high and low level counters (refer to I2C master initialization). t<sub>PRESC </sub>= (PRESC + 1) x t<sub>I2CCLK</sub>
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<TIMINGRrs> {
        PRESC_W::new(self, 28)
    }
}
/**I2C timing register

You can [`read`](crate::Reg::read) this register and get [`timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#I2C1:TIMINGR)*/
pub struct TIMINGRrs;
impl crate::RegisterSpec for TIMINGRrs {
    type Ux = u32;
}
///`read()` method returns [`timingr::R`](R) reader structure
impl crate::Readable for TIMINGRrs {}
///`write(|w| ..)` method takes [`timingr::W`](W) writer structure
impl crate::Writable for TIMINGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMINGR to value 0
impl crate::Resettable for TIMINGRrs {
    const RESET_VALUE: u32 = 0;
}
