///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    ///1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    Reset = 1,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_R = crate::BitReader<RESETW>;
impl RESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RESETW> {
        match self.bits {
            true => Some(RESETW::Reset),
            _ => None,
        }
    }
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETW::Reset
    }
}
///Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Reset)
    }
}
/**Polynomial size These bits control the size of the polynomial.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE {
    ///0: 32-bit polynomial
    Polysize32 = 0,
    ///1: 16-bit polynomial
    Polysize16 = 1,
    ///2: 8-bit polynomial
    Polysize8 = 2,
    ///3: 7-bit polynomial
    Polysize7 = 3,
}
impl From<POLYSIZE> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLYSIZE {
    type Ux = u8;
}
impl crate::IsEnum for POLYSIZE {}
///Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial.
pub type POLYSIZE_R = crate::FieldReader<POLYSIZE>;
impl POLYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POLYSIZE {
        match self.bits {
            0 => POLYSIZE::Polysize32,
            1 => POLYSIZE::Polysize16,
            2 => POLYSIZE::Polysize8,
            3 => POLYSIZE::Polysize7,
            _ => unreachable!(),
        }
    }
    ///32-bit polynomial
    #[inline(always)]
    pub fn is_polysize32(&self) -> bool {
        *self == POLYSIZE::Polysize32
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn is_polysize16(&self) -> bool {
        *self == POLYSIZE::Polysize16
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn is_polysize8(&self) -> bool {
        *self == POLYSIZE::Polysize8
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn is_polysize7(&self) -> bool {
        *self == POLYSIZE::Polysize7
    }
}
///Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial.
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, POLYSIZE, crate::Safe>;
impl<'a, REG> POLYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///32-bit polynomial
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize32)
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize16)
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize8)
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize7)
    }
}
/**Reverse input data This bitfield controls the reversal of the bit order of the input data

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversal done by byte
    Byte = 1,
    ///2: Bit reversal done by half-word
    HalfWord = 2,
    ///3: Bit reversal done by word
    Word = 3,
}
impl From<REV_IN> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_IN {
    type Ux = u8;
}
impl crate::IsEnum for REV_IN {}
///Field `REV_IN` reader - Reverse input data This bitfield controls the reversal of the bit order of the input data
pub type REV_IN_R = crate::FieldReader<REV_IN>;
impl REV_IN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REV_IN {
        match self.bits {
            0 => REV_IN::Normal,
            1 => REV_IN::Byte,
            2 => REV_IN::HalfWord,
            3 => REV_IN::Word,
            _ => unreachable!(),
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_IN::Normal
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == REV_IN::Byte
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == REV_IN::HalfWord
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == REV_IN::Word
    }
}
///Field `REV_IN` writer - Reverse input data This bitfield controls the reversal of the bit order of the input data
pub type REV_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REV_IN, crate::Safe>;
impl<'a, REG> REV_IN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Normal)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Byte)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::HalfWord)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Word)
    }
}
/**Reverse output data This bitfield controls the reversal of the bit order of the output data.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_OUT {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversed output
    Reversed = 1,
}
impl From<REV_OUT> for u8 {
    #[inline(always)]
    fn from(variant: REV_OUT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_OUT {
    type Ux = u8;
}
impl crate::IsEnum for REV_OUT {}
///Field `REV_OUT` reader - Reverse output data This bitfield controls the reversal of the bit order of the output data.
pub type REV_OUT_R = crate::FieldReader<REV_OUT>;
impl REV_OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV_OUT> {
        match self.bits {
            0 => Some(REV_OUT::Normal),
            1 => Some(REV_OUT::Reversed),
            _ => None,
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_OUT::Normal
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == REV_OUT::Reversed
    }
}
///Field `REV_OUT` writer - Reverse output data This bitfield controls the reversal of the bit order of the output data.
pub type REV_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REV_OUT>;
impl<'a, REG> REV_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::Normal)
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::Reversed)
    }
}
///Field `RTYPE_IN` reader - Reverse type input This bit controls the reversal granularity of the input data.
pub type RTYPE_IN_R = crate::BitReader;
///Field `RTYPE_IN` writer - Reverse type input This bit controls the reversal granularity of the input data.
pub type RTYPE_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTYPE_OUT` reader - Reverse type output This bit controls the reversal granularity of the output data.
pub type RTYPE_OUT_R = crate::BitReader;
///Field `RTYPE_OUT` writer - Reverse type output This bit controls the reversal granularity of the output data.
pub type RTYPE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial.
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Reverse input data This bitfield controls the reversal of the bit order of the input data
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bits 7:8 - Reverse output data This bitfield controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Reverse type input This bit controls the reversal granularity of the input data.
    #[inline(always)]
    pub fn rtype_in(&self) -> RTYPE_IN_R {
        RTYPE_IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reverse type output This bit controls the reversal granularity of the output data.
    #[inline(always)]
    pub fn rtype_out(&self) -> RTYPE_OUT_R {
        RTYPE_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("reset", &self.reset())
            .field("polysize", &self.polysize())
            .field("rev_in", &self.rev_in())
            .field("rev_out", &self.rev_out())
            .field("rtype_in", &self.rtype_in())
            .field("rtype_out", &self.rtype_out())
            .finish()
    }
}
impl W {
    ///Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 0)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial.
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W<CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    ///Bits 5:6 - Reverse input data This bitfield controls the reversal of the bit order of the input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W<CRrs> {
        REV_IN_W::new(self, 5)
    }
    ///Bits 7:8 - Reverse output data This bitfield controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W<CRrs> {
        REV_OUT_W::new(self, 7)
    }
    ///Bit 9 - Reverse type input This bit controls the reversal granularity of the input data.
    #[inline(always)]
    pub fn rtype_in(&mut self) -> RTYPE_IN_W<CRrs> {
        RTYPE_IN_W::new(self, 9)
    }
    ///Bit 10 - Reverse type output This bit controls the reversal granularity of the output data.
    #[inline(always)]
    pub fn rtype_out(&mut self) -> RTYPE_OUT_W<CRrs> {
        RTYPE_OUT_W::new(self, 10)
    }
}
/**CRC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#CRC:CR)*/
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
