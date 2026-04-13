///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CTAMP1F` writer - CTAMP1F
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` writer - CTAMP2F
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP1F` writer - CITAMP1F
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` writer - CITAMP3F
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP4F` writer - CITAMP4F
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` writer - CITAMP5F
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP6F` writer - CITAMP6F
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP7F` writer - CITAMP7F
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 16 - CITAMP1F
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<'_, SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    ///Bit 18 - CITAMP3F
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<'_, SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 19 - CITAMP4F
    #[inline(always)]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<'_, SCRrs> {
        CITAMP4F_W::new(self, 19)
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
    ///Bit 22 - CITAMP7F
    #[inline(always)]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<'_, SCRrs> {
        CITAMP7F_W::new(self, 22)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#TAMP:SCR)*/
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
