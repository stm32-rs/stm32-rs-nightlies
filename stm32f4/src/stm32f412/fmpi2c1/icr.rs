///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**ADDRCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCF {
    ///1: Clears the ADDR flag in ISR register
    Clear = 1,
}
impl From<ADDRCF> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRCF` writer - ADDRCF
pub type ADDRCF_W<'a, REG> = crate::BitWriter1C<'a, REG, ADDRCF>;
impl<'a, REG> ADDRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the ADDR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRCF::Clear)
    }
}
/**NACKCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKCF {
    ///1: Clears the NACK flag in ISR register
    Clear = 1,
}
impl From<NACKCF> for bool {
    #[inline(always)]
    fn from(variant: NACKCF) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKCF` writer - NACKCF
pub type NACKCF_W<'a, REG> = crate::BitWriter1C<'a, REG, NACKCF>;
impl<'a, REG> NACKCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the NACK flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NACKCF::Clear)
    }
}
/**STOPCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPCF {
    ///1: Clears the STOP flag in ISR register
    Clear = 1,
}
impl From<STOPCF> for bool {
    #[inline(always)]
    fn from(variant: STOPCF) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPCF` writer - STOPCF
pub type STOPCF_W<'a, REG> = crate::BitWriter1C<'a, REG, STOPCF>;
impl<'a, REG> STOPCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the STOP flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STOPCF::Clear)
    }
}
/**BERRCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRCF {
    ///1: Clears the BERR flag in ISR register
    Clear = 1,
}
impl From<BERRCF> for bool {
    #[inline(always)]
    fn from(variant: BERRCF) -> Self {
        variant as u8 != 0
    }
}
///Field `BERRCF` writer - BERRCF
pub type BERRCF_W<'a, REG> = crate::BitWriter1C<'a, REG, BERRCF>;
impl<'a, REG> BERRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the BERR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BERRCF::Clear)
    }
}
/**ARLOCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOCF {
    ///1: Clears the ARLO flag in ISR register
    Clear = 1,
}
impl From<ARLOCF> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLOCF` writer - ARLOCF
pub type ARLOCF_W<'a, REG> = crate::BitWriter1C<'a, REG, ARLOCF>;
impl<'a, REG> ARLOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the ARLO flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARLOCF::Clear)
    }
}
/**OVRCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCF {
    ///1: Clears the OVR flag in ISR register
    Clear = 1,
}
impl From<OVRCF> for bool {
    #[inline(always)]
    fn from(variant: OVRCF) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRCF` writer - OVRCF
pub type OVRCF_W<'a, REG> = crate::BitWriter1C<'a, REG, OVRCF>;
impl<'a, REG> OVRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the OVR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCF::Clear)
    }
}
/**PECCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECCF {
    ///1: Clears the PEC flag in ISR register
    Clear = 1,
}
impl From<PECCF> for bool {
    #[inline(always)]
    fn from(variant: PECCF) -> Self {
        variant as u8 != 0
    }
}
///Field `PECCF` writer - PECCF
pub type PECCF_W<'a, REG> = crate::BitWriter1C<'a, REG, PECCF>;
impl<'a, REG> PECCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the PEC flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECCF::Clear)
    }
}
/**TIMOUTCF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTCF {
    ///1: Clears the TIMOUT flag in ISR register
    Clear = 1,
}
impl From<TIMOUTCF> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMOUTCF` writer - TIMOUTCF
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter1C<'a, REG, TIMOUTCF>;
impl<'a, REG> TIMOUTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the TIMOUT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTCF::Clear)
    }
}
/**ALERTC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTCF {
    ///1: Clears the ALERT flag in ISR register
    Clear = 1,
}
impl From<ALERTCF> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERTCF` writer - ALERTC
pub type ALERTCF_W<'a, REG> = crate::BitWriter1C<'a, REG, ALERTCF>;
impl<'a, REG> ALERTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the ALERT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTCF::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - ADDRCF
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W<ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    ///Bit 4 - NACKCF
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W<ICRrs> {
        NACKCF_W::new(self, 4)
    }
    ///Bit 5 - STOPCF
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W<ICRrs> {
        STOPCF_W::new(self, 5)
    }
    ///Bit 8 - BERRCF
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W<ICRrs> {
        BERRCF_W::new(self, 8)
    }
    ///Bit 9 - ARLOCF
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W<ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    ///Bit 10 - OVRCF
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<ICRrs> {
        OVRCF_W::new(self, 10)
    }
    ///Bit 11 - PECCF
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W<ICRrs> {
        PECCF_W::new(self, 11)
    }
    ///Bit 12 - TIMOUTCF
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    ///Bit 13 - ALERTC
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W<ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
/**Interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#FMPI2C1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f38;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
