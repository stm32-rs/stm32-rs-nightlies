///Register `MMCTIMR` reader
pub type R = crate::R<MMCTIMRrs>;
///Register `MMCTIMR` writer
pub type W = crate::W<MMCTIMRrs>;
/**Transmitted good frames single collision mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFSCM {
    ///0: Transmitted-good-single-collision half-full interrupt enabled
    Unmasked = 0,
    ///1: Transmitted-good-single-collision half-full interrupt disabled
    Masked = 1,
}
impl From<TGFSCM> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM) -> Self {
        variant as u8 != 0
    }
}
///Field `TGFSCM` reader - Transmitted good frames single collision mask
pub type TGFSCM_R = crate::BitReader<TGFSCM>;
impl TGFSCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TGFSCM {
        match self.bits {
            false => TGFSCM::Unmasked,
            true => TGFSCM::Masked,
        }
    }
    ///Transmitted-good-single-collision half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCM::Unmasked
    }
    ///Transmitted-good-single-collision half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCM::Masked
    }
}
///Field `TGFSCM` writer - Transmitted good frames single collision mask
pub type TGFSCM_W<'a, REG> = crate::BitWriter<'a, REG, TGFSCM>;
impl<'a, REG> TGFSCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitted-good-single-collision half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFSCM::Unmasked)
    }
    ///Transmitted-good-single-collision half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFSCM::Masked)
    }
}
/**Transmitted good frames more than single collision mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFMSCM {
    ///0: Transmitted-good-multiple-collision half-full interrupt enabled
    Unmasked = 0,
    ///1: Transmitted-good-multiple-collision half-full interrupt disabled
    Masked = 1,
}
impl From<TGFMSCM> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM) -> Self {
        variant as u8 != 0
    }
}
///Field `TGFMSCM` reader - Transmitted good frames more than single collision mask
pub type TGFMSCM_R = crate::BitReader<TGFMSCM>;
impl TGFMSCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TGFMSCM {
        match self.bits {
            false => TGFMSCM::Unmasked,
            true => TGFMSCM::Masked,
        }
    }
    ///Transmitted-good-multiple-collision half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCM::Unmasked
    }
    ///Transmitted-good-multiple-collision half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCM::Masked
    }
}
///Field `TGFMSCM` writer - Transmitted good frames more than single collision mask
pub type TGFMSCM_W<'a, REG> = crate::BitWriter<'a, REG, TGFMSCM>;
impl<'a, REG> TGFMSCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitted-good-multiple-collision half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFMSCM::Unmasked)
    }
    ///Transmitted-good-multiple-collision half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFMSCM::Masked)
    }
}
/**Transmitted good frames mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFM {
    ///0: Transmitted-good counter half-full interrupt enabled
    Unmasked = 0,
    ///1: Transmitted-good counter half-full interrupt disabled
    Masked = 1,
}
impl From<TGFM> for bool {
    #[inline(always)]
    fn from(variant: TGFM) -> Self {
        variant as u8 != 0
    }
}
///Field `TGFM` reader - Transmitted good frames mask
pub type TGFM_R = crate::BitReader<TGFM>;
impl TGFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TGFM {
        match self.bits {
            false => TGFM::Unmasked,
            true => TGFM::Masked,
        }
    }
    ///Transmitted-good counter half-full interrupt enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFM::Unmasked
    }
    ///Transmitted-good counter half-full interrupt disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFM::Masked
    }
}
///Field `TGFM` writer - Transmitted good frames mask
pub type TGFM_W<'a, REG> = crate::BitWriter<'a, REG, TGFM>;
impl<'a, REG> TGFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitted-good counter half-full interrupt enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFM::Unmasked)
    }
    ///Transmitted-good counter half-full interrupt disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFM::Masked)
    }
}
impl R {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmitted good frames more than single collision mask
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIMR")
            .field("tgfscm", &self.tgfscm())
            .field("tgfmscm", &self.tgfmscm())
            .field("tgfm", &self.tgfm())
            .finish()
    }
}
impl W {
    ///Bit 14 - Transmitted good frames single collision mask
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W<'_, MMCTIMRrs> {
        TGFSCM_W::new(self, 14)
    }
    ///Bit 15 - Transmitted good frames more than single collision mask
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W<'_, MMCTIMRrs> {
        TGFMSCM_W::new(self, 15)
    }
    ///Bit 21 - Transmitted good frames mask
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W<'_, MMCTIMRrs> {
        TGFM_W::new(self, 21)
    }
}
/**Ethernet MMC transmit interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`mmctimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#Ethernet_MMC:MMCTIMR)*/
pub struct MMCTIMRrs;
impl crate::RegisterSpec for MMCTIMRrs {
    type Ux = u32;
}
///`read()` method returns [`mmctimr::R`](R) reader structure
impl crate::Readable for MMCTIMRrs {}
///`write(|w| ..)` method takes [`mmctimr::W`](W) writer structure
impl crate::Writable for MMCTIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCTIMR to value 0
impl crate::Resettable for MMCTIMRrs {}
