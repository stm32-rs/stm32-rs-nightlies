///Register `HUFFMIN1_0` reader
pub type R = crate::R<HUFFMIN1_0rs>;
///Register `HUFFMIN1_0` writer
pub type W = crate::W<HUFFMIN1_0rs>;
///Field `DATA1` reader - Minimum Huffman value
pub type DATA1_R = crate::FieldReader<u32>;
///Field `DATA1` writer - Minimum Huffman value
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN1_0")
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, HUFFMIN1_0rs> {
        DATA1_W::new(self, 0)
    }
}
/**JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFMIN1_0)*/
pub struct HUFFMIN1_0rs;
impl crate::RegisterSpec for HUFFMIN1_0rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin1_0::R`](R) reader structure
impl crate::Readable for HUFFMIN1_0rs {}
///`write(|w| ..)` method takes [`huffmin1_0::W`](W) writer structure
impl crate::Writable for HUFFMIN1_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN1_0 to value 0
impl crate::Resettable for HUFFMIN1_0rs {}
