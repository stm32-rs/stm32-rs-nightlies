///Register `PWR_BDCR1` reader
pub type R = crate::R<PWR_BDCR1rs>;
///Register `PWR_BDCR1` writer
pub type W = crate::W<PWR_BDCR1rs>;
///Field `BREN` reader - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_R = crate::BitReader;
///Field `BREN` writer - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONEN` reader - Backup domain voltage and temperature monitoring enable
pub type MONEN_R = crate::BitReader;
///Field `MONEN` writer - Backup domain voltage and temperature monitoring enable
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_BDCR1")
            .field("bren", &self.bren())
            .field("monen", &self.monen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Backup RAM retention in Standby and VBAT modes When this bit is set, the backup RAM content is kept in Standby and VBAT modes. If BREN is reset, the backup RAM can still be used in Run, Sleep and Stop modes. However, its content is lost in Standby, Shutdown and VBAT modes. This bit can be written only when the regulator is LDO, which must be configured before switching to SMPS. Note: Backup RAM cannot be preserved in Shutdown mode.
    #[inline(always)]
    #[must_use]
    pub fn bren(&mut self) -> BREN_W<PWR_BDCR1rs> {
        BREN_W::new(self, 0)
    }
    ///Bit 4 - Backup domain voltage and temperature monitoring enable
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<PWR_BDCR1rs> {
        MONEN_W::new(self, 4)
    }
}
/**PWR Backup domain control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_bdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:PWR_BDCR1)*/
pub struct PWR_BDCR1rs;
impl crate::RegisterSpec for PWR_BDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_bdcr1::R`](R) reader structure
impl crate::Readable for PWR_BDCR1rs {}
///`write(|w| ..)` method takes [`pwr_bdcr1::W`](W) writer structure
impl crate::Writable for PWR_BDCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_BDCR1 to value 0
impl crate::Resettable for PWR_BDCR1rs {
    const RESET_VALUE: u32 = 0;
}
