///Register `CLRFR` writer
pub type W = crate::W<CLRFRrs>;
/**Clear PKA End of Operation flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDFC {
    ///1: Clear PROCENDF flag
    Clear = 1,
}
impl From<PROCENDFC> for bool {
    #[inline(always)]
    fn from(variant: PROCENDFC) -> Self {
        variant as u8 != 0
    }
}
///Field `PROCENDFC` writer - Clear PKA End of Operation flag
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG, PROCENDFC>;
impl<'a, REG> PROCENDFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear PROCENDF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDFC::Clear)
    }
}
/**Clear PKA RAM error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRFC {
    ///1: Clear RAMERRF flag
    Clear = 1,
}
impl From<RAMERRFC> for bool {
    #[inline(always)]
    fn from(variant: RAMERRFC) -> Self {
        variant as u8 != 0
    }
}
///Field `RAMERRFC` writer - Clear PKA RAM error flag
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG, RAMERRFC>;
impl<'a, REG> RAMERRFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear RAMERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRFC::Clear)
    }
}
/**Clear Address error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRFC {
    ///1: Clear ADDRERRF flag
    Clear = 1,
}
impl From<ADDRERRFC> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRFC) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRERRFC` writer - Clear Address error flag
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG, ADDRERRFC>;
impl<'a, REG> ADDRERRFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear ADDRERRF flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRFC::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 17 - Clear PKA End of Operation flag
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W<'_, CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    ///Bit 19 - Clear PKA RAM error flag
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<'_, CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    ///Bit 20 - Clear Address error flag
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<'_, CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
}
/**clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#PKA:CLRFR)*/
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clrfr::W`](W) writer structure
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFRrs {}
