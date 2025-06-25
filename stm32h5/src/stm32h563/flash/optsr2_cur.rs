///Register `OPTSR2_CUR` reader
pub type R = crate::R<OPTSR2_CURrs>;
///Field `SRAM13_RST` reader - SRAM1 and SRAM3 erase upon system reset
pub type SRAM13_RST_R = crate::BitReader;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable
pub type BKPRAM_ECC_R = crate::BitReader;
///Field `SRAM3_ECC` reader - SRAM3 ECC detection and correction disable
pub type SRAM3_ECC_R = crate::BitReader;
///Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable
pub type SRAM2_ECC_R = crate::BitReader;
///Field `USBPD_DIS` reader - USB power delivery configuration option bit
pub type USBPD_DIS_R = crate::BitReader;
///Field `TZEN` reader - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change.
pub type TZEN_R = crate::FieldReader;
impl R {
    ///Bit 2 - SRAM1 and SRAM3 erase upon system reset
    #[inline(always)]
    pub fn sram13_rst(&self) -> SRAM13_RST_R {
        SRAM13_RST_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bits 24:31 - TrustZone enable configuration bits This bit enables the device is in TrustZone mode during an option byte change.
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR2_CUR")
            .field("sram13_rst", &self.sram13_rst())
            .field("sram2_rst", &self.sram2_rst())
            .field("bkpram_ecc", &self.bkpram_ecc())
            .field("sram3_ecc", &self.sram3_ecc())
            .field("sram2_ecc", &self.sram2_ecc())
            .field("usbpd_dis", &self.usbpd_dis())
            .field("tzen", &self.tzen())
            .finish()
    }
}
/**FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:OPTSR2_CUR)*/
pub struct OPTSR2_CURrs;
impl crate::RegisterSpec for OPTSR2_CURrs {
    type Ux = u32;
}
///`read()` method returns [`optsr2_cur::R`](R) reader structure
impl crate::Readable for OPTSR2_CURrs {}
///`reset()` method sets OPTSR2_CUR to value 0
impl crate::Resettable for OPTSR2_CURrs {}
