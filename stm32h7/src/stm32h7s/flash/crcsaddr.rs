///Register `CRCSADDR` reader
pub type R = crate::R<CRCSADDRrs>;
///Register `CRCSADDR` writer
pub type W = crate::W<CRCSADDRrs>;
///Field `CRC_START_ADDR` reader - CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
pub type CRC_START_ADDR_R = crate::FieldReader<u16>;
///Field `CRC_START_ADDR` writer - CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
pub type CRC_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 6:16 - CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCSADDR")
            .field("crc_start_addr", &self.crc_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 6:16 - CRC start address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the first Flash word to use for the CRC calculation, done burst by burst. CRC computation starts at an address aligned to the burst size defined in CRC_BURST of FLASH_CRCCR register. Hence least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
    #[inline(always)]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W<'_, CRCSADDRrs> {
        CRC_START_ADDR_W::new(self, 6)
    }
}
/**FLASH CRC start address register

You can [`read`](crate::Reg::read) this register and get [`crcsaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:CRCSADDR)*/
pub struct CRCSADDRrs;
impl crate::RegisterSpec for CRCSADDRrs {
    type Ux = u32;
}
///`read()` method returns [`crcsaddr::R`](R) reader structure
impl crate::Readable for CRCSADDRrs {}
///`write(|w| ..)` method takes [`crcsaddr::W`](W) writer structure
impl crate::Writable for CRCSADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCSADDR to value 0
impl crate::Resettable for CRCSADDRrs {}
