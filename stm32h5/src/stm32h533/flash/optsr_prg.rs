///Register `OPTSR_PRG` reader
pub type R = crate::R<OPTSR_PRGrs>;
///Register `OPTSR_PRG` writer
pub type W = crate::W<OPTSR_PRGrs>;
///Field `BOR_LEV` reader - Brownout level option configuration bit
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - Brownout level option configuration bit
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BORH_EN` reader - Brownout high enable configuration bit
pub type BORH_EN_R = crate::BitReader;
///Field `BORH_EN` writer - Brownout high enable configuration bit
pub type BORH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - IWDG control mode option configuration bit
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - IWDG control mode option configuration bit
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - WWDG control mode option configuration bit
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - WWDG control mode option configuration bit
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP` reader - Core domain Stop entry reset option configuration bit
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - Core domain Stop entry reset option configuration bit
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDBY` reader - Core domain Standby entry reset option configuration bit
pub type NRST_STDBY_R = crate::BitReader;
///Field `NRST_STDBY` writer - Core domain Standby entry reset option configuration bit
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRODUCT_STATE` reader - Life state code (based on Hamming 8,4).
pub type PRODUCT_STATE_R = crate::FieldReader;
///Field `PRODUCT_STATE` writer - Life state code (based on Hamming 8,4).
pub type PRODUCT_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage configuration bit.
pub type IO_VDD_HSLV_R = crate::BitReader;
///Field `IO_VDD_HSLV` writer - High-speed IO at low VDD voltage configuration bit.
pub type IO_VDD_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IO_VDDIO2_HSLV` reader - High-speed IO at low Vless thansub>DDIO2less than/sub> voltage configuration bit.
pub type IO_VDDIO2_HSLV_R = crate::BitReader;
///Field `IO_VDDIO2_HSLV` writer - High-speed IO at low Vless thansub>DDIO2less than/sub> voltage configuration bit.
pub type IO_VDDIO2_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - IWDG Stop mode freeze option status bit
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - IWDG Stop mode freeze option status bit
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - IWDG Standby mode freeze option status bit
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - IWDG Standby mode freeze option status bit
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT_UBE` reader - Available only on cryptography enabled devices.
pub type BOOT_UBE_R = crate::FieldReader;
///Field `BOOT_UBE` writer - Available only on cryptography enabled devices.
pub type BOOT_UBE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SWAP_BANK` reader - Bank swapping option configuration bit
pub type SWAP_BANK_R = crate::BitReader;
///Field `SWAP_BANK` writer - Bank swapping option configuration bit
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Brownout level option configuration bit
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Brownout high enable configuration bit
    #[inline(always)]
    pub fn borh_en(&self) -> BORH_EN_R {
        BORH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IWDG control mode option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WWDG control mode option configuration bit
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Core domain Stop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Core domain Standby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Life state code (based on Hamming 8,4).
    #[inline(always)]
    pub fn product_state(&self) -> PRODUCT_STATE_R {
        PRODUCT_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - High-speed IO at low VDD voltage configuration bit.
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - High-speed IO at low Vless thansub>DDIO2less than/sub> voltage configuration bit.
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:29 - Available only on cryptography enabled devices.
    #[inline(always)]
    pub fn boot_ube(&self) -> BOOT_UBE_R {
        BOOT_UBE_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR_PRG")
            .field("bor_lev", &self.bor_lev())
            .field("borh_en", &self.borh_en())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("product_state", &self.product_state())
            .field("io_vdd_hslv", &self.io_vdd_hslv())
            .field("io_vddio2_hslv", &self.io_vddio2_hslv())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("boot_ube", &self.boot_ube())
            .field("swap_bank", &self.swap_bank())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Brownout level option configuration bit
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OPTSR_PRGrs> {
        BOR_LEV_W::new(self, 0)
    }
    ///Bit 2 - Brownout high enable configuration bit
    #[inline(always)]
    pub fn borh_en(&mut self) -> BORH_EN_W<'_, OPTSR_PRGrs> {
        BORH_EN_W::new(self, 2)
    }
    ///Bit 3 - IWDG control mode option configuration bit
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTSR_PRGrs> {
        IWDG_SW_W::new(self, 3)
    }
    ///Bit 4 - WWDG control mode option configuration bit
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTSR_PRGrs> {
        WWDG_SW_W::new(self, 4)
    }
    ///Bit 6 - Core domain Stop entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTSR_PRGrs> {
        NRST_STOP_W::new(self, 6)
    }
    ///Bit 7 - Core domain Standby entry reset option configuration bit
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<'_, OPTSR_PRGrs> {
        NRST_STDBY_W::new(self, 7)
    }
    ///Bits 8:15 - Life state code (based on Hamming 8,4).
    #[inline(always)]
    pub fn product_state(&mut self) -> PRODUCT_STATE_W<'_, OPTSR_PRGrs> {
        PRODUCT_STATE_W::new(self, 8)
    }
    ///Bit 16 - High-speed IO at low VDD voltage configuration bit.
    #[inline(always)]
    pub fn io_vdd_hslv(&mut self) -> IO_VDD_HSLV_W<'_, OPTSR_PRGrs> {
        IO_VDD_HSLV_W::new(self, 16)
    }
    ///Bit 17 - High-speed IO at low Vless thansub>DDIO2less than/sub> voltage configuration bit.
    #[inline(always)]
    pub fn io_vddio2_hslv(&mut self) -> IO_VDDIO2_HSLV_W<'_, OPTSR_PRGrs> {
        IO_VDDIO2_HSLV_W::new(self, 17)
    }
    ///Bit 20 - IWDG Stop mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTSR_PRGrs> {
        IWDG_STOP_W::new(self, 20)
    }
    ///Bit 21 - IWDG Standby mode freeze option status bit
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<'_, OPTSR_PRGrs> {
        IWDG_STDBY_W::new(self, 21)
    }
    ///Bits 22:29 - Available only on cryptography enabled devices.
    #[inline(always)]
    pub fn boot_ube(&mut self) -> BOOT_UBE_W<'_, OPTSR_PRGrs> {
        BOOT_UBE_W::new(self, 22)
    }
    ///Bit 31 - Bank swapping option configuration bit
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<'_, OPTSR_PRGrs> {
        SWAP_BANK_W::new(self, 31)
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:OPTSR_PRG)*/
pub struct OPTSR_PRGrs;
impl crate::RegisterSpec for OPTSR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`optsr_prg::R`](R) reader structure
impl crate::Readable for OPTSR_PRGrs {}
///`write(|w| ..)` method takes [`optsr_prg::W`](W) writer structure
impl crate::Writable for OPTSR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR_PRG to value 0
impl crate::Resettable for OPTSR_PRGrs {}
