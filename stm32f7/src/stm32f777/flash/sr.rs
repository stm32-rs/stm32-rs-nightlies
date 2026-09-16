///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR {
    ///0: No error
    Inactive = 0,
    ///1: One or more Flash operations has/have completed successfully
    Active = 1,
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
            false => EOPR::Inactive,
            true => EOPR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == EOPR::Inactive
    }
    ///One or more Flash operations has/have completed successfully
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EOPR::Active
    }
}
/**End of operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, REG> = crate::BitWriter1C<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Clear)
    }
}
/**Operation error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRR {
    ///0: No error
    Inactive = 0,
    ///1: A Flash operation request is detected and cannot be run because of parallelism
    Active = 1,
}
impl From<OPERRR> for bool {
    #[inline(always)]
    fn from(variant: OPERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader<OPERRR>;
impl OPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OPERRR {
        match self.bits {
            false => OPERRR::Inactive,
            true => OPERRR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OPERRR::Inactive
    }
    ///A Flash operation request is detected and cannot be run because of parallelism
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OPERRR::Active
    }
}
/**Operation error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<OPERRW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, OPERRW>;
impl<'a, REG> OPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPERRW::Clear)
    }
}
/**Write protection error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR {
    ///0: No error
    Inactive = 0,
    ///1: The address to be erased/programmed belongs to a write-protected part of the Flash memory
    Active = 1,
}
impl From<WRPERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` reader - Write protection error
pub type WRPERR_R = crate::BitReader<WRPERRR>;
impl WRPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRPERRR {
        match self.bits {
            false => WRPERRR::Inactive,
            true => WRPERRR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WRPERRR::Inactive
    }
    ///The address to be erased/programmed belongs to a write-protected part of the Flash memory
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == WRPERRR::Active
    }
}
/**Write protection error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<WRPERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` writer - Write protection error
pub type WRPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, WRPERRW>;
impl<'a, REG> WRPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRW::Clear)
    }
}
/**Programming alignment error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR {
    ///0: No error
    Inactive = 0,
    ///1: The data to program cannot be contained in the same 128-bit Flash memory row
    Active = 1,
}
impl From<PGAERRR> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader<PGAERRR>;
impl PGAERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGAERRR {
        match self.bits {
            false => PGAERRR::Inactive,
            true => PGAERRR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PGAERRR::Inactive
    }
    ///The data to program cannot be contained in the same 128-bit Flash memory row
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PGAERRR::Active
    }
}
/**Programming alignment error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<PGAERRW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PGAERRW>;
impl<'a, REG> PGAERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGAERRW::Clear)
    }
}
/**Programming parallelism error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGPERRR {
    ///0: No error
    Inactive = 0,
    ///1: The size of the access during the program sequence doesn't correspond to the parallelism configuration PSIZE
    Active = 1,
}
impl From<PGPERRR> for bool {
    #[inline(always)]
    fn from(variant: PGPERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `PGPERR` reader - Programming parallelism error
pub type PGPERR_R = crate::BitReader<PGPERRR>;
impl PGPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PGPERRR {
        match self.bits {
            false => PGPERRR::Inactive,
            true => PGPERRR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PGPERRR::Inactive
    }
    ///The size of the access during the program sequence doesn't correspond to the parallelism configuration PSIZE
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PGPERRR::Active
    }
}
/**Programming parallelism error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGPERRW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<PGPERRW> for bool {
    #[inline(always)]
    fn from(variant: PGPERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGPERR` writer - Programming parallelism error
pub type PGPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PGPERRW>;
impl<'a, REG> PGPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGPERRW::Clear)
    }
}
/**Erase Sequence Error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSERRR {
    ///0: No error
    Inactive = 0,
    ///1: A write access to the Flash memory is performed by the code while the control register has not been correctly configured
    Active = 1,
}
impl From<ERSERRR> for bool {
    #[inline(always)]
    fn from(variant: ERSERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `ERSERR` reader - Erase Sequence Error
pub type ERSERR_R = crate::BitReader<ERSERRR>;
impl ERSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERSERRR {
        match self.bits {
            false => ERSERRR::Inactive,
            true => ERSERRR::Active,
        }
    }
    ///No error
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ERSERRR::Inactive
    }
    ///A write access to the Flash memory is performed by the code while the control register has not been correctly configured
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ERSERRR::Active
    }
}
/**Erase Sequence Error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSERRW {
    ///1: Clear error flag
    Clear = 1,
}
impl From<ERSERRW> for bool {
    #[inline(always)]
    fn from(variant: ERSERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `ERSERR` writer - Erase Sequence Error
pub type ERSERR_W<'a, REG> = crate::BitWriter1C<'a, REG, ERSERRW>;
impl<'a, REG> ERSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERSERRW::Clear)
    }
}
/**Busy

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    ///0: No Flash memory operation ongoing
    NotBusy = 0,
    ///1: Flash memory operation ongoing
    Busy = 1,
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
            false => BSYR::NotBusy,
            true => BSYR::Busy,
        }
    }
    ///No Flash memory operation ongoing
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR::NotBusy
    }
    ///Flash memory operation ongoing
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR::Busy
    }
}
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    pub fn pgperr(&self) -> PGPERR_R {
        PGPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Erase Sequence Error
    #[inline(always)]
    pub fn erserr(&self) -> ERSERR_R {
        ERSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("pgperr", &self.pgperr())
            .field("erserr", &self.erserr())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<'_, SRrs> {
        EOP_W::new(self, 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<'_, SRrs> {
        OPERR_W::new(self, 1)
    }
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<'_, SRrs> {
        WRPERR_W::new(self, 4)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<'_, SRrs> {
        PGAERR_W::new(self, 5)
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    pub fn pgperr(&mut self) -> PGPERR_W<'_, SRrs> {
        PGPERR_W::new(self, 6)
    }
    ///Bit 7 - Erase Sequence Error
    #[inline(always)]
    pub fn erserr(&mut self) -> ERSERR_W<'_, SRrs> {
        ERSERR_W::new(self, 7)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#FLASH:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf3;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
