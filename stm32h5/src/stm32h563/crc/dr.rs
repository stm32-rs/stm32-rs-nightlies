#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `DR` reader - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
pub type DR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data register bits This register is used to write new data to the CRC calculator. It holds the previous CRC calculation result when it is read. If the data size is less than 32 bits, the least significant bits are used to write/read the correct value."]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DRrs> {
        DR_W::new(self, 0)
    }
}
#[doc = "CRC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0xffff_ffff"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
