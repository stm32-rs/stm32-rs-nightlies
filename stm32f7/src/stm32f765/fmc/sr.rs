#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "IRS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRS {
    #[doc = "0: Interrupt rising edge did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt rising edge occurred"]
    Occurred = 1,
}
impl From<IRS> for bool {
    #[inline(always)]
    fn from(variant: IRS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRS` reader - IRS"]
pub type IRS_R = crate::BitReader<IRS>;
impl IRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRS {
        match self.bits {
            false => IRS::DidNotOccur,
            true => IRS::Occurred,
        }
    }
    #[doc = "Interrupt rising edge did not occur"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IRS::DidNotOccur
    }
    #[doc = "Interrupt rising edge occurred"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IRS::Occurred
    }
}
#[doc = "Field `IRS` writer - IRS"]
pub type IRS_W<'a, REG> = crate::BitWriter<'a, REG, IRS>;
impl<'a, REG> IRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt rising edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(IRS::DidNotOccur)
    }
    #[doc = "Interrupt rising edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(IRS::Occurred)
    }
}
#[doc = "ILS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILS {
    #[doc = "0: Interrupt high-level did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt high-level occurred"]
    Occurred = 1,
}
impl From<ILS> for bool {
    #[inline(always)]
    fn from(variant: ILS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILS` reader - ILS"]
pub type ILS_R = crate::BitReader<ILS>;
impl ILS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILS {
        match self.bits {
            false => ILS::DidNotOccur,
            true => ILS::Occurred,
        }
    }
    #[doc = "Interrupt high-level did not occur"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == ILS::DidNotOccur
    }
    #[doc = "Interrupt high-level occurred"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == ILS::Occurred
    }
}
#[doc = "Field `ILS` writer - ILS"]
pub type ILS_W<'a, REG> = crate::BitWriter<'a, REG, ILS>;
impl<'a, REG> ILS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt high-level did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(ILS::DidNotOccur)
    }
    #[doc = "Interrupt high-level occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(ILS::Occurred)
    }
}
#[doc = "IFS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFS {
    #[doc = "0: Interrupt falling edge did not occur"]
    DidNotOccur = 0,
    #[doc = "1: Interrupt falling edge occurred"]
    Occurred = 1,
}
impl From<IFS> for bool {
    #[inline(always)]
    fn from(variant: IFS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFS` reader - IFS"]
pub type IFS_R = crate::BitReader<IFS>;
impl IFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IFS {
        match self.bits {
            false => IFS::DidNotOccur,
            true => IFS::Occurred,
        }
    }
    #[doc = "Interrupt falling edge did not occur"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        *self == IFS::DidNotOccur
    }
    #[doc = "Interrupt falling edge occurred"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == IFS::Occurred
    }
}
#[doc = "Field `IFS` writer - IFS"]
pub type IFS_W<'a, REG> = crate::BitWriter<'a, REG, IFS>;
impl<'a, REG> IFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt falling edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut crate::W<REG> {
        self.variant(IFS::DidNotOccur)
    }
    #[doc = "Interrupt falling edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(IFS::Occurred)
    }
}
#[doc = "IREN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    #[doc = "0: Interrupt rising edge detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt rising edge detection request enabled"]
    Enabled = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IREN"]
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::Disabled,
            true => IREN::Enabled,
        }
    }
    #[doc = "Interrupt rising edge detection request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN::Disabled
    }
    #[doc = "Interrupt rising edge detection request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN::Enabled
    }
}
#[doc = "Field `IREN` writer - IREN"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt rising edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Disabled)
    }
    #[doc = "Interrupt rising edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Enabled)
    }
}
#[doc = "ILEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILEN {
    #[doc = "0: Interrupt high-level detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt high-level detection request enabled"]
    Enabled = 1,
}
impl From<ILEN> for bool {
    #[inline(always)]
    fn from(variant: ILEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILEN` reader - ILEN"]
pub type ILEN_R = crate::BitReader<ILEN>;
impl ILEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ILEN {
        match self.bits {
            false => ILEN::Disabled,
            true => ILEN::Enabled,
        }
    }
    #[doc = "Interrupt high-level detection request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILEN::Disabled
    }
    #[doc = "Interrupt high-level detection request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILEN::Enabled
    }
}
#[doc = "Field `ILEN` writer - ILEN"]
pub type ILEN_W<'a, REG> = crate::BitWriter<'a, REG, ILEN>;
impl<'a, REG> ILEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt high-level detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ILEN::Disabled)
    }
    #[doc = "Interrupt high-level detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ILEN::Enabled)
    }
}
#[doc = "IFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFEN {
    #[doc = "0: Interrupt falling edge detection request disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt falling edge detection request enabled"]
    Enabled = 1,
}
impl From<IFEN> for bool {
    #[inline(always)]
    fn from(variant: IFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFEN` reader - IFEN"]
pub type IFEN_R = crate::BitReader<IFEN>;
impl IFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IFEN {
        match self.bits {
            false => IFEN::Disabled,
            true => IFEN::Enabled,
        }
    }
    #[doc = "Interrupt falling edge detection request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IFEN::Disabled
    }
    #[doc = "Interrupt falling edge detection request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IFEN::Enabled
    }
}
#[doc = "Field `IFEN` writer - IFEN"]
pub type IFEN_W<'a, REG> = crate::BitWriter<'a, REG, IFEN>;
impl<'a, REG> IFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt falling edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IFEN::Disabled)
    }
    #[doc = "Interrupt falling edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IFEN::Enabled)
    }
}
#[doc = "FEMPT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEMPT {
    #[doc = "0: FIFO not empty"]
    NotEmpty = 0,
    #[doc = "1: FIFO empty"]
    Empty = 1,
}
impl From<FEMPT> for bool {
    #[inline(always)]
    fn from(variant: FEMPT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEMPT` reader - FEMPT"]
pub type FEMPT_R = crate::BitReader<FEMPT>;
impl FEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEMPT {
        match self.bits {
            false => FEMPT::NotEmpty,
            true => FEMPT::Empty,
        }
    }
    #[doc = "FIFO not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FEMPT::NotEmpty
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FEMPT::Empty
    }
}
impl R {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IRS_W<SRrs> {
        IRS_W::new(self, 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    #[must_use]
    pub fn ils(&mut self) -> ILS_W<SRrs> {
        ILS_W::new(self, 1)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<SRrs> {
        IFS_W::new(self, 2)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<SRrs> {
        IREN_W::new(self, 3)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    #[must_use]
    pub fn ilen(&mut self) -> ILEN_W<SRrs> {
        ILEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    #[must_use]
    pub fn ifen(&mut self) -> IFEN_W<SRrs> {
        IFEN_W::new(self, 5)
    }
}
#[doc = "FIFO status and interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x40;
}
