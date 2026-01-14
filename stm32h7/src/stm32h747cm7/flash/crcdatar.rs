///Register `CRCDATAR` reader
pub type R = crate::R<CRCDATARrs>;
///Register `CRCDATAR` writer
pub type W = crate::W<CRCDATARrs>;
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
        f.debug_struct("CRCDATAR")
            .field("crc_data", &self.crc_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CRC result
    #[inline(always)]
    pub fn crc_data(&mut self) -> CRC_DATA_W<'_, CRCDATARrs> {
        CRC_DATA_W::new(self, 0)
    }
}
/**FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#FLASH:CRCDATAR)*/
pub struct CRCDATARrs;
impl crate::RegisterSpec for CRCDATARrs {
    type Ux = u32;
}
///`read()` method returns [`crcdatar::R`](R) reader structure
impl crate::Readable for CRCDATARrs {}
///`write(|w| ..)` method takes [`crcdatar::W`](W) writer structure
impl crate::Writable for CRCDATARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDATAR to value 0
impl crate::Resettable for CRCDATARrs {}
