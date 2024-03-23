#[doc = "Register `OPTSR_PRG` reader"]
pub type R = crate::R<OPTSR_PRGrs>;
#[doc = "Register `OPTSR_PRG` writer"]
pub type W = crate::W<OPTSR_PRGrs>;
#[doc = "Field `BOR_LEV` reader - Brownout level option configuration bit These bits reflects the power level that generates a system reset."]
pub type BOR_LEV_R = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - Brownout level option configuration bit These bits reflects the power level that generates a system reset."]
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BORH_EN` reader - Brownout high enable configuration bit"]
pub type BORH_EN_R = crate::BitReader;
#[doc = "Field `BORH_EN` writer - Brownout high enable configuration bit"]
pub type BORH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_SW` reader - IWDG control mode option configuration bit"]
pub type IWDG_SW_R = crate::BitReader;
#[doc = "Field `IWDG_SW` writer - IWDG control mode option configuration bit"]
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - WWDG control mode option configuration bit"]
pub type WWDG_SW_R = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - WWDG control mode option configuration bit"]
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_SHDW` reader - Core domain Shutdown entry reset option configuration bit"]
pub type NRST_SHDW_R = crate::BitReader;
#[doc = "Field `NRST_SHDW` writer - Core domain Shutdown entry reset option configuration bit"]
pub type NRST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STOP` reader - Core domain Stop entry reset option configuration bit"]
pub type NRST_STOP_R = crate::BitReader;
#[doc = "Field `NRST_STOP` writer - Core domain Stop entry reset option configuration bit"]
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_STDBY` reader - Core domain Standby entry reset option configuration bit"]
pub type NRST_STDBY_R = crate::BitReader;
#[doc = "Field `NRST_STDBY` writer - Core domain Standby entry reset option configuration bit"]
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRODUCT_STATE` reader - Life state code (based on Hamming 8,4). More information in ."]
pub type PRODUCT_STATE_R = crate::FieldReader;
#[doc = "Field `PRODUCT_STATE` writer - Life state code (based on Hamming 8,4). More information in ."]
pub type PRODUCT_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage configuration bit. This bit can be set only with VDD below 2.5 V."]
pub type IO_VDD_HSLV_R = crate::BitReader;
#[doc = "Field `IO_VDD_HSLV` writer - High-speed IO at low VDD voltage configuration bit. This bit can be set only with VDD below 2.5 V."]
pub type IO_VDD_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage configuration bit. This bit can be set only with VDDIO2 below 2.5 V."]
pub type IO_VDDIO2_HSLV_R = crate::BitReader;
#[doc = "Field `IO_VDDIO2_HSLV` writer - High-speed IO at low VDDIO2 voltage configuration bit. This bit can be set only with VDDIO2 below 2.5 V."]
pub type IO_VDDIO2_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STOP` reader - IWDG Stop mode freeze option configuration bit When set the independent watchdog IWDG is in system Stop mode."]
pub type IWDG_STOP_R = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - IWDG Stop mode freeze option configuration bit When set the independent watchdog IWDG is in system Stop mode."]
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - IWDG Standby mode freeze option configuration bit When set the independent watchdog IWDG is frozen in system Standby mode."]
pub type IWDG_STDBY_R = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - IWDG Standby mode freeze option configuration bit When set the independent watchdog IWDG is frozen in system Standby mode."]
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping option configuration bit SWAP_BANK option bit is used to configure whether the Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register after a reset."]
pub type SWAP_BANK_R = crate::BitReader;
#[doc = "Field `SWAP_BANK` writer - Bank swapping option configuration bit SWAP_BANK option bit is used to configure whether the Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register after a reset."]
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Brownout level option configuration bit These bits reflects the power level that generates a system reset."]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Brownout high enable configuration bit"]
    #[inline(always)]
    pub fn borh_en(&self) -> BORH_EN_R {
        BORH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IWDG control mode option configuration bit"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WWDG control mode option configuration bit"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core domain Shutdown entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core domain Stop entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core domain Standby entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Life state code (based on Hamming 8,4). More information in ."]
    #[inline(always)]
    pub fn product_state(&self) -> PRODUCT_STATE_R {
        PRODUCT_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - High-speed IO at low VDD voltage configuration bit. This bit can be set only with VDD below 2.5 V."]
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High-speed IO at low VDDIO2 voltage configuration bit. This bit can be set only with VDDIO2 below 2.5 V."]
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - IWDG Stop mode freeze option configuration bit When set the independent watchdog IWDG is in system Stop mode."]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IWDG Standby mode freeze option configuration bit When set the independent watchdog IWDG is frozen in system Standby mode."]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit SWAP_BANK option bit is used to configure whether the Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register after a reset."]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Brownout level option configuration bit These bits reflects the power level that generates a system reset."]
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<OPTSR_PRGrs> {
        BOR_LEV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Brownout high enable configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn borh_en(&mut self) -> BORH_EN_W<OPTSR_PRGrs> {
        BORH_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IWDG control mode option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTSR_PRGrs> {
        IWDG_SW_W::new(self, 3)
    }
    #[doc = "Bit 4 - WWDG control mode option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTSR_PRGrs> {
        WWDG_SW_W::new(self, 4)
    }
    #[doc = "Bit 5 - Core domain Shutdown entry reset option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_shdw(&mut self) -> NRST_SHDW_W<OPTSR_PRGrs> {
        NRST_SHDW_W::new(self, 5)
    }
    #[doc = "Bit 6 - Core domain Stop entry reset option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<OPTSR_PRGrs> {
        NRST_STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Core domain Standby entry reset option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<OPTSR_PRGrs> {
        NRST_STDBY_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Life state code (based on Hamming 8,4). More information in ."]
    #[inline(always)]
    #[must_use]
    pub fn product_state(&mut self) -> PRODUCT_STATE_W<OPTSR_PRGrs> {
        PRODUCT_STATE_W::new(self, 8)
    }
    #[doc = "Bit 16 - High-speed IO at low VDD voltage configuration bit. This bit can be set only with VDD below 2.5 V."]
    #[inline(always)]
    #[must_use]
    pub fn io_vdd_hslv(&mut self) -> IO_VDD_HSLV_W<OPTSR_PRGrs> {
        IO_VDD_HSLV_W::new(self, 16)
    }
    #[doc = "Bit 17 - High-speed IO at low VDDIO2 voltage configuration bit. This bit can be set only with VDDIO2 below 2.5 V."]
    #[inline(always)]
    #[must_use]
    pub fn io_vddio2_hslv(&mut self) -> IO_VDDIO2_HSLV_W<OPTSR_PRGrs> {
        IO_VDDIO2_HSLV_W::new(self, 17)
    }
    #[doc = "Bit 20 - IWDG Stop mode freeze option configuration bit When set the independent watchdog IWDG is in system Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTSR_PRGrs> {
        IWDG_STOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - IWDG Standby mode freeze option configuration bit When set the independent watchdog IWDG is frozen in system Standby mode."]
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<OPTSR_PRGrs> {
        IWDG_STDBY_W::new(self, 21)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit SWAP_BANK option bit is used to configure whether the Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register after a reset."]
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<OPTSR_PRGrs> {
        SWAP_BANK_W::new(self, 31)
    }
}
#[doc = "FLASH option status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR_PRGrs;
impl crate::RegisterSpec for OPTSR_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr_prg::R`](R) reader structure"]
impl crate::Readable for OPTSR_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`optsr_prg::W`](W) writer structure"]
impl crate::Writable for OPTSR_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTSR_PRG to value 0"]
impl crate::Resettable for OPTSR_PRGrs {
    const RESET_VALUE: u32 = 0;
}
