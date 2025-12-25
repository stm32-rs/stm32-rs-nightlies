///Register `HUFFBASE26` reader
pub type R = crate::R<HUFFBASE26rs>;
///Register `HUFFBASE26` writer
pub type W = crate::W<HUFFBASE26rs>;
///Field `DATA52` reader - Data 52
pub type DATA52_R = crate::FieldReader<u16>;
///Field `DATA52` writer - Data 52
pub type DATA52_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA53` reader - Data 53
pub type DATA53_R = crate::FieldReader<u16>;
///Field `DATA53` writer - Data 53
pub type DATA53_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 52
    #[inline(always)]
    pub fn data52(&self) -> DATA52_R {
        DATA52_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 53
    #[inline(always)]
    pub fn data53(&self) -> DATA53_R {
        DATA53_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE26")
            .field("data52", &self.data52())
            .field("data53", &self.data53())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 52
    #[inline(always)]
    pub fn data52(&mut self) -> DATA52_W<'_, HUFFBASE26rs> {
        DATA52_W::new(self, 0)
    }
    ///Bits 16:24 - Data 53
    #[inline(always)]
    pub fn data53(&mut self) -> DATA53_W<'_, HUFFBASE26rs> {
        DATA53_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE26)*/
pub struct HUFFBASE26rs;
impl crate::RegisterSpec for HUFFBASE26rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase26::R`](R) reader structure
impl crate::Readable for HUFFBASE26rs {}
///`write(|w| ..)` method takes [`huffbase26::W`](W) writer structure
impl crate::Writable for HUFFBASE26rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE26 to value 0
impl crate::Resettable for HUFFBASE26rs {}
