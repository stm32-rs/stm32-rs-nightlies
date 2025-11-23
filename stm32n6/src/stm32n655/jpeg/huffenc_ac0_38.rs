///Register `HUFFENC_AC0_38` reader
pub type R = crate::R<HUFFENC_AC0_38rs>;
///Register `HUFFENC_AC0_38` writer
pub type W = crate::W<HUFFENC_AC0_38rs>;
///Field `HCODE76` reader - Huffman code 76
pub type HCODE76_R = crate::FieldReader;
///Field `HCODE76` writer - Huffman code 76
pub type HCODE76_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN76` reader - Huffman length 76
pub type HLEN76_R = crate::FieldReader;
///Field `HLEN76` writer - Huffman length 76
pub type HLEN76_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE77` reader - Huffman code 77
pub type HCODE77_R = crate::FieldReader;
///Field `HCODE77` writer - Huffman code 77
pub type HCODE77_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN77` reader - Huffman length 77
pub type HLEN77_R = crate::FieldReader;
///Field `HLEN77` writer - Huffman length 77
pub type HLEN77_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 76
    #[inline(always)]
    pub fn hcode76(&self) -> HCODE76_R {
        HCODE76_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 76
    #[inline(always)]
    pub fn hlen76(&self) -> HLEN76_R {
        HLEN76_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 77
    #[inline(always)]
    pub fn hcode77(&self) -> HCODE77_R {
        HCODE77_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 77
    #[inline(always)]
    pub fn hlen77(&self) -> HLEN77_R {
        HLEN77_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_38")
            .field("hcode76", &self.hcode76())
            .field("hlen76", &self.hlen76())
            .field("hcode77", &self.hcode77())
            .field("hlen77", &self.hlen77())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 76
    #[inline(always)]
    pub fn hcode76(&mut self) -> HCODE76_W<'_, HUFFENC_AC0_38rs> {
        HCODE76_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 76
    #[inline(always)]
    pub fn hlen76(&mut self) -> HLEN76_W<'_, HUFFENC_AC0_38rs> {
        HLEN76_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 77
    #[inline(always)]
    pub fn hcode77(&mut self) -> HCODE77_W<'_, HUFFENC_AC0_38rs> {
        HCODE77_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 77
    #[inline(always)]
    pub fn hlen77(&mut self) -> HLEN77_W<'_, HUFFENC_AC0_38rs> {
        HLEN77_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_38)*/
pub struct HUFFENC_AC0_38rs;
impl crate::RegisterSpec for HUFFENC_AC0_38rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_38::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_38rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_38::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_38rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_38 to value 0
impl crate::Resettable for HUFFENC_AC0_38rs {}
