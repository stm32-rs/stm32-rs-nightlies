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
#[doc = "Field `BOOSTE` reader - BOOSTE"]
pub type BOOSTE_R = crate::BitReader;
#[doc = "Field `BOOSTE` writer - BOOSTE"]
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVD_READY` reader - AVD_READY"]
pub type AVD_READY_R = crate::BitReader;
#[doc = "Field `AVD_READY` writer - AVD_READY"]
pub type AVD_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `AXIRAM1SO` reader - AXIRAM1SO"]
pub type AXIRAM1SO_R = crate::BitReader;
#[doc = "Field `AXIRAM1SO` writer - AXIRAM1SO"]
pub type AXIRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAM2SO` reader - AXIRAM2SO"]
pub type AXIRAM2SO_R = crate::BitReader;
#[doc = "Field `AXIRAM2SO` writer - AXIRAM2SO"]
pub type AXIRAM2SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAM3SO` reader - AXIRAM3SO"]
pub type AXIRAM3SO_R = crate::BitReader;
#[doc = "Field `AXIRAM3SO` writer - AXIRAM3SO"]
pub type AXIRAM3SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBRAM1SO` reader - AHBRAM1SO"]
pub type AHBRAM1SO_R = crate::BitReader;
#[doc = "Field `AHBRAM1SO` writer - AHBRAM1SO"]
pub type AHBRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBRAM2SO` reader - AHBRAM2SO"]
pub type AHBRAM2SO_R = crate::BitReader;
#[doc = "Field `AHBRAM2SO` writer - AHBRAM2SO"]
pub type AHBRAM2SO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCMSO` reader - ITCMSO"]
pub type ITCMSO_R = crate::BitReader;
#[doc = "Field `ITCMSO` writer - ITCMSO"]
pub type ITCMSO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXSO` reader - GFXSO"]
pub type GFXSO_R = crate::BitReader;
#[doc = "Field `GFXSO` writer - GFXSO"]
pub type GFXSO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSITFSO` reader - HSITFSO"]
pub type HSITFSO_R = crate::BitReader;
#[doc = "Field `HSITFSO` writer - HSITFSO"]
pub type HSITFSO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRDRAMSO` reader - SRDRAMSO"]
pub type SRDRAMSO_R = crate::BitReader;
#[doc = "Field `SRDRAMSO` writer - SRDRAMSO"]
pub type SRDRAMSO_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 12 - BOOSTE"]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AVD_READY"]
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 19 - AXIRAM1SO"]
    #[inline(always)]
    pub fn axiram1so(&self) -> AXIRAM1SO_R {
        AXIRAM1SO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AXIRAM2SO"]
    #[inline(always)]
    pub fn axiram2so(&self) -> AXIRAM2SO_R {
        AXIRAM2SO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AXIRAM3SO"]
    #[inline(always)]
    pub fn axiram3so(&self) -> AXIRAM3SO_R {
        AXIRAM3SO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AHBRAM1SO"]
    #[inline(always)]
    pub fn ahbram1so(&self) -> AHBRAM1SO_R {
        AHBRAM1SO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AHBRAM2SO"]
    #[inline(always)]
    pub fn ahbram2so(&self) -> AHBRAM2SO_R {
        AHBRAM2SO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ITCMSO"]
    #[inline(always)]
    pub fn itcmso(&self) -> ITCMSO_R {
        ITCMSO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GFXSO"]
    #[inline(always)]
    pub fn gfxso(&self) -> GFXSO_R {
        GFXSO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HSITFSO"]
    #[inline(always)]
    pub fn hsitfso(&self) -> HSITFSO_R {
        HSITFSO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SRDRAMSO"]
    #[inline(always)]
    pub fn srdramso(&self) -> SRDRAMSO_R {
        SRDRAMSO_R::new(((self.bits >> 27) & 1) != 0)
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
    #[doc = "Bit 12 - BOOSTE"]
    #[inline(always)]
    #[must_use]
    pub fn booste(&mut self) -> BOOSTE_W<CR1rs> {
        BOOSTE_W::new(self, 12)
    }
    #[doc = "Bit 13 - AVD_READY"]
    #[inline(always)]
    #[must_use]
    pub fn avd_ready(&mut self) -> AVD_READY_W<CR1rs> {
        AVD_READY_W::new(self, 13)
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
    #[doc = "Bit 19 - AXIRAM1SO"]
    #[inline(always)]
    #[must_use]
    pub fn axiram1so(&mut self) -> AXIRAM1SO_W<CR1rs> {
        AXIRAM1SO_W::new(self, 19)
    }
    #[doc = "Bit 20 - AXIRAM2SO"]
    #[inline(always)]
    #[must_use]
    pub fn axiram2so(&mut self) -> AXIRAM2SO_W<CR1rs> {
        AXIRAM2SO_W::new(self, 20)
    }
    #[doc = "Bit 21 - AXIRAM3SO"]
    #[inline(always)]
    #[must_use]
    pub fn axiram3so(&mut self) -> AXIRAM3SO_W<CR1rs> {
        AXIRAM3SO_W::new(self, 21)
    }
    #[doc = "Bit 22 - AHBRAM1SO"]
    #[inline(always)]
    #[must_use]
    pub fn ahbram1so(&mut self) -> AHBRAM1SO_W<CR1rs> {
        AHBRAM1SO_W::new(self, 22)
    }
    #[doc = "Bit 23 - AHBRAM2SO"]
    #[inline(always)]
    #[must_use]
    pub fn ahbram2so(&mut self) -> AHBRAM2SO_W<CR1rs> {
        AHBRAM2SO_W::new(self, 23)
    }
    #[doc = "Bit 24 - ITCMSO"]
    #[inline(always)]
    #[must_use]
    pub fn itcmso(&mut self) -> ITCMSO_W<CR1rs> {
        ITCMSO_W::new(self, 24)
    }
    #[doc = "Bit 25 - GFXSO"]
    #[inline(always)]
    #[must_use]
    pub fn gfxso(&mut self) -> GFXSO_W<CR1rs> {
        GFXSO_W::new(self, 25)
    }
    #[doc = "Bit 26 - HSITFSO"]
    #[inline(always)]
    #[must_use]
    pub fn hsitfso(&mut self) -> HSITFSO_W<CR1rs> {
        HSITFSO_W::new(self, 26)
    }
    #[doc = "Bit 27 - SRDRAMSO"]
    #[inline(always)]
    #[must_use]
    pub fn srdramso(&mut self) -> SRDRAMSO_W<CR1rs> {
        SRDRAMSO_W::new(self, 27)
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
