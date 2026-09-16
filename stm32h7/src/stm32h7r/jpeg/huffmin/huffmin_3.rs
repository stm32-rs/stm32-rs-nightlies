///Register `HUFFMIN_3` reader
pub type R = crate::R<HUFFMIN_3rs>;
///Register `HUFFMIN_3` writer
pub type W = crate::W<HUFFMIN_3rs>;
///Field `DATA0` reader - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
pub type DATA0_R = crate::FieldReader;
///Field `DATA0` writer - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFMIN_3")
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Minimum Huffman value 100-bit minimum Huffman value used internally by the JPEG decoder.
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, HUFFMIN_3rs> {
        DATA0_W::new(self, 0)
    }
}
/**Bits 96-99 of the minimum Huffman value

You can [`read`](crate::Reg::read) this register and get [`huffmin_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HUFFMIN_3rs;
impl crate::RegisterSpec for HUFFMIN_3rs {
    type Ux = u32;
}
///`read()` method returns [`huffmin_3::R`](R) reader structure
impl crate::Readable for HUFFMIN_3rs {}
///`write(|w| ..)` method takes [`huffmin_3::W`](W) writer structure
impl crate::Writable for HUFFMIN_3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFMIN_3 to value 0
impl crate::Resettable for HUFFMIN_3rs {}
