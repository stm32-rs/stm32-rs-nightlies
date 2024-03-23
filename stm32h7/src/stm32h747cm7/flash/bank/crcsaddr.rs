#[doc = "Register `CRCSADDR` reader"]
pub type R = crate::R<CRCSADDRrs>;
#[doc = "Register `CRCSADDR` writer"]
pub type W = crate::W<CRCSADDRrs>;
#[doc = "Field `CRC_START_ADDR` reader - CRC start address on bank 1"]
pub type CRC_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_START_ADDR` writer - CRC start address on bank 1"]
pub type CRC_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 2:19 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new((self.bits >> 2) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 2:19 - CRC start address on bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W<CRCSADDRrs> {
        CRC_START_ADDR_W::new(self, 2)
    }
}
#[doc = "FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCSADDRrs;
impl crate::RegisterSpec for CRCSADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcsaddr::R`](R) reader structure"]
impl crate::Readable for CRCSADDRrs {}
#[doc = "`write(|w| ..)` method takes [`crcsaddr::W`](W) writer structure"]
impl crate::Writable for CRCSADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCSADDR to value 0"]
impl crate::Resettable for CRCSADDRrs {
    const RESET_VALUE: u32 = 0;
}
