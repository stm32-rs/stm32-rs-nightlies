#[doc = "Register `BCR2` reader"]
pub type R = crate::R<BCR2rs>;
#[doc = "Register `BCR2` writer"]
pub type W = crate::W<BCR2rs>;
#[doc = "FIFO threshold. This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTH {
    #[doc = "0: FIFO empty"]
    Empty = 0,
    #[doc = "1: 1⁄4 FIFO"]
    Quarter1 = 1,
    #[doc = "2: 1⁄2 FIFO"]
    Quarter2 = 2,
    #[doc = "3: 3⁄4 FIFO"]
    Quarter3 = 3,
    #[doc = "4: FIFO full"]
    Full = 4,
}
impl From<FTH> for u8 {
    #[inline(always)]
    fn from(variant: FTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTH {
    type Ux = u8;
}
#[doc = "Field `FTH` reader - FIFO threshold. This bit is set and cleared by software."]
pub type FTH_R = crate::FieldReader<FTH>;
impl FTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FTH> {
        match self.bits {
            0 => Some(FTH::Empty),
            1 => Some(FTH::Quarter1),
            2 => Some(FTH::Quarter2),
            3 => Some(FTH::Quarter3),
            4 => Some(FTH::Full),
            _ => None,
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTH::Empty
    }
    #[doc = "1⁄4 FIFO"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FTH::Quarter1
    }
    #[doc = "1⁄2 FIFO"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FTH::Quarter2
    }
    #[doc = "3⁄4 FIFO"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FTH::Quarter3
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH::Full
    }
}
#[doc = "Field `FTH` writer - FIFO threshold. This bit is set and cleared by software."]
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FTH>;
impl<'a, REG> FTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Empty)
    }
    #[doc = "1⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Quarter1)
    }
    #[doc = "1⁄2 FIFO"]
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Quarter2)
    }
    #[doc = "3⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Quarter3)
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Full)
    }
}
#[doc = "FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLUSH {
    #[doc = "0: No FIFO flush"]
    NoFlush = 0,
    #[doc = "1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    Flush = 1,
}
impl From<FFLUSH> for bool {
    #[inline(always)]
    fn from(variant: FFLUSH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLUSH` writer - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG, FFLUSH>;
impl<'a, REG> FFLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No FIFO flush"]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut crate::W<REG> {
        self.variant(FFLUSH::NoFlush)
    }
    #[doc = "FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FFLUSH::Flush)
    }
}
#[doc = "Field `TRIS` reader - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
pub type TRIS_R = crate::BitReader;
#[doc = "Field `TRIS` writer - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
pub type TRIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTE {
    #[doc = "0: No mute mode"]
    Disabled = 0,
    #[doc = "1: Mute mode enabled"]
    Enabled = 1,
}
impl From<MUTE> for bool {
    #[inline(always)]
    fn from(variant: MUTE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTE` reader - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTE_R = crate::BitReader<MUTE>;
impl MUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUTE {
        match self.bits {
            false => MUTE::Disabled,
            true => MUTE::Enabled,
        }
    }
    #[doc = "No mute mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTE::Disabled
    }
    #[doc = "Mute mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTE::Enabled
    }
}
#[doc = "Field `MUTE` writer - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTE_W<'a, REG> = crate::BitWriter<'a, REG, MUTE>;
impl<'a, REG> MUTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No mute mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTE::Disabled)
    }
    #[doc = "Mute mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTE::Enabled)
    }
}
#[doc = "Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEVAL {
    #[doc = "0: Bit value 0 is sent during the mute mode"]
    SendZero = 0,
    #[doc = "1: Last values are sent during the mute mode"]
    SendLast = 1,
}
impl From<MUTEVAL> for bool {
    #[inline(always)]
    fn from(variant: MUTEVAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEVAL` reader - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTEVAL_R = crate::BitReader<MUTEVAL>;
impl MUTEVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUTEVAL {
        match self.bits {
            false => MUTEVAL::SendZero,
            true => MUTEVAL::SendLast,
        }
    }
    #[doc = "Bit value 0 is sent during the mute mode"]
    #[inline(always)]
    pub fn is_send_zero(&self) -> bool {
        *self == MUTEVAL::SendZero
    }
    #[doc = "Last values are sent during the mute mode"]
    #[inline(always)]
    pub fn is_send_last(&self) -> bool {
        *self == MUTEVAL::SendLast
    }
}
#[doc = "Field `MUTEVAL` writer - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MUTEVAL_W<'a, REG> = crate::BitWriter<'a, REG, MUTEVAL>;
impl<'a, REG> MUTEVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit value 0 is sent during the mute mode"]
    #[inline(always)]
    pub fn send_zero(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEVAL::SendZero)
    }
    #[doc = "Last values are sent during the mute mode"]
    #[inline(always)]
    pub fn send_last(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEVAL::SendLast)
    }
}
#[doc = "Field `MUTECNT` reader - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
pub type MUTECNT_R = crate::FieldReader;
#[doc = "Field `MUTECNT` writer - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
pub type MUTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPL {
    #[doc = "0: 1’s complement representation"]
    OnesComplement = 0,
    #[doc = "1: 2’s complement representation"]
    TwosComplement = 1,
}
impl From<CPL> for bool {
    #[inline(always)]
    fn from(variant: CPL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
pub type CPL_R = crate::BitReader<CPL>;
impl CPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPL {
        match self.bits {
            false => CPL::OnesComplement,
            true => CPL::TwosComplement,
        }
    }
    #[doc = "1’s complement representation"]
    #[inline(always)]
    pub fn is_ones_complement(&self) -> bool {
        *self == CPL::OnesComplement
    }
    #[doc = "2’s complement representation"]
    #[inline(always)]
    pub fn is_twos_complement(&self) -> bool {
        *self == CPL::TwosComplement
    }
}
#[doc = "Field `CPL` writer - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
pub type CPL_W<'a, REG> = crate::BitWriter<'a, REG, CPL>;
impl<'a, REG> CPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1’s complement representation"]
    #[inline(always)]
    pub fn ones_complement(self) -> &'a mut crate::W<REG> {
        self.variant(CPL::OnesComplement)
    }
    #[doc = "2’s complement representation"]
    #[inline(always)]
    pub fn twos_complement(self) -> &'a mut crate::W<REG> {
        self.variant(CPL::TwosComplement)
    }
}
#[doc = "Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP {
    #[doc = "0: No companding algorithm"]
    NoCompanding = 0,
    #[doc = "2: μ-Law algorithm"]
    MuLaw = 2,
    #[doc = "3: A-Law algorithm"]
    Alaw = 3,
}
impl From<COMP> for u8 {
    #[inline(always)]
    fn from(variant: COMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP {
    type Ux = u8;
}
#[doc = "Field `COMP` reader - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
pub type COMP_R = crate::FieldReader<COMP>;
impl COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP> {
        match self.bits {
            0 => Some(COMP::NoCompanding),
            2 => Some(COMP::MuLaw),
            3 => Some(COMP::Alaw),
            _ => None,
        }
    }
    #[doc = "No companding algorithm"]
    #[inline(always)]
    pub fn is_no_companding(&self) -> bool {
        *self == COMP::NoCompanding
    }
    #[doc = "μ-Law algorithm"]
    #[inline(always)]
    pub fn is_mu_law(&self) -> bool {
        *self == COMP::MuLaw
    }
    #[doc = "A-Law algorithm"]
    #[inline(always)]
    pub fn is_alaw(&self) -> bool {
        *self == COMP::Alaw
    }
}
#[doc = "Field `COMP` writer - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, COMP>;
impl<'a, REG> COMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No companding algorithm"]
    #[inline(always)]
    pub fn no_companding(self) -> &'a mut crate::W<REG> {
        self.variant(COMP::NoCompanding)
    }
    #[doc = "μ-Law algorithm"]
    #[inline(always)]
    pub fn mu_law(self) -> &'a mut crate::W<REG> {
        self.variant(COMP::MuLaw)
    }
    #[doc = "A-Law algorithm"]
    #[inline(always)]
    pub fn alaw(self) -> &'a mut crate::W<REG> {
        self.variant(COMP::Alaw)
    }
}
impl R {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<BCR2rs> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 3 - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<BCR2rs> {
        FFLUSH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<BCR2rs> {
        TRIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<BCR2rs> {
        MUTE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<BCR2rs> {
        MUTEVAL_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an interrupt is generated if bit MUTEDETIE is set. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<BCR2rs> {
        MUTECNT_W::new(self, 7)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is �-Law algorithm or A-Law algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<BCR2rs> {
        CPL_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The �-Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that is used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to for more details. Note: Companding mode is applicable only when Free protocol mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<BCR2rs> {
        COMP_W::new(self, 14)
    }
}
#[doc = "SAI configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR2rs;
impl crate::RegisterSpec for BCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr2::R`](R) reader structure"]
impl crate::Readable for BCR2rs {}
#[doc = "`write(|w| ..)` method takes [`bcr2::W`](W) writer structure"]
impl crate::Writable for BCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCR2 to value 0"]
impl crate::Resettable for BCR2rs {
    const RESET_VALUE: u32 = 0;
}
