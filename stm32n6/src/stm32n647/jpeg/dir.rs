///Register `DIR` writer
pub type W = crate::W<DIRrs>;
///Field `DATAIN` writer - Data input FIFO
pub type DATAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data input FIFO
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W<'_, DIRrs> {
        DATAIN_W::new(self, 0)
    }
}
/**JPEG data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:DIR)*/
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dir::W`](W) writer structure
impl crate::Writable for DIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIR to value 0
impl crate::Resettable for DIRrs {}
