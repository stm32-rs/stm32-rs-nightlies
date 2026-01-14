///Register `BCR2` reader
pub type R = crate::R<BCR2rs>;
///Register `BCR2` writer
pub type W = crate::W<BCR2rs>;
///Field `FTH` reader - FIFO threshold. This bit is set and cleared by software.
pub type FTH_R = crate::FieldReader;
///Field `FTH` writer - FIFO threshold. This bit is set and cleared by software.
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FFLUSH` writer - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled.
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIS` reader - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section : Output data line management on an inactive slot for more details.
pub type TRIS_R = crate::BitReader;
///Field `TRIS` writer - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section : Output data line management on an inactive slot for more details.
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTE` reader - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
pub type MUTE_R = crate::BitReader;
///Field `MUTE` writer - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEVAL` reader - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
pub type MUTEVAL_R = crate::BitReader;
///Field `MUTEVAL` writer - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTECNT` reader - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to Section : Mute mode for more details.
pub type MUTECNT_R = crate::FieldReader;
///Field `MUTECNT` writer - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to Section : Mute mode for more details.
pub type MUTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CPL` reader - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm.
pub type CPL_R = crate::BitReader;
///Field `CPL` writer - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm.
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP` reader - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\[0\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section : Companding mode for more details. Note: Companding mode is applicable only when Free protocol mode is selected.
pub type COMP_R = crate::FieldReader;
///Field `COMP` writer - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\[0\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section : Companding mode for more details. Note: Companding mode is applicable only when Free protocol mode is selected.
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - FIFO threshold. This bit is set and cleared by software.
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section : Output data line management on an inactive slot for more details.
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to Section : Mute mode for more details.
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm.
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\[0\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section : Companding mode for more details. Note: Companding mode is applicable only when Free protocol mode is selected.
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR2")
            .field("fth", &self.fth())
            .field("tris", &self.tris())
            .field("mute", &self.mute())
            .field("muteval", &self.muteval())
            .field("mutecnt", &self.mutecnt())
            .field("cpl", &self.cpl())
            .field("comp", &self.comp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - FIFO threshold. This bit is set and cleared by software.
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<'_, BCR2rs> {
        FTH_W::new(self, 0)
    }
    ///Bit 3 - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled.
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<'_, BCR2rs> {
        FFLUSH_W::new(self, 3)
    }
    ///Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section : Output data line management on an inactive slot for more details.
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W<'_, BCR2rs> {
        TRIS_W::new(self, 4)
    }
    ///Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W<'_, BCR2rs> {
        MUTE_W::new(self, 5)
    }
    ///Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section : Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W<'_, BCR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    ///Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to Section : Mute mode for more details.
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MUTECNT_W<'_, BCR2rs> {
        MUTECNT_W::new(self, 7)
    }
    ///Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm.
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W<'_, BCR2rs> {
        CPL_W::new(self, 13)
    }
    ///Bits 14:15 - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\[0\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section : Companding mode for more details. Note: Companding mode is applicable only when Free protocol mode is selected.
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<'_, BCR2rs> {
        COMP_W::new(self, 14)
    }
}
/**SAI configuration register 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SAI1:BCR2)*/
pub struct BCR2rs;
impl crate::RegisterSpec for BCR2rs {
    type Ux = u32;
}
///`read()` method returns [`bcr2::R`](R) reader structure
impl crate::Readable for BCR2rs {}
///`write(|w| ..)` method takes [`bcr2::W`](W) writer structure
impl crate::Writable for BCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR2 to value 0
impl crate::Resettable for BCR2rs {}
