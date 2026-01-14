///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RDERRF` reader - Read error flag This bit is set when an unexpected read to the AES_DOUTR register occurred. When set RDERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.
pub type RDERRF_R = crate::BitReader;
///Field `WRERRF` reader - Write error flag This bit is set when an unexpected write to the AES_DINR register occurred. When set WRERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.
pub type WRERRF_R = crate::BitReader;
///Field `BUSY` reader - Busy This flag indicates whether AES is idle or busy. AES is flagged as idle when disabled (when EN is low) or when the last processing is completed. AES is flagged as busy when processing a block data, preparing a key (ECB or CBC decryption only). When GCM encryption is selected, this flag must be at zero before suspending current process to manage a higher-priority message.
pub type BUSY_R = crate::BitReader;
///Field `KEYVALID` reader - Key valid flag This bit is set by hardware when the key of size defined by KEYSIZE is loaded in AES_KEYRx key registers. The EN bit can only be set when KEYVALID is set. The key must be written in the key registers in the correct sequence, otherwise the KEIF flag is set and KEYVALID remains cleared. If set, KEIF must be cleared through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF flag description for more details. For further information on key loading, refer to Section121.4.15: AES key registers.
pub type KEYVALID_R = crate::BitReader;
impl R {
    ///Bit 1 - Read error flag This bit is set when an unexpected read to the AES_DOUTR register occurred. When set RDERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.
    #[inline(always)]
    pub fn rderrf(&self) -> RDERRF_R {
        RDERRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write error flag This bit is set when an unexpected write to the AES_DINR register occurred. When set WRERRF bit has no impact on the AES operations. The flag setting generates an interrupt if the RWEIE bit of the AES_IER register is set. The flag is cleared by setting the RWEIF bit of the AES_ICR register.
    #[inline(always)]
    pub fn wrerrf(&self) -> WRERRF_R {
        WRERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy This flag indicates whether AES is idle or busy. AES is flagged as idle when disabled (when EN is low) or when the last processing is completed. AES is flagged as busy when processing a block data, preparing a key (ECB or CBC decryption only). When GCM encryption is selected, this flag must be at zero before suspending current process to manage a higher-priority message.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Key valid flag This bit is set by hardware when the key of size defined by KEYSIZE is loaded in AES_KEYRx key registers. The EN bit can only be set when KEYVALID is set. The key must be written in the key registers in the correct sequence, otherwise the KEIF flag is set and KEYVALID remains cleared. If set, KEIF must be cleared through the AES_ICR register, otherwise KEYVALID cannot be set. See the KEIF flag description for more details. For further information on key loading, refer to Section121.4.15: AES key registers.
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rderrf", &self.rderrf())
            .field("wrerrf", &self.wrerrf())
            .field("busy", &self.busy())
            .field("keyvalid", &self.keyvalid())
            .finish()
    }
}
/**AES status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
