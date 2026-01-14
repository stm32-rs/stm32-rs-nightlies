///Register `HUFFMIN3_0` reader
pub type R = crate::R<HUFFMIN3_0rs>;
///Register `HUFFMIN3_0` writer
pub type W = crate::W<HUFFMIN3_0rs>;
///Field `DATA3` reader - Minimum Huffman value
pub type DATA3_R = crate::FieldReader<u32>;
///Field `DATA3` writer - Minimum Huffman value
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN3_0")
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<'_, HUFFMIN3_0rs> {
        DATA3_W::new(self, 0)
    }
}
/**JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin3_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin3_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN3_0)*/
pub struct HUFFMIN3_0rs;
impl crate::RegisterSpec for HUFFMIN3_0rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin3_0::R`](R) reader structure
impl crate::Readable for HUFFMIN3_0rs {}
///`write(|w| ..)` method takes [`huffmin3_0::W`](W) writer structure
impl crate::Writable for HUFFMIN3_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN3_0 to value 0
impl crate::Resettable for HUFFMIN3_0rs {}
