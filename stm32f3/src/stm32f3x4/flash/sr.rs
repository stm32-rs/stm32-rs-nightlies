///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    ///0: No write/erase operation is in progress
    Inactive = 0,
    ///1: No write/erase operation is in progress
    Active = 1,
}
impl From<BSYR> for bool {
    #[inline(always)]
    fn from(variant: BSYR) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader<BSYR>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSYR {
        match self.bits {
            false => BSYR::Inactive,
            true => BSYR::Active,
        }
    }
    ///No write/erase operation is in progress
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSYR::Inactive
    }
    ///No write/erase operation is in progress
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR::Active
    }
}
/**Programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRR {
    ///0: No programming error occurred
    NoError = 0,
    ///1: A programming error occurred
    Error = 1,
}
impl From<PGERRR> for bool {
    #[inline(always)]
    fn from(variant: PGERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGERR` reader - Programming error
pub type PGERR_R = crate::BitReader<PGERRR>;
impl PGERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGERRR {
        match self.bits {
            false => PGERRR::NoError,
            true => PGERRR::Error,
        }
    }
    ///No programming error occurred
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERRR::NoError
    }
    ///A programming error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERRR::Error
    }
}
/**Programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRW {
    ///1: Reset programming error
    Reset = 1,
}
impl From<PGERRW> for bool {
    #[inline(always)]
    fn from(variant: PGERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGERR` writer - Programming error
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG, PGERRW>;
impl<'a, REG> PGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset programming error
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PGERRW::Reset)
    }
}
/**Write protection error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRTERRR {
    ///0: No write protection error occurred
    NoError = 0,
    ///1: A write protection error occurred
    Error = 1,
}
impl From<WRPRTERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPRTERR` reader - Write protection error
pub type WRPRTERR_R = crate::BitReader<WRPRTERRR>;
impl WRPRTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRPRTERRR {
        match self.bits {
            false => WRPRTERRR::NoError,
            true => WRPRTERRR::Error,
        }
    }
    ///No write protection error occurred
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRTERRR::NoError
    }
    ///A write protection error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRTERRR::Error
    }
}
/**Write protection error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRTERRW {
    ///1: Reset write protection error
    Reset = 1,
}
impl From<WRPRTERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPRTERR` writer - Write protection error
pub type WRPRTERR_W<'a, REG> = crate::BitWriter<'a, REG, WRPRTERRW>;
impl<'a, REG> WRPRTERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset write protection error
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRTERRW::Reset)
    }
}
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR {
    ///0: No EOP event occurred
    NoEvent = 0,
    ///1: An EOP event occurred
    Event = 1,
}
impl From<EOPR> for bool {
    #[inline(always)]
    fn from(variant: EOPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOPR>;
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOPR {
        match self.bits {
            false => EOPR::NoEvent,
            true => EOPR::Event,
        }
    }
    ///No EOP event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR::NoEvent
    }
    ///An EOP event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR::Event
    }
}
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    ///1: Reset EOP event
    Reset = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset EOP event
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Reset)
    }
}
impl R {
    ///Bit 0 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Programming error
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("wrprterr", &self.wrprterr())
            .field("pgerr", &self.pgerr())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 2 - Programming error
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W<'_, SRrs> {
        PGERR_W::new(self, 2)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WRPRTERR_W<'_, SRrs> {
        WRPRTERR_W::new(self, 4)
    }
    ///Bit 5 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 5)
    }
}
/**Flash status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#FLASH:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
