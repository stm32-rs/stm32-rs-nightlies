///Register `CRCEADDR` reader
pub type R = crate::R<CRCEADDRrs>;
///Register `CRCEADDR` writer
pub type W = crate::W<CRCEADDRrs>;
///Field `CRC_END_ADDR` reader - CRC end address on bank 1
pub type CRC_END_ADDR_R = crate::FieldReader<u32>;
///Field `CRC_END_ADDR` writer - CRC end address on bank 1
pub type CRC_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 2:19 - CRC end address on bank 1
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CRC_END_ADDR_R {
        CRC_END_ADDR_R::new((self.bits >> 2) & 0x0003_ffff)
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
    ///Bits 2:19 - CRC end address on bank 1
    #[inline(always)]
    pub fn crc_end_addr(&mut self) -> CRC_END_ADDR_W<'_, CRCEADDRrs> {
        CRC_END_ADDR_W::new(self, 2)
    }
}
/**FLASH CRC end address register for bank 1

You can [`read`](crate::Reg::read) this register and get [`crceaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
