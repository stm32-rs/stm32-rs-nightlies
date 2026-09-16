///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `HSESEL` reader - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested
pub type HSESEL_R = crate::BitReader;
///Field `HSESEL` writer - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested
pub type HSESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPHSI` reader - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested
pub type STOPHSI_R = crate::BitReader;
///Field `STOPHSI` writer - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested
pub type STOPHSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEL_STATUS` reader - Clock source selection Status 0: HSI clock source is selected 1: HSE clock source is selected Mirror the actual system clock source, depending on clock switching mechanism and limitations
pub type HSESEL_STATUS_R = crate::BitReader;
///Field `CLKSYSDIV` reader - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register
pub type CLKSYSDIV_R = crate::FieldReader;
///Field `CLKSYSDIV` writer - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register
pub type CLKSYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLKSYSDIV_STATUS` reader - system clock frequency selection status 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) 100: div16/div12 (HSI/HSE) (4M) 101: div32/div24 (HSI/HSE) (2M) 110: div64/div48 (HSI/HSE) (1M) Note: behavior depends on depending on CFGR.HSESEL and APB2ENR.MRSUBGEN register
pub type CLKSYSDIV_STATUS_R = crate::FieldReader;
///Field `SMPSDIV` reader - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz
pub type SMPSDIV_R = crate::BitReader;
///Field `SMPSDIV` writer - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz
pub type SMPSDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUCLKSEL` reader - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)
pub type LPUCLKSEL_R = crate::BitReader;
///Field `LPUCLKSEL` writer - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)
pub type LPUCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSLOWSEL` reader - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock
pub type CLKSLOWSEL_R = crate::FieldReader;
///Field `CLKSLOWSEL` writer - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock
pub type CLKSLOWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOBOOSTEN` reader - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled.
pub type IOBOOSTEN_R = crate::BitReader;
///Field `IOBOOSTEN` writer - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled.
pub type IOBOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCOEN` reader - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled.
pub type LCOEN_R = crate::BitReader;
///Field `LCOEN` writer - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled.
pub type LCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3I2SCLKSEL` reader - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\[2:0\] bit field).
pub type SPI3I2SCLKSEL_R = crate::FieldReader;
///Field `SPI3I2SCLKSEL` writer - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\[2:0\] bit field).
pub type SPI3I2SCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCOSEL` reader - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected
pub type LCOSEL_R = crate::FieldReader;
///Field `LCOSEL` writer - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected
pub type LCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCOSEL` reader - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CCOPRE` reader - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used
pub type CCOPRE_R = crate::FieldReader;
///Field `CCOPRE` writer - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used
pub type CCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 1 - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested
    #[inline(always)]
    pub fn hsesel(&self) -> HSESEL_R {
        HSESEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested
    #[inline(always)]
    pub fn stophsi(&self) -> STOPHSI_R {
        STOPHSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clock source selection Status 0: HSI clock source is selected 1: HSE clock source is selected Mirror the actual system clock source, depending on clock switching mechanism and limitations
    #[inline(always)]
    pub fn hsesel_status(&self) -> HSESEL_STATUS_R {
        HSESEL_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 5:7 - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register
    #[inline(always)]
    pub fn clksysdiv(&self) -> CLKSYSDIV_R {
        CLKSYSDIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - system clock frequency selection status 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) 100: div16/div12 (HSI/HSE) (4M) 101: div32/div24 (HSI/HSE) (2M) 110: div64/div48 (HSI/HSE) (1M) Note: behavior depends on depending on CFGR.HSESEL and APB2ENR.MRSUBGEN register
    #[inline(always)]
    pub fn clksysdiv_status(&self) -> CLKSYSDIV_STATUS_R {
        CLKSYSDIV_STATUS_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)
    #[inline(always)]
    pub fn lpuclksel(&self) -> LPUCLKSEL_R {
        LPUCLKSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock
    #[inline(always)]
    pub fn clkslowsel(&self) -> CLKSLOWSEL_R {
        CLKSLOWSEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled.
    #[inline(always)]
    pub fn ioboosten(&self) -> IOBOOSTEN_R {
        IOBOOSTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled.
    #[inline(always)]
    pub fn lcoen(&self) -> LCOEN_R {
        LCOEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 22:23 - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\[2:0\] bit field).
    #[inline(always)]
    pub fn spi3i2sclksel(&self) -> SPI3I2SCLKSEL_R {
        SPI3I2SCLKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected
    #[inline(always)]
    pub fn lcosel(&self) -> LCOSEL_R {
        LCOSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used
    #[inline(always)]
    pub fn ccopre(&self) -> CCOPRE_R {
        CCOPRE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("hsesel", &self.hsesel())
            .field("stophsi", &self.stophsi())
            .field("hsesel_status", &self.hsesel_status())
            .field("clksysdiv", &self.clksysdiv())
            .field("clksysdiv_status", &self.clksysdiv_status())
            .field("smpsdiv", &self.smpsdiv())
            .field("lpuclksel", &self.lpuclksel())
            .field("clkslowsel", &self.clkslowsel())
            .field("ioboosten", &self.ioboosten())
            .field("lcoen", &self.lcoen())
            .field("spi3i2sclksel", &self.spi3i2sclksel())
            .field("lcosel", &self.lcosel())
            .field("mcosel", &self.mcosel())
            .field("ccopre", &self.ccopre())
            .finish()
    }
}
impl W {
    ///Bit 1 - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested
    #[inline(always)]
    pub fn hsesel(&mut self) -> HSESEL_W<'_, CFGRrs> {
        HSESEL_W::new(self, 1)
    }
    ///Bit 2 - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested
    #[inline(always)]
    pub fn stophsi(&mut self) -> STOPHSI_W<'_, CFGRrs> {
        STOPHSI_W::new(self, 2)
    }
    ///Bits 5:7 - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register
    #[inline(always)]
    pub fn clksysdiv(&mut self) -> CLKSYSDIV_W<'_, CFGRrs> {
        CLKSYSDIV_W::new(self, 5)
    }
    ///Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W<'_, CFGRrs> {
        SMPSDIV_W::new(self, 12)
    }
    ///Bit 13 - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)
    #[inline(always)]
    pub fn lpuclksel(&mut self) -> LPUCLKSEL_W<'_, CFGRrs> {
        LPUCLKSEL_W::new(self, 13)
    }
    ///Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock
    #[inline(always)]
    pub fn clkslowsel(&mut self) -> CLKSLOWSEL_W<'_, CFGRrs> {
        CLKSLOWSEL_W::new(self, 15)
    }
    ///Bit 17 - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled.
    #[inline(always)]
    pub fn ioboosten(&mut self) -> IOBOOSTEN_W<'_, CFGRrs> {
        IOBOOSTEN_W::new(self, 17)
    }
    ///Bit 19 - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled.
    #[inline(always)]
    pub fn lcoen(&mut self) -> LCOEN_W<'_, CFGRrs> {
        LCOEN_W::new(self, 19)
    }
    ///Bits 22:23 - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\[2:0\] bit field).
    #[inline(always)]
    pub fn spi3i2sclksel(&mut self) -> SPI3I2SCLKSEL_W<'_, CFGRrs> {
        SPI3I2SCLKSEL_W::new(self, 22)
    }
    ///Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected
    #[inline(always)]
    pub fn lcosel(&mut self) -> LCOSEL_W<'_, CFGRrs> {
        LCOSEL_W::new(self, 24)
    }
    ///Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGRrs> {
        MCOSEL_W::new(self, 26)
    }
    ///Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used
    #[inline(always)]
    pub fn ccopre(&mut self) -> CCOPRE_W<'_, CFGRrs> {
        CCOPRE_W::new(self, 29)
    }
}
/**CFGR register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:CFGR)*/
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
///`reset()` method sets CFGR to value 0x0240
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x0240;
}
