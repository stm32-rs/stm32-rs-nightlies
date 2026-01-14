///Register `BDCR1` reader
pub type R = crate::R<BDCR1rs>;
///Register `BDCR1` writer
pub type W = crate::W<BDCR1rs>;
///Field `LSEON` reader - LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - LSE oscillator ready Set and cleared by hardware to indicate when the external 32kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEDRV` reader - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in 'Xtal' mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in 'Xtal mode' when it is not in bypass mode.
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in 'Xtal' mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in 'Xtal mode' when it is not in bypass mode.
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSECSSON` reader - Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD=1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSECSSON_R = crate::BitReader;
///Field `LSECSSON` writer - Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD=1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - Low speed external clock security, LSE failure Detection Set by hardware to indicate when a failure is detected by the LSECCS on the external 32kHz oscillator. Reset when LSCSSON bit is cleared. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSECSSD_R = crate::BitReader;
///Field `LSESYSEN` reader - LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSESYSEN_R = crate::BitReader;
///Field `LSESYSEN` writer - LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSESYSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSEL` reader - RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RTCSEL_R = crate::FieldReader;
///Field `RTCSEL` writer - RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSESYSRDY` reader - LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSESYSRDY_R = crate::BitReader;
///Field `LSEGFON` reader - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEGFON_R = crate::BitReader;
///Field `LSEGFON` writer - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSEGFON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSETRIM` reader - LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.
pub type LSETRIM_R = crate::FieldReader;
///Field `LSETRIM` writer - LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.
pub type LSETRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BDRST` reader - Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type BDRST_R = crate::BitReader;
///Field `BDRST` writer - Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIOSTSEL` reader - 2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIOSTSEL_R = crate::FieldReader;
///Field `RADIOSTSEL` writer - 2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RADIOSTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSCOEN_R = crate::BitReader;
///Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSCOSEL_R = crate::BitReader;
///Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSI1ON` reader - LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1ON_R = crate::BitReader;
///Field `LSI1ON` writer - LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSI1RDY` reader - LSI1 oscillator ready Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1RDY_R = crate::BitReader;
///Field `LSI1PREDIV` reader - LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1PREDIV_R = crate::BitReader;
///Field `LSI1PREDIV` writer - LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LSI1PREDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSI2ON` reader - LSI2 oscillator enable
pub type LSI2ON_R = crate::BitReader;
///Field `LSI2ON` writer - LSI2 oscillator enable
pub type LSI2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSI2RDY` reader - LSI2 oscillator ready
pub type LSI2RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready Set and cleared by hardware to indicate when the external 32kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in 'Xtal' mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in 'Xtal mode' when it is not in bypass mode.
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD=1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Low speed external clock security, LSE failure Detection Set by hardware to indicate when a failure is detected by the LSECCS on the external 32kHz oscillator. Reset when LSCSSON bit is cleared. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsegfon(&self) -> LSEGFON_R {
        LSEGFON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.
    #[inline(always)]
    pub fn lsetrim(&self) -> LSETRIM_R {
        LSETRIM_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 18:19 - 2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiostsel(&self) -> RADIOSTSEL_R {
        RADIOSTSEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1on(&self) -> LSI1ON_R {
        LSI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - LSI1 oscillator ready Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1rdy(&self) -> LSI1RDY_R {
        LSI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1prediv(&self) -> LSI1PREDIV_R {
        LSI1PREDIV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - LSI2 oscillator enable
    #[inline(always)]
    pub fn lsi2on(&self) -> LSI2ON_R {
        LSI2ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - LSI2 oscillator ready
    #[inline(always)]
    pub fn lsi2rdy(&self) -> LSI2RDY_R {
        LSI2RDY_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR1")
            .field("lseon", &self.lseon())
            .field("lserdy", &self.lserdy())
            .field("lsebyp", &self.lsebyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("lsesysen", &self.lsesysen())
            .field("rtcsel", &self.rtcsel())
            .field("lsesysrdy", &self.lsesysrdy())
            .field("lsegfon", &self.lsegfon())
            .field("lsetrim", &self.lsetrim())
            .field("bdrst", &self.bdrst())
            .field("radiostsel", &self.radiostsel())
            .field("lscoen", &self.lscoen())
            .field("lscosel", &self.lscosel())
            .field("lsi1on", &self.lsi1on())
            .field("lsi1rdy", &self.lsi1rdy())
            .field("lsi1prediv", &self.lsi1prediv())
            .field("lsi2on", &self.lsi2on())
            .field("lsi2rdy", &self.lsi2rdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCR1rs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCR1rs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in 'Xtal' mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in 'Xtal mode' when it is not in bypass mode.
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCR1rs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD=1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCR1rs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bit 7 - LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<'_, BDCR1rs> {
        LSESYSEN_W::new(self, 7)
    }
    ///Bits 8:9 - RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, BDCR1rs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 12 - LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsegfon(&mut self) -> LSEGFON_W<'_, BDCR1rs> {
        LSEGFON_W::new(self, 12)
    }
    ///Bits 13:14 - LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0.
    #[inline(always)]
    pub fn lsetrim(&mut self) -> LSETRIM_W<'_, BDCR1rs> {
        LSETRIM_W::new(self, 13)
    }
    ///Bit 16 - Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, BDCR1rs> {
        BDRST_W::new(self, 16)
    }
    ///Bits 18:19 - 2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn radiostsel(&mut self) -> RADIOSTSEL_W<'_, BDCR1rs> {
        RADIOSTSEL_W::new(self, 18)
    }
    ///Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, BDCR1rs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, BDCR1rs> {
        LSCOSEL_W::new(self, 25)
    }
    ///Bit 26 - LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1on(&mut self) -> LSI1ON_W<'_, BDCR1rs> {
        LSI1ON_W::new(self, 26)
    }
    ///Bit 28 - LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lsi1prediv(&mut self) -> LSI1PREDIV_W<'_, BDCR1rs> {
        LSI1PREDIV_W::new(self, 28)
    }
    ///Bit 29 - LSI2 oscillator enable
    #[inline(always)]
    pub fn lsi2on(&mut self) -> LSI2ON_W<'_, BDCR1rs> {
        LSI2ON_W::new(self, 29)
    }
}
/**RCC backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:BDCR1)*/
pub struct BDCR1rs;
impl crate::RegisterSpec for BDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bdcr1::R`](R) reader structure
impl crate::Readable for BDCR1rs {}
///`write(|w| ..)` method takes [`bdcr1::W`](W) writer structure
impl crate::Writable for BDCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR1 to value 0
impl crate::Resettable for BDCR1rs {}
