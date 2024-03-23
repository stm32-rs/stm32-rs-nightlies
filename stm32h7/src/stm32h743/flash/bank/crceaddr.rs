#[doc = "Register `CRCEADDR` reader"]
pub type R = crate::R<CRCEADDRrs>;
#[doc = "Register `CRCEADDR` writer"]
pub type W = crate::W<CRCEADDRrs>;
#[doc = "Field `CRC_END_ADDR` reader - CRC end address on bank 1"]
pub type CRC_END_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_END_ADDR` writer - CRC end address on bank 1"]
pub type CRC_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC end address on bank 1"]
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CRC_END_ADDR_R {
        CRC_END_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC end address on bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn crc_end_addr(&mut self) -> CRC_END_ADDR_W<CRCEADDRrs> {
        CRC_END_ADDR_W::new(self, 0)
    }
}
#[doc = "FLASH CRC end address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crceaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crceaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCEADDRrs;
impl crate::RegisterSpec for CRCEADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crceaddr::R`](R) reader structure"]
impl crate::Readable for CRCEADDRrs {}
#[doc = "`write(|w| ..)` method takes [`crceaddr::W`](W) writer structure"]
impl crate::Writable for CRCEADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCEADDR to value 0"]
impl crate::Resettable for CRCEADDRrs {
    const RESET_VALUE: u32 = 0;
}
