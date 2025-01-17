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
///Field `WDG_SW` reader - WDG_SW User option bytes
pub type WDG_SW_R = crate::BitReader;
///Field `WDG_SW` writer - WDG_SW User option bytes
pub type WDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STOP` reader - nRST_STOP User option bytes
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STOP` writer - nRST_STOP User option bytes
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `nRST_STDBY` reader - nRST_STDBY User option bytes
pub type N_RST_STDBY_R = crate::BitReader;
///Field `nRST_STDBY` writer - nRST_STDBY User option bytes
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDP` reader - Read protect
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protect
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `nWRP` reader - Not write protect
pub type N_WRP_R = crate::FieldReader;
///Field `nWRP` writer - Not write protect
pub type N_WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SPRMOD` reader - Selection of Protection Mode of nWPRi bits
pub type SPRMOD_R = crate::BitReader;
///Field `SPRMOD` writer - Selection of Protection Mode of nWPRi bits
pub type SPRMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Not write protect
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - Selection of Protection Mode of nWPRi bits
    #[inline(always)]
    pub fn sprmod(&self) -> SPRMOD_R {
        SPRMOD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("optstrt", &self.optstrt())
            .field("bor_lev", &self.bor_lev())
            .field("wdg_sw", &self.wdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("rdp", &self.rdp())
            .field("n_wrp", &self.n_wrp())
            .field("sprmod", &self.sprmod())
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
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    pub fn wdg_sw(&mut self) -> WDG_SW_W<OPTCRrs> {
        WDG_SW_W::new(self, 5)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTCRrs> {
        N_RST_STOP_W::new(self, 6)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTCRrs> {
        N_RST_STDBY_W::new(self, 7)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<OPTCRrs> {
        RDP_W::new(self, 8)
    }
    ///Bits 16:23 - Not write protect
    #[inline(always)]
    pub fn n_wrp(&mut self) -> N_WRP_W<OPTCRrs> {
        N_WRP_W::new(self, 16)
    }
    ///Bit 31 - Selection of Protection Mode of nWPRi bits
    #[inline(always)]
    pub fn sprmod(&mut self) -> SPRMOD_W<OPTCRrs> {
        SPRMOD_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#FLASH:OPTCR)*/
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
