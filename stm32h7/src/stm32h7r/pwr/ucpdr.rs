///Register `UCPDR` reader
pub type R = crate::R<UCPDRrs>;
///Register `UCPDR` writer
pub type W = crate::W<UCPDRrs>;
///Field `UCPD_DBDIS` reader - UCPD dead battery disable
pub type UCPD_DBDIS_R = crate::BitReader;
///Field `UCPD_DBDIS` writer - UCPD dead battery disable
pub type UCPD_DBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD_STBY` reader - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_R = crate::BitReader;
///Field `UCPD_STBY` writer - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
pub type UCPD_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UCPD dead battery disable
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPDR")
            .field("ucpd_dbdis", &self.ucpd_dbdis())
            .field("ucpd_stby", &self.ucpd_stby())
            .finish()
    }
}
impl W {
    ///Bit 0 - UCPD dead battery disable
    #[inline(always)]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<'_, UCPDRrs> {
        UCPD_DBDIS_W::new(self, 0)
    }
    ///Bit 1 - UCPD Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD. It must be written to 0 after exiting the Standby mode and before writing any UCPD registers.
    #[inline(always)]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<'_, UCPDRrs> {
        UCPD_STBY_W::new(self, 1)
    }
}
/**PWR USB Type-C and Power Delivery register

You can [`read`](crate::Reg::read) this register and get [`ucpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:UCPDR)*/
pub struct UCPDRrs;
impl crate::RegisterSpec for UCPDRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpdr::R`](R) reader structure
impl crate::Readable for UCPDRrs {}
///`write(|w| ..)` method takes [`ucpdr::W`](W) writer structure
impl crate::Writable for UCPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UCPDR to value 0x02
impl crate::Resettable for UCPDRrs {
    const RESET_VALUE: u32 = 0x02;
}
