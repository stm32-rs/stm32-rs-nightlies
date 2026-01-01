///Register `HUFFENC_AC0_26` reader
pub type R = crate::R<HUFFENC_AC0_26rs>;
///Register `HUFFENC_AC0_26` writer
pub type W = crate::W<HUFFENC_AC0_26rs>;
///Field `HCODE52` reader - Huffman code 52
pub type HCODE52_R = crate::FieldReader;
///Field `HCODE52` writer - Huffman code 52
pub type HCODE52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN52` reader - Huffman length 52
pub type HLEN52_R = crate::FieldReader;
///Field `HLEN52` writer - Huffman length 52
pub type HLEN52_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE53` reader - Huffman code 53
pub type HCODE53_R = crate::FieldReader;
///Field `HCODE53` writer - Huffman code 53
pub type HCODE53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN53` reader - Huffman length 53
pub type HLEN53_R = crate::FieldReader;
///Field `HLEN53` writer - Huffman length 53
pub type HLEN53_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 52
    #[inline(always)]
    pub fn hcode52(&self) -> HCODE52_R {
        HCODE52_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 52
    #[inline(always)]
    pub fn hlen52(&self) -> HLEN52_R {
        HLEN52_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 53
    #[inline(always)]
    pub fn hcode53(&self) -> HCODE53_R {
        HCODE53_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 53
    #[inline(always)]
    pub fn hlen53(&self) -> HLEN53_R {
        HLEN53_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_26")
            .field("hcode52", &self.hcode52())
            .field("hlen52", &self.hlen52())
            .field("hcode53", &self.hcode53())
            .field("hlen53", &self.hlen53())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 52
    #[inline(always)]
    pub fn hcode52(&mut self) -> HCODE52_W<'_, HUFFENC_AC0_26rs> {
        HCODE52_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 52
    #[inline(always)]
    pub fn hlen52(&mut self) -> HLEN52_W<'_, HUFFENC_AC0_26rs> {
        HLEN52_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 53
    #[inline(always)]
    pub fn hcode53(&mut self) -> HCODE53_W<'_, HUFFENC_AC0_26rs> {
        HCODE53_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 53
    #[inline(always)]
    pub fn hlen53(&mut self) -> HLEN53_W<'_, HUFFENC_AC0_26rs> {
        HLEN53_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC0_26)*/
pub struct HUFFENC_AC0_26rs;
impl crate::RegisterSpec for HUFFENC_AC0_26rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_26::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_26rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_26::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_26 to value 0
impl crate::Resettable for HUFFENC_AC0_26rs {}
