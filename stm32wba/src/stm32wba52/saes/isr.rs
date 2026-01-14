///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `CCF` reader - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. CCF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the CCFIE bit has been previously set in the AES_IER register.
pub type CCF_R = crate::BitReader;
///Field `RWEIF` reader - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the SAES_SR register. RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the SAES_IER register. This flags has no meaning when key derivation mode is selected.
pub type RWEIF_R = crate::BitReader;
///Field `KEIF` reader - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers or key register usage is forbidden. Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the SAES_IER register is set. KEIF is triggered upon any of the following errors: SAES fails to load the DHUK (KEYSEL\[2:0\]=001 or 100). SAES fails to load the BHK (KEYSEL\[2:0\]=010 or 100) respecting the correct order. AES fails to load the key shared by SAES peripheral (KMOD=10). When KEYVALID is set and either KEYPROT is set or KEYSEL\[2:0\] is other than 000, the security context of the application that loads the key (secure or non-secure) does not match the security attribute of the access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared. SAES_KEYRx register write does not respect the correct order. (For KEYSIZE=0, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For KEYSIZE set, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
pub type KEIF_R = crate::BitReader;
///Field `RNGEIF` reader - RNG error interrupt flag This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad entropy). RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing this bit triggers the reload of a new random number from RNG peripheral.
pub type RNGEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Computation complete flag This flag indicates whether the computation is completed. It is significant only when the DMAOUTEN bit is cleared, and it may stay high when DMAOUTEN is set. CCF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the CCFIE bit has been previously set in the AES_IER register.
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag This read-only bit is set by hardware when a RDERRF or a WRERRF error flag is set in the SAES_SR register. RWEIF bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the SAES_IER register. This flags has no meaning when key derivation mode is selected.
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers or key register usage is forbidden. Setting the corresponding bit of the SAES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the SAES_IER register is set. KEIF is triggered upon any of the following errors: SAES fails to load the DHUK (KEYSEL\[2:0\]=001 or 100). SAES fails to load the BHK (KEYSEL\[2:0\]=010 or 100) respecting the correct order. AES fails to load the key shared by SAES peripheral (KMOD=10). When KEYVALID is set and either KEYPROT is set or KEYSEL\[2:0\] is other than 000, the security context of the application that loads the key (secure or non-secure) does not match the security attribute of the access to SAES_CR or SAES_DOUT. In this case, KEYVALID and EN bits are cleared. SAES_KEYRx register write does not respect the correct order. (For KEYSIZE=0, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 register, or reverse. For KEYSIZE set, SAES_KEYR0 then SAES_KEYR1 then SAES_KEYR2 then SAES_KEYR3 then SAES_KEYR4 then SAES_KEYR5 then SAES_KEYR6 then SAES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNG error interrupt flag This read-only bit is set by hardware when an error is detected on RNG bus interface (e.g. bad entropy). RNGEIE bit is cleared when application sets the corresponding bit of SAES_ICR register. An interrupt is generated if the RNGEIE bit has been previously set in the SAES_IER register. Clearing this bit triggers the reload of a new random number from RNG peripheral.
    #[inline(always)]
    pub fn rngeif(&self) -> RNGEIF_R {
        RNGEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("ccf", &self.ccf())
            .field("rweif", &self.rweif())
            .field("keif", &self.keif())
            .field("rngeif", &self.rngeif())
            .finish()
    }
}
/**SAES interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#SAES:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
