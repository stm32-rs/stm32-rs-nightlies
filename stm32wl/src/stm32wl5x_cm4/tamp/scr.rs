///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**CTAMP1F

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTAMP1FW {
    ///1: Clear tamper flag
    Clear = 1,
}
impl From<CTAMP1FW> for bool {
    #[inline(always)]
    fn from(variant: CTAMP1FW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTAMP1F` writer - CTAMP1F
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG, CTAMP1FW>;
impl<'a, REG> CTAMP1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear tamper flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTAMP1FW::Clear)
    }
}
///Field `CTAMP2F` writer - CTAMP2F
pub use CTAMP1F_W as CTAMP2F_W;
///Field `CTAMP3F` writer - CTAMP3F
pub use CTAMP1F_W as CTAMP3F_W;
/**CITAMP3F

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CITAMP3FW {
    ///1: Clear tamper flag
    Clear = 1,
}
impl From<CITAMP3FW> for bool {
    #[inline(always)]
    fn from(variant: CITAMP3FW) -> Self {
        variant as u8 != 0
    }
}
///Field `CITAMP3F` writer - CITAMP3F
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG, CITAMP3FW>;
impl<'a, REG> CITAMP3F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear tamper flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CITAMP3FW::Clear)
    }
}
///Field `CITAMP5F` writer - CITAMP5F
pub use CITAMP3F_W as CITAMP5F_W;
///Field `CITAMP6F` writer - CITAMP6F
pub use CITAMP3F_W as CITAMP6F_W;
///Field `CITAMP8F` writer - CITAMP8F
pub use CITAMP3F_W as CITAMP8F_W;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CTAMP1F
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<'_, SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    ///Bit 1 - CTAMP2F
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<'_, SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    ///Bit 2 - CTAMP3F
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<'_, SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    ///Bit 18 - CITAMP3F
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<'_, SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 20 - CITAMP5F
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<'_, SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    ///Bit 21 - CITAMP6F
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<'_, SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
    ///Bit 23 - CITAMP8F
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<'_, SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TAMP:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
