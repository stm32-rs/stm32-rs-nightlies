///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
///Field `OPTLOCK` reader - Option lock
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Option lock
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Option start
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Option start
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WWDG_SW` reader - User option bytes
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - User option bytes
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - User option bytes
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - User option bytes
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STOP` reader - User option bytes
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - User option bytes
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - User option bytes
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - User option bytes
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Read protect
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protect
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `nWRP` reader - Not write protect
pub type N_WRP_R = crate::FieldReader<u16>;
///Field `nWRP` writer - Not write protect
pub type N_WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `nDBOOT` reader - Dual Boot mode (valid only when nDBANK=0)
pub type N_DBOOT_R = crate::BitReader;
///Field `nDBOOT` writer - Dual Boot mode (valid only when nDBANK=0)
pub type N_DBOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nDBANK` reader - Not dual bank mode
pub type N_DBANK_R = crate::BitReader;
///Field `nDBANK` writer - Not dual bank mode
pub type N_DBANK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in standby mode
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - User option bytes
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - User option bytes
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:27 - Not write protect
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 28 - Dual Boot mode (valid only when nDBANK=0)
    #[inline(always)]
    pub fn n_dboot(&self) -> N_DBOOT_R {
        N_DBOOT_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Not dual bank mode
    #[inline(always)]
    pub fn n_dbank(&self) -> N_DBANK_R {
        N_DBANK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Independent watchdog counter freeze in standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("optstrt", &self.optstrt())
            .field("bor_lev", &self.bor_lev())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("rdp", &self.rdp())
            .field("n_wrp", &self.n_wrp())
            .field("n_dboot", &self.n_dboot())
            .field("n_dbank", &self.n_dbank())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("iwdg_stop", &self.iwdg_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<OPTCRrs> {
        OPTSTRT_W::new(self, 1)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTCRrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - User option bytes
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTCRrs> {
        WWDG_SW_W::new(self, 4)
    }
    ///Bit 5 - User option bytes
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTCRrs> {
        IWDG_SW_W::new(self, 5)
    }
    ///Bit 6 - User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTCRrs> {
        N_RST_STOP_W::new(self, 6)
    }
    ///Bit 7 - User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTCRrs> {
        N_RST_STDBY_W::new(self, 7)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<OPTCRrs> {
        RDP_W::new(self, 8)
    }
    ///Bits 16:27 - Not write protect
    #[inline(always)]
    pub fn n_wrp(&mut self) -> N_WRP_W<OPTCRrs> {
        N_WRP_W::new(self, 16)
    }
    ///Bit 28 - Dual Boot mode (valid only when nDBANK=0)
    #[inline(always)]
    pub fn n_dboot(&mut self) -> N_DBOOT_W<OPTCRrs> {
        N_DBOOT_W::new(self, 28)
    }
    ///Bit 29 - Not dual bank mode
    #[inline(always)]
    pub fn n_dbank(&mut self) -> N_DBANK_W<OPTCRrs> {
        N_DBANK_W::new(self, 29)
    }
    ///Bit 30 - Independent watchdog counter freeze in standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTCRrs> {
        IWDG_STDBY_W::new(self, 30)
    }
    ///Bit 31 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTCRrs> {
        IWDG_STOP_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#FLASH:OPTCR)*/
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`optcr::R`](R) reader structure
impl crate::Readable for OPTCRrs {}
///`write(|w| ..)` method takes [`optcr::W`](W) writer structure
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTCR to value 0x0fff_aaed
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x0fff_aaed;
}
