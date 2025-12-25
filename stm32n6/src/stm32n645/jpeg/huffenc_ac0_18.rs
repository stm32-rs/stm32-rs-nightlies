///Register `HUFFENC_AC0_18` reader
pub type R = crate::R<HUFFENC_AC0_18rs>;
///Register `HUFFENC_AC0_18` writer
pub type W = crate::W<HUFFENC_AC0_18rs>;
///Field `HCODE36` reader - Huffman code 36
pub type HCODE36_R = crate::FieldReader;
///Field `HCODE36` writer - Huffman code 36
pub type HCODE36_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN36` reader - Huffman length 36
pub type HLEN36_R = crate::FieldReader;
///Field `HLEN36` writer - Huffman length 36
pub type HLEN36_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE37` reader - Huffman code 37
pub type HCODE37_R = crate::FieldReader;
///Field `HCODE37` writer - Huffman code 37
pub type HCODE37_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN37` reader - Huffman length 37
pub type HLEN37_R = crate::FieldReader;
///Field `HLEN37` writer - Huffman length 37
pub type HLEN37_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 36
    #[inline(always)]
    pub fn hcode36(&self) -> HCODE36_R {
        HCODE36_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 36
    #[inline(always)]
    pub fn hlen36(&self) -> HLEN36_R {
        HLEN36_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 37
    #[inline(always)]
    pub fn hcode37(&self) -> HCODE37_R {
        HCODE37_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 37
    #[inline(always)]
    pub fn hlen37(&self) -> HLEN37_R {
        HLEN37_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_18")
            .field("hcode36", &self.hcode36())
            .field("hlen36", &self.hlen36())
            .field("hcode37", &self.hcode37())
            .field("hlen37", &self.hlen37())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 36
    #[inline(always)]
    pub fn hcode36(&mut self) -> HCODE36_W<'_, HUFFENC_AC0_18rs> {
        HCODE36_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 36
    #[inline(always)]
    pub fn hlen36(&mut self) -> HLEN36_W<'_, HUFFENC_AC0_18rs> {
        HLEN36_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 37
    #[inline(always)]
    pub fn hcode37(&mut self) -> HCODE37_W<'_, HUFFENC_AC0_18rs> {
        HCODE37_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 37
    #[inline(always)]
    pub fn hlen37(&mut self) -> HLEN37_W<'_, HUFFENC_AC0_18rs> {
        HLEN37_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_18)*/
pub struct HUFFENC_AC0_18rs;
impl crate::RegisterSpec for HUFFENC_AC0_18rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_18::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_18rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_18::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_18 to value 0
impl crate::Resettable for HUFFENC_AC0_18rs {}
