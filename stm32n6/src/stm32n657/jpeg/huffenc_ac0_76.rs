///Register `HUFFENC_AC0_76` reader
pub type R = crate::R<HUFFENC_AC0_76rs>;
///Register `HUFFENC_AC0_76` writer
pub type W = crate::W<HUFFENC_AC0_76rs>;
///Field `HCODE152` reader - Huffman code 152
pub type HCODE152_R = crate::FieldReader;
///Field `HCODE152` writer - Huffman code 152
pub type HCODE152_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN152` reader - Huffman length 152
pub type HLEN152_R = crate::FieldReader;
///Field `HLEN152` writer - Huffman length 152
pub type HLEN152_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE153` reader - Huffman code 153
pub type HCODE153_R = crate::FieldReader;
///Field `HCODE153` writer - Huffman code 153
pub type HCODE153_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN153` reader - Huffman length 153
pub type HLEN153_R = crate::FieldReader;
///Field `HLEN153` writer - Huffman length 153
pub type HLEN153_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 152
    #[inline(always)]
    pub fn hcode152(&self) -> HCODE152_R {
        HCODE152_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 152
    #[inline(always)]
    pub fn hlen152(&self) -> HLEN152_R {
        HLEN152_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 153
    #[inline(always)]
    pub fn hcode153(&self) -> HCODE153_R {
        HCODE153_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 153
    #[inline(always)]
    pub fn hlen153(&self) -> HLEN153_R {
        HLEN153_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_76")
            .field("hcode152", &self.hcode152())
            .field("hlen152", &self.hlen152())
            .field("hcode153", &self.hcode153())
            .field("hlen153", &self.hlen153())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 152
    #[inline(always)]
    pub fn hcode152(&mut self) -> HCODE152_W<'_, HUFFENC_AC0_76rs> {
        HCODE152_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 152
    #[inline(always)]
    pub fn hlen152(&mut self) -> HLEN152_W<'_, HUFFENC_AC0_76rs> {
        HLEN152_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 153
    #[inline(always)]
    pub fn hcode153(&mut self) -> HCODE153_W<'_, HUFFENC_AC0_76rs> {
        HCODE153_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 153
    #[inline(always)]
    pub fn hlen153(&mut self) -> HLEN153_W<'_, HUFFENC_AC0_76rs> {
        HLEN153_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_76::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_76::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_76)*/
pub struct HUFFENC_AC0_76rs;
impl crate::RegisterSpec for HUFFENC_AC0_76rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_76::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_76rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_76::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_76rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_76 to value 0
impl crate::Resettable for HUFFENC_AC0_76rs {}
