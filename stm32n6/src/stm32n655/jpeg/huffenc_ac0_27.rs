///Register `HUFFENC_AC0_27` reader
pub type R = crate::R<HUFFENC_AC0_27rs>;
///Register `HUFFENC_AC0_27` writer
pub type W = crate::W<HUFFENC_AC0_27rs>;
///Field `HCODE54` reader - Huffman code 54
pub type HCODE54_R = crate::FieldReader;
///Field `HCODE54` writer - Huffman code 54
pub type HCODE54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN54` reader - Huffman length 54
pub type HLEN54_R = crate::FieldReader;
///Field `HLEN54` writer - Huffman length 54
pub type HLEN54_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HCODE55` reader - Huffman code 55
pub type HCODE55_R = crate::FieldReader;
///Field `HCODE55` writer - Huffman code 55
pub type HCODE55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HLEN55` reader - Huffman length 55
pub type HLEN55_R = crate::FieldReader;
///Field `HLEN55` writer - Huffman length 55
pub type HLEN55_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Huffman code 54
    #[inline(always)]
    pub fn hcode54(&self) -> HCODE54_R {
        HCODE54_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Huffman length 54
    #[inline(always)]
    pub fn hlen54(&self) -> HLEN54_R {
        HLEN54_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:23 - Huffman code 55
    #[inline(always)]
    pub fn hcode55(&self) -> HCODE55_R {
        HCODE55_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Huffman length 55
    #[inline(always)]
    pub fn hlen55(&self) -> HLEN55_R {
        HLEN55_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFENC_AC0_27")
            .field("hcode54", &self.hcode54())
            .field("hlen54", &self.hlen54())
            .field("hcode55", &self.hcode55())
            .field("hlen55", &self.hlen55())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Huffman code 54
    #[inline(always)]
    pub fn hcode54(&mut self) -> HCODE54_W<'_, HUFFENC_AC0_27rs> {
        HCODE54_W::new(self, 0)
    }
    ///Bits 8:11 - Huffman length 54
    #[inline(always)]
    pub fn hlen54(&mut self) -> HLEN54_W<'_, HUFFENC_AC0_27rs> {
        HLEN54_W::new(self, 8)
    }
    ///Bits 16:23 - Huffman code 55
    #[inline(always)]
    pub fn hcode55(&mut self) -> HCODE55_W<'_, HUFFENC_AC0_27rs> {
        HCODE55_W::new(self, 16)
    }
    ///Bits 24:27 - Huffman length 55
    #[inline(always)]
    pub fn hlen55(&mut self) -> HLEN55_W<'_, HUFFENC_AC0_27rs> {
        HLEN55_W::new(self, 24)
    }
}
/**JPEG Huffman encoder AC0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0_27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0_27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFENC_AC0_27)*/
pub struct HUFFENC_AC0_27rs;
impl crate::RegisterSpec for HUFFENC_AC0_27rs {
    type Ux = u32;
}
///`read()` method returns [`huffenc_ac0_27::R`](R) reader structure
impl crate::Readable for HUFFENC_AC0_27rs {}
///`write(|w| ..)` method takes [`huffenc_ac0_27::W`](W) writer structure
impl crate::Writable for HUFFENC_AC0_27rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFENC_AC0_27 to value 0
impl crate::Resettable for HUFFENC_AC0_27rs {}
