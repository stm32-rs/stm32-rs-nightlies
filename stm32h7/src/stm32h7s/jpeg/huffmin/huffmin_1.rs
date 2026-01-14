///Register `HUFFMIN_1` reader
pub type R = crate::R<HUFFMIN_1rs>;
///Register `HUFFMIN_1` writer
pub type W = crate::W<HUFFMIN_1rs>;
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
        f.debug_struct("HUFFMIN_1")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFMIN_1rs> {
        DATA0_W::new(self, 0)
    }
}
/**Bits 32-63 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HUFFMIN_1rs;
impl crate::RegisterSpec for HUFFMIN_1rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin_1::R`](R) reader structure
impl crate::Readable for HUFFMIN_1rs {}
///`write(|w| ..)` method takes [`huffmin_1::W`](W) writer structure
impl crate::Writable for HUFFMIN_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN_1 to value 0
impl crate::Resettable for HUFFMIN_1rs {}
