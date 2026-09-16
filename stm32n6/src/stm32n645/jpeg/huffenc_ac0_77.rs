///Register `HUFFENC_AC0_77` reader
pub type R = crate::R<HUFFENC_AC0_77rs>;
///Register `HUFFENC_AC0_77` writer
pub type W = crate::W<HUFFENC_AC0_77rs>;
///Field `HCODE154` reader - Huffman code 154
pub type HCODE154_R = crate::FieldReader;
///Field `HCODE154` writer - Huffman code 154
pub type HCODE154_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN154` reader - Huffman length 154
pub type HLEN154_R = crate::FieldReader;
///Field `HLEN154` writer - Huffman length 154
pub type HLEN154_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE155` reader - Huffman code 155
pub type HCODE155_R = crate::FieldReader;
///Field `HCODE155` writer - Huffman code 155
pub type HCODE155_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN155` reader - Huffman length 155
pub type HLEN155_R = crate::FieldReader;
///Field `HLEN155` writer - Huffman length 155
pub type HLEN155_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 154
    #[inline(always)]
    pub fn hcode154(&self) -> HCODE154_R {
        HCODE154_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 154
    #[inline(always)]
    pub fn hlen154(&self) -> HLEN154_R {
        HLEN154_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 155
    #[inline(always)]
    pub fn hcode155(&self) -> HCODE155_R {
        HCODE155_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 155
    #[inline(always)]
    pub fn hlen155(&self) -> HLEN155_R {
        HLEN155_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_77")
            .field("hcode154", &self.hcode154())
            .field("hlen154", &self.hlen154())
            .field("hcode155", &self.hcode155())
            .field("hlen155", &self.hlen155())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 154
    #[inline(always)]
    pub fn hcode154(&mut self) -> HCODE154_W<'_, HUFFENC_AC0_77rs> {
        HCODE154_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 154
    #[inline(always)]
    pub fn hlen154(&mut self) -> HLEN154_W<'_, HUFFENC_AC0_77rs> {
        HLEN154_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 155
    #[inline(always)]
    pub fn hcode155(&mut self) -> HCODE155_W<'_, HUFFENC_AC0_77rs> {
        HCODE155_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 155
    #[inline(always)]
    pub fn hlen155(&mut self) -> HLEN155_W<'_, HUFFENC_AC0_77rs> {
        HLEN155_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_77::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_77::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_77)*/
pub struct HUFFENC_AC0_77rs;
impl crate::RegisterSpec for HUFFENC_AC0_77rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_77::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_77rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_77::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_77rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_77 to value 0
impl crate::Resettable for HUFFENC_AC0_77rs {}
