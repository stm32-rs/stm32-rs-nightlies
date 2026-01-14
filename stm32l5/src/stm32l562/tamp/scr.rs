///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CTAMP1F` writer - CTAMP1F
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` writer - CTAMP2F
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP3F` writer - CTAMP3F
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP4F` writer - CTAMP4F
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP5F` writer - CTAMP5F
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP6F` writer - CTAMP6F
pub type CTAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP7F` writer - CTAMP7F
pub type CTAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP8F` writer - CTAMP8F
pub type CTAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP1F` writer - CITAMP1F
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP2F` writer - CITAMP2F
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` writer - CITAMP3F
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` writer - CITAMP5F
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP8F` writer - CITAMP8F
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - CTAMP4F
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<'_, SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    ///Bit 4 - CTAMP5F
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<'_, SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    ///Bit 5 - CTAMP6F
    #[inline(always)]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<'_, SCRrs> {
        CTAMP6F_W::new(self, 5)
    }
    ///Bit 6 - CTAMP7F
    #[inline(always)]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W<'_, SCRrs> {
        CTAMP7F_W::new(self, 6)
    }
    ///Bit 7 - CTAMP8F
    #[inline(always)]
    pub fn ctamp8f(&mut self) -> CTAMP8F_W<'_, SCRrs> {
        CTAMP8F_W::new(self, 7)
    }
    ///Bit 16 - CITAMP1F
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<'_, SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    ///Bit 17 - CITAMP2F
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<'_, SCRrs> {
        CITAMP2F_W::new(self, 17)
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
    ///Bit 23 - CITAMP8F
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<'_, SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:SCR)*/
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
