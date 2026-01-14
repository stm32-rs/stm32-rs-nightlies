///Register `HUFFENC_AC0_79` reader
pub type R = crate::R<HUFFENC_AC0_79rs>;
///Register `HUFFENC_AC0_79` writer
pub type W = crate::W<HUFFENC_AC0_79rs>;
///Field `HCODE158` reader - Huffman code 158
pub type HCODE158_R = crate::FieldReader;
///Field `HCODE158` writer - Huffman code 158
pub type HCODE158_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN158` reader - Huffman length 158
pub type HLEN158_R = crate::FieldReader;
///Field `HLEN158` writer - Huffman length 158
pub type HLEN158_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE159` reader - Huffman code 159
pub type HCODE159_R = crate::FieldReader;
///Field `HCODE159` writer - Huffman code 159
pub type HCODE159_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN159` reader - Huffman length 159
pub type HLEN159_R = crate::FieldReader;
///Field `HLEN159` writer - Huffman length 159
pub type HLEN159_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 158
    #[inline(always)]
    pub fn hcode158(&self) -> HCODE158_R {
        HCODE158_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 158
    #[inline(always)]
    pub fn hlen158(&self) -> HLEN158_R {
        HLEN158_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 159
    #[inline(always)]
    pub fn hcode159(&self) -> HCODE159_R {
        HCODE159_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 159
    #[inline(always)]
    pub fn hlen159(&self) -> HLEN159_R {
        HLEN159_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_79")
            .field("hcode158", &self.hcode158())
            .field("hlen158", &self.hlen158())
            .field("hcode159", &self.hcode159())
            .field("hlen159", &self.hlen159())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 158
    #[inline(always)]
    pub fn hcode158(&mut self) -> HCODE158_W<'_, HUFFENC_AC0_79rs> {
        HCODE158_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 158
    #[inline(always)]
    pub fn hlen158(&mut self) -> HLEN158_W<'_, HUFFENC_AC0_79rs> {
        HLEN158_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 159
    #[inline(always)]
    pub fn hcode159(&mut self) -> HCODE159_W<'_, HUFFENC_AC0_79rs> {
        HCODE159_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 159
    #[inline(always)]
    pub fn hlen159(&mut self) -> HLEN159_W<'_, HUFFENC_AC0_79rs> {
        HLEN159_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_79::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_79::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_79)*/
pub struct HUFFENC_AC0_79rs;
impl crate::RegisterSpec for HUFFENC_AC0_79rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_79::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_79rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_79::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_79rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_79 to value 0
impl crate::Resettable for HUFFENC_AC0_79rs {}
