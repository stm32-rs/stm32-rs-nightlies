///Register `HUFFENC_AC0_48` reader
pub type R = crate::R<HUFFENC_AC0_48rs>;
///Register `HUFFENC_AC0_48` writer
pub type W = crate::W<HUFFENC_AC0_48rs>;
///Field `HCODE96` reader - Huffman code 96
pub type HCODE96_R = crate::FieldReader;
///Field `HCODE96` writer - Huffman code 96
pub type HCODE96_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN96` reader - Huffman length 96
pub type HLEN96_R = crate::FieldReader;
///Field `HLEN96` writer - Huffman length 96
pub type HLEN96_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE97` reader - Huffman code 97
pub type HCODE97_R = crate::FieldReader;
///Field `HCODE97` writer - Huffman code 97
pub type HCODE97_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN97` reader - Huffman length 97
pub type HLEN97_R = crate::FieldReader;
///Field `HLEN97` writer - Huffman length 97
pub type HLEN97_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 96
    #[inline(always)]
    pub fn hcode96(&self) -> HCODE96_R {
        HCODE96_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 96
    #[inline(always)]
    pub fn hlen96(&self) -> HLEN96_R {
        HLEN96_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 97
    #[inline(always)]
    pub fn hcode97(&self) -> HCODE97_R {
        HCODE97_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 97
    #[inline(always)]
    pub fn hlen97(&self) -> HLEN97_R {
        HLEN97_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_48")
            .field("hcode96", &self.hcode96())
            .field("hlen96", &self.hlen96())
            .field("hcode97", &self.hcode97())
            .field("hlen97", &self.hlen97())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 96
    #[inline(always)]
    pub fn hcode96(&mut self) -> HCODE96_W<'_, HUFFENC_AC0_48rs> {
        HCODE96_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 96
    #[inline(always)]
    pub fn hlen96(&mut self) -> HLEN96_W<'_, HUFFENC_AC0_48rs> {
        HLEN96_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 97
    #[inline(always)]
    pub fn hcode97(&mut self) -> HCODE97_W<'_, HUFFENC_AC0_48rs> {
        HCODE97_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 97
    #[inline(always)]
    pub fn hlen97(&mut self) -> HLEN97_W<'_, HUFFENC_AC0_48rs> {
        HLEN97_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_48)*/
pub struct HUFFENC_AC0_48rs;
impl crate::RegisterSpec for HUFFENC_AC0_48rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_48::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_48rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_48::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_48rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_48 to value 0
impl crate::Resettable for HUFFENC_AC0_48rs {}
