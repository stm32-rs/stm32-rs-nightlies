///Register `HUFFMIN_2` reader
pub type R = crate::R<HUFFMIN_2rs>;
///Register `HUFFMIN_2` writer
pub type W = crate::W<HUFFMIN_2rs>;
///Field `DATA0` reader - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
pub type DATA0_R = crate::FieldReader<u32>;
///Field `DATA0` writer - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN_2")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFMIN_2rs> {
        DATA0_W::new(self, 0)
    }
}
/**Bits 64-95 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HUFFMIN_2rs;
impl crate::RegisterSpec for HUFFMIN_2rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin_2::R`](R) reader structure
impl crate::Readable for HUFFMIN_2rs {}
///`write(|w| ..)` method takes [`huffmin_2::W`](W) writer structure
impl crate::Writable for HUFFMIN_2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN_2 to value 0
impl crate::Resettable for HUFFMIN_2rs {}
