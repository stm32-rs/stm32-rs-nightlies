#[doc = "Register `AFRCR` reader"]
pub type R = crate::R<AFRCRrs>;
#[doc = "Register `AFRCR` writer"]
pub type W = crate::W<AFRCRrs>;
#[doc = "Field `FRL` reader - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
pub type FRL_R = crate::FieldReader;
#[doc = "Field `FRL` writer - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSALL` reader - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
pub type FSALL_R = crate::FieldReader;
#[doc = "Field `FSALL` writer - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FSDEF` reader - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots are dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC’97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
pub type FSDEF_R = crate::BitReader;
#[doc = "Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPOL {
    #[doc = "0: FS is active low (falling edge)"]
    FallingEdge = 0,
    #[doc = "1: FS is active high (rising edge)"]
    RisingEdge = 1,
}
impl From<FSPOL> for bool {
    #[inline(always)]
    fn from(variant: FSPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPOL` reader - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
pub type FSPOL_R = crate::BitReader<FSPOL>;
impl FSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSPOL {
        match self.bits {
            false => FSPOL::FallingEdge,
            true => FSPOL::RisingEdge,
        }
    }
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL::FallingEdge
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL::RisingEdge
    }
}
#[doc = "Field `FSPOL` writer - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG, FSPOL>;
impl<'a, REG> FSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::FallingEdge)
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::RisingEdge)
    }
}
#[doc = "Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOFF {
    #[doc = "0: FS is asserted on the first bit of the slot 0"]
    OnFirst = 0,
    #[doc = "1: FS is asserted one bit before the first bit of the slot 0"]
    BeforeFirst = 1,
}
impl From<FSOFF> for bool {
    #[inline(always)]
    fn from(variant: FSOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSOFF` reader - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
pub type FSOFF_R = crate::BitReader<FSOFF>;
impl FSOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSOFF {
        match self.bits {
            false => FSOFF::OnFirst,
            true => FSOFF::BeforeFirst,
        }
    }
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF::OnFirst
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF::BeforeFirst
    }
}
#[doc = "Field `FSOFF` writer - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG, FSOFF>;
impl<'a, REG> FSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn on_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::OnFirst)
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn before_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::BeforeFirst)
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots are dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC’97 or SPDIF audio block configuration. It must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\\[7:0\\]
+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\\[4:0\\]
of SAI_xSLOTR register (NBSLOT\\[3:0\\]
= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<AFRCRrs> {
        FRL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\\[6:0\\]
+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC’97 or SPDIF audio block configuration. They must be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<AFRCRrs> {
        FSALL_W::new(self, 8)
    }
    #[doc = "Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<AFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC’97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<AFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
#[doc = "SAI frame configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRCRrs;
impl crate::RegisterSpec for AFRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrcr::R`](R) reader structure"]
impl crate::Readable for AFRCRrs {}
#[doc = "`write(|w| ..)` method takes [`afrcr::W`](W) writer structure"]
impl crate::Writable for AFRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRCR to value 0x07"]
impl crate::Resettable for AFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
