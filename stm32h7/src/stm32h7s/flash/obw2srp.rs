///Register `OBW2SRP` reader
pub type R = crate::R<OBW2SRPrs>;
///Register `OBW2SRP` writer
pub type W = crate::W<OBW2SRPrs>;
///Field `ITCM_AXI_SHARE` reader - ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes
pub type ITCM_AXI_SHARE_R = crate::FieldReader;
///Field `ITCM_AXI_SHARE` writer - ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes
pub type ITCM_AXI_SHARE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DTCM_AXI_SHARE` reader - DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes
pub type DTCM_AXI_SHARE_R = crate::FieldReader;
///Field `DTCM_AXI_SHARE` writer - DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes
pub type DTCM_AXI_SHARE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ECC_ON_SRAM` reader - ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register.
pub type ECC_ON_SRAM_R = crate::BitReader;
///Field `ECC_ON_SRAM` writer - ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register.
pub type ECC_ON_SRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C_NI3C` reader - I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register.
pub type I2C_NI3C_R = crate::BitReader;
///Field `I2C_NI3C` writer - I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register.
pub type I2C_NI3C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes
    #[inline(always)]
    pub fn itcm_axi_share(&self) -> ITCM_AXI_SHARE_R {
        ITCM_AXI_SHARE_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes
    #[inline(always)]
    pub fn dtcm_axi_share(&self) -> DTCM_AXI_SHARE_R {
        DTCM_AXI_SHARE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register.
    #[inline(always)]
    pub fn ecc_on_sram(&self) -> ECC_ON_SRAM_R {
        ECC_ON_SRAM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register.
    #[inline(always)]
    pub fn i2c_ni3c(&self) -> I2C_NI3C_R {
        I2C_NI3C_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBW2SRP")
            .field("itcm_axi_share", &self.itcm_axi_share())
            .field("dtcm_axi_share", &self.dtcm_axi_share())
            .field("ecc_on_sram", &self.ecc_on_sram())
            .field("i2c_ni3c", &self.i2c_ni3c())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - ITCM AXI share programming Write to change corresponding bits in FLASH_OBW2SR register. Bit 2 should be kept to 0: ITCM_AXI_SHARE: = 000 or 011: ITCM 64 Kbytes ITCM_AXI_SHARE = 001: ITCM 128 Kbytes ITCM_AXI_SHARE = 010: ITCM 192 Kbytes
    #[inline(always)]
    pub fn itcm_axi_share(&mut self) -> ITCM_AXI_SHARE_W<'_, OBW2SRPrs> {
        ITCM_AXI_SHARE_W::new(self, 0)
    }
    ///Bits 4:6 - DTCM AXI share programming Write to change corresponding bits in the FLASH_OBW2SR register. Bit 2 should be kept to 0: DTCM_AXI_SHARE = 000 or 011: DTCM 64 Kbytes DTCM_AXI_SHARE = 001: DTCM 128 Kbytes DTCM_AXI_SHARE = 010: DTCM 192 Kbytes
    #[inline(always)]
    pub fn dtcm_axi_share(&mut self) -> DTCM_AXI_SHARE_W<'_, OBW2SRPrs> {
        DTCM_AXI_SHARE_W::new(self, 4)
    }
    ///Bit 8 - ECC on SRAM programming Write to change corresponding bit in FLASH_OBW2SR register.
    #[inline(always)]
    pub fn ecc_on_sram(&mut self) -> ECC_ON_SRAM_W<'_, OBW2SRPrs> {
        ECC_ON_SRAM_W::new(self, 8)
    }
    ///Bit 9 - I2C Not I3C Write to change corresponding bit in FLASH_OBW2SR register.
    #[inline(always)]
    pub fn i2c_ni3c(&mut self) -> I2C_NI3C_W<'_, OBW2SRPrs> {
        I2C_NI3C_W::new(self, 9)
    }
}
/**FLASH option byte word 2 status register programming

You can [`read`](crate::Reg::read) this register and get [`obw2srp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obw2srp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OBW2SRP)*/
pub struct OBW2SRPrs;
impl crate::RegisterSpec for OBW2SRPrs {
    type Ux = u32;
}
///`read()` method returns [`obw2srp::R`](R) reader structure
impl crate::Readable for OBW2SRPrs {}
///`write(|w| ..)` method takes [`obw2srp::W`](W) writer structure
impl crate::Writable for OBW2SRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OBW2SRP to value 0
impl crate::Resettable for OBW2SRPrs {}
