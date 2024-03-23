#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `BREN` reader - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub type BREN_R = crate::BitReader;
#[doc = "Field `BREN` writer - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONEN` reader - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub type MONEN_R = crate::BitReader;
#[doc = "Field `MONEN` writer - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRRDY` reader - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
pub type BRRDY_R = crate::BitReader;
#[doc = "Field `VBATL` reader - VBAT level monitoring versus low threshold"]
pub type VBATL_R = crate::BitReader;
#[doc = "Field `VBATH` reader - VBAT level monitoring versus high threshold"]
pub type VBATH_R = crate::BitReader;
#[doc = "Field `TEMPL` reader - Temperature level monitoring versus low threshold"]
pub type TEMPL_R = crate::BitReader;
#[doc = "Field `TEMPH` reader - Temperature level monitoring versus high threshold"]
pub type TEMPH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - VBAT level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBAT level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<CR2rs> {
        BREN_W::new(self, 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<CR2rs> {
        MONEN_W::new(self, 4)
    }
}
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
