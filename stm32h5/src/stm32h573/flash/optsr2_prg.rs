#[doc = "Register `OPTSR2_PRG` reader"]
pub type R = crate::R<OPTSR2_PRGrs>;
#[doc = "Register `OPTSR2_PRG` writer"]
pub type W = crate::W<OPTSR2_PRGrs>;
#[doc = "Field `SRAM1_3_RST` reader - SRAM1 and SRAM3 erase upon system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
pub type SRAM1_3_RST_R = crate::BitReader;
#[doc = "Field `SRAM1_3_RST` writer - SRAM1 and SRAM3 erase upon system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
pub type SRAM1_3_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
pub type SRAM2_RST_R = crate::BitReader;
#[doc = "Field `SRAM2_RST` writer - SRAM2 erase when system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
pub type SRAM2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_R = crate::BitReader;
#[doc = "Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3_ECC` reader - SRAM3 ECC detection and correction disable"]
pub type SRAM3_ECC_R = crate::BitReader;
#[doc = "Field `SRAM3_ECC` writer - SRAM3 ECC detection and correction disable"]
pub type SRAM3_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_R = crate::BitReader;
#[doc = "Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPD_DIS` reader - USB power delivery configuration option bit"]
pub type USBPD_DIS_R = crate::BitReader;
#[doc = "Field `USBPD_DIS` writer - USB power delivery configuration option bit"]
pub type USBPD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZEN` reader - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
pub type TZEN_R = crate::FieldReader;
#[doc = "Field `TZEN` writer - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
pub type TZEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - SRAM1 and SRAM3 erase upon system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
    #[inline(always)]
    pub fn sram1_3_rst(&self) -> SRAM1_3_RST_R {
        SRAM1_3_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM2 erase when system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM3 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram3_ecc(&self) -> SRAM3_ECC_R {
        SRAM3_ECC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USB power delivery configuration option bit"]
    #[inline(always)]
    pub fn usbpd_dis(&self) -> USBPD_DIS_R {
        USBPD_DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 24:31 - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - SRAM1 and SRAM3 erase upon system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
    #[inline(always)]
    #[must_use]
    pub fn sram1_3_rst(&mut self) -> SRAM1_3_RST_W<OPTSR2_PRGrs> {
        SRAM1_3_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM2 erase when system reset Note: SRAM erase is triggered by option byte change operation, when enabling this feature."]
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<OPTSR2_PRGrs> {
        SRAM2_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<OPTSR2_PRGrs> {
        BKPRAM_ECC_W::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM3 ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn sram3_ecc(&mut self) -> SRAM3_ECC_W<OPTSR2_PRGrs> {
        SRAM3_ECC_W::new(self, 5)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<OPTSR2_PRGrs> {
        SRAM2_ECC_W::new(self, 6)
    }
    #[doc = "Bit 8 - USB power delivery configuration option bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbpd_dis(&mut self) -> USBPD_DIS_W<OPTSR2_PRGrs> {
        USBPD_DIS_W::new(self, 8)
    }
    #[doc = "Bits 24:31 - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change."]
    #[inline(always)]
    #[must_use]
    pub fn tzen(&mut self) -> TZEN_W<OPTSR2_PRGrs> {
        TZEN_W::new(self, 24)
    }
}
#[doc = "FLASH option status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr2_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optsr2_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR2_PRGrs;
impl crate::RegisterSpec for OPTSR2_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr2_prg::R`](R) reader structure"]
impl crate::Readable for OPTSR2_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`optsr2_prg::W`](W) writer structure"]
impl crate::Writable for OPTSR2_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTSR2_PRG to value 0"]
impl crate::Resettable for OPTSR2_PRGrs {
    const RESET_VALUE: u32 = 0;
}
