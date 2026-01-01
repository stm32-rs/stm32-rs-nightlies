///Register `HUFFENC_AC0_82` reader
pub type R = crate::R<HUFFENC_AC0_82rs>;
///Register `HUFFENC_AC0_82` writer
pub type W = crate::W<HUFFENC_AC0_82rs>;
///Field `HCODE164` reader - Huffman code 164
pub type HCODE164_R = crate::FieldReader;
///Field `HCODE164` writer - Huffman code 164
pub type HCODE164_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN164` reader - Huffman length 164
pub type HLEN164_R = crate::FieldReader;
///Field `HLEN164` writer - Huffman length 164
pub type HLEN164_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE165` reader - Huffman code 165
pub type HCODE165_R = crate::FieldReader;
///Field `HCODE165` writer - Huffman code 165
pub type HCODE165_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN165` reader - Huffman length 165
pub type HLEN165_R = crate::FieldReader;
///Field `HLEN165` writer - Huffman length 165
pub type HLEN165_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 164
    #[inline(always)]
    pub fn hcode164(&self) -> HCODE164_R {
        HCODE164_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 164
    #[inline(always)]
    pub fn hlen164(&self) -> HLEN164_R {
        HLEN164_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 165
    #[inline(always)]
    pub fn hcode165(&self) -> HCODE165_R {
        HCODE165_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 165
    #[inline(always)]
    pub fn hlen165(&self) -> HLEN165_R {
        HLEN165_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_82")
            .field("hcode164", &self.hcode164())
            .field("hlen164", &self.hlen164())
            .field("hcode165", &self.hcode165())
            .field("hlen165", &self.hlen165())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 164
    #[inline(always)]
    pub fn hcode164(&mut self) -> HCODE164_W<'_, HUFFENC_AC0_82rs> {
        HCODE164_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 164
    #[inline(always)]
    pub fn hlen164(&mut self) -> HLEN164_W<'_, HUFFENC_AC0_82rs> {
        HLEN164_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 165
    #[inline(always)]
    pub fn hcode165(&mut self) -> HCODE165_W<'_, HUFFENC_AC0_82rs> {
        HCODE165_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 165
    #[inline(always)]
    pub fn hlen165(&mut self) -> HLEN165_W<'_, HUFFENC_AC0_82rs> {
        HLEN165_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_82::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_82::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFENC_AC0_82)*/
pub struct HUFFENC_AC0_82rs;
impl crate::RegisterSpec for HUFFENC_AC0_82rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_82::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_82rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_82::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_82rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_82 to value 0
impl crate::Resettable for HUFFENC_AC0_82rs {}
