///Register `TAMP_SCR` writer
pub type W = crate::W<TAMP_SCRrs>;
///Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register.
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register.
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP3F` writer - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register.
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP4F` writer - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register.
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTAMP5F` writer - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register.
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register.
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP4F` writer - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register.
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register.
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register.
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TAMP_SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<TAMP_SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    ///Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<TAMP_SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    ///Bit 2 - Clear TAMP3 detection flag Writing 1 in this bit clears the TAMP3F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<TAMP_SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    ///Bit 3 - Clear TAMP4 detection flag Writing 1 in this bit clears the TAMP4F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<TAMP_SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    ///Bit 4 - Clear TAMP5 detection flag Writing 1 in this bit clears the TAMP5F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<TAMP_SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    ///Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<TAMP_SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    ///Bit 19 - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<TAMP_SCRrs> {
        CITAMP4F_W::new(self, 19)
    }
    ///Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<TAMP_SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    ///Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register.
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<TAMP_SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
}
/**TAMP status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TAMP:TAMP_SCR)*/
pub struct TAMP_SCRrs;
impl crate::RegisterSpec for TAMP_SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tamp_scr::W`](W) writer structure
impl crate::Writable for TAMP_SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAMP_SCR to value 0
impl crate::Resettable for TAMP_SCRrs {
    const RESET_VALUE: u32 = 0;
}
