///Register `FCR` reader
pub type R = crate::R<FCRrs>;
///Register `FCR` writer
pub type W = crate::W<FCRrs>;
/**FIFO threshold selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTH {
    ///0: 1/4 full FIFO
    Quarter = 0,
    ///1: 1/2 full FIFO
    Half = 1,
    ///2: 3/4 full FIFO
    ThreeQuarters = 2,
    ///3: Full FIFO
    Full = 3,
}
impl From<FTH> for u8 {
    #[inline(always)]
    fn from(variant: FTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTH {
    type Ux = u8;
}
impl crate::IsEnum for FTH {}
///Field `FTH` reader - FIFO threshold selection
pub type FTH_R = crate::FieldReader<FTH>;
impl FTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTH {
        match self.bits {
            0 => FTH::Quarter,
            1 => FTH::Half,
            2 => FTH::ThreeQuarters,
            3 => FTH::Full,
            _ => unreachable!(),
        }
    }
    ///1/4 full FIFO
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTH::Quarter
    }
    ///1/2 full FIFO
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTH::Half
    }
    ///3/4 full FIFO
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTH::ThreeQuarters
    }
    ///Full FIFO
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH::Full
    }
}
///Field `FTH` writer - FIFO threshold selection
pub type FTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FTH, crate::Safe>;
impl<'a, REG> FTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1/4 full FIFO
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Quarter)
    }
    ///1/2 full FIFO
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Half)
    }
    ///3/4 full FIFO
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::ThreeQuarters)
    }
    ///Full FIFO
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Full)
    }
}
/**Direct mode disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMDIS {
    ///0: Direct mode is enabled
    Enabled = 0,
    ///1: Direct mode is disabled
    Disabled = 1,
}
impl From<DMDIS> for bool {
    #[inline(always)]
    fn from(variant: DMDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `DMDIS` reader - Direct mode disable
pub type DMDIS_R = crate::BitReader<DMDIS>;
impl DMDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMDIS {
        match self.bits {
            false => DMDIS::Enabled,
            true => DMDIS::Disabled,
        }
    }
    ///Direct mode is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMDIS::Enabled
    }
    ///Direct mode is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMDIS::Disabled
    }
}
///Field `DMDIS` writer - Direct mode disable
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG, DMDIS>;
impl<'a, REG> DMDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Direct mode is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMDIS::Enabled)
    }
    ///Direct mode is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMDIS::Disabled)
    }
}
/**FIFO status

Value on reset: 4*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS {
    ///0: 0 < fifo_level < 1/4
    Quarter1 = 0,
    ///1: 1/4 <= fifo_level < 1/2
    Quarter2 = 1,
    ///2: 1/2 <= fifo_level < 3/4
    Quarter3 = 2,
    ///3: 3/4 <= fifo_level < full
    Quarter4 = 3,
    ///4: FIFO is empty
    Empty = 4,
    ///5: FIFO is full
    Full = 5,
}
impl From<FS> for u8 {
    #[inline(always)]
    fn from(variant: FS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FS {
    type Ux = u8;
}
impl crate::IsEnum for FS {}
///Field `FS` reader - FIFO status
pub type FS_R = crate::FieldReader<FS>;
impl FS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FS> {
        match self.bits {
            0 => Some(FS::Quarter1),
            1 => Some(FS::Quarter2),
            2 => Some(FS::Quarter3),
            3 => Some(FS::Quarter4),
            4 => Some(FS::Empty),
            5 => Some(FS::Full),
            _ => None,
        }
    }
    ///0 < fifo_level < 1/4
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FS::Quarter1
    }
    ///1/4 <= fifo_level < 1/2
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FS::Quarter2
    }
    ///1/2 <= fifo_level < 3/4
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FS::Quarter3
    }
    ///3/4 <= fifo_level < full
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FS::Quarter4
    }
    ///FIFO is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FS::Empty
    }
    ///FIFO is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FS::Full
    }
}
/**FIFO error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE {
    ///0: FE interrupt disabled
    Disabled = 0,
    ///1: FE interrupt enabled
    Enabled = 1,
}
impl From<FEIE> for bool {
    #[inline(always)]
    fn from(variant: FEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `FEIE` reader - FIFO error interrupt enable
pub type FEIE_R = crate::BitReader<FEIE>;
impl FEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEIE {
        match self.bits {
            false => FEIE::Disabled,
            true => FEIE::Enabled,
        }
    }
    ///FE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE::Disabled
    }
    ///FE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE::Enabled
    }
}
///Field `FEIE` writer - FIFO error interrupt enable
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG, FEIE>;
impl<'a, REG> FEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE::Disabled)
    }
    ///FE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE::Enabled)
    }
}
impl R {
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - FIFO status
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("feie", &self.feie())
            .field("fs", &self.fs())
            .field("dmdis", &self.dmdis())
            .field("fth", &self.fth())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FIFO threshold selection
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<'_, FCRrs> {
        FTH_W::new(self, 0)
    }
    ///Bit 2 - Direct mode disable
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W<'_, FCRrs> {
        DMDIS_W::new(self, 2)
    }
    ///Bit 7 - FIFO error interrupt enable
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<'_, FCRrs> {
        FEIE_W::new(self, 7)
    }
}
/**stream x FIFO control register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`read()` method returns [`fcr::R`](R) reader structure
impl crate::Readable for FCRrs {}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0x21
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
