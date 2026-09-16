///Register `CRCDATAR_` reader
pub type R = crate::R<CRCDATAR_rs>;
///Register `CRCDATAR_` writer
pub type W = crate::W<CRCDATAR_rs>;
///Field `CRC_DATA` reader - CRC result
pub type CRC_DATA_R = crate::FieldReader<u32>;
///Field `CRC_DATA` writer - CRC result
pub type CRC_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC result
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDATAR_")
            .field("crc_data", &self.crc_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC result
    #[inline(always)]
    pub fn crc_data(&mut self) -> CRC_DATA_W<'_, CRCDATAR_rs> {
        CRC_DATA_W::new(self, 0)
    }
}
/**FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#FLASH:CRCDATAR_)*/
pub struct CRCDATAR_rs;
impl crate::RegisterSpec for CRCDATAR_rs {
    type Ux = u32;
}
///`read()` method returns [`crcdatar_::R`](R) reader structure
impl crate::Readable for CRCDATAR_rs {}
///`write(|w| ..)` method takes [`crcdatar_::W`](W) writer structure
impl crate::Writable for CRCDATAR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDATAR_ to value 0
impl crate::Resettable for CRCDATAR_rs {}
