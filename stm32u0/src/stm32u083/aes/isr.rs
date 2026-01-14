///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `CCF` reader - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. The flag setting generates an interrupt if the CCFIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register.
pub type CCF_R = crate::BitReader;
///Field `RWEIF` reader - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the AES_SR register. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. The flags has no meaning when key derivation mode is selected. See the AES_SR register for details.
pub type RWEIF_R = crate::BitReader;
///Field `KEIF` reader - Key error interrupt flag This read-only bit is set by hardware when the key information fails to load into key registers. The flag setting generates an interrupt if the KEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. KEIF is raised upon any of the following events: AES_KEYRx register write does not respect the correct order. (For KEYSIZE1cleared, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE set, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
pub type KEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. The flag setting generates an interrupt if the CCFIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register.
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the AES_SR register. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. The flags has no meaning when key derivation mode is selected. See the AES_SR register for details.
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag This read-only bit is set by hardware when the key information fails to load into key registers. The flag setting generates an interrupt if the KEIE bit of the AES_IER register is set. The flag is cleared by setting the corresponding bit of the AES_ICR register. KEIF is raised upon any of the following events: AES_KEYRx register write does not respect the correct order. (For KEYSIZE1cleared, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE set, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("ccf", &self.ccf())
            .field("rweif", &self.rweif())
            .field("keif", &self.keif())
            .finish()
    }
}
/**AES interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
