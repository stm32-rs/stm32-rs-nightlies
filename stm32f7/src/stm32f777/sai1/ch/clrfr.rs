///Register `CLRFR` writer
pub type W = crate::W<CLRFRrs>;
/**Clear overrun / underrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVRUDRW {
    ///1: Clears the OVRUDR flag
    Clear = 1,
}
impl From<COVRUDRW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW) -> Self {
        variant as u8 != 0
    }
}
///Field `COVRUDR` writer - Clear overrun / underrun
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG, COVRUDRW>;
impl<'a, REG> COVRUDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the OVRUDR flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COVRUDRW::Clear)
    }
}
/**Mute detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUTEDETW {
    ///1: Clears the MUTEDET flag
    Clear = 1,
}
impl From<CMUTEDETW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMUTEDET` writer - Mute detection flag
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG, CMUTEDETW>;
impl<'a, REG> CMUTEDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the MUTEDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMUTEDETW::Clear)
    }
}
/**Clear wrong clock configuration flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWCKCFGW {
    ///1: Clears the WCKCFG flag
    Clear = 1,
}
impl From<CWCKCFGW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWCKCFG` writer - Clear wrong clock configuration flag
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG, CWCKCFGW>;
impl<'a, REG> CWCKCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the WCKCFG flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWCKCFGW::Clear)
    }
}
/**Clear codec not ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCNRDYW {
    ///1: Clears the CNRDY flag
    Clear = 1,
}
impl From<CCNRDYW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCNRDY` writer - Clear codec not ready flag
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCNRDYW>;
impl<'a, REG> CCNRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the CNRDY flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCNRDYW::Clear)
    }
}
/**Clear anticipated frame synchronization detection flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAFSDETW {
    ///1: Clears the AFSDET flag
    Clear = 1,
}
impl From<CAFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag.
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CAFSDETW>;
impl<'a, REG> CAFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the AFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CAFSDETW::Clear)
    }
}
/**Clear late frame synchronization detection flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLFSDETW {
    ///1: Clears the LFSDET flag
    Clear = 1,
}
impl From<CLFSDETW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLFSDET` writer - Clear late frame synchronization detection flag
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG, CLFSDETW>;
impl<'a, REG> CLFSDET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears the LFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLFSDETW::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<'_, CLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<'_, CLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<'_, CLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<'_, CLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag.
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, CLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<'_, CLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
/**AClear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
