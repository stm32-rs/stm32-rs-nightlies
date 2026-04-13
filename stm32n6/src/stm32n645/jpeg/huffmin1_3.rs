///Register `HUFFMIN1_3` reader
pub type R = crate::R<HUFFMIN1_3rs>;
///Register `HUFFMIN1_3` writer
pub type W = crate::W<HUFFMIN1_3rs>;
///Field `DATA1` reader - Minimum Huffman value
pub type DATA1_R = crate::FieldReader;
///Field `DATA1` writer - Minimum Huffman value
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Minimum Huffman value
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN1_3")
            .field("data1", &self.data1())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum Huffman value
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, HUFFMIN1_3rs> {
        DATA1_W::new(self, 0)
    }
}
/**JPEG Huffman min 1

You can [`read`](crate::Reg::read) this register and get [`huffmin1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFMIN1_3)*/
pub struct HUFFMIN1_3rs;
impl crate::RegisterSpec for HUFFMIN1_3rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin1_3::R`](R) reader structure
impl crate::Readable for HUFFMIN1_3rs {}
///`write(|w| ..)` method takes [`huffmin1_3::W`](W) writer structure
impl crate::Writable for HUFFMIN1_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN1_3 to value 0
impl crate::Resettable for HUFFMIN1_3rs {}
