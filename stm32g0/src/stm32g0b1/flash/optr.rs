///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOREN` reader - BOR reset Level
pub type BOREN_R = crate::BitReader;
///Field `BOREN` writer - BOR reset Level
pub type BOREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset
pub type BORF_LEV_R = crate::FieldReader;
///Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset
pub type BORF_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BORR_LEV` reader - These bits contain the VDD supply level threshold that releases the reset.
pub type BORR_LEV_R = crate::FieldReader;
///Field `BORR_LEV` writer - These bits contain the VDD supply level threshold that releases the reset.
pub type BORR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRSTS_HDW` reader - nRSTS_HDW
pub type N_RSTS_HDW_R = crate::BitReader;
///Field `nRSTS_HDW` writer - nRSTS_HDW
pub type N_RSTS_HDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - Independent watchdog selection
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
///Field `RAM_PARITY_CHECK` reader - SRAM parity check control
pub type RAM_PARITY_CHECK_R = crate::BitReader;
///Field `RAM_PARITY_CHECK` writer - SRAM parity check control
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT_SEL` reader - nBOOT_SEL
pub type N_BOOT_SEL_R = crate::BitReader;
///Field `nBOOT_SEL` writer - nBOOT_SEL
pub type N_BOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nBOOT0` reader - nBOOT0 option bit
pub type N_BOOT0_R = crate::BitReader;
///Field `nBOOT0` writer - nBOOT0 option bit
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_MODE` reader - NRST_MODE
pub type NRST_MODE_R = crate::FieldReader;
///Field `NRST_MODE` writer - NRST_MODE
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IRHEN` reader - Internal reset holder enable bit
pub type IRHEN_R = crate::BitReader;
///Field `IRHEN` writer - Internal reset holder enable bit
pub type IRHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - BOR reset Level
    #[inline(always)]
    pub fn boren(&self) -> BOREN_R {
        BOREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset.
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - nRSTS_HDW
    #[inline(always)]
    pub fn n_rsts_hdw(&self) -> N_RSTS_HDW_R {
        N_RSTS_HDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
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
    ///Bit 22 - SRAM parity check control
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - nBOOT_SEL
    #[inline(always)]
    pub fn n_boot_sel(&self) -> N_BOOT_SEL_R {
        N_BOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("boren", &self.boren())
            .field("borf_lev", &self.borf_lev())
            .field("borr_lev", &self.borr_lev())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("n_rsts_hdw", &self.n_rsts_hdw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("ram_parity_check", &self.ram_parity_check())
            .field("n_boot_sel", &self.n_boot_sel())
            .field("n_boot1", &self.n_boot1())
            .field("n_boot0", &self.n_boot0())
            .field("nrst_mode", &self.nrst_mode())
            .field("irhen", &self.irhen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bit 8 - BOR reset Level
    #[inline(always)]
    pub fn boren(&mut self) -> BOREN_W<'_, OPTRrs> {
        BOREN_W::new(self, 8)
    }
    ///Bits 9:10 - These bits contain the VDD supply level threshold that activates the reset
    #[inline(always)]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<'_, OPTRrs> {
        BORF_LEV_W::new(self, 9)
    }
    ///Bits 11:12 - These bits contain the VDD supply level threshold that releases the reset.
    #[inline(always)]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<'_, OPTRrs> {
        BORR_LEV_W::new(self, 11)
    }
    ///Bit 13 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<'_, OPTRrs> {
        N_RST_STOP_W::new(self, 13)
    }
    ///Bit 14 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<'_, OPTRrs> {
        N_RST_STDBY_W::new(self, 14)
    }
    ///Bit 15 - nRSTS_HDW
    #[inline(always)]
    pub fn n_rsts_hdw(&mut self) -> N_RSTS_HDW_W<'_, OPTRrs> {
        N_RSTS_HDW_W::new(self, 15)
    }
    ///Bit 16 - Independent watchdog selection
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
    ///Bit 22 - SRAM parity check control
    #[inline(always)]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<'_, OPTRrs> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    ///Bit 24 - nBOOT_SEL
    #[inline(always)]
    pub fn n_boot_sel(&mut self) -> N_BOOT_SEL_W<'_, OPTRrs> {
        N_BOOT_SEL_W::new(self, 24)
    }
    ///Bit 25 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<'_, OPTRrs> {
        N_BOOT1_W::new(self, 25)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<'_, OPTRrs> {
        N_BOOT0_W::new(self, 26)
    }
    ///Bits 27:28 - NRST_MODE
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<'_, OPTRrs> {
        NRST_MODE_W::new(self, 27)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W<'_, OPTRrs> {
        IRHEN_W::new(self, 29)
    }
}
/**Flash option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#FLASH:OPTR)*/
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
///`reset()` method sets OPTR to value 0xf000_0000
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
