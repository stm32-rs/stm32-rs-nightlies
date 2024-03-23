#[doc = "Register `CRCDATAR` reader"]
pub type R = crate::R<CRCDATARrs>;
#[doc = "Register `CRCDATAR` writer"]
pub type W = crate::W<CRCDATARrs>;
#[doc = "Field `CRC_DATA` reader - CRC result"]
pub type CRC_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_DATA` writer - CRC result"]
pub type CRC_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    #[must_use]
    pub fn crc_data(&mut self) -> CRC_DATA_W<CRCDATARrs> {
        CRC_DATA_W::new(self, 0)
    }
}
#[doc = "FLASH CRC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCDATARrs;
impl crate::RegisterSpec for CRCDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdatar::R`](R) reader structure"]
impl crate::Readable for CRCDATARrs {}
#[doc = "`write(|w| ..)` method takes [`crcdatar::W`](W) writer structure"]
impl crate::Writable for CRCDATARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCDATAR to value 0"]
impl crate::Resettable for CRCDATARrs {
    const RESET_VALUE: u32 = 0;
}
