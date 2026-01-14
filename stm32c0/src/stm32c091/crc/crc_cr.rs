///Register `CRC_CR` reader
pub type R = crate::R<CRC_CRrs>;
///Register `CRC_CR` writer
pub type W = crate::W<CRC_CRrs>;
///Field `RESET` reader - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_R = crate::BitReader;
///Field `RESET` writer - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Polynomial size These bits control the size of the polynomial.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE {
    ///0: 32 bit polynomial
    B0x0 = 0,
    ///1: 16 bit polynomial
    B0x1 = 1,
    ///2: 8 bit polynomial
    B0x2 = 2,
    ///3: 7 bit polynomial
    B0x3 = 3,
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
            0 => POLYSIZE::B0x0,
            1 => POLYSIZE::B0x1,
            2 => POLYSIZE::B0x2,
            3 => POLYSIZE::B0x3,
            _ => unreachable!(),
        }
    }
    ///32 bit polynomial
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == POLYSIZE::B0x0
    }
    ///16 bit polynomial
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == POLYSIZE::B0x1
    }
    ///8 bit polynomial
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == POLYSIZE::B0x2
    }
    ///7 bit polynomial
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == POLYSIZE::B0x3
    }
}
///Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial.
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, POLYSIZE, crate::Safe>;
impl<'a, REG> POLYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///32 bit polynomial
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::B0x0)
    }
    ///16 bit polynomial
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::B0x1)
    }
    ///8 bit polynomial
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::B0x2)
    }
    ///7 bit polynomial
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::B0x3)
    }
}
/**Reverse input data This bitfield controls the reversal of the bit order of the input data

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN {
    ///0: Bit order not affected
    B0x0 = 0,
    ///1: Bit reversal done by byte
    B0x1 = 1,
    ///2: Bit reversal done by half-word
    B0x2 = 2,
    ///3: Bit reversal done by word
    B0x3 = 3,
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
            0 => REV_IN::B0x0,
            1 => REV_IN::B0x1,
            2 => REV_IN::B0x2,
            3 => REV_IN::B0x3,
            _ => unreachable!(),
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_IN::B0x0
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_IN::B0x1
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == REV_IN::B0x2
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == REV_IN::B0x3
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
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::B0x0)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::B0x1)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::B0x2)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::B0x3)
    }
}
/**Reverse output data This bit controls the reversal of the bit order of the output data.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT {
    ///0: Bit order not affected
    B0x0 = 0,
    ///1: Bit-reversed output format
    B0x1 = 1,
}
impl From<REV_OUT> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT) -> Self {
        variant as u8 != 0
    }
}
///Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data.
pub type REV_OUT_R = crate::BitReader<REV_OUT>;
impl REV_OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REV_OUT {
        match self.bits {
            false => REV_OUT::B0x0,
            true => REV_OUT::B0x1,
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_OUT::B0x0
    }
    ///Bit-reversed output format
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_OUT::B0x1
    }
}
///Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data.
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG, REV_OUT>;
impl<'a, REG> REV_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bit order not affected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::B0x0)
    }
    ///Bit-reversed output format
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::B0x1)
    }
}
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
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CR")
            .field("reset", &self.reset())
            .field("polysize", &self.polysize())
            .field("rev_in", &self.rev_in())
            .field("rev_out", &self.rev_out())
            .finish()
    }
}
impl W {
    ///Bit 0 - RESET bit This bit is set by software to reset the CRC calculation unit and set the data register to the value stored in the CRC_INIT register. This bit can only be set, it is automatically cleared by hardware
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, CRC_CRrs> {
        RESET_W::new(self, 0)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial.
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W<'_, CRC_CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    ///Bits 5:6 - Reverse input data This bitfield controls the reversal of the bit order of the input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W<'_, CRC_CRrs> {
        REV_IN_W::new(self, 5)
    }
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W<'_, CRC_CRrs> {
        REV_OUT_W::new(self, 7)
    }
}
/**CRC control register

You can [`read`](crate::Reg::read) this register and get [`crc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#CRC:CRC_CR)*/
pub struct CRC_CRrs;
impl crate::RegisterSpec for CRC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`crc_cr::R`](R) reader structure
impl crate::Readable for CRC_CRrs {}
///`write(|w| ..)` method takes [`crc_cr::W`](W) writer structure
impl crate::Writable for CRC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRC_CR to value 0
impl crate::Resettable for CRC_CRrs {}
