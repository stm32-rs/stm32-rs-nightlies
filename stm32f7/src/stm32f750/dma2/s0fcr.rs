#[doc = "Register `S0FCR` reader"]
pub type R = crate::R<S0FCRrs>;
#[doc = "Register `S0FCR` writer"]
pub type W = crate::W<S0FCRrs>;
#[doc = "FIFO threshold selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTH {
    #[doc = "0: 1/4 full FIFO"]
    Quarter = 0,
    #[doc = "1: 1/2 full FIFO"]
    Half = 1,
    #[doc = "2: 3/4 full FIFO"]
    ThreeQuarters = 2,
    #[doc = "3: Full FIFO"]
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
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FTH_R = crate::FieldReader<FTH>;
impl FTH_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1/4 full FIFO"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTH::Quarter
    }
    #[doc = "1/2 full FIFO"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTH::Half
    }
    #[doc = "3/4 full FIFO"]
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == FTH::ThreeQuarters
    }
    #[doc = "Full FIFO"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH::Full
    }
}
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FTH>;
impl<'a, REG> FTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/4 full FIFO"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Quarter)
    }
    #[doc = "1/2 full FIFO"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Half)
    }
    #[doc = "3/4 full FIFO"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::ThreeQuarters)
    }
    #[doc = "Full FIFO"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(FTH::Full)
    }
}
#[doc = "Direct mode disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMDIS {
    #[doc = "0: Direct mode is enabled"]
    Enabled = 0,
    #[doc = "1: Direct mode is disabled"]
    Disabled = 1,
}
impl From<DMDIS> for bool {
    #[inline(always)]
    fn from(variant: DMDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DMDIS_R = crate::BitReader<DMDIS>;
impl DMDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMDIS {
        match self.bits {
            false => DMDIS::Enabled,
            true => DMDIS::Disabled,
        }
    }
    #[doc = "Direct mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMDIS::Enabled
    }
    #[doc = "Direct mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMDIS::Disabled
    }
}
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DMDIS_W<'a, REG> = crate::BitWriter<'a, REG, DMDIS>;
impl<'a, REG> DMDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMDIS::Enabled)
    }
    #[doc = "Direct mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMDIS::Disabled)
    }
}
#[doc = "FIFO status\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS {
    #[doc = "0: 0 &lt; fifo_level &lt; 1/4"]
    Quarter1 = 0,
    #[doc = "1: 1/4 &lt;= fifo_level &lt; 1/2"]
    Quarter2 = 1,
    #[doc = "2: 1/2 &lt;= fifo_level &lt; 3/4"]
    Quarter3 = 2,
    #[doc = "3: 3/4 &lt;= fifo_level &lt; full"]
    Quarter4 = 3,
    #[doc = "4: FIFO is empty"]
    Empty = 4,
    #[doc = "5: FIFO is full"]
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
#[doc = "Field `FS` reader - FIFO status"]
pub type FS_R = crate::FieldReader<FS>;
impl FS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "0 &lt; fifo_level &lt; 1/4"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FS::Quarter1
    }
    #[doc = "1/4 &lt;= fifo_level &lt; 1/2"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FS::Quarter2
    }
    #[doc = "1/2 &lt;= fifo_level &lt; 3/4"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FS::Quarter3
    }
    #[doc = "3/4 &lt;= fifo_level &lt; full"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FS::Quarter4
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FS::Empty
    }
    #[doc = "FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FS::Full
    }
}
#[doc = "FIFO error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE {
    #[doc = "0: FE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: FE interrupt enabled"]
    Enabled = 1,
}
impl From<FEIE> for bool {
    #[inline(always)]
    fn from(variant: FEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FEIE_R = crate::BitReader<FEIE>;
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEIE {
        match self.bits {
            false => FEIE::Disabled,
            true => FEIE::Enabled,
        }
    }
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE::Disabled
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE::Enabled
    }
}
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FEIE_W<'a, REG> = crate::BitWriter<'a, REG, FEIE>;
impl<'a, REG> FEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE::Disabled)
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FEIE::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<S0FCRrs> {
        FTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DMDIS_W<S0FCRrs> {
        DMDIS_W::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<S0FCRrs> {
        FEIE_W::new(self, 7)
    }
}
#[doc = "stream x FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0FCRrs;
impl crate::RegisterSpec for S0FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0fcr::R`](R) reader structure"]
impl crate::Readable for S0FCRrs {}
#[doc = "`write(|w| ..)` method takes [`s0fcr::W`](W) writer structure"]
impl crate::Writable for S0FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0FCR to value 0x21"]
impl crate::Resettable for S0FCRrs {
    const RESET_VALUE: u32 = 0x21;
}
