///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Field `EN` reader - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
other than 00, use the IPRST bit rather than the bit EN.*/
pub type EN_R = crate::BitReader;
/**Field `EN` writer - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
other than 00, use the IPRST bit rather than the bit EN.*/
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD1` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD1_R = crate::FieldReader;
///Field `CHMOD1` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `DMAINEN` reader - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
bitfield. It is not effective for Mode 2 (key derivation).*/
pub type DMAINEN_R = crate::BitReader;
/**Field `DMAINEN` writer - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
bitfield. It is not effective for Mode 2 (key derivation).*/
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `DMAOUTEN` reader - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
bitfield. It is not effective for Mode 2 (key derivation).*/
pub type DMAOUTEN_R = crate::BitReader;
/**Field `DMAOUTEN` writer - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
bitfield. It is not effective for Mode 2 (key derivation).*/
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD2` reader - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD2_R = crate::BitReader;
///Field `CHMOD2` writer - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type CHMOD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYPROT` reader - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYPROT_R = crate::BitReader;
///Field `KEYPROT` writer - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KEYPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Field `KMOD` reader - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
pub type KMOD_R = crate::FieldReader;
/**Field `KMOD` writer - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KSHAREID` reader - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KSHAREID_R = crate::FieldReader;
///Field `KSHAREID` writer - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
pub type KSHAREID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `KEYSEL` reader - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
is other than zero, KEYSEL\[2:0\]
is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
pub type KEYSEL_R = crate::FieldReader;
/**Field `KEYSEL` writer - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
is other than zero, KEYSEL\[2:0\]
is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
pub type KEYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `IPRST` reader - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
    other than 00, use the IPRST bit rather than the bit EN.*/
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod1(&self) -> CHMOD1_R {
        CHMOD1_R::new(((self.bits >> 5) & 3) as u8)
    }
    /**Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    bitfield. It is not effective for Mode 2 (key derivation).*/
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    /**Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    bitfield. It is not effective for Mode 2 (key derivation).*/
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod2(&self) -> CHMOD2_R {
        CHMOD2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    /**Bits 24:25 - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
    bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
    other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
    Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    /**Bits 28:30 - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
    with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
    bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
    is other than zero, KEYSEL\[2:0\]
    is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("chmod1", &self.chmod1())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod2", &self.chmod2())
            .field("keysize", &self.keysize())
            .field("keyprot", &self.keyprot())
            .field("npblb", &self.npblb())
            .field("kmod", &self.kmod())
            .field("kshareid", &self.kshareid())
            .field("keysel", &self.keysel())
            .field("iprst", &self.iprst())
            .finish()
    }
}
impl W {
    /**Bit 0 - SAES enable This bit enables/disables the SAES peripheral: At any moment, clearing then setting the bit re-initializes the SAES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (Mode 2) and upon the completion of GCM/GMAC/CCM initial phase. The bit cannot be set as long as KEYVALID = 0 nor along with the following settings: KMOD = 01 + CHMOD = 011 and KMOD = 01 + CHMOD = 010 + MODE = 00. Note: With KMOD\[1:0\]
    other than 00, use the IPRST bit rather than the bit EN.*/
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type selection This bitfield defines the format of data written in the SAES_DINR register or read from the SAES_DOUTR register, through selecting the mode of data swapping: For more details, refer to . Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - SAES operating mode This bitfield selects the SAES operating mode: Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod1(&mut self) -> CHMOD1_W<CRrs> {
        CHMOD1_W::new(self, 5)
    }
    /**Bit 11 - DMA input enable This bit enables/disables data transferring with DMA, in the input phase: When the bit is set, DMA requests are automatically generated by SAES during the input data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    bitfield. It is not effective for Mode 2 (key derivation).*/
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    /**Bit 12 - DMA output enable This bit enables/disables data transferring with DMA, in the output phase: When the bit is set, DMA requests are automatically generated by SAES during the output data phase. This feature is only effective when Mode 1 or Mode 3 is selected through the MODE\[1:0\]
    bitfield. It is not effective for Mode 2 (key derivation).*/
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase of GCM, GMAC or CCM algorithm: The bitfield has no effect if other than GCM, GMAC or CCM algorithms are selected (through the ALGOMODE bitfield).
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - Chaining mode selection This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod2(&mut self) -> CHMOD2_W<CRrs> {
        CHMOD2_W::new(self, 16)
    }
    ///Bit 18 - Key size selection This bitfield defines the length of the key used in the SAES cryptographic core, in bits: When KMOD\[1:0\]=01 or 10 KEYSIZE also defines the length of the key to encrypt or decrypt. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bit 19 - Key protection When set, hardware-based key protection is enabled. Attempts to write the bit are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keyprot(&mut self) -> KEYPROT_W<CRrs> {
        KEYPROT_W::new(self, 19)
    }
    ///Bits 20:23 - Number of padding bytes in last block The bitfield sets the number of padding bytes in last block of payload: ...
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
    /**Bits 24:25 - Key mode selection The bitfield defines how the SAES key can be used by the application: Others: Reserved With normal key selection, the key registers are freely usable, no specific usage or protection applies to SAES_DIN and SAES_DOUT registers. With wrapped key selection, the key loaded in key registers can only be used to encrypt or decrypt AES keys. Hence, when a decryption is selected in Wrapped-key mode read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With shared key selection, after a successful decryption process, SAES key registers are shared with the peripheral described in KSHAREID(1:0\]
    bitfield. This sharing is valid only while KMOD\[1:0\]=10 and KEYVALID = 1. When a decryption is selected, read-as-zero SAES_DOUT register is automatically loaded into SAES key registers after a successful decryption process. With KMOD\[1:0\]
    other than zero, any attempt to configure the SAES peripheral for use by an application belonging to a different security domain (secure or non-secure) results in automatic key erasure and setting of the KEIF flag.
    Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bits 26:27 - Key share identification This bitfield defines, at the end of a decryption process with KMOD\[1:0\]=10 (shared key), which target can read the SAES key registers using a dedicated hardware bus. Others: Reserved Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn kshareid(&mut self) -> KSHAREID_W<CRrs> {
        KSHAREID_W::new(self, 26)
    }
    /**Bits 28:30 - Key selection The bitfield defines the source of the key information to use in the AES cryptographic core. Others: Reserved (if used, unfreeze SAES with IPRST) When KEYSEL is different from zero, selected key value is available in key registers when BUSY bit is cleared and KEYVALID is set in the SAES_SR register. Otherwise, the key error flag KEIF is set. Repeated writing of KEYSEL\[2:0\]
    with the same non-zero value only triggers the loading of DHUK or BHK if KEYVALID = 0. When the application software changes the key selection by writing the KEYSEL\[2:0\]
    bitfield, the key registers are immediately erased and the KEYVALID flag cleared. At the end of the decryption process, if KMOD\[1:0\]
    is other than zero, KEYSEL\[2:0\]
    is cleared. With the bitfield value other than zero and KEYVALID set, the application cannot transfer the ownership of SAES with a loaded key to an application running in another security context (such as secure, non-secure). More specifically, when security of an access to any register does not match the information recorded by SAES, the KEIF flag is set. Attempts to write the bitfield are ignored when the BUSY flag of SAES_SR register is set, as well as when the EN bit of the SAES_CR register is set before the write access and it is not cleared by that write access.*/
    #[inline(always)]
    pub fn keysel(&mut self) -> KEYSEL_W<CRrs> {
        KEYSEL_W::new(self, 28)
    }
    ///Bit 31 - SAES peripheral software reset Setting the bit resets the SAES peripheral, putting all registers to their default values, except the IPRST bit itself and the SAES_DPACFG register. Hence, any key-relative data is lost. For this reason, it is recommended to set the bit before handing over the SAES to a less secure application. The bit must be low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**SAES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAES:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}