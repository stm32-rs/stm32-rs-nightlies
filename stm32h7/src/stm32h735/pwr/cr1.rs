#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `LPDS` reader - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
pub type LPDS_R = crate::BitReader;
#[doc = "Field `LPDS` writer - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
pub type LPDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Programmable voltage detector enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Programmable voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBP` reader - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPS` reader - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
pub type FLPS_R = crate::BitReader;
#[doc = "Field `FLPS` writer - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVOS` reader - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_R = crate::FieldReader;
#[doc = "Field `SVOS` writer - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AVDEN` reader - Peripheral voltage monitor on VDDA enable"]
pub type AVDEN_R = crate::BitReader;
#[doc = "Field `AVDEN` writer - Peripheral voltage monitor on VDDA enable"]
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALS` reader - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_R = crate::FieldReader;
#[doc = "Field `ALS` writer - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Programmable voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Peripheral voltage monitor on VDDA enable"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power Deepsleep with SVOS3 (SVOS4 and SVOS5 always use low-power, regardless of the setting of this bit)"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<CR1rs> {
        LPDS_W::new(self, 0)
    }
    #[doc = "Bit 4 - Programmable voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR1rs> {
        PVDE_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Programmable voltage detector level selection These bits select the voltage threshold detected by the PVD. Note: Refer to Section Electrical characteristics of the product datasheet for more details."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CR1rs> {
        PLS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection In reset state, the RCC_BDCR register, the RTC registers (including the backup registers), BREN and MOEN bits in PWR_CR2 register, are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CR1rs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Flash low-power mode in DStop mode This bit allows to obtain the best trade-off between low-power consumption and restart time when exiting from DStop mode. When it is set, the Flash memory enters low-power mode when D1 domain is in DStop mode."]
    #[inline(always)]
    #[must_use]
    pub fn flps(&mut self) -> FLPS_W<CR1rs> {
        FLPS_W::new(self, 9)
    }
    #[doc = "Bits 14:15 - System Stop mode voltage scaling selection These bits control the VCORE voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
    #[inline(always)]
    #[must_use]
    pub fn svos(&mut self) -> SVOS_W<CR1rs> {
        SVOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Peripheral voltage monitor on VDDA enable"]
    #[inline(always)]
    #[must_use]
    pub fn avden(&mut self) -> AVDEN_W<CR1rs> {
        AVDEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Analog voltage detector level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    #[must_use]
    pub fn als(&mut self) -> ALS_W<CR1rs> {
        ALS_W::new(self, 17)
    }
}
#[doc = "PWR control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0xf000_c000"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xf000_c000;
}
