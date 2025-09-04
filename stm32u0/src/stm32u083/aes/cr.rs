///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\[1:0\] at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared
pub type EN_R = crate::BitReader;
///Field `EN` writer - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\[1:0\] at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD` reader - CHMOD\[1:0\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type CHMOD_R = crate::FieldReader;
///Field `CHMOD` writer - CHMOD\[1:0\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMAINEN` reader - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
pub type DMAINEN_R = crate::BitReader;
///Field `DMAINEN` writer - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAOUTEN` reader - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
pub type DMAOUTEN_R = crate::BitReader;
///Field `DMAOUTEN` writer - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCMPH` reader - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes.
pub type GCMPH_R = crate::FieldReader;
///Field `GCMPH` writer - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes.
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CHMOD_1` reader - CHMOD\[2\]
pub type CHMOD_1_R = crate::BitReader;
///Field `CHMOD_1` writer - CHMOD\[2\]
pub type CHMOD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYSIZE` reader - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_R = crate::BitReader;
///Field `KEYSIZE` writer - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access.
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ...
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ...
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IPRST` reader - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers.
pub type IPRST_R = crate::BitReader;
///Field `IPRST` writer - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers.
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\[1:0\] at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes.
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 16 - CHMOD\[2\]
    #[inline(always)]
    pub fn chmod_1(&self) -> CHMOD_1_R {
        CHMOD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ...
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers.
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
            .field("chmod", &self.chmod())
            .field("dmainen", &self.dmainen())
            .field("dmaouten", &self.dmaouten())
            .field("gcmph", &self.gcmph())
            .field("chmod_1", &self.chmod_1())
            .field("keysize", &self.keysize())
            .field("npblb", &self.npblb())
            .field("iprst", &self.iprst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable This bit enables/disables the AES peripheral. At any moment, clearing then setting the bit re-initializes the AES peripheral. This bit is automatically cleared by hardware upon the completion of the key preparation (MODE\[1:0\] at 0x1) and upon the completion of GCM/GMAC/CCM initialization phase. The bit cannot be set as long as KEYVALID1is cleared
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Data type This bitfield defines the format of data written in the AES_DINR register or read from the AES_DOUTR register, through selecting the mode of data swapping. This swapping is defined in Section121.4.14: AES data registers and data swapping. Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    ///Bits 3:4 - Operating mode This bitfield selects the AES operating mode: Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    ///Bits 5:6 - CHMOD\[1:0\]: Chaining mode This bitfield selects the AES chaining mode: others: Reserved Attempts to write the bitfield are ignored when EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    ///Bit 11 - DMA input enable This bit enables automatic generation of DMA requests during the data phase, for incoming data transfers to AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    ///Bit 12 - DMA output enable This bit enables automatic generation of DMA requests during the data phase, for outgoing data transfers from AES via DMA. Setting this bit is ignored when MODE\[1:0\] is at 0x1 (key derivation).
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    ///Bits 13:14 - GCM or CCM phase selection This bitfield selects the phase, applicable only with GCM, GMAC or CCM chaining modes.
    #[inline(always)]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    ///Bit 16 - CHMOD\[2\]
    #[inline(always)]
    pub fn chmod_1(&mut self) -> CHMOD_1_W<CRrs> {
        CHMOD_1_W::new(self, 16)
    }
    ///Bit 18 - Key size selection This bitfield defines the key length in bits of the key used by AES. Attempts to write the bit are ignored when the EN is set before the write access and it is not cleared by that write access.
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    ///Bits 20:23 - Number of padding bytes in last block This padding information must be filled by software before processing the last block of GCM payload encryption or CCM payload decryption, otherwise authentication tag computation is incorrect. ...
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
    ///Bit 31 - AES peripheral software reset Setting the bit resets the AES peripheral, putting all registers to their default values, except the IPRST bit itself. Hence, any key-relative data are lost. For this reason, it is recommended to set the bit before handing over the AES to a less secure application. The bit must be kept low while writing any configuration registers.
    #[inline(always)]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
    }
}
/**AES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:CR)*/
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
