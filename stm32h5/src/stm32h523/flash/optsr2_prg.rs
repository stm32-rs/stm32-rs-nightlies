///Register `OPTSR2_PRG` reader
pub type R = crate::R<OPTSR2_PRGrs>;
///Register `OPTSR2_PRG` writer
pub type W = crate::W<OPTSR2_PRGrs>;
///Field `SRAM1_3_RST` reader - SRAM1 and SRAM3 erase upon system reset
pub type SRAM1_3_RST_R = crate::BitReader;
///Field `SRAM1_3_RST` writer - SRAM1 and SRAM3 erase upon system reset
pub type SRAM1_3_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `SRAM2_RST` writer - SRAM2 erase when system reset
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable
pub type BKPRAM_ECC_R = crate::BitReader;
///Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction disable
pub type BKPRAM_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM3_ECC` reader - SRAM3 ECC detection and correction disable
pub type SRAM3_ECC_R = crate::BitReader;
///Field `SRAM3_ECC` writer - SRAM3 ECC detection and correction disable
pub type SRAM3_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable
pub type SRAM2_ECC_R = crate::BitReader;
///Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction disable
pub type SRAM2_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBPD_DIS` reader - USB power delivery configuration option bit
pub type USBPD_DIS_R = crate::BitReader;
///Field `USBPD_DIS` writer - USB power delivery configuration option bit
pub type USBPD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZEN` reader - TrustZone enable configuration bits
pub type TZEN_R = crate::FieldReader;
///Field `TZEN` writer - TrustZone enable configuration bits
pub type TZEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 2 - SRAM1 and SRAM3 erase upon system reset
    #[inline(always)]
    pub fn sram1_3_rst(&self) -> SRAM1_3_RST_R {
        SRAM1_3_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Backup RAM ECC detection and correction disable
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM3 ECC detection and correction disable
    #[inline(always)]
    pub fn sram3_ecc(&self) -> SRAM3_ECC_R {
        SRAM3_ECC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 ECC detection and correction disable
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - USB power delivery configuration option bit
    #[inline(always)]
    pub fn usbpd_dis(&self) -> USBPD_DIS_R {
        USBPD_DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 24:31 - TrustZone enable configuration bits
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR2_PRG")
            .field("sram1_3_rst", &self.sram1_3_rst())
            .field("sram2_rst", &self.sram2_rst())
            .field("bkpram_ecc", &self.bkpram_ecc())
            .field("sram3_ecc", &self.sram3_ecc())
            .field("sram2_ecc", &self.sram2_ecc())
            .field("usbpd_dis", &self.usbpd_dis())
            .field("tzen", &self.tzen())
            .finish()
    }
}
impl W {
    ///Bit 2 - SRAM1 and SRAM3 erase upon system reset
    #[inline(always)]
    pub fn sram1_3_rst(&mut self) -> SRAM1_3_RST_W<'_, OPTSR2_PRGrs> {
        SRAM1_3_RST_W::new(self, 2)
    }
    ///Bit 3 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<'_, OPTSR2_PRGrs> {
        SRAM2_RST_W::new(self, 3)
    }
    ///Bit 4 - Backup RAM ECC detection and correction disable
    #[inline(always)]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<'_, OPTSR2_PRGrs> {
        BKPRAM_ECC_W::new(self, 4)
    }
    ///Bit 5 - SRAM3 ECC detection and correction disable
    #[inline(always)]
    pub fn sram3_ecc(&mut self) -> SRAM3_ECC_W<'_, OPTSR2_PRGrs> {
        SRAM3_ECC_W::new(self, 5)
    }
    ///Bit 6 - SRAM2 ECC detection and correction disable
    #[inline(always)]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<'_, OPTSR2_PRGrs> {
        SRAM2_ECC_W::new(self, 6)
    }
    ///Bit 8 - USB power delivery configuration option bit
    #[inline(always)]
    pub fn usbpd_dis(&mut self) -> USBPD_DIS_W<'_, OPTSR2_PRGrs> {
        USBPD_DIS_W::new(self, 8)
    }
    ///Bits 24:31 - TrustZone enable configuration bits
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W<'_, OPTSR2_PRGrs> {
        TZEN_W::new(self, 24)
    }
}
/**FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:OPTSR2_PRG)*/
pub struct OPTSR2_PRGrs;
impl crate::RegisterSpec for OPTSR2_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`optsr2_prg::R`](R) reader structure
impl crate::Readable for OPTSR2_PRGrs {}
///`write(|w| ..)` method takes [`optsr2_prg::W`](W) writer structure
impl crate::Writable for OPTSR2_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR2_PRG to value 0
impl crate::Resettable for OPTSR2_PRGrs {}
