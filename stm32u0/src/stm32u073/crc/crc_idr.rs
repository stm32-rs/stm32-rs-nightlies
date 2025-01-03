///Register `CRC_IDR` reader
pub type R = crate::R<CRC_IDRrs>;
///Register `CRC_IDR` writer
pub type W = crate::W<CRC_IDRrs>;
///Field `IDR` reader - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
pub type IDR_R = crate::FieldReader<u32>;
///Field `IDR` writer - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
pub type IDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_IDR").field("idr", &self.idr()).finish()
    }
}
impl W {
    ///Bits 0:31 - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W<CRC_IDRrs> {
        IDR_W::new(self, 0)
    }
}
/**CRC independent data register

You can [`read`](crate::Reg::read) this register and get [`crc_idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRC:CRC_IDR)*/
pub struct CRC_IDRrs;
impl crate::RegisterSpec for CRC_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`crc_idr::R`](R) reader structure
impl crate::Readable for CRC_IDRrs {}
///`write(|w| ..)` method takes [`crc_idr::W`](W) writer structure
impl crate::Writable for CRC_IDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRC_IDR to value 0
impl crate::Resettable for CRC_IDRrs {
    const RESET_VALUE: u32 = 0;
}
