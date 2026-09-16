///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECF {
    ///1: Clears the PE flag in the ISR register
    Clear = 1,
}
impl From<PECF> for bool {
    #[inline(always)]
    fn from(variant: PECF) -> Self {
        variant as u8 != 0
    }
}
///Field `PECF` writer - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
pub type PECF_W<'a, REG> = crate::BitWriter1C<'a, REG, PECF>;
impl<'a, REG> PECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the PE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECF::Clear)
    }
}
/**FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FECF {
    ///1: Clears the FE flag in the ISR register
    Clear = 1,
}
impl From<FECF> for bool {
    #[inline(always)]
    fn from(variant: FECF) -> Self {
        variant as u8 != 0
    }
}
///Field `FECF` writer - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register
pub type FECF_W<'a, REG> = crate::BitWriter1C<'a, REG, FECF>;
impl<'a, REG> FECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the FE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FECF::Clear)
    }
}
/**NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NECF {
    ///1: Clears the NF flag in the ISR register
    Clear = 1,
}
impl From<NECF> for bool {
    #[inline(always)]
    fn from(variant: NECF) -> Self {
        variant as u8 != 0
    }
}
///Field `NECF` writer - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register.
pub type NECF_W<'a, REG> = crate::BitWriter1C<'a, REG, NECF>;
impl<'a, REG> NECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the NF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NECF::Clear)
    }
}
/**ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORECF {
    ///1: Clears the ORE flag in the ISR register
    Clear = 1,
}
impl From<ORECF> for bool {
    #[inline(always)]
    fn from(variant: ORECF) -> Self {
        variant as u8 != 0
    }
}
///Field `ORECF` writer - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
pub type ORECF_W<'a, REG> = crate::BitWriter1C<'a, REG, ORECF>;
impl<'a, REG> ORECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the ORE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ORECF::Clear)
    }
}
/**IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLECF {
    ///1: Clears the IDLE flag in the ISR register
    Clear = 1,
}
impl From<IDLECF> for bool {
    #[inline(always)]
    fn from(variant: IDLECF) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLECF` writer - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
pub type IDLECF_W<'a, REG> = crate::BitWriter1C<'a, REG, IDLECF>;
impl<'a, REG> IDLECF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the IDLE flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IDLECF::Clear)
    }
}
/**TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCCF {
    ///1: Clears the TC flag in the ISR register
    Clear = 1,
}
impl From<TCCF> for bool {
    #[inline(always)]
    fn from(variant: TCCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TCCF` writer - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register
pub type TCCF_W<'a, REG> = crate::BitWriter1C<'a, REG, TCCF>;
impl<'a, REG> TCCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the TC flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCCF::Clear)
    }
}
/**CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCF {
    ///1: Clears the CTSIF flag in the ISR register
    Clear = 1,
}
impl From<CTSCF> for bool {
    #[inline(always)]
    fn from(variant: CTSCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSCF` writer - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register
pub type CTSCF_W<'a, REG> = crate::BitWriter1C<'a, REG, CTSCF>;
impl<'a, REG> CTSCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CTSIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCF::Clear)
    }
}
/**CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMCF {
    ///1: Clears the CMF flag in the ISR register
    Clear = 1,
}
impl From<CMCF> for bool {
    #[inline(always)]
    fn from(variant: CMCF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMCF` writer - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register
pub type CMCF_W<'a, REG> = crate::BitWriter1C<'a, REG, CMCF>;
impl<'a, REG> CMCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CMF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMCF::Clear)
    }
}
///Field `WUCF` writer - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register
pub type WUCF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR").finish()
    }
}
impl W {
    ///Bit 0 - PECF: Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<'_, ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - FECF: Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<'_, ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - NECF: Noise detected clear flag Writing 1 to this bit clears the NF flag in the USART_ISR register.
    #[inline(always)]
    pub fn necf(&mut self) -> NECF_W<'_, ICRrs> {
        NECF_W::new(self, 2)
    }
    ///Bit 3 - ORECF: Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<'_, ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - IDLECF: Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<'_, ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 6 - TCCF: Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<'_, ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 9 - CTSCF: CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 17 - CMCF: Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<'_, ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - WUCF: Wakeup from Stop mode clear flag Writing 1 to this bit clears the WUF flag in the LPUART_ISR register
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<'_, ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**ICR register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#LPUART:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0012_025f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
