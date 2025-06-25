///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
///Field `BS3` writer - Port H set I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3.
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BR3` writer - Port H reset I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3. Note: If both BS3 and BR3 are set, BS3 has priority.
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - Port H set I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 19 - Port H reset I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3. Note: If both BS3 and BR3 are set, BS3 has priority.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
}
/**GPIO port H bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOH:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {}
