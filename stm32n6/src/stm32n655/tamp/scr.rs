///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CTAMP1F` writer - Clear TAMP1 detection flag
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` writer - Clear TAMP2 detection flag
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP3F` writer - Clear TAMP3 detection flag
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP4F` writer - Clear TAMP4 detection flag
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP5F` writer - Clear TAMP5 detection flag
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP6F` writer - Clear TAMP6 detection flag
pub type CTAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP7F` writer - Clear TAMP7 detection flag
pub type CTAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP1F` writer - Clear ITAMP1 detection flag
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP2F` writer - Clear ITAMP2 detection flag
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` writer - Clear ITAMP3 detection flag
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP4F` writer - Clear ITAMP4 detection flag
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` writer - Clear ITAMP5 detection flag
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP6F` writer - Clear ITAMP6 detection flag
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP7F` writer - Clear ITAMP7 detection flag
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP8F` writer - Clear ITAMP8 detection flag
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP9F` writer - Clear ITAMP9 detection flag
pub type CITAMP9F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP11F` writer - Clear ITAMP11 detection flag
pub type CITAMP11F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear TAMP1 detection flag
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<'_, SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    ///Bit 1 - Clear TAMP2 detection flag
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<'_, SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    ///Bit 2 - Clear TAMP3 detection flag
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<'_, SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    ///Bit 3 - Clear TAMP4 detection flag
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<'_, SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    ///Bit 4 - Clear TAMP5 detection flag
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<'_, SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    ///Bit 5 - Clear TAMP6 detection flag
    #[inline(always)]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<'_, SCRrs> {
        CTAMP6F_W::new(self, 5)
    }
    ///Bit 6 - Clear TAMP7 detection flag
    #[inline(always)]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W<'_, SCRrs> {
        CTAMP7F_W::new(self, 6)
    }
    ///Bit 16 - Clear ITAMP1 detection flag
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<'_, SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    ///Bit 17 - Clear ITAMP2 detection flag
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<'_, SCRrs> {
        CITAMP2F_W::new(self, 17)
    }
    ///Bit 18 - Clear ITAMP3 detection flag
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<'_, SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 19 - Clear ITAMP4 detection flag
    #[inline(always)]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<'_, SCRrs> {
        CITAMP4F_W::new(self, 19)
    }
    ///Bit 20 - Clear ITAMP5 detection flag
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<'_, SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    ///Bit 21 - Clear ITAMP6 detection flag
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<'_, SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
    ///Bit 22 - Clear ITAMP7 detection flag
    #[inline(always)]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<'_, SCRrs> {
        CITAMP7F_W::new(self, 22)
    }
    ///Bit 23 - Clear ITAMP8 detection flag
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<'_, SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
    ///Bit 24 - Clear ITAMP9 detection flag
    #[inline(always)]
    pub fn citamp9f(&mut self) -> CITAMP9F_W<'_, SCRrs> {
        CITAMP9F_W::new(self, 24)
    }
    ///Bit 26 - Clear ITAMP11 detection flag
    #[inline(always)]
    pub fn citamp11f(&mut self) -> CITAMP11F_W<'_, SCRrs> {
        CITAMP11F_W::new(self, 26)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TAMP:SCR)*/
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
