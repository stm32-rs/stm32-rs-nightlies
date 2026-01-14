///Register `OPTISR` reader
pub type R = crate::R<OPTISRrs>;
///Field `KVEF` reader - Key valid error flag This bit is set when loading an unknown or corrupted option byte key. More specifically: Embedded Flash did not find an option byte key that corresponds to the given OBKINDEX\[4:0\] and the requested HDPL (optionally modified by NEXTKL\[1:0\]). It can happen for example when requested key has not being provisioned. A double error detection was found when loading the requested option byte key. In this case, if this key is provisioned again the error should disappear. When KVEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KVEIE bit of FLASH_OPTCR register is set. Setting KVEF bit of register FLASH_OPTICR clears this bit.
pub type KVEF_R = crate::BitReader;
///Field `KTEF` reader - Key transfer error flag This bit is set when embedded Flash signals an error to the SAES peripheral. It happens when the key size (128-bit or 256-bit) is not matching between embedded Flash OBKSIZE\[1:0\] and KEYSIZE bit in SAES_CR register. It also happen when an ECC dual error detection occurred while embedded Flash loaded an option byte key for the SAES peripheral. When KTEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KTEIE bit of FLASH_OPTCR register is set. Setting KTEF bit of register FLASH_OPTICR clears this bit.
pub type KTEF_R = crate::BitReader;
///Field `OPTERRF` reader - Option byte change error flag When OPTERRF is set, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTERRIE bit of FLASH_OPTCR register is set. Setting OPTERRF of register FLASH_OPTICR clears this bit.
pub type OPTERRF_R = crate::BitReader;
impl R {
    ///Bit 27 - Key valid error flag This bit is set when loading an unknown or corrupted option byte key. More specifically: Embedded Flash did not find an option byte key that corresponds to the given OBKINDEX\[4:0\] and the requested HDPL (optionally modified by NEXTKL\[1:0\]). It can happen for example when requested key has not being provisioned. A double error detection was found when loading the requested option byte key. In this case, if this key is provisioned again the error should disappear. When KVEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KVEIE bit of FLASH_OPTCR register is set. Setting KVEF bit of register FLASH_OPTICR clears this bit.
    #[inline(always)]
    pub fn kvef(&self) -> KVEF_R {
        KVEF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Key transfer error flag This bit is set when embedded Flash signals an error to the SAES peripheral. It happens when the key size (128-bit or 256-bit) is not matching between embedded Flash OBKSIZE\[1:0\] and KEYSIZE bit in SAES_CR register. It also happen when an ECC dual error detection occurred while embedded Flash loaded an option byte key for the SAES peripheral. When KTEF is set write to START bit in FLASH_OBKCR is ignored. An interrupt is generated when this flag is raised if the KTEIE bit of FLASH_OPTCR register is set. Setting KTEF bit of register FLASH_OPTICR clears this bit.
    #[inline(always)]
    pub fn ktef(&self) -> KTEF_R {
        KTEF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Option byte change error flag When OPTERRF is set, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTERRIE bit of FLASH_OPTCR register is set. Setting OPTERRF of register FLASH_OPTICR clears this bit.
    #[inline(always)]
    pub fn opterrf(&self) -> OPTERRF_R {
        OPTERRF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTISR")
            .field("kvef", &self.kvef())
            .field("ktef", &self.ktef())
            .field("opterrf", &self.opterrf())
            .finish()
    }
}
/**FLASH options interrupt status register

You can [`read`](crate::Reg::read) this register and get [`optisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OPTISR)*/
pub struct OPTISRrs;
impl crate::RegisterSpec for OPTISRrs {
    type Ux = u32;
}
///`read()` method returns [`optisr::R`](R) reader structure
impl crate::Readable for OPTISRrs {}
///`reset()` method sets OPTISR to value 0
impl crate::Resettable for OPTISRrs {}
