///Register `FCR` writer
pub type W = crate::W<FCRrs>;
///Field `CB0OF` writer - Clear buffer 0 overflow flag
pub type CB0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB1OF` writer - Clear buffer 1 overflow flag
pub type CB1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB2OF` writer - Clear buffer 2 overflow flag
pub type CB2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CB3OF` writer - Clear buffer 3 overflow flag
pub type CB3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAMEF` writer - Clear AHB master error flag
pub type CAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear buffer 0 overflow flag
    #[inline(always)]
    pub fn cb0of(&mut self) -> CB0OF_W<'_, FCRrs> {
        CB0OF_W::new(self, 0)
    }
    ///Bit 1 - Clear buffer 1 overflow flag
    #[inline(always)]
    pub fn cb1of(&mut self) -> CB1OF_W<'_, FCRrs> {
        CB1OF_W::new(self, 1)
    }
    ///Bit 2 - Clear buffer 2 overflow flag
    #[inline(always)]
    pub fn cb2of(&mut self) -> CB2OF_W<'_, FCRrs> {
        CB2OF_W::new(self, 2)
    }
    ///Bit 3 - Clear buffer 3 overflow flag
    #[inline(always)]
    pub fn cb3of(&mut self) -> CB3OF_W<'_, FCRrs> {
        CB3OF_W::new(self, 3)
    }
    ///Bit 4 - Clear AHB master error flag
    #[inline(always)]
    pub fn camef(&mut self) -> CAMEF_W<'_, FCRrs> {
        CAMEF_W::new(self, 4)
    }
}
/**Graphic MMU flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:FCR)*/
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr::W`](W) writer structure
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCRrs {}
