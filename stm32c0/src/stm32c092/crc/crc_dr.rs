///Register `CRC_DR` reader
pub type R = crate::R<CRC_DRrs>;
///Register `CRC_DR` writer
pub type W = crate::W<CRC_DRrs>;
///Field `DR` reader - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value.
pub type DR_R = crate::FieldReader<u32>;
///Field `DR` writer - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value.
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_DR").field("dr", &self.dr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value.
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, CRC_DRrs> {
        DR_W::new(self, 0)
    }
}
/**CRC data register

You can [`read`](crate::Reg::read) this register and get [`crc_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#CRC:CRC_DR)*/
pub struct CRC_DRrs;
impl crate::RegisterSpec for CRC_DRrs {
    type Ux = u32;
}
///`read()` method returns [`crc_dr::R`](R) reader structure
impl crate::Readable for CRC_DRrs {}
///`write(|w| ..)` method takes [`crc_dr::W`](W) writer structure
impl crate::Writable for CRC_DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRC_DR to value 0xffff_ffff
impl crate::Resettable for CRC_DRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
