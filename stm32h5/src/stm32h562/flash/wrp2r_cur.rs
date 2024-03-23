#[doc = "Register `WRP2R_CUR` reader"]
pub type R = crate::R<WRP2R_CURrs>;
#[doc = "Field `WRPSG2` reader - Bank2 sector group protection option status byte Each FLASH_WRP2R_CUR bit reflects the write protection status of the corresponding group of 4 consecutive sectors in bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bank2 sector group protection option status byte Each FLASH_WRP2R_CUR bit reflects the write protection status of the corresponding group of 4 consecutive sectors in bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new(self.bits)
    }
}
#[doc = "FLASH write sector group protection for Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP2R_CURrs;
impl crate::RegisterSpec for WRP2R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2r_cur::R`](R) reader structure"]
impl crate::Readable for WRP2R_CURrs {}
#[doc = "`reset()` method sets WRP2R_CUR to value 0"]
impl crate::Resettable for WRP2R_CURrs {
    const RESET_VALUE: u32 = 0;
}
