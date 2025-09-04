///Register `BDR` reader
pub type R = crate::R<BDRrs>;
///Register `BDR` writer
pub type W = crate::W<BDRrs>;
///Field `DATA` reader - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty.
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<BDRrs> {
        DATA_W::new(self, 0)
    }
}
/**SAI data register

You can [`read`](crate::Reg::read) this register and get [`bdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SAI1:BDR)*/
pub struct BDRrs;
impl crate::RegisterSpec for BDRrs {
    type Ux = u32;
}
///`read()` method returns [`bdr::R`](R) reader structure
impl crate::Readable for BDRrs {}
///`write(|w| ..)` method takes [`bdr::W`](W) writer structure
impl crate::Writable for BDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDR to value 0
impl crate::Resettable for BDRrs {}
