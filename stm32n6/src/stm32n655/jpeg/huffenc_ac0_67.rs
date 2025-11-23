///Register `HUFFENC_AC0_67` reader
pub type R = crate::R<HUFFENC_AC0_67rs>;
///Register `HUFFENC_AC0_67` writer
pub type W = crate::W<HUFFENC_AC0_67rs>;
///Field `HCODE134` reader - Huffman code 134
pub type HCODE134_R = crate::FieldReader;
///Field `HCODE134` writer - Huffman code 134
pub type HCODE134_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN134` reader - Huffman length 134
pub type HLEN134_R = crate::FieldReader;
///Field `HLEN134` writer - Huffman length 134
pub type HLEN134_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE135` reader - Huffman code 135
pub type HCODE135_R = crate::FieldReader;
///Field `HCODE135` writer - Huffman code 135
pub type HCODE135_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN135` reader - Huffman length 135
pub type HLEN135_R = crate::FieldReader;
///Field `HLEN135` writer - Huffman length 135
pub type HLEN135_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 134
    #[inline(always)]
    pub fn hcode134(&self) -> HCODE134_R {
        HCODE134_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 134
    #[inline(always)]
    pub fn hlen134(&self) -> HLEN134_R {
        HLEN134_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 135
    #[inline(always)]
    pub fn hcode135(&self) -> HCODE135_R {
        HCODE135_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 135
    #[inline(always)]
    pub fn hlen135(&self) -> HLEN135_R {
        HLEN135_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_67")
            .field("hcode134", &self.hcode134())
            .field("hlen134", &self.hlen134())
            .field("hcode135", &self.hcode135())
            .field("hlen135", &self.hlen135())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 134
    #[inline(always)]
    pub fn hcode134(&mut self) -> HCODE134_W<'_, HUFFENC_AC0_67rs> {
        HCODE134_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 134
    #[inline(always)]
    pub fn hlen134(&mut self) -> HLEN134_W<'_, HUFFENC_AC0_67rs> {
        HLEN134_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 135
    #[inline(always)]
    pub fn hcode135(&mut self) -> HCODE135_W<'_, HUFFENC_AC0_67rs> {
        HCODE135_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 135
    #[inline(always)]
    pub fn hlen135(&mut self) -> HLEN135_W<'_, HUFFENC_AC0_67rs> {
        HLEN135_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_67::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_67)*/
pub struct HUFFENC_AC0_67rs;
impl crate::RegisterSpec for HUFFENC_AC0_67rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_67::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_67rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_67::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_67rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_67 to value 0
impl crate::Resettable for HUFFENC_AC0_67rs {}
