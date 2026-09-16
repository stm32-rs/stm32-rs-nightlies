///Register `CRC_POL` reader
pub type R = crate::R<CRC_POLrs>;
///Register `CRC_POL` writer
pub type W = crate::W<CRC_POLrs>;
///Field `POL` reader - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
pub type POL_R = crate::FieldReader<u32>;
///Field `POL` writer - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
pub type POL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_POL").field("pol", &self.pol()).finish()
    }
}
impl W {
    ///Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, CRC_POLrs> {
        POL_W::new(self, 0)
    }
}
/**CRC polynomial

You can [`read`](crate::Reg::read) this register and get [`crc_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#CRC:CRC_POL)*/
pub struct CRC_POLrs;
impl crate::RegisterSpec for CRC_POLrs {
    type Ux = u32;
}
///`read()` method returns [`crc_pol::R`](R) reader structure
impl crate::Readable for CRC_POLrs {}
///`write(|w| ..)` method takes [`crc_pol::W`](W) writer structure
impl crate::Writable for CRC_POLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRC_POL to value 0
impl crate::Resettable for CRC_POLrs {}
