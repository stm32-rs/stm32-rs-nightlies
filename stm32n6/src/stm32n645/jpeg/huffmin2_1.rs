///Register `HUFFMIN2_1` reader
pub type R = crate::R<HUFFMIN2_1rs>;
///Register `HUFFMIN2_1` writer
pub type W = crate::W<HUFFMIN2_1rs>;
///Field `DATA2` reader - Minimum Huffman value
pub type DATA2_R = crate::FieldReader<u32>;
///Field `DATA2` writer - Minimum Huffman value
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN2_1")
            .field("data2", &self.data2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<'_, HUFFMIN2_1rs> {
        DATA2_W::new(self, 0)
    }
}
/**JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin2_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin2_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFMIN2_1)*/
pub struct HUFFMIN2_1rs;
impl crate::RegisterSpec for HUFFMIN2_1rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin2_1::R`](R) reader structure
impl crate::Readable for HUFFMIN2_1rs {}
///`write(|w| ..)` method takes [`huffmin2_1::W`](W) writer structure
impl crate::Writable for HUFFMIN2_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN2_1 to value 0
impl crate::Resettable for HUFFMIN2_1rs {}
