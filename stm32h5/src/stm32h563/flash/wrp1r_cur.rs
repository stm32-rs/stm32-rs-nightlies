#[doc = "Register `WRP1R_CUR` reader"]
pub type R = crate::R<WRP1R_CURrs>;
#[doc = "Field `WRPSG1` reader - Bank 1 sector group protection option status byte Each FLASH_WRP1R_CUR bit reflects the write protection status of the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 1 sector group protection option status byte Each FLASH_WRP1R_CUR bit reflects the write protection status of the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg1(&self) -> WRPSG1_R {
        WRPSG1_R::new(self.bits)
    }
}
#[doc = "FLASH write sector group protection for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1R_CURrs;
impl crate::RegisterSpec for WRP1R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1r_cur::R`](R) reader structure"]
impl crate::Readable for WRP1R_CURrs {}
#[doc = "`reset()` method sets WRP1R_CUR to value 0"]
impl crate::Resettable for WRP1R_CURrs {
    const RESET_VALUE: u32 = 0;
}
