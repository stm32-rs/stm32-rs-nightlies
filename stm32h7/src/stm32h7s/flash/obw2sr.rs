///Register `OBW2SR` reader
pub type R = crate::R<OBW2SRrs>;
///Field `ITCM_AXI_SHARE` reader - ITCM SRAM configuration
pub type ITCM_AXI_SHARE_R = crate::FieldReader;
///Field `DTCM_AXI_SHARE` reader - DTCM SRAM configuration
pub type DTCM_AXI_SHARE_R = crate::FieldReader;
///Field `ECC_ON_SRAM` reader - ECC on SRAM
pub type ECC_ON_SRAM_R = crate::BitReader;
///Field `I2C_NI3C` reader - I2C Not I3C
pub type I2C_NI3C_R = crate::BitReader;
impl R {
    ///Bits 0:2 - ITCM SRAM configuration
    #[inline(always)]
    pub fn itcm_axi_share(&self) -> ITCM_AXI_SHARE_R {
        ITCM_AXI_SHARE_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - DTCM SRAM configuration
    #[inline(always)]
    pub fn dtcm_axi_share(&self) -> DTCM_AXI_SHARE_R {
        DTCM_AXI_SHARE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - ECC on SRAM
    #[inline(always)]
    pub fn ecc_on_sram(&self) -> ECC_ON_SRAM_R {
        ECC_ON_SRAM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I2C Not I3C
    #[inline(always)]
    pub fn i2c_ni3c(&self) -> I2C_NI3C_R {
        I2C_NI3C_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBW2SR")
            .field("itcm_axi_share", &self.itcm_axi_share())
            .field("dtcm_axi_share", &self.dtcm_axi_share())
            .field("ecc_on_sram", &self.ecc_on_sram())
            .field("i2c_ni3c", &self.i2c_ni3c())
            .finish()
    }
}
/**FLASH option byte word 2 status register

You can [`read`](crate::Reg::read) this register and get [`obw2sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OBW2SR)*/
pub struct OBW2SRrs;
impl crate::RegisterSpec for OBW2SRrs {
    type Ux = u32;
}
///`read()` method returns [`obw2sr::R`](R) reader structure
impl crate::Readable for OBW2SRrs {}
///`reset()` method sets OBW2SR to value 0
impl crate::Resettable for OBW2SRrs {}
