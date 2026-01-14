///Register `FRCR` reader
pub type R = crate::R<FRCRrs>;
///Register `FRCR` writer
pub type W = crate::W<FRCRrs>;
///Field `FRL` reader - Frame length
pub type FRL_R = crate::FieldReader;
///Field `FRL` writer - Frame length
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FSALL` reader - Frame synchronization active level length
pub type FSALL_R = crate::FieldReader;
///Field `FSALL` writer - Frame synchronization active level length
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `FSDEF` reader - Frame synchronization definition
pub type FSDEF_R = crate::BitReader;
///Field `FSDEF` writer - Frame synchronization definition
pub type FSDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Frame synchronization polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPOL {
    ///0: FS is active low (falling edge)
    FallingEdge = 0,
    ///1: FS is active high (rising edge)
    RisingEdge = 1,
}
impl From<FSPOL> for bool {
    #[inline(always)]
    fn from(variant: FSPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `FSPOL` reader - Frame synchronization polarity
pub type FSPOL_R = crate::BitReader<FSPOL>;
impl FSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSPOL {
        match self.bits {
            false => FSPOL::FallingEdge,
            true => FSPOL::RisingEdge,
        }
    }
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL::FallingEdge
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL::RisingEdge
    }
}
///Field `FSPOL` writer - Frame synchronization polarity
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG, FSPOL>;
impl<'a, REG> FSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::FallingEdge)
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::RisingEdge)
    }
}
/**Frame synchronization offset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOFF {
    ///0: FS is asserted on the first bit of the slot 0
    OnFirst = 0,
    ///1: FS is asserted one bit before the first bit of the slot 0
    BeforeFirst = 1,
}
impl From<FSOFF> for bool {
    #[inline(always)]
    fn from(variant: FSOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `FSOFF` reader - Frame synchronization offset
pub type FSOFF_R = crate::BitReader<FSOFF>;
impl FSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSOFF {
        match self.bits {
            false => FSOFF::OnFirst,
            true => FSOFF::BeforeFirst,
        }
    }
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF::OnFirst
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF::BeforeFirst
    }
}
///Field `FSOFF` writer - Frame synchronization offset
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG, FSOFF>;
impl<'a, REG> FSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn on_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::OnFirst)
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn before_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::BeforeFirst)
    }
}
impl R {
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRCR")
            .field("fsoff", &self.fsoff())
            .field("fspol", &self.fspol())
            .field("fsdef", &self.fsdef())
            .field("fsall", &self.fsall())
            .field("frl", &self.frl())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W<'_, FRCRrs> {
        FRL_W::new(self, 0)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W<'_, FRCRrs> {
        FSALL_W::new(self, 8)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W<'_, FRCRrs> {
        FSDEF_W::new(self, 16)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W<'_, FRCRrs> {
        FSPOL_W::new(self, 17)
    }
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W<'_, FRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
/**AFRCR

You can [`read`](crate::Reg::read) this register and get [`frcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FRCRrs;
impl crate::RegisterSpec for FRCRrs {
    type Ux = u32;
}
///`read()` method returns [`frcr::R`](R) reader structure
impl crate::Readable for FRCRrs {}
///`write(|w| ..)` method takes [`frcr::W`](W) writer structure
impl crate::Writable for FRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRCR to value 0x07
impl crate::Resettable for FRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
