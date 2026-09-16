///Register `CRCPR` reader
pub type R = crate::R<CRCPRrs>;
///Register `CRCPR` writer
pub type W = crate::W<CRCPRrs>;
///Field `CRCPOLY` reader - CRC polynomial register
pub type CRCPOLY_R = crate::FieldReader<u16>;
///Field `CRCPOLY` writer - CRC polynomial register
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCPR")
            .field("crcpoly", &self.crcpoly())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<'_, CRCPRrs> {
        CRCPOLY_W::new(self, 0)
    }
}
/**CRC polynomial register

You can [`read`](crate::Reg::read) this register and get [`crcpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#SPI1:CRCPR)*/
pub struct CRCPRrs;
impl crate::RegisterSpec for CRCPRrs {
    type Ux = u16;
}
///`read()` method returns [`crcpr::R`](R) reader structure
impl crate::Readable for CRCPRrs {}
///`write(|w| ..)` method takes [`crcpr::W`](W) writer structure
impl crate::Writable for CRCPRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets CRCPR to value 0x07
impl crate::Resettable for CRCPRrs {
    const RESET_VALUE: u16 = 0x07;
}
