///Register `FCR` reader
pub type R = crate::R<FCRrs>;
///Register `FCR` writer
pub type W = crate::W<FCRrs>;
/**Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEF {
    ///1: clears the TEF flag in the QUADSPI_SR register
    Clear = 1,
}
impl From<CTEF> for bool {
    #[inline(always)]
    fn from(variant: CTEF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEF` reader - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
pub type CTEF_R = crate::BitReader<CTEF>;
impl CTEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTEF> {
        match self.bits {
            true => Some(CTEF::Clear),
            _ => None,
        }
    }
    ///clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEF::Clear
    }
}
///Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG, CTEF>;
impl<'a, REG> CTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEF::Clear)
    }
}
/**Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCF {
    ///1: clears the TCF flag in the QUADSPI_SR register
    Clear = 1,
}
impl From<CTCF> for bool {
    #[inline(always)]
    fn from(variant: CTCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCF` reader - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
pub type CTCF_R = crate::BitReader<CTCF>;
impl CTCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTCF> {
        match self.bits {
            true => Some(CTCF::Clear),
            _ => None,
        }
    }
    ///clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCF::Clear
    }
}
///Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG, CTCF>;
impl<'a, REG> CTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCF::Clear)
    }
}
/**Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSMF {
    ///1: clears the SMF flag in the QUADSPI_SR register
    Clear = 1,
}
impl From<CSMF> for bool {
    #[inline(always)]
    fn from(variant: CSMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CSMF` reader - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
pub type CSMF_R = crate::BitReader<CSMF>;
impl CSMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSMF> {
        match self.bits {
            true => Some(CSMF::Clear),
            _ => None,
        }
    }
    ///clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSMF::Clear
    }
}
///Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG, CSMF>;
impl<'a, REG> CSMF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSMF::Clear)
    }
}
/**Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOF {
    ///1: clears the TOF flag in the QUADSPI_SR register
    Clear = 1,
}
impl From<CTOF> for bool {
    #[inline(always)]
    fn from(variant: CTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTOF` reader - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
pub type CTOF_R = crate::BitReader<CTOF>;
impl CTOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTOF> {
        match self.bits {
            true => Some(CTOF::Clear),
            _ => None,
        }
    }
    ///clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTOF::Clear
    }
}
///Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG, CTOF>;
impl<'a, REG> CTOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTOF::Clear)
    }
}
impl R {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctef(&self) -> CTEF_R {
        CTEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctcf(&self) -> CTCF_R {
        CTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn csmf(&self) -> CSMF_R {
        CSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctof(&self) -> CTOF_R {
        CTOF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCR")
            .field("ctef", &self.ctef())
            .field("ctcf", &self.ctcf())
            .field("csmf", &self.csmf())
            .field("ctof", &self.ctof())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W<'_, FCRrs> {
        CTEF_W::new(self, 0)
    }
    ///Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<'_, FCRrs> {
        CTCF_W::new(self, 1)
    }
    ///Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W<'_, FCRrs> {
        CSMF_W::new(self, 3)
    }
    ///Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W<'_, FCRrs> {
        CTOF_W::new(self, 4)
    }
}
/**QUADSPI flag clear register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#QUADSPI:FCR)*/
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
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
