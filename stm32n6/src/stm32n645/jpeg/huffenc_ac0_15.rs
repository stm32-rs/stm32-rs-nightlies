///Register `HUFFENC_AC0_15` reader
pub type R = crate::R<HUFFENC_AC0_15rs>;
///Register `HUFFENC_AC0_15` writer
pub type W = crate::W<HUFFENC_AC0_15rs>;
///Field `HCODE30` reader - Huffman code 30
pub type HCODE30_R = crate::FieldReader;
///Field `HCODE30` writer - Huffman code 30
pub type HCODE30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN30` reader - Huffman length 30
pub type HLEN30_R = crate::FieldReader;
///Field `HLEN30` writer - Huffman length 30
pub type HLEN30_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE31` reader - Huffman code 31
pub type HCODE31_R = crate::FieldReader;
///Field `HCODE31` writer - Huffman code 31
pub type HCODE31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN31` reader - Huffman length 31
pub type HLEN31_R = crate::FieldReader;
///Field `HLEN31` writer - Huffman length 31
pub type HLEN31_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 30
    #[inline(always)]
    pub fn hcode30(&self) -> HCODE30_R {
        HCODE30_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 30
    #[inline(always)]
    pub fn hlen30(&self) -> HLEN30_R {
        HLEN30_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 31
    #[inline(always)]
    pub fn hcode31(&self) -> HCODE31_R {
        HCODE31_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 31
    #[inline(always)]
    pub fn hlen31(&self) -> HLEN31_R {
        HLEN31_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_15")
            .field("hcode30", &self.hcode30())
            .field("hlen30", &self.hlen30())
            .field("hcode31", &self.hcode31())
            .field("hlen31", &self.hlen31())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 30
    #[inline(always)]
    pub fn hcode30(&mut self) -> HCODE30_W<'_, HUFFENC_AC0_15rs> {
        HCODE30_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 30
    #[inline(always)]
    pub fn hlen30(&mut self) -> HLEN30_W<'_, HUFFENC_AC0_15rs> {
        HLEN30_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 31
    #[inline(always)]
    pub fn hcode31(&mut self) -> HCODE31_W<'_, HUFFENC_AC0_15rs> {
        HCODE31_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 31
    #[inline(always)]
    pub fn hlen31(&mut self) -> HLEN31_W<'_, HUFFENC_AC0_15rs> {
        HLEN31_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC0_15)*/
pub struct HUFFENC_AC0_15rs;
impl crate::RegisterSpec for HUFFENC_AC0_15rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_15::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_15rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_15::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_15 to value 0
impl crate::Resettable for HUFFENC_AC0_15rs {}
