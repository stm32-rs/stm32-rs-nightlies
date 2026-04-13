///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Clears the Line Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIFW {
    ///1: Clears the LIF flag in the ISR register
    Clear = 1,
}
impl From<CLIFW> for bool {
    #[inline(always)]
    fn from(variant: CLIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLIF` writer - Clears the Line Interrupt Flag
pub type CLIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLIFW>;
impl<'a, REG> CLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the LIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLIFW::Clear)
    }
}
/**Clears the FIFO Underrun Interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFUIFW {
    ///1: Clears the FUIF flag in the ISR register
    Clear = 1,
}
impl From<CFUIFW> for bool {
    #[inline(always)]
    fn from(variant: CFUIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag
pub type CFUIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CFUIFW>;
impl<'a, REG> CFUIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the FUIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFUIFW::Clear)
    }
}
/**Clears the Transfer Error Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTERRIFW {
    ///1: Clears the TERRIF flag in the ISR register
    Clear = 1,
}
impl From<CTERRIFW> for bool {
    #[inline(always)]
    fn from(variant: CTERRIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag
pub type CTERRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CTERRIFW>;
impl<'a, REG> CTERRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the TERRIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTERRIFW::Clear)
    }
}
/**Clears Register Reload Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRRIFW {
    ///1: Clears the RRIF flag in the ISR register
    Clear = 1,
}
impl From<CRRIFW> for bool {
    #[inline(always)]
    fn from(variant: CRRIFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CRRIF` writer - Clears Register Reload Interrupt Flag
pub type CRRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CRRIFW>;
impl<'a, REG> CRRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the RRIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRRIFW::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clears the Line Interrupt Flag
    #[inline(always)]
    pub fn clif(&mut self) -> CLIF_W<'_, ICRrs> {
        CLIF_W::new(self, 0)
    }
    ///Bit 1 - Clears the FIFO Underrun Interrupt flag
    #[inline(always)]
    pub fn cfuif(&mut self) -> CFUIF_W<'_, ICRrs> {
        CFUIF_W::new(self, 1)
    }
    ///Bit 2 - Clears the Transfer Error Interrupt Flag
    #[inline(always)]
    pub fn cterrif(&mut self) -> CTERRIF_W<'_, ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    ///Bit 3 - Clears Register Reload Interrupt Flag
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<'_, ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
/**LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
