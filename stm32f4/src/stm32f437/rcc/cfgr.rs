///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SW0` reader - System clock switch
pub type SW0_R = crate::BitReader;
///Field `SW0` writer - System clock switch
pub type SW0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW1` reader - System clock switch
pub type SW1_R = crate::BitReader;
///Field `SW1` writer - System clock switch
pub type SW1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWS0` reader - System clock switch status
pub type SWS0_R = crate::BitReader;
///Field `SWS1` reader - System clock switch status
pub type SWS1_R = crate::BitReader;
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPRE1` reader - APB Low speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - APB Low speed prescaler (APB1)
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RTCPRE` reader - HSE division factor for RTC clock
pub type RTCPRE_R = crate::FieldReader;
///Field `RTCPRE` writer - HSE division factor for RTC clock
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `MCO1` reader - Microcontroller clock output 1
pub type MCO1_R = crate::FieldReader;
///Field `MCO1` writer - Microcontroller clock output 1
pub type MCO1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2SSRC` reader - I2S clock selection
pub type I2SSRC_R = crate::BitReader;
///Field `I2SSRC` writer - I2S clock selection
pub type I2SSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCO1PRE` reader - MCO1 prescaler
pub type MCO1PRE_R = crate::FieldReader;
///Field `MCO1PRE` writer - MCO1 prescaler
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO2PRE` reader - MCO2 prescaler
pub type MCO2PRE_R = crate::FieldReader;
///Field `MCO2PRE` writer - MCO2 prescaler
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO2` reader - Microcontroller clock output 2
pub type MCO2_R = crate::FieldReader;
///Field `MCO2` writer - Microcontroller clock output 2
pub type MCO2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - System clock switch
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - System clock switch
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - System clock switch status
    #[inline(always)]
    pub fn sws0(&self) -> SWS0_R {
        SWS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - System clock switch status
    #[inline(always)]
    pub fn sws1(&self) -> SWS1_R {
        SWS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 10:12 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 13:15 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:20 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:22 - Microcontroller clock output 1
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - I2S clock selection
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:26 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bits 30:31 - Microcontroller clock output 2
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("mco2", &self.mco2())
            .field("mco2pre", &self.mco2pre())
            .field("mco1pre", &self.mco1pre())
            .field("i2ssrc", &self.i2ssrc())
            .field("mco1", &self.mco1())
            .field("rtcpre", &self.rtcpre())
            .field("ppre2", &self.ppre2())
            .field("ppre1", &self.ppre1())
            .field("hpre", &self.hpre())
            .field("sws1", &self.sws1())
            .field("sws0", &self.sws0())
            .field("sw1", &self.sw1())
            .field("sw0", &self.sw0())
            .finish()
    }
}
impl W {
    ///Bit 0 - System clock switch
    #[inline(always)]
    pub fn sw0(&mut self) -> SW0_W<'_, CFGRrs> {
        SW0_W::new(self, 0)
    }
    ///Bit 1 - System clock switch
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W<'_, CFGRrs> {
        SW1_W::new(self, 1)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGRrs> {
        HPRE_W::new(self, 4)
    }
    ///Bits 10:12 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGRrs> {
        PPRE1_W::new(self, 10)
    }
    ///Bits 13:15 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGRrs> {
        PPRE2_W::new(self, 13)
    }
    ///Bits 16:20 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<'_, CFGRrs> {
        RTCPRE_W::new(self, 16)
    }
    ///Bits 21:22 - Microcontroller clock output 1
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W<'_, CFGRrs> {
        MCO1_W::new(self, 21)
    }
    ///Bit 23 - I2S clock selection
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<'_, CFGRrs> {
        I2SSRC_W::new(self, 23)
    }
    ///Bits 24:26 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<'_, CFGRrs> {
        MCO1PRE_W::new(self, 24)
    }
    ///Bits 27:29 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<'_, CFGRrs> {
        MCO2PRE_W::new(self, 27)
    }
    ///Bits 30:31 - Microcontroller clock output 2
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W<'_, CFGRrs> {
        MCO2_W::new(self, 30)
    }
}
/**clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
