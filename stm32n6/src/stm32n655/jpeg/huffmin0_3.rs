///Register `HUFFMIN0_3` reader
pub type R = crate::R<HUFFMIN0_3rs>;
///Register `HUFFMIN0_3` writer
pub type W = crate::W<HUFFMIN0_3rs>;
///Field `DATA0` reader - Minimum Huffman value
pub type DATA0_R = crate::FieldReader;
///Field `DATA0` writer - Minimum Huffman value
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Minimum Huffman value
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN0_3")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum Huffman value
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFMIN0_3rs> {
        DATA0_W::new(self, 0)
    }
}
/**JPEG Huffman min 0

You can [`read`](crate::Reg::read) this register and get [`huffmin0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFMIN0_3)*/
pub struct HUFFMIN0_3rs;
impl crate::RegisterSpec for HUFFMIN0_3rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin0_3::R`](R) reader structure
impl crate::Readable for HUFFMIN0_3rs {}
///`write(|w| ..)` method takes [`huffmin0_3::W`](W) writer structure
impl crate::Writable for HUFFMIN0_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN0_3 to value 0
impl crate::Resettable for HUFFMIN0_3rs {}
