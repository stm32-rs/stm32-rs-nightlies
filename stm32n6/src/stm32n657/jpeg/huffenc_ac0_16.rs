///Register `HUFFENC_AC0_16` reader
pub type R = crate::R<HUFFENC_AC0_16rs>;
///Register `HUFFENC_AC0_16` writer
pub type W = crate::W<HUFFENC_AC0_16rs>;
///Field `HCODE32` reader - Huffman code 32
pub type HCODE32_R = crate::FieldReader;
///Field `HCODE32` writer - Huffman code 32
pub type HCODE32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN32` reader - Huffman length 32
pub type HLEN32_R = crate::FieldReader;
///Field `HLEN32` writer - Huffman length 32
pub type HLEN32_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE33` reader - Huffman code 33
pub type HCODE33_R = crate::FieldReader;
///Field `HCODE33` writer - Huffman code 33
pub type HCODE33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN33` reader - Huffman length 33
pub type HLEN33_R = crate::FieldReader;
///Field `HLEN33` writer - Huffman length 33
pub type HLEN33_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 32
    #[inline(always)]
    pub fn hcode32(&self) -> HCODE32_R {
        HCODE32_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 32
    #[inline(always)]
    pub fn hlen32(&self) -> HLEN32_R {
        HLEN32_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 33
    #[inline(always)]
    pub fn hcode33(&self) -> HCODE33_R {
        HCODE33_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 33
    #[inline(always)]
    pub fn hlen33(&self) -> HLEN33_R {
        HLEN33_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_16")
            .field("hcode32", &self.hcode32())
            .field("hlen32", &self.hlen32())
            .field("hcode33", &self.hcode33())
            .field("hlen33", &self.hlen33())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 32
    #[inline(always)]
    pub fn hcode32(&mut self) -> HCODE32_W<'_, HUFFENC_AC0_16rs> {
        HCODE32_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 32
    #[inline(always)]
    pub fn hlen32(&mut self) -> HLEN32_W<'_, HUFFENC_AC0_16rs> {
        HLEN32_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 33
    #[inline(always)]
    pub fn hcode33(&mut self) -> HCODE33_W<'_, HUFFENC_AC0_16rs> {
        HCODE33_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 33
    #[inline(always)]
    pub fn hlen33(&mut self) -> HLEN33_W<'_, HUFFENC_AC0_16rs> {
        HLEN33_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_16)*/
pub struct HUFFENC_AC0_16rs;
impl crate::RegisterSpec for HUFFENC_AC0_16rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_16::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_16rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_16::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_16rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_16 to value 0
impl crate::Resettable for HUFFENC_AC0_16rs {}
