///Register `HUFFENC_AC0_71` reader
pub type R = crate::R<HUFFENC_AC0_71rs>;
///Register `HUFFENC_AC0_71` writer
pub type W = crate::W<HUFFENC_AC0_71rs>;
///Field `HCODE142` reader - Huffman code 142
pub type HCODE142_R = crate::FieldReader;
///Field `HCODE142` writer - Huffman code 142
pub type HCODE142_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN142` reader - Huffman length 142
pub type HLEN142_R = crate::FieldReader;
///Field `HLEN142` writer - Huffman length 142
pub type HLEN142_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE143` reader - Huffman code 143
pub type HCODE143_R = crate::FieldReader;
///Field `HCODE143` writer - Huffman code 143
pub type HCODE143_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN143` reader - Huffman length 143
pub type HLEN143_R = crate::FieldReader;
///Field `HLEN143` writer - Huffman length 143
pub type HLEN143_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 142
    #[inline(always)]
    pub fn hcode142(&self) -> HCODE142_R {
        HCODE142_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 142
    #[inline(always)]
    pub fn hlen142(&self) -> HLEN142_R {
        HLEN142_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 143
    #[inline(always)]
    pub fn hcode143(&self) -> HCODE143_R {
        HCODE143_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 143
    #[inline(always)]
    pub fn hlen143(&self) -> HLEN143_R {
        HLEN143_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_71")
            .field("hcode142", &self.hcode142())
            .field("hlen142", &self.hlen142())
            .field("hcode143", &self.hcode143())
            .field("hlen143", &self.hlen143())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 142
    #[inline(always)]
    pub fn hcode142(&mut self) -> HCODE142_W<'_, HUFFENC_AC0_71rs> {
        HCODE142_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 142
    #[inline(always)]
    pub fn hlen142(&mut self) -> HLEN142_W<'_, HUFFENC_AC0_71rs> {
        HLEN142_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 143
    #[inline(always)]
    pub fn hcode143(&mut self) -> HCODE143_W<'_, HUFFENC_AC0_71rs> {
        HCODE143_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 143
    #[inline(always)]
    pub fn hlen143(&mut self) -> HLEN143_W<'_, HUFFENC_AC0_71rs> {
        HLEN143_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_71::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_71::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_71)*/
pub struct HUFFENC_AC0_71rs;
impl crate::RegisterSpec for HUFFENC_AC0_71rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_71::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_71rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_71::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_71rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_71 to value 0
impl crate::Resettable for HUFFENC_AC0_71rs {}
