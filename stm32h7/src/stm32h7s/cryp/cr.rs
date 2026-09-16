///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ALGODIR` reader - Algorithm direction This bit selects the algorithm direction. Attempts to write the bitfield are ignored when BUSY is set.
pub type ALGODIR_R = crate::BitReader;
///Field `ALGODIR` writer - Algorithm direction This bit selects the algorithm direction. Attempts to write the bitfield are ignored when BUSY is set.
pub type ALGODIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGOMODE` reader - ALGOMODE\[2:0\]: Algorithm mode This bitfield selects the AES algorithm/chaining mode. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
pub type ALGOMODE_R = crate::FieldReader;
///Field `ALGOMODE` writer - ALGOMODE\[2:0\]: Algorithm mode This bitfield selects the AES algorithm/chaining mode. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
pub type ALGOMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DATATYPE` reader - Data type This bitfield defines the format of data written in the CRYP_DINR register or read from the CRYP_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section 60.4.15: CRYP data registers and data swapping. Attempts to write the bitfield are ignored when BUSY is set.
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type This bitfield defines the format of data written in the CRYP_DINR register or read from the CRYP_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section 60.4.15: CRYP data registers and data swapping. Attempts to write the bitfield are ignored when BUSY is set.
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSIZE` reader - Key size selection This bitfield defines the key length in bits of the key used by CRYP. When KEYSIZE is changed, KEYVALID bit is cleared. Attempts to write the bitfield are ignored when BUSY is set.
pub type KEYSIZE_R = crate::FieldReader;
///Field `KEYSIZE` writer - Key size selection This bitfield defines the key length in bits of the key used by CRYP. When KEYSIZE is changed, KEYVALID bit is cleared. Attempts to write the bitfield are ignored when BUSY is set.
pub type KEYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FFLUSH` reader - FIFO flush This bit enables/disables the flushing of CRYP input and output FIFOs. Reading this bit always returns 0. When CRYPEN is cleared, writing this bit to 1 flushes both input and output FIFOs (that is read and write pointers of the FIFOs are reset). FFLUSH bit must be set when BUSY is cleared, otherwise the FIFO is flushed, but the block being processed may be pushed into the output FIFO just after the flush operation, resulting in a non-empty FIFO condition. Attempts to write FFLUSH are ignored when CRYPEN is set.
pub type FFLUSH_R = crate::BitReader;
///Field `FFLUSH` writer - FIFO flush This bit enables/disables the flushing of CRYP input and output FIFOs. Reading this bit always returns 0. When CRYPEN is cleared, writing this bit to 1 flushes both input and output FIFOs (that is read and write pointers of the FIFOs are reset). FFLUSH bit must be set when BUSY is cleared, otherwise the FIFO is flushed, but the block being processed may be pushed into the output FIFO just after the flush operation, resulting in a non-empty FIFO condition. Attempts to write FFLUSH are ignored when CRYPEN is set.
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPEN` reader - CRYP enable This bit enables/disables the CRYP peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (ALGOMODE\[3:0\] at 0x7) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID is cleared.
pub type CRYPEN_R = crate::BitReader;
///Field `CRYPEN` writer - CRYP enable This bit enables/disables the CRYP peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (ALGOMODE\[3:0\] at 0x7) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID is cleared.
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCM_CCMPH` reader - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes. Attempts to write the bitfield are ignored when BUSY is set.
pub type GCM_CCMPH_R = crate::FieldReader;
///Field `GCM_CCMPH` writer - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes. Attempts to write the bitfield are ignored when BUSY is set.
pub type GCM_CCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ALGOMODE_1` reader - ALGOMODE\[3\]
pub type ALGOMODE_1_R = crate::BitReader;
///Field `ALGOMODE_1` writer - ALGOMODE\[3\]
pub type ALGOMODE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ... Attempts to write the bitfield are ignored when BUSY is set.
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ... Attempts to write the bitfield are ignored when BUSY is set.
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `KMOD` reader - Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\[1:0\] different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
pub type KMOD_R = crate::FieldReader;
///Field `KMOD` writer - Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\[1:0\] different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IPRST` reader - CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers.
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers.
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Algorithm direction This bit selects the algorithm direction. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - ALGOMODE\[2:0\]: Algorithm mode This bitfield selects the AES algorithm/chaining mode. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - Data type This bitfield defines the format of data written in the CRYP_DINR register or read from the CRYP_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section 60.4.15: CRYP data registers and data swapping. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Key size selection This bitfield defines the key length in bits of the key used by CRYP. When KEYSIZE is changed, KEYVALID bit is cleared. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 14 - FIFO flush This bit enables/disables the flushing of CRYP input and output FIFOs. Reading this bit always returns 0. When CRYPEN is cleared, writing this bit to 1 flushes both input and output FIFOs (that is read and write pointers of the FIFOs are reset). FFLUSH bit must be set when BUSY is cleared, otherwise the FIFO is flushed, but the block being processed may be pushed into the output FIFO just after the flush operation, resulting in a non-empty FIFO condition. Attempts to write FFLUSH are ignored when CRYPEN is set.
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CRYP enable This bit enables/disables the CRYP peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (ALGOMODE\[3:0\] at 0x7) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID is cleared.
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - ALGOMODE\[3\]
    #[inline(always)]
    pub fn algomode_1(&self) -> ALGOMODE_1_R {
        ALGOMODE_1_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ... Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:25 - Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\[1:0\] different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 31 - CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("algodir", &self.algodir())
            .field("algomode", &self.algomode())
            .field("datatype", &self.datatype())
            .field("keysize", &self.keysize())
            .field("fflush", &self.fflush())
            .field("crypen", &self.crypen())
            .field("gcm_ccmph", &self.gcm_ccmph())
            .field("algomode_1", &self.algomode_1())
            .field("npblb", &self.npblb())
            .field("kmod", &self.kmod())
            .field("iprst", &self.iprst())
            .finish()
    }
}
impl W {
    ///Bit 2 - Algorithm direction This bit selects the algorithm direction. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn algodir(&mut self) -> ALGODIR_W<'_, CRrs> {
        ALGODIR_W::new(self, 2)
    }
    ///Bits 3:5 - ALGOMODE\[2:0\]: Algorithm mode This bitfield selects the AES algorithm/chaining mode. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn algomode(&mut self) -> ALGOMODE_W<'_, CRrs> {
        ALGOMODE_W::new(self, 3)
    }
    ///Bits 6:7 - Data type This bitfield defines the format of data written in the CRYP_DINR register or read from the CRYP_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section 60.4.15: CRYP data registers and data swapping. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 6)
    }
    ///Bits 8:9 - Key size selection This bitfield defines the key length in bits of the key used by CRYP. When KEYSIZE is changed, KEYVALID bit is cleared. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 8)
    }
    ///Bit 14 - FIFO flush This bit enables/disables the flushing of CRYP input and output FIFOs. Reading this bit always returns 0. When CRYPEN is cleared, writing this bit to 1 flushes both input and output FIFOs (that is read and write pointers of the FIFOs are reset). FFLUSH bit must be set when BUSY is cleared, otherwise the FIFO is flushed, but the block being processed may be pushed into the output FIFO just after the flush operation, resulting in a non-empty FIFO condition. Attempts to write FFLUSH are ignored when CRYPEN is set.
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<'_, CRrs> {
        FFLUSH_W::new(self, 14)
    }
    ///Bit 15 - CRYP enable This bit enables/disables the CRYP peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (ALGOMODE\[3:0\] at 0x7) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID is cleared.
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, CRrs> {
        CRYPEN_W::new(self, 15)
    }
    ///Bits 16:17 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes. Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W<'_, CRrs> {
        GCM_CCMPH_W::new(self, 16)
    }
    ///Bit 19 - ALGOMODE\[3\]
    #[inline(always)]
    pub fn algomode_1(&mut self) -> ALGOMODE_1_W<'_, CRrs> {
        ALGOMODE_1_W::new(self, 19)
    }
    ///Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ... Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bits 24:25 - Key mode selection This bitfield defines how the CRYP key can be used by the application. KEYSIZE must be correctly initialized when setting KMOD\[1:0\] different from zero. Others: Reserved Attempts to write the bitfield are ignored when BUSY is set.
    #[inline(always)]
    pub fn kmod(&mut self) -> KMOD_W<'_, CRrs> {
        KMOD_W::new(self, 24)
    }
    ///Bit 31 - CRYP peripheral software reset Setting the bit resets the CRYP peripheral, putting all registers to their default values, except the IPRST bit itself. This bit must be kept cleared while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<'_, CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**CRYP control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
