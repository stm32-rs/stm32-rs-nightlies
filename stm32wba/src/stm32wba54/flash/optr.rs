///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section7.6.2: Readout protection (RDP) for more details.
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section7.6.2: Readout protection (RDP) for more details.
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOR_LEV` reader - BOR reset level These bits contain the VsubDD/sub supply level threshold that activates/releases the reset.
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - BOR reset level These bits contain the VsubDD/sub supply level threshold that activates/releases the reset.
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NRST_STOP` reader - Reset generation in Stop mode
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - Reset generation in Stop mode
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDBY` reader - Reset generation in Standby mode
pub type NRST_STDBY_R = crate::BitReader;
///Field `NRST_STDBY` writer - Reset generation in Standby mode
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1_RST` reader - SRAM1 erase upon system reset
pub type SRAM1_RST_R = crate::BitReader;
///Field `SRAM1_RST` writer - SRAM1 erase upon system reset
pub type SRAM1_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - Independent watchdog enable selection
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - Independent watchdog enable selection
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader;
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub type SRAM2_PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `SRAM2_RST` writer - SRAM2 erase when system reset
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSWBOOT0` reader - Software BOOT0
pub type NSWBOOT0_R = crate::BitReader;
///Field `NSWBOOT0` writer - Software BOOT0
pub type NSWBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT0` reader - NBOOT0 option bit
pub type NBOOT0_R = crate::BitReader;
///Field `NBOOT0` writer - NBOOT0 option bit
pub type NBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN` reader - Global TrustZone security enable
pub type TZEN_R = crate::BitReader;
///Field `TZEN` writer - Global TrustZone security enable
pub type TZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section7.6.2: Readout protection (RDP) for more details.
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VsubDD/sub supply level threshold that activates/releases the reset.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - SRAM1 erase upon system reset
    #[inline(always)]
    pub fn sram1_rst(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog enable selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn nswboot0(&self) -> NSWBOOT0_R {
        NSWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - NBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("bor_lev", &self.bor_lev())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("sram1_rst", &self.sram1_rst())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("sram2_pe", &self.sram2_pe())
            .field("sram2_rst", &self.sram2_rst())
            .field("nswboot0", &self.nswboot0())
            .field("nboot0", &self.nboot0())
            .field("tzen", &self.tzen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section7.6.2: Readout protection (RDP) for more details.
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset level These bits contain the VsubDD/sub supply level threshold that activates/releases the reset.
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTRrs> {
        BOR_LEV_W::new(self, 8)
    }
    ///Bit 12 - Reset generation in Stop mode
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTRrs> {
        NRST_STOP_W::new(self, 12)
    }
    ///Bit 13 - Reset generation in Standby mode
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<'_, OPTRrs> {
        NRST_STDBY_W::new(self, 13)
    }
    ///Bit 15 - SRAM1 erase upon system reset
    #[inline(always)]
    pub fn sram1_rst(&mut self) -> SRAM1_RST_W<'_, OPTRrs> {
        SRAM1_RST_W::new(self, 15)
    }
    ///Bit 16 - Independent watchdog enable selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<'_, OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<'_, OPTRrs> {
        SRAM2_PE_W::new(self, 24)
    }
    ///Bit 25 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<'_, OPTRrs> {
        SRAM2_RST_W::new(self, 25)
    }
    ///Bit 26 - Software BOOT0
    #[inline(always)]
    pub fn nswboot0(&mut self) -> NSWBOOT0_W<'_, OPTRrs> {
        NSWBOOT0_W::new(self, 26)
    }
    ///Bit 27 - NBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&mut self) -> NBOOT0_W<'_, OPTRrs> {
        NBOOT0_W::new(self, 27)
    }
    ///Bit 31 - Global TrustZone security enable
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W<'_, OPTRrs> {
        TZEN_W::new(self, 31)
    }
}
/**FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {}
