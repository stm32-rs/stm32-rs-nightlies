///Register `HUFFBASE19` reader
pub type R = crate::R<HUFFBASE19rs>;
///Register `HUFFBASE19` writer
pub type W = crate::W<HUFFBASE19rs>;
///Field `DATA38` reader - Data 38
pub type DATA38_R = crate::FieldReader<u16>;
///Field `DATA38` writer - Data 38
pub type DATA38_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA39` reader - Data 39
pub type DATA39_R = crate::FieldReader<u16>;
///Field `DATA39` writer - Data 39
pub type DATA39_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 38
    #[inline(always)]
    pub fn data38(&self) -> DATA38_R {
        DATA38_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 39
    #[inline(always)]
    pub fn data39(&self) -> DATA39_R {
        DATA39_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE19")
            .field("data38", &self.data38())
            .field("data39", &self.data39())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 38
    #[inline(always)]
    pub fn data38(&mut self) -> DATA38_W<'_, HUFFBASE19rs> {
        DATA38_W::new(self, 0)
    }
    ///Bits 16:24 - Data 39
    #[inline(always)]
    pub fn data39(&mut self) -> DATA39_W<'_, HUFFBASE19rs> {
        DATA39_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE19)*/
pub struct HUFFBASE19rs;
impl crate::RegisterSpec for HUFFBASE19rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase19::R`](R) reader structure
impl crate::Readable for HUFFBASE19rs {}
///`write(|w| ..)` method takes [`huffbase19::W`](W) writer structure
impl crate::Writable for HUFFBASE19rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE19 to value 0
impl crate::Resettable for HUFFBASE19rs {}
