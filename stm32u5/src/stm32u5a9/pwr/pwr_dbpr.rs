///Register `PWR_DBPR` reader
pub type R = crate::R<PWR_DBPRrs>;
///Register `PWR_DBPR` writer
pub type W = crate::W<PWR_DBPRrs>;
///Field `DBP` reader - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
pub type DBP_R = crate::BitReader;
///Field `DBP` writer - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_DBPR")
            .field("dbp", &self.dbp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable the write access to these registers.
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<PWR_DBPRrs> {
        DBP_W::new(self, 0)
    }
}
/**PWR disable Backup domain register

You can [`read`](crate::Reg::read) this register and get [`pwr_dbpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_dbpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#PWR:PWR_DBPR)*/
pub struct PWR_DBPRrs;
impl crate::RegisterSpec for PWR_DBPRrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_dbpr::R`](R) reader structure
impl crate::Readable for PWR_DBPRrs {}
///`write(|w| ..)` method takes [`pwr_dbpr::W`](W) writer structure
impl crate::Writable for PWR_DBPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_DBPR to value 0
impl crate::Resettable for PWR_DBPRrs {
    const RESET_VALUE: u32 = 0;
}
