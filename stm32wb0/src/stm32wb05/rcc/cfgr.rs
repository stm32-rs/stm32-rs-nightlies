///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SMPSINV` reader - bit to control inversion of the SMPS clock
pub type SMPSINV_R = crate::BitReader;
///Field `SMPSINV` writer - bit to control inversion of the SMPS clock
pub type SMPSINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEL` reader - Clock source selection request:
pub type HSESEL_R = crate::BitReader;
///Field `HSESEL` writer - Clock source selection request:
pub type HSESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPHSI` reader - Stop HSI clock source request
pub type STOPHSI_R = crate::BitReader;
///Field `STOPHSI` writer - Stop HSI clock source request
pub type STOPHSI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEL_STATUS` reader - Clock source selection Status
pub type HSESEL_STATUS_R = crate::BitReader;
///Field `CLKSYSDIV` reader - CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio.
pub type CLKSYSDIV_R = crate::FieldReader;
///Field `CLKSYSDIV` writer - CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio.
pub type CLKSYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLKSYSDIV_STATUS` reader - CLKSYSDIV_STATUS: system clock frequency status Set and cleared by hardware to indicate the actual system clock frequency. This register must be read to be sure that the new frequency, selected by CLKSYSDIV, has been applied. 000: system clock frequency is 64 MHz 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz 100: system clock frequency is 4 MHz 101: system clock frequency is 2 MHz 110: system clock frequency is 1 MHz 111: not used. The actual clock frequency switching can be delayed of up to 128 system clock cycles, depending on the RCC internal counter status at the moment the new CLKSYSDIV is applied
pub type CLKSYSDIV_STATUS_R = crate::FieldReader;
///Field `SMPSDIV` reader - SMPS clock prescaling factor to generate 4MHz or 8MHz
pub type SMPSDIV_R = crate::BitReader;
///Field `SMPSDIV` writer - SMPS clock prescaling factor to generate 4MHz or 8MHz
pub type SMPSDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUCLKSEL` reader - Selection of LPUART clock:
pub type LPUCLKSEL_R = crate::BitReader;
///Field `LPUCLKSEL` writer - Selection of LPUART clock:
pub type LPUCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSLOWSEL` reader - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn
pub type CLKSLOWSEL_R = crate::FieldReader;
///Field `CLKSLOWSEL` writer - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn
pub type CLKSLOWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOBOOSTEN` reader - IO BOOSTER enable Set and reset by software.
pub type IOBOOSTEN_R = crate::BitReader;
///Field `IOBOOSTEN` writer - IO BOOSTER enable Set and reset by software.
pub type IOBOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOBOOSTCLKEXTEN` reader - IO BOOSTER clock enable as external clock Set and reset by software.
pub type IOBOOSTCLKEXTEN_R = crate::BitReader;
///Field `IOBOOSTCLKEXTEN` writer - IO BOOSTER clock enable as external clock Set and reset by software.
pub type IOBOOSTCLKEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCOEN` reader - LCO output enable
pub type LCOEN_R = crate::BitReader;
///Field `LCOEN` writer - LCO output enable
pub type LCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3I2SCLKSEL` reader - Selection of I2S1 clock: 1x:64MHz peripheral clock
pub type SPI3I2SCLKSEL_R = crate::FieldReader;
///Field `SPI3I2SCLKSEL` writer - Selection of I2S1 clock: 1x:64MHz peripheral clock
pub type SPI3I2SCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCOSEL` reader - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn
pub type LCOSEL_R = crate::FieldReader;
///Field `LCOSEL` writer - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn
pub type LCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCOSEL` reader - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible.
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CCOPRE` reader - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used
pub type CCOPRE_R = crate::FieldReader;
///Field `CCOPRE` writer - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used
pub type CCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - bit to control inversion of the SMPS clock
    #[inline(always)]
    pub fn smpsinv(&self) -> SMPSINV_R {
        SMPSINV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock source selection request:
    #[inline(always)]
    pub fn hsesel(&self) -> HSESEL_R {
        HSESEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Stop HSI clock source request
    #[inline(always)]
    pub fn stophsi(&self) -> STOPHSI_R {
        STOPHSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clock source selection Status
    #[inline(always)]
    pub fn hsesel_status(&self) -> HSESEL_STATUS_R {
        HSESEL_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 5:7 - CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio.
    #[inline(always)]
    pub fn clksysdiv(&self) -> CLKSYSDIV_R {
        CLKSYSDIV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - CLKSYSDIV_STATUS: system clock frequency status Set and cleared by hardware to indicate the actual system clock frequency. This register must be read to be sure that the new frequency, selected by CLKSYSDIV, has been applied. 000: system clock frequency is 64 MHz 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz 100: system clock frequency is 4 MHz 101: system clock frequency is 2 MHz 110: system clock frequency is 1 MHz 111: not used. The actual clock frequency switching can be delayed of up to 128 system clock cycles, depending on the RCC internal counter status at the moment the new CLKSYSDIV is applied
    #[inline(always)]
    pub fn clksysdiv_status(&self) -> CLKSYSDIV_STATUS_R {
        CLKSYSDIV_STATUS_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Selection of LPUART clock:
    #[inline(always)]
    pub fn lpuclksel(&self) -> LPUCLKSEL_R {
        LPUCLKSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn clkslowsel(&self) -> CLKSLOWSEL_R {
        CLKSLOWSEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - IO BOOSTER enable Set and reset by software.
    #[inline(always)]
    pub fn ioboosten(&self) -> IOBOOSTEN_R {
        IOBOOSTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IO BOOSTER clock enable as external clock Set and reset by software.
    #[inline(always)]
    pub fn ioboostclkexten(&self) -> IOBOOSTCLKEXTEN_R {
        IOBOOSTCLKEXTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LCO output enable
    #[inline(always)]
    pub fn lcoen(&self) -> LCOEN_R {
        LCOEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 22:23 - Selection of I2S1 clock: 1x:64MHz peripheral clock
    #[inline(always)]
    pub fn spi3i2sclksel(&self) -> SPI3I2SCLKSEL_R {
        SPI3I2SCLKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lcosel(&self) -> LCOSEL_R {
        LCOSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used
    #[inline(always)]
    pub fn ccopre(&self) -> CCOPRE_R {
        CCOPRE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("smpsinv", &self.smpsinv())
            .field("hsesel", &self.hsesel())
            .field("stophsi", &self.stophsi())
            .field("hsesel_status", &self.hsesel_status())
            .field("clksysdiv", &self.clksysdiv())
            .field("clksysdiv_status", &self.clksysdiv_status())
            .field("smpsdiv", &self.smpsdiv())
            .field("lpuclksel", &self.lpuclksel())
            .field("clkslowsel", &self.clkslowsel())
            .field("ioboosten", &self.ioboosten())
            .field("ioboostclkexten", &self.ioboostclkexten())
            .field("lcoen", &self.lcoen())
            .field("spi3i2sclksel", &self.spi3i2sclksel())
            .field("lcosel", &self.lcosel())
            .field("mcosel", &self.mcosel())
            .field("ccopre", &self.ccopre())
            .finish()
    }
}
impl W {
    ///Bit 0 - bit to control inversion of the SMPS clock
    #[inline(always)]
    pub fn smpsinv(&mut self) -> SMPSINV_W<'_, CFGRrs> {
        SMPSINV_W::new(self, 0)
    }
    ///Bit 1 - Clock source selection request:
    #[inline(always)]
    pub fn hsesel(&mut self) -> HSESEL_W<'_, CFGRrs> {
        HSESEL_W::new(self, 1)
    }
    ///Bit 2 - Stop HSI clock source request
    #[inline(always)]
    pub fn stophsi(&mut self) -> STOPHSI_W<'_, CFGRrs> {
        STOPHSI_W::new(self, 2)
    }
    ///Bits 5:7 - CLKSYSDIV: system clock divided factor from HSI_64M. 000: system clock frequency is 64 MHz (not available when HSESEL=1) 001: system clock frequency is 32 MHz 010: system clock frequency is 16 MHz 011: system clock frequency is 8 MHz * 100: system clock frequency is 4 MHz * 101: system clock frequency is 2 MHz * 110: system clock frequency is 1 MHz * 111: not used. *: If RCC_APB2ENR.MRBLEEN bit is set, writing in CLKSYSDIV one of those values is replaced by a 010b = 16 MHz writing at hardware level. Warning: if the software programs the 64 MHz frequency target while the RCC_CFGR.HSESEL=1, the hardware will switch the system clock tree on HSI64MPLL again (and restart HSIPLL64M analog block if RCC_CFGR.STOPHSI=1) To switch the system frequency between 64 / 32 / 16 MHz without risk when the MR_BLE is used, prefer the RCC_CSCMDR register to change the system frequency. the MR_BLE frequency must always be equal or less than the CPU/system clock to have functional radio.
    #[inline(always)]
    pub fn clksysdiv(&mut self) -> CLKSYSDIV_W<'_, CFGRrs> {
        CLKSYSDIV_W::new(self, 5)
    }
    ///Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W<'_, CFGRrs> {
        SMPSDIV_W::new(self, 12)
    }
    ///Bit 13 - Selection of LPUART clock:
    #[inline(always)]
    pub fn lpuclksel(&mut self) -> LPUCLKSEL_W<'_, CFGRrs> {
        LPUCLKSEL_W::new(self, 13)
    }
    ///Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn clkslowsel(&mut self) -> CLKSLOWSEL_W<'_, CFGRrs> {
        CLKSLOWSEL_W::new(self, 15)
    }
    ///Bit 17 - IO BOOSTER enable Set and reset by software.
    #[inline(always)]
    pub fn ioboosten(&mut self) -> IOBOOSTEN_W<'_, CFGRrs> {
        IOBOOSTEN_W::new(self, 17)
    }
    ///Bit 18 - IO BOOSTER clock enable as external clock Set and reset by software.
    #[inline(always)]
    pub fn ioboostclkexten(&mut self) -> IOBOOSTCLKEXTEN_W<'_, CFGRrs> {
        IOBOOSTCLKEXTEN_W::new(self, 18)
    }
    ///Bit 19 - LCO output enable
    #[inline(always)]
    pub fn lcoen(&mut self) -> LCOEN_W<'_, CFGRrs> {
        LCOEN_W::new(self, 19)
    }
    ///Bits 22:23 - Selection of I2S1 clock: 1x:64MHz peripheral clock
    #[inline(always)]
    pub fn spi3i2sclksel(&mut self) -> SPI3I2SCLKSEL_W<'_, CFGRrs> {
        SPI3I2SCLKSEL_W::new(self, 22)
    }
    ///Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lcosel(&mut self) -> LCOSEL_W<'_, CFGRrs> {
        LCOSEL_W::new(self, 24)
    }
    ///Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGRrs> {
        MCOSEL_W::new(self, 26)
    }
    ///Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. Others: not used
    #[inline(always)]
    pub fn ccopre(&mut self) -> CCOPRE_W<'_, CFGRrs> {
        CCOPRE_W::new(self, 29)
    }
}
/**CFGR register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:CFGR)*/
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
