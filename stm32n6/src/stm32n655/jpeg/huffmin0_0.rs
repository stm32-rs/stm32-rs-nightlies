///Register `HUFFMIN0_0` reader
pub type R = crate::R<HUFFMIN0_0rs>;
///Register `HUFFMIN0_0` writer
pub type W = crate::W<HUFFMIN0_0rs>;
///Field `DATA0` reader - Minimum Huffman value
pub type DATA0_R = crate::FieldReader<u32>;
///Field `DATA0` writer - Minimum Huffman value
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN0_0")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<HUFFMIN0_0rs> {
        DATA0_W::new(self, 0)
    }
}
/**JPEG Huffman min

You can [`read`](crate::Reg::read) this register and get [`huffmin0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_0)*/
pub struct HUFFMIN0_0rs;
impl crate::RegisterSpec for HUFFMIN0_0rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin0_0::R`](R) reader structure
impl crate::Readable for HUFFMIN0_0rs {}
///`write(|w| ..)` method takes [`huffmin0_0::W`](W) writer structure
impl crate::Writable for HUFFMIN0_0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN0_0 to value 0
impl crate::Resettable for HUFFMIN0_0rs {}
