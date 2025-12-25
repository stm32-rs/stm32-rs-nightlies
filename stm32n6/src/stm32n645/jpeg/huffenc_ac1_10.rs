///Register `HUFFENC_AC1_10` reader
pub type R = crate::R<HUFFENC_AC1_10rs>;
///Register `HUFFENC_AC1_10` writer
pub type W = crate::W<HUFFENC_AC1_10rs>;
///Field `HCODE20` reader - Huffman code 20
pub type HCODE20_R = crate::FieldReader;
///Field `HCODE20` writer - Huffman code 20
pub type HCODE20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN20` reader - Huffman length 20
pub type HLEN20_R = crate::FieldReader;
///Field `HLEN20` writer - Huffman length 20
pub type HLEN20_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE21` reader - Huffman code 21
pub type HCODE21_R = crate::FieldReader;
///Field `HCODE21` writer - Huffman code 21
pub type HCODE21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN21` reader - Huffman length 21
pub type HLEN21_R = crate::FieldReader;
///Field `HLEN21` writer - Huffman length 21
pub type HLEN21_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 20
    #[inline(always)]
    pub fn hcode20(&self) -> HCODE20_R {
        HCODE20_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 20
    #[inline(always)]
    pub fn hlen20(&self) -> HLEN20_R {
        HLEN20_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 21
    #[inline(always)]
    pub fn hcode21(&self) -> HCODE21_R {
        HCODE21_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 21
    #[inline(always)]
    pub fn hlen21(&self) -> HLEN21_R {
        HLEN21_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC1_10")
            .field("hcode20", &self.hcode20())
            .field("hlen20", &self.hlen20())
            .field("hcode21", &self.hcode21())
            .field("hlen21", &self.hlen21())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 20
    #[inline(always)]
    pub fn hcode20(&mut self) -> HCODE20_W<'_, HUFFENC_AC1_10rs> {
        HCODE20_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 20
    #[inline(always)]
    pub fn hlen20(&mut self) -> HLEN20_W<'_, HUFFENC_AC1_10rs> {
        HLEN20_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 21
    #[inline(always)]
    pub fn hcode21(&mut self) -> HCODE21_W<'_, HUFFENC_AC1_10rs> {
        HCODE21_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 21
    #[inline(always)]
    pub fn hlen21(&mut self) -> HLEN21_W<'_, HUFFENC_AC1_10rs> {
        HLEN21_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFENC_AC1_10)*/
pub struct HUFFENC_AC1_10rs;
impl crate::RegisterSpec for HUFFENC_AC1_10rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac1_10::R`](R) reader structure
impl crate::Readable for HUFFENC_AC1_10rs {}
///`write(|w| ..)` method takes [`huffenc_ac1_10::W`](W) writer structure
impl crate::Writable for HUFFENC_AC1_10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC1_10 to value 0
impl crate::Resettable for HUFFENC_AC1_10rs {}
