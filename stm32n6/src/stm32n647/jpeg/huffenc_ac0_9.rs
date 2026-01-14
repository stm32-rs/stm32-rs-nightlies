///Register `HUFFENC_AC0_9` reader
pub type R = crate::R<HUFFENC_AC0_9rs>;
///Register `HUFFENC_AC0_9` writer
pub type W = crate::W<HUFFENC_AC0_9rs>;
///Field `HCODE18` reader - Huffman code 18
pub type HCODE18_R = crate::FieldReader;
///Field `HCODE18` writer - Huffman code 18
pub type HCODE18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN18` reader - Huffman length 18
pub type HLEN18_R = crate::FieldReader;
///Field `HLEN18` writer - Huffman length 18
pub type HLEN18_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE19` reader - Huffman code 19
pub type HCODE19_R = crate::FieldReader;
///Field `HCODE19` writer - Huffman code 19
pub type HCODE19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN19` reader - Huffman length 19
pub type HLEN19_R = crate::FieldReader;
///Field `HLEN19` writer - Huffman length 19
pub type HLEN19_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 18
    #[inline(always)]
    pub fn hcode18(&self) -> HCODE18_R {
        HCODE18_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 18
    #[inline(always)]
    pub fn hlen18(&self) -> HLEN18_R {
        HLEN18_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 19
    #[inline(always)]
    pub fn hcode19(&self) -> HCODE19_R {
        HCODE19_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 19
    #[inline(always)]
    pub fn hlen19(&self) -> HLEN19_R {
        HLEN19_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_9")
            .field("hcode18", &self.hcode18())
            .field("hlen18", &self.hlen18())
            .field("hcode19", &self.hcode19())
            .field("hlen19", &self.hlen19())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 18
    #[inline(always)]
    pub fn hcode18(&mut self) -> HCODE18_W<'_, HUFFENC_AC0_9rs> {
        HCODE18_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 18
    #[inline(always)]
    pub fn hlen18(&mut self) -> HLEN18_W<'_, HUFFENC_AC0_9rs> {
        HLEN18_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 19
    #[inline(always)]
    pub fn hcode19(&mut self) -> HCODE19_W<'_, HUFFENC_AC0_9rs> {
        HCODE19_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 19
    #[inline(always)]
    pub fn hlen19(&mut self) -> HLEN19_W<'_, HUFFENC_AC0_9rs> {
        HLEN19_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFENC_AC0_9)*/
pub struct HUFFENC_AC0_9rs;
impl crate::RegisterSpec for HUFFENC_AC0_9rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_9::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_9rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_9::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_9 to value 0
impl crate::Resettable for HUFFENC_AC0_9rs {}
