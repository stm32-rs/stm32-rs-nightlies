#[doc = "Register `FRCR` reader"]
pub type R = crate::R<FRCRrs>;
#[doc = "Register `FRCR` writer"]
pub type W = crate::W<FRCRrs>;
#[doc = "Field `FRL` reader - Frame length"]
pub type FRL_R = crate::FieldReader;
#[doc = "Field `FRL` writer - Frame length"]
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub type FSALL_R = crate::FieldReader;
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub type FSDEF_R = crate::BitReader;
#[doc = "Field `FSDEF` writer - Frame synchronization definition"]
pub type FSDEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame synchronization polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPOL {
    #[doc = "0: FS is active low (falling edge)"]
    FallingEdge = 0,
    #[doc = "1: FS is active high (rising edge)"]
    RisingEdge = 1,
}
impl From<FSPOL> for bool {
    #[inline(always)]
    fn from(variant: FSPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub type FSPOL_R = crate::BitReader<FSPOL>;
impl FSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSPOL {
        match self.bits {
            false => FSPOL::FallingEdge,
            true => FSPOL::RisingEdge,
        }
    }
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL::FallingEdge
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL::RisingEdge
    }
}
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG, FSPOL>;
impl<'a, REG> FSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS is active low (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::FallingEdge)
    }
    #[doc = "FS is active high (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(FSPOL::RisingEdge)
    }
}
#[doc = "Frame synchronization offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOFF {
    #[doc = "0: FS is asserted on the first bit of the slot 0"]
    OnFirst = 0,
    #[doc = "1: FS is asserted one bit before the first bit of the slot 0"]
    BeforeFirst = 1,
}
impl From<FSOFF> for bool {
    #[inline(always)]
    fn from(variant: FSOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub type FSOFF_R = crate::BitReader<FSOFF>;
impl FSOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSOFF {
        match self.bits {
            false => FSOFF::OnFirst,
            true => FSOFF::BeforeFirst,
        }
    }
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF::OnFirst
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF::BeforeFirst
    }
}
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG, FSOFF>;
impl<'a, REG> FSOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS is asserted on the first bit of the slot 0"]
    #[inline(always)]
    pub fn on_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::OnFirst)
    }
    #[doc = "FS is asserted one bit before the first bit of the slot 0"]
    #[inline(always)]
    pub fn before_first(self) -> &'a mut crate::W<REG> {
        self.variant(FSOFF::BeforeFirst)
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<FRCRrs> {
        FRL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<FRCRrs> {
        FSALL_W::new(self, 8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    #[must_use]
    pub fn fsdef(&mut self) -> FSDEF_W<FRCRrs> {
        FSDEF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<FRCRrs> {
        FSPOL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<FRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
#[doc = "AFRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRCRrs;
impl crate::RegisterSpec for FRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frcr::R`](R) reader structure"]
impl crate::Readable for FRCRrs {}
#[doc = "`write(|w| ..)` method takes [`frcr::W`](W) writer structure"]
impl crate::Writable for FRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRCR to value 0x07"]
impl crate::Resettable for FRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
