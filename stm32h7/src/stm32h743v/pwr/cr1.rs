///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPDS` reader - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)
pub type LPDS_R = crate::BitReader;
///Field `LPDS` writer - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDE` reader - Programmable voltage detector enable
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - Programmable voltage detector enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLS` reader - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
pub type PLS_R = crate::FieldReader;
///Field `PLS` writer - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBP` reader - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers.
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers.
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLPS` reader - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode.
pub type FLPS_R = crate::BitReader;
///Field `FLPS` writer - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode.
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SVOS` reader - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_R = crate::FieldReader;
///Field `SVOS` writer - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AVDEN` reader - Peripheral voltage monitor on VDDA enable
pub type AVDEN_R = crate::BitReader;
///Field `AVDEN` writer - Peripheral voltage monitor on VDDA enable
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALS` reader - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD.
pub type ALS_R = crate::FieldReader;
///Field `ALS` writer - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD.
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers.
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode.
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Peripheral voltage monitor on VDDA enable
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD.
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpds", &self.lpds())
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("dbp", &self.dbp())
            .field("flps", &self.flps())
            .field("svos", &self.svos())
            .field("avden", &self.avden())
            .field("als", &self.als())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W<'_, CR1rs> {
        LPDS_W::new(self, 0)
    }
    ///Bit 4 - Programmable voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR1rs> {
        PVDE_W::new(self, 4)
    }
    ///Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, CR1rs> {
        PLS_W::new(self, 5)
    }
    ///Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers.
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode.
    #[inline(always)]
    pub fn flps(&mut self) -> FLPS_W<'_, CR1rs> {
        FLPS_W::new(self, 9)
    }
    ///Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.
    #[inline(always)]
    pub fn svos(&mut self) -> SVOS_W<'_, CR1rs> {
        SVOS_W::new(self, 14)
    }
    ///Bit 16 - Peripheral voltage monitor on VDDA enable
    #[inline(always)]
    pub fn avden(&mut self) -> AVDEN_W<'_, CR1rs> {
        AVDEN_W::new(self, 16)
    }
    ///Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD.
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W<'_, CR1rs> {
        ALS_W::new(self, 17)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0xf000_c000
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xf000_c000;
}
