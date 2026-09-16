///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
///Field `LSEON` reader - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEDRV` reader - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSECSSON` reader - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
pub type LSECSSON_R = crate::BitReader;
///Field `LSECSSON` writer - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):
pub type LSECSSD_R = crate::BitReader;
///Field `LSESYSEN` reader - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.
pub type LSESYSEN_R = crate::BitReader;
///Field `LSESYSEN` writer - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.
pub type LSESYSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.
pub type RTCSEL_R = crate::FieldReader;
///Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSESYSRDY` reader - LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system.
pub type LSESYSRDY_R = crate::BitReader;
///Field `RTCEN` reader - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BDRST` reader - RTC domain software reset Set and cleared by software to reset the RTC domain:
pub type BDRST_R = crate::BitReader;
///Field `BDRST` writer - RTC domain software reset Set and cleared by software to reset the RTC domain:
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_R = crate::BitReader;
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software.
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
pub type LSCOSEL_R = crate::BitReader;
///Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 321kHz oscillator is ready (stable): After the LSEON bit is cleared, LSERDY goes low after 6 external low-speed oscillator clock cycles.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the clock security system on the external 321kHz oscillator (LSE):
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE clock ready for system usage This flag is set by hardware to indicate that the LSE clock is ready for being used by the system (see LSESYSEN bit). This flag is set when LSE clock is ready (LSEON1=11 and LSERDY1=11) and two LSE clock cycles after that LSESYSEN is set. Cleared by hardware to indicate that the LSE clock is not ready to be used by the system.
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("lsesysen", &self.lsesysen())
            .field("rtcsel", &self.rtcsel())
            .field("lsesysrdy", &self.lsesysrdy())
            .field("rtcen", &self.rtcen())
            .field("bdrst", &self.bdrst())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable Set and cleared by software to enable LSE oscillator:
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass the LSE oscillator (in debug mode). This bit can be written only when the external 321kHz oscillator is disabled (LSEON=0 and LSERDY=0).
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to select the LSE oscillator drive capability as follows: Applicable when the LSE oscillator is in Xtal mode, as opposed to bypass mode.
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - CSS on LSE enable Set by software to enable the clock security system on LSE (321kHz) oscillator as follows: LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD =1). In that case the software must disable the LSECSSON bit.
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 7 - LSE clock enable for system usage This bit must be set by software to enable the LSE clock for a system usage.
    #[inline(always)]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<'_, BDCRrs> {
        LSESYSEN_W::new(self, 7)
    }
    ///Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC as follows: Once the RTC clock source is selected, it cannot be changed anymore unless the RTC domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset this bitfield to 00.
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 15 - RTC clock enable Set and cleared by software. The bit enables clock to RTC and TAMP.
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - RTC domain software reset Set and cleared by software to reset the RTC domain:
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, BDCRrs> {
        BDRST_W::new(self, 16)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software.
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software to select the low-speed output clock:
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
}
/**RTC domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
