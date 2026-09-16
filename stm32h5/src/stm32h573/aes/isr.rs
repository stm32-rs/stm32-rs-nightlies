///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF {
    ///0: Computation not completed
    NotCompleted = 0,
    ///1: Computation completed
    Completed = 1,
}
impl From<CCF> for bool {
    #[inline(always)]
    fn from(variant: CCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CCF` reader - Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.
pub type CCF_R = crate::BitReader<CCF>;
impl CCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCF {
        match self.bits {
            false => CCF::NotCompleted,
            true => CCF::Completed,
        }
    }
    ///Computation not completed
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == CCF::NotCompleted
    }
    ///Computation completed
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CCF::Completed
    }
}
/**Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWEIF {
    ///0: No error detected
    NoError = 0,
    ///1: The error detected
    Error = 1,
}
impl From<RWEIF> for bool {
    #[inline(always)]
    fn from(variant: RWEIF) -> Self {
        variant as u8 != 0
    }
}
///Field `RWEIF` reader - Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected.
pub type RWEIF_R = crate::BitReader<RWEIF>;
impl RWEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWEIF {
        match self.bits {
            false => RWEIF::NoError,
            true => RWEIF::Error,
        }
    }
    ///No error detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RWEIF::NoError
    }
    ///The error detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RWEIF::Error
    }
}
///Field `KEIF` reader - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers. Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the AES_IER register is set. KEIF is triggered upon any of the following errors: AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
pub use RWEIF_R as KEIF_R;
impl R {
    ///Bit 0 - Computation complete flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCF bit of the AES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_IER register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read or write error interrupt flag This read-only bit is set by hardware when a RDERR or a WRERR error flag is set in the AES_SR register. RWEIF bit is cleared when application sets the corresponding bit of AES_ICR register. An interrupt is generated if the RWEIE bit has been previously set in the AES_IER register. This flags has no meaning when key derivation mode is selected.
    #[inline(always)]
    pub fn rweif(&self) -> RWEIF_R {
        RWEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key error interrupt flag This read-only bit is set by hardware when key information failed to load into key registers. Setting the corresponding bit of the AES_ICR register clears the KEIF and generates interrupt if the KEIE bit of the AES_IER register is set. KEIF is triggered upon any of the following errors: AES_KEYRx register write does not respect the correct order. (For KEYSIZE = 0, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 register, or reverse. For KEYSIZE = 1, AES_KEYR0 then AES_KEYR1 then AES_KEYR2 then AES_KEYR3 then AES_KEYR4 then AES_KEYR5 then AES_KEYR6 then AES_KEYR7, or reverse). KEIF must be cleared by the application software, otherwise KEYVALID cannot be set.
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#AES:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
