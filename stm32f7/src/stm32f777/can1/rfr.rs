///Register `RF%sR` reader
pub type R = crate::R<RFRrs>;
///Register `RF%sR` writer
pub type W = crate::W<RFRrs>;
///Field `FMP` reader - FMP0
pub type FMP_R = crate::FieldReader;
/**FULL0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0R {
    ///0: FIFO x is not full
    NotFull = 0,
    ///1: FIFO x is full
    Full = 1,
}
impl From<FULL0R> for bool {
    #[inline(always)]
    fn from(variant: FULL0R) -> Self {
        variant as u8 != 0
    }
}
///Field `FULL` reader - FULL0
pub type FULL_R = crate::BitReader<FULL0R>;
impl FULL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FULL0R {
        match self.bits {
            false => FULL0R::NotFull,
            true => FULL0R::Full,
        }
    }
    ///FIFO x is not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FULL0R::NotFull
    }
    ///FIFO x is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FULL0R::Full
    }
}
/**FULL0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0W {
    ///1: Clear flag
    Clear = 1,
}
impl From<FULL0W> for bool {
    #[inline(always)]
    fn from(variant: FULL0W) -> Self {
        variant as u8 != 0
    }
}
///Field `FULL` writer - FULL0
pub type FULL_W<'a, REG> = crate::BitWriter<'a, REG, FULL0W>;
impl<'a, REG> FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FULL0W::Clear)
    }
}
/**FOVR0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0R {
    ///0: No FIFO x overrun
    NoOverrun = 0,
    ///1: FIFO x overrun
    Overrun = 1,
}
impl From<FOVR0R> for bool {
    #[inline(always)]
    fn from(variant: FOVR0R) -> Self {
        variant as u8 != 0
    }
}
///Field `FOVR` reader - FOVR0
pub type FOVR_R = crate::BitReader<FOVR0R>;
impl FOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FOVR0R {
        match self.bits {
            false => FOVR0R::NoOverrun,
            true => FOVR0R::Overrun,
        }
    }
    ///No FIFO x overrun
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == FOVR0R::NoOverrun
    }
    ///FIFO x overrun
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == FOVR0R::Overrun
    }
}
/**FOVR0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0W {
    ///1: Clear flag
    Clear = 1,
}
impl From<FOVR0W> for bool {
    #[inline(always)]
    fn from(variant: FOVR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `FOVR` writer - FOVR0
pub type FOVR_W<'a, REG> = crate::BitWriter<'a, REG, FOVR0W>;
impl<'a, REG> FOVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FOVR0W::Clear)
    }
}
/**RFOM0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOM0W {
    ///1: Set by software to release the output mailbox of the FIFO
    Release = 1,
}
impl From<RFOM0W> for bool {
    #[inline(always)]
    fn from(variant: RFOM0W) -> Self {
        variant as u8 != 0
    }
}
///Field `RFOM` reader - RFOM0
pub type RFOM_R = crate::BitReader<RFOM0W>;
impl RFOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RFOM0W> {
        match self.bits {
            true => Some(RFOM0W::Release),
            _ => None,
        }
    }
    ///Set by software to release the output mailbox of the FIFO
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == RFOM0W::Release
    }
}
///Field `RFOM` writer - RFOM0
pub type RFOM_W<'a, REG> = crate::BitWriter<'a, REG, RFOM0W>;
impl<'a, REG> RFOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set by software to release the output mailbox of the FIFO
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(RFOM0W::Release)
    }
}
impl R {
    ///Bits 0:1 - FMP0
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFR")
            .field("rfom", &self.rfom())
            .field("fovr", &self.fovr())
            .field("full", &self.full())
            .field("fmp", &self.fmp())
            .finish()
    }
}
impl W {
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W<'_, RFRrs> {
        FULL_W::new(self, 3)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr(&mut self) -> FOVR_W<'_, RFRrs> {
        FOVR_W::new(self, 4)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom(&mut self) -> RFOM_W<'_, RFRrs> {
        RFOM_W::new(self, 5)
    }
}
/**receive FIFO %s register

You can [`read`](crate::Reg::read) this register and get [`rfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#CAN1:RF[0]R)*/
pub struct RFRrs;
impl crate::RegisterSpec for RFRrs {
    type Ux = u32;
}
///`read()` method returns [`rfr::R`](R) reader structure
impl crate::Readable for RFRrs {}
///`write(|w| ..)` method takes [`rfr::W`](W) writer structure
impl crate::Writable for RFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF%sR to value 0
impl crate::Resettable for RFRrs {}
