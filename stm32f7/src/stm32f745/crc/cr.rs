#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "RESET bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    #[doc = "1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    Reset = 1,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - RESET bit"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Reset)
    }
}
#[doc = "Polynomial size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE {
    #[doc = "0: 32-bit polynomial"]
    Polysize32 = 0,
    #[doc = "1: 16-bit polynomial"]
    Polysize16 = 1,
    #[doc = "2: 8-bit polynomial"]
    Polysize8 = 2,
    #[doc = "3: 7-bit polynomial"]
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
#[doc = "Field `POLYSIZE` writer - Polynomial size"]
pub type POLYSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, POLYSIZE>;
impl<'a, REG> POLYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit polynomial"]
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize32)
    }
    #[doc = "16-bit polynomial"]
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize16)
    }
    #[doc = "8-bit polynomial"]
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize8)
    }
    #[doc = "7-bit polynomial"]
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE::Polysize7)
    }
}
#[doc = "Reverse input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN {
    #[doc = "0: Bit order not affected"]
    Normal = 0,
    #[doc = "1: Bit reversal done by byte"]
    Byte = 1,
    #[doc = "2: Bit reversal done by half-word"]
    HalfWord = 2,
    #[doc = "3: Bit reversal done by word"]
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
#[doc = "Field `REV_IN` writer - Reverse input data"]
pub type REV_IN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REV_IN>;
impl<'a, REG> REV_IN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Normal)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Byte)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::HalfWord)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN::Word)
    }
}
#[doc = "Reverse output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT {
    #[doc = "0: Bit order not affected"]
    Normal = 0,
    #[doc = "1: Bit reversed output"]
    Reversed = 1,
}
impl From<REV_OUT> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG, REV_OUT>;
impl<'a, REG> REV_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::Normal)
    }
    #[doc = "Bit reversed output"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT::Reversed)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<CRrs> {
        REV_IN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<CRrs> {
        REV_OUT_W::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
