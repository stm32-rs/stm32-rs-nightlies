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
    ///1: A write/erase operation is in progress
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
    ///A write/erase operation is in progress
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR::Active
    }
}
/**Programming error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERR {
    ///0: No programming error occurred
    NoError = 0,
    ///1: A programming error occurred
    Error = 1,
}
impl From<PGERR> for bool {
    #[inline(always)]
    fn from(variant: PGERR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGERR` reader - Programming error
pub type PGERR_R = crate::BitReader<PGERR>;
impl PGERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGERR {
        match self.bits {
            false => PGERR::NoError,
            true => PGERR::Error,
        }
    }
    ///No programming error occurred
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERR::NoError
    }
    ///A programming error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERR::Error
    }
}
///Field `PGERR` writer - Programming error
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG, PGERR>;
impl<'a, REG> PGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No programming error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(PGERR::NoError)
    }
    ///A programming error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(PGERR::Error)
    }
}
/**Write protection error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRT {
    ///0: No write protection error occurred
    NoError = 0,
    ///1: A write protection error occurred
    Error = 1,
}
impl From<WRPRT> for bool {
    #[inline(always)]
    fn from(variant: WRPRT) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPRT` reader - Write protection error
pub type WRPRT_R = crate::BitReader<WRPRT>;
impl WRPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRPRT {
        match self.bits {
            false => WRPRT::NoError,
            true => WRPRT::Error,
        }
    }
    ///No write protection error occurred
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRT::NoError
    }
    ///A write protection error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRT::Error
    }
}
///Field `WRPRT` writer - Write protection error
pub type WRPRT_W<'a, REG> = crate::BitWriter<'a, REG, WRPRT>;
impl<'a, REG> WRPRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No write protection error occurred
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRT::NoError)
    }
    ///A write protection error occurred
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRT::Error)
    }
}
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOP {
    ///0: No EOP operation occurred
    NoEvent = 0,
    ///1: An EOP event occurred
    Event = 1,
}
impl From<EOP> for bool {
    #[inline(always)]
    fn from(variant: EOP) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOP>;
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOP {
        match self.bits {
            false => EOP::NoEvent,
            true => EOP::Event,
        }
    }
    ///No EOP operation occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP::NoEvent
    }
    ///An EOP event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP::Event
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOP>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No EOP operation occurred
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(EOP::NoEvent)
    }
    ///An EOP event occurred
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(EOP::Event)
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
    pub fn wrprt(&self) -> WRPRT_R {
        WRPRT_R::new(((self.bits >> 4) & 1) != 0)
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
            .field("wrprt", &self.wrprt())
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
    pub fn wrprt(&mut self) -> WRPRT_W<'_, SRrs> {
        WRPRT_W::new(self, 4)
    }
    ///Bit 5 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 5)
    }
}
/**Flash status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#Flash:SR)*/
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
