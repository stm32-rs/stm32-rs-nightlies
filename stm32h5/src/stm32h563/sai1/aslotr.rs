///Register `ASLOTR` reader
pub type R = crate::R<ASLOTRrs>;
///Register `ASLOTR` writer
pub type W = crate::W<ASLOTRrs>;
///Field `FBOFF` reader - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type FBOFF_R = crate::FieldReader;
///Field `FBOFF` writer - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type FBOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOTSZ {
    ///0: The slot size is equivalent to the data size (specified in DS\[3:0\] in the SAI_xCR1 register)
    DataSize = 0,
    ///1: 16-bit
    Bit16 = 1,
    ///2: 32-bit
    Bit32 = 2,
}
impl From<SLOTSZ> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSZ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLOTSZ {
    type Ux = u8;
}
impl crate::IsEnum for SLOTSZ {}
///Field `SLOTSZ` reader - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type SLOTSZ_R = crate::FieldReader<SLOTSZ>;
impl SLOTSZ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLOTSZ> {
        match self.bits {
            0 => Some(SLOTSZ::DataSize),
            1 => Some(SLOTSZ::Bit16),
            2 => Some(SLOTSZ::Bit32),
            _ => None,
        }
    }
    ///The slot size is equivalent to the data size (specified in DS\[3:0\] in the SAI_xCR1 register)
    #[inline(always)]
    pub fn is_data_size(&self) -> bool {
        *self == SLOTSZ::DataSize
    }
    ///16-bit
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == SLOTSZ::Bit16
    }
    ///32-bit
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == SLOTSZ::Bit32
    }
}
///Field `SLOTSZ` writer - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type SLOTSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SLOTSZ>;
impl<'a, REG> SLOTSZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The slot size is equivalent to the data size (specified in DS\[3:0\] in the SAI_xCR1 register)
    #[inline(always)]
    pub fn data_size(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTSZ::DataSize)
    }
    ///16-bit
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTSZ::Bit16)
    }
    ///32-bit
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTSZ::Bit32)
    }
}
///Field `NBSLOT` reader - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type NBSLOT_R = crate::FieldReader;
///Field `NBSLOT` writer - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type NBSLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SLOTEN {
    ///0: Inactive slot
    Inactive = 0,
    ///1: Active slot
    Active = 1,
}
impl From<SLOTEN> for u16 {
    #[inline(always)]
    fn from(variant: SLOTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLOTEN {
    type Ux = u16;
}
impl crate::IsEnum for SLOTEN {}
///Field `SLOTEN` reader - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type SLOTEN_R = crate::FieldReader<SLOTEN>;
impl SLOTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLOTEN> {
        match self.bits {
            0 => Some(SLOTEN::Inactive),
            1 => Some(SLOTEN::Active),
            _ => None,
        }
    }
    ///Inactive slot
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SLOTEN::Inactive
    }
    ///Active slot
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLOTEN::Active
    }
}
///Field `SLOTEN` writer - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
pub type SLOTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, SLOTEN>;
impl<'a, REG> SLOTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Inactive slot
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTEN::Inactive)
    }
    ///Active slot
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(SLOTEN::Active)
    }
}
impl R {
    ///Bits 0:4 - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASLOTR")
            .field("fboff", &self.fboff())
            .field("slotsz", &self.slotsz())
            .field("nbslot", &self.nbslot())
            .field("sloten", &self.sloten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W<'_, ASLOTRrs> {
        FBOFF_W::new(self, 0)
    }
    ///Bits 6:7 - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W<'_, ASLOTRrs> {
        SLOTSZ_W::new(self, 6)
    }
    ///Bits 8:11 - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W<'_, ASLOTRrs> {
        NBSLOT_W::new(self, 8)
    }
    ///Bits 16:31 - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC'97 or SPDIF mode.
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W<'_, ASLOTRrs> {
        SLOTEN_W::new(self, 16)
    }
}
/**SAI slot register

You can [`read`](crate::Reg::read) this register and get [`aslotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aslotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SAI1:ASLOTR)*/
pub struct ASLOTRrs;
impl crate::RegisterSpec for ASLOTRrs {
    type Ux = u32;
}
///`read()` method returns [`aslotr::R`](R) reader structure
impl crate::Readable for ASLOTRrs {}
///`write(|w| ..)` method takes [`aslotr::W`](W) writer structure
impl crate::Writable for ASLOTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASLOTR to value 0
impl crate::Resettable for ASLOTRrs {}
