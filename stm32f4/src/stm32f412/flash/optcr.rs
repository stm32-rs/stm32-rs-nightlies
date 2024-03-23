#[doc = "Register `OPTCR` reader"]
pub type R = crate::R<OPTCRrs>;
#[doc = "Register `OPTCR` writer"]
pub type W = crate::W<OPTCRrs>;
#[doc = "Field `OPTLOCK` reader - Option lock"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Option lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Option start"]
pub type OPTSTRT_R = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Option start"]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDG_SW` reader - WDG_SW User option bytes"]
pub type WDG_SW_R = crate::BitReader;
#[doc = "Field `WDG_SW` writer - WDG_SW User option bytes"]
pub type WDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP User option bytes"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP User option bytes"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY User option bytes"]
pub type N_RST_STDBY_R = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY User option bytes"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Read protect"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protect"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `nWRP` reader - Not write protect"]
pub type N_WRP_R = crate::FieldReader<u16>;
#[doc = "Field `nWRP` writer - Not write protect"]
pub type N_WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - WDG_SW User option bytes"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - nRST_STOP User option bytes"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - nRST_STDBY User option bytes"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Option lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<OPTCRrs> {
        OPTSTRT_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - BOR reset Level"]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTCRrs> {
        BOR_LEV_W::new(self, 2)
    }
    #[doc = "Bit 5 - WDG_SW User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn wdg_sw(&mut self) -> WDG_SW_W<OPTCRrs> {
        WDG_SW_W::new(self, 5)
    }
    #[doc = "Bit 6 - nRST_STOP User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<OPTCRrs> {
        N_RST_STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - nRST_STDBY User option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<OPTCRrs> {
        N_RST_STDBY_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Read protect"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<OPTCRrs> {
        RDP_W::new(self, 8)
    }
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    #[must_use]
    pub fn n_wrp(&mut self) -> N_WRP_W<OPTCRrs> {
        N_WRP_W::new(self, 16)
    }
}
#[doc = "Flash option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr::R`](R) reader structure"]
impl crate::Readable for OPTCRrs {}
#[doc = "`write(|w| ..)` method takes [`optcr::W`](W) writer structure"]
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCR to value 0x14"]
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x14;
}
