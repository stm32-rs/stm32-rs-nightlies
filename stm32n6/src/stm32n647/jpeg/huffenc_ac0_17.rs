///Register `HUFFENC_AC0_17` reader
pub type R = crate::R<HUFFENC_AC0_17rs>;
///Register `HUFFENC_AC0_17` writer
pub type W = crate::W<HUFFENC_AC0_17rs>;
///Field `HCODE34` reader - Huffman code 34
pub type HCODE34_R = crate::FieldReader;
///Field `HCODE34` writer - Huffman code 34
pub type HCODE34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN34` reader - Huffman length 34
pub type HLEN34_R = crate::FieldReader;
///Field `HLEN34` writer - Huffman length 34
pub type HLEN34_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE35` reader - Huffman code 35
pub type HCODE35_R = crate::FieldReader;
///Field `HCODE35` writer - Huffman code 35
pub type HCODE35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN35` reader - Huffman length 35
pub type HLEN35_R = crate::FieldReader;
///Field `HLEN35` writer - Huffman length 35
pub type HLEN35_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 34
    #[inline(always)]
    pub fn hcode34(&self) -> HCODE34_R {
        HCODE34_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 34
    #[inline(always)]
    pub fn hlen34(&self) -> HLEN34_R {
        HLEN34_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 35
    #[inline(always)]
    pub fn hcode35(&self) -> HCODE35_R {
        HCODE35_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 35
    #[inline(always)]
    pub fn hlen35(&self) -> HLEN35_R {
        HLEN35_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_17")
            .field("hcode34", &self.hcode34())
            .field("hlen34", &self.hlen34())
            .field("hcode35", &self.hcode35())
            .field("hlen35", &self.hlen35())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 34
    #[inline(always)]
    pub fn hcode34(&mut self) -> HCODE34_W<'_, HUFFENC_AC0_17rs> {
        HCODE34_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 34
    #[inline(always)]
    pub fn hlen34(&mut self) -> HLEN34_W<'_, HUFFENC_AC0_17rs> {
        HLEN34_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 35
    #[inline(always)]
    pub fn hcode35(&mut self) -> HCODE35_W<'_, HUFFENC_AC0_17rs> {
        HCODE35_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 35
    #[inline(always)]
    pub fn hlen35(&mut self) -> HLEN35_W<'_, HUFFENC_AC0_17rs> {
        HLEN35_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC0_17)*/
pub struct HUFFENC_AC0_17rs;
impl crate::RegisterSpec for HUFFENC_AC0_17rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_17::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_17rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_17::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_17rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_17 to value 0
impl crate::Resettable for HUFFENC_AC0_17rs {}
