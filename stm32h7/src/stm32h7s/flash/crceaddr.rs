///Register `CRCEADDR` reader
pub type R = crate::R<CRCEADDRrs>;
///Register `CRCEADDR` writer
pub type W = crate::W<CRCEADDRrs>;
///Field `CRC_END_ADDR` reader - CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
pub type CRC_END_ADDR_R = crate::FieldReader<u16>;
///Field `CRC_END_ADDR` writer - CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
pub type CRC_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 6:16 - CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CRC_END_ADDR_R {
        CRC_END_ADDR_R::new(((self.bits >> 6) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCEADDR")
            .field("crc_end_addr", &self.crc_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 6:16 - CRC end address This register is used when CRC_BY_SECT is cleared. It must be programmed to the address of the Flash word starting the last burst of the CRC calculation. The burst size is defined in CRC_BURST of FLASH_CRCCR register. The least significant bits \[5:0\] of the address are set by hardware to 0 (minimum burst size= 64 bytes). The address is relative to the Flash bank.
    #[inline(always)]
    pub fn crc_end_addr(&mut self) -> CRC_END_ADDR_W<'_, CRCEADDRrs> {
        CRC_END_ADDR_W::new(self, 6)
    }
}
/**FLASH CRC end address register

You can [`read`](crate::Reg::read) this register and get [`crceaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:CRCEADDR)*/
pub struct CRCEADDRrs;
impl crate::RegisterSpec for CRCEADDRrs {
    type Ux = u32;
}
///`read()` method returns [`crceaddr::R`](R) reader structure
impl crate::Readable for CRCEADDRrs {}
///`write(|w| ..)` method takes [`crceaddr::W`](W) writer structure
impl crate::Writable for CRCEADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCEADDR to value 0
impl crate::Resettable for CRCEADDRrs {}
