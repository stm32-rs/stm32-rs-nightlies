#[doc = "Register `DCR1` reader"]
pub type R = crate::R<DCR1rs>;
#[doc = "Register `DCR1` writer"]
pub type W = crate::W<DCR1rs>;
#[doc = "Mode 0 / mode 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMODE {
    #[doc = "0: CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0"]
    Mode0 = 0,
    #[doc = "1: CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3"]
    Mode3 = 1,
}
impl From<CKMODE> for bool {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub type CKMODE_R = crate::BitReader<CKMODE>;
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE {
        match self.bits {
            false => CKMODE::Mode0,
            true => CKMODE::Mode3,
        }
    }
    #[doc = "CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == CKMODE::Mode0
    }
    #[doc = "CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == CKMODE::Mode3
    }
}
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK must stay low while NCS is high (chip-select released). This is referred to as Mode 0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Mode0)
    }
    #[doc = "CLK must stay high while NCS is high (chip-select released). This is referred to as Mode 3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Mode3)
    }
}
#[doc = "Free running clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRCK {
    #[doc = "0: CLK is not free running"]
    Disabled = 0,
    #[doc = "1: CLK is free running (always provided)"]
    Enabled = 1,
}
impl From<FRCK> for bool {
    #[inline(always)]
    fn from(variant: FRCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCK` reader - Free running clock"]
pub type FRCK_R = crate::BitReader<FRCK>;
impl FRCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRCK {
        match self.bits {
            false => FRCK::Disabled,
            true => FRCK::Enabled,
        }
    }
    #[doc = "CLK is not free running"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRCK::Disabled
    }
    #[doc = "CLK is free running (always provided)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRCK::Enabled
    }
}
#[doc = "Field `FRCK` writer - Free running clock"]
pub type FRCK_W<'a, REG> = crate::BitWriter<'a, REG, FRCK>;
impl<'a, REG> FRCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK is not free running"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRCK::Disabled)
    }
    #[doc = "CLK is free running (always provided)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FRCK::Enabled)
    }
}
#[doc = "Delay block bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBYP {
    #[doc = "0: The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)"]
    DelayBlockEnabled = 0,
    #[doc = "1: The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block"]
    DelayBlockBypassed = 1,
}
impl From<DLYBYP> for bool {
    #[inline(always)]
    fn from(variant: DLYBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYBYP` reader - Delay block bypass"]
pub type DLYBYP_R = crate::BitReader<DLYBYP>;
impl DLYBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYBYP {
        match self.bits {
            false => DLYBYP::DelayBlockEnabled,
            true => DLYBYP::DelayBlockBypassed,
        }
    }
    #[doc = "The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)"]
    #[inline(always)]
    pub fn is_delay_block_enabled(&self) -> bool {
        *self == DLYBYP::DelayBlockEnabled
    }
    #[doc = "The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block"]
    #[inline(always)]
    pub fn is_delay_block_bypassed(&self) -> bool {
        *self == DLYBYP::DelayBlockBypassed
    }
}
#[doc = "Field `DLYBYP` writer - Delay block bypass"]
pub type DLYBYP_W<'a, REG> = crate::BitWriter<'a, REG, DLYBYP>;
impl<'a, REG> DLYBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The internal sampling clock (called feedback clock) or the DQS data strobe external signal is delayed by the delay block (for more details on this block, refer to the dedicated section of the reference manual as it is not part of the OCTOSPI peripheral)"]
    #[inline(always)]
    pub fn delay_block_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBYP::DelayBlockEnabled)
    }
    #[doc = "The delay block is bypassed, so the internal sampling clock or the DQS data strobe external signal is not affected by the delay block. The delay is shorter than when the delay block is not bypassed, even with the delay value set to minimum value in delay block"]
    #[inline(always)]
    pub fn delay_block_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBYP::DelayBlockBypassed)
    }
}
#[doc = "Field `CSHT` reader - Chip-select high time"]
pub type CSHT_R = crate::FieldReader;
#[doc = "Field `CSHT` writer - Chip-select high time"]
pub type CSHT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `DEVSIZE` reader - Device size"]
pub type DEVSIZE_R = crate::FieldReader;
#[doc = "Field `DEVSIZE` writer - Device size"]
pub type DEVSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Memory type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTYP {
    #[doc = "0: Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    MicronMode = 0,
    #[doc = "1: Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    MacronixMode = 1,
    #[doc = "2: Standard Mode"]
    StandardMode = 2,
    #[doc = "3: Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping"]
    MacronixRamMode = 3,
    #[doc = "4: HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected"]
    HyperBusMemoryMode = 4,
    #[doc = "5: HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used"]
    HyperBusMode = 5,
}
impl From<MTYP> for u8 {
    #[inline(always)]
    fn from(variant: MTYP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTYP {
    type Ux = u8;
}
#[doc = "Field `MTYP` reader - Memory type"]
pub type MTYP_R = crate::FieldReader<MTYP>;
impl MTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MTYP> {
        match self.bits {
            0 => Some(MTYP::MicronMode),
            1 => Some(MTYP::MacronixMode),
            2 => Some(MTYP::StandardMode),
            3 => Some(MTYP::MacronixRamMode),
            4 => Some(MTYP::HyperBusMemoryMode),
            5 => Some(MTYP::HyperBusMode),
            _ => None,
        }
    }
    #[doc = "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn is_micron_mode(&self) -> bool {
        *self == MTYP::MicronMode
    }
    #[doc = "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn is_macronix_mode(&self) -> bool {
        *self == MTYP::MacronixMode
    }
    #[doc = "Standard Mode"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        *self == MTYP::StandardMode
    }
    #[doc = "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping"]
    #[inline(always)]
    pub fn is_macronix_ram_mode(&self) -> bool {
        *self == MTYP::MacronixRamMode
    }
    #[doc = "HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected"]
    #[inline(always)]
    pub fn is_hyper_bus_memory_mode(&self) -> bool {
        *self == MTYP::HyperBusMemoryMode
    }
    #[doc = "HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used"]
    #[inline(always)]
    pub fn is_hyper_bus_mode(&self) -> bool {
        *self == MTYP::HyperBusMode
    }
}
#[doc = "Field `MTYP` writer - Memory type"]
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MTYP>;
impl<'a, REG> MTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Micron mode, D0/D1 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn micron_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::MicronMode)
    }
    #[doc = "Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes"]
    #[inline(always)]
    pub fn macronix_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::MacronixMode)
    }
    #[doc = "Standard Mode"]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::StandardMode)
    }
    #[doc = "Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol in Single-, Dual-, Quad- and Octal-SPI modes with dedicated address mapping"]
    #[inline(always)]
    pub fn macronix_ram_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::MacronixRamMode)
    }
    #[doc = "HyperBus memory mode, the protocol follows the HyperBus specification. 8-data-bit DTR mode must be selected"]
    #[inline(always)]
    pub fn hyper_bus_memory_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::HyperBusMemoryMode)
    }
    #[doc = "HyperBus register mode, addressing register space. The memory-mapped accesses in this mode must be non-cacheable, or Indirect read/write modes must be used"]
    #[inline(always)]
    pub fn hyper_bus_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MTYP::HyperBusMode)
    }
}
impl R {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Chip-select high time"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<DCR1rs> {
        CKMODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Free running clock"]
    #[inline(always)]
    #[must_use]
    pub fn frck(&mut self) -> FRCK_W<DCR1rs> {
        FRCK_W::new(self, 1)
    }
    #[doc = "Bit 3 - Delay block bypass"]
    #[inline(always)]
    #[must_use]
    pub fn dlybyp(&mut self) -> DLYBYP_W<DCR1rs> {
        DLYBYP_W::new(self, 3)
    }
    #[doc = "Bits 8:13 - Chip-select high time"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<DCR1rs> {
        CSHT_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Device size"]
    #[inline(always)]
    #[must_use]
    pub fn devsize(&mut self) -> DEVSIZE_W<DCR1rs> {
        DEVSIZE_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<DCR1rs> {
        MTYP_W::new(self, 24)
    }
}
#[doc = "device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR1rs;
impl crate::RegisterSpec for DCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr1::R`](R) reader structure"]
impl crate::Readable for DCR1rs {}
#[doc = "`write(|w| ..)` method takes [`dcr1::W`](W) writer structure"]
impl crate::Writable for DCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for DCR1rs {
    const RESET_VALUE: u32 = 0;
}
