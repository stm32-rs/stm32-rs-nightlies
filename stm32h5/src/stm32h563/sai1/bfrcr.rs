///Register `BFRCR` reader
pub type R = crate::R<BFRCRrs>;
///Register `BFRCR` writer
pub type W = crate::W<BFRCRrs>;
///Field `FRL` reader - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\] of SAI_xSLOTR register (NBSLOT\[3:0\] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration.
pub type FRL_R = crate::FieldReader;
///Field `FRL` writer - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\] of SAI_xSLOTR register (NBSLOT\[3:0\] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration.
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FSALL` reader - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
pub type FSALL_R = crate::FieldReader;
///Field `FSALL` writer - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `FSDEF` reader - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots is dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC'97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
pub type FSDEF_R = crate::BitReader;
///Field `FSDEF` writer - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots is dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC'97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
pub type FSDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPOL {
    ///0: FS is active low (falling edge)
    FallingEdge = 0,
    ///1: FS is active high (rising edge)
    RisingEdge = 1,
}
impl From<FSPOL> for bool {
    #[inline(always)]
    fn from(variant: FSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `FSPOL` reader - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSPOL_R = crate::BitReader<FSPOL>;
impl FSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSPOL {
        match self.bits {
            false => FSPOL::FallingEdge,
            true => FSPOL::RisingEdge,
        }
    }
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL::FallingEdge
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL::RisingEdge
    }
}
///Field `FSPOL` writer - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG, FSPOL>;
impl<'a, REG> FSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::FallingEdge)
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::RisingEdge)
    }
}
/**Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOFF {
    ///0: FS is asserted on the first bit of the slot 0
    OnFirst = 0,
    ///1: FS is asserted one bit before the first bit of the slot 0
    BeforeFirst = 1,
}
impl From<FSOFF> for bool {
    #[inline(always)]
    fn from(variant: FSOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `FSOFF` reader - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSOFF_R = crate::BitReader<FSOFF>;
impl FSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSOFF {
        match self.bits {
            false => FSOFF::OnFirst,
            true => FSOFF::BeforeFirst,
        }
    }
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF::OnFirst
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF::BeforeFirst
    }
}
///Field `FSOFF` writer - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG, FSOFF>;
impl<'a, REG> FSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn on_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::OnFirst)
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn before_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::BeforeFirst)
    }
}
impl R {
    ///Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\] of SAI_xSLOTR register (NBSLOT\[3:0\] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration.
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots is dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC'97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BFRCR")
            .field("frl", &self.frl())
            .field("fsall", &self.fsall())
            .field("fsdef", &self.fsdef())
            .field("fspol", &self.fspol())
            .field("fsoff", &self.fsoff())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\] + 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\] of SAI_xSLOTR register (NBSLOT\[3:0\] = 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration.
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W<'_, BFRCRrs> {
        FRL_W::new(self, 0)
    }
    ///Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\] + 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC'97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W<'_, BFRCRrs> {
        FSALL_W::new(self, 8)
    }
    ///Bit 16 - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots is dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC'97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W<'_, BFRCRrs> {
        FSDEF_W::new(self, 16)
    }
    ///Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W<'_, BFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    ///Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC'97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W<'_, BFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
/**SAI frame configuration register

You can [`read`](crate::Reg::read) this register and get [`bfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#SAI1:BFRCR)*/
pub struct BFRCRrs;
impl crate::RegisterSpec for BFRCRrs {
    type Ux = u32;
}
///`read()` method returns [`bfrcr::R`](R) reader structure
impl crate::Readable for BFRCRrs {}
///`write(|w| ..)` method takes [`bfrcr::W`](W) writer structure
impl crate::Writable for BFRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BFRCR to value 0x07
impl crate::Resettable for BFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
