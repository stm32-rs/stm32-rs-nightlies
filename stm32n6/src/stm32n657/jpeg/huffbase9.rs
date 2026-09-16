///Register `HUFFBASE9` reader
pub type R = crate::R<HUFFBASE9rs>;
///Register `HUFFBASE9` writer
pub type W = crate::W<HUFFBASE9rs>;
///Field `DATA18` reader - Data 18
pub type DATA18_R = crate::FieldReader<u16>;
///Field `DATA18` writer - Data 18
pub type DATA18_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA19` reader - Data 19
pub type DATA19_R = crate::FieldReader<u16>;
///Field `DATA19` writer - Data 19
pub type DATA19_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 18
    #[inline(always)]
    pub fn data18(&self) -> DATA18_R {
        DATA18_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 19
    #[inline(always)]
    pub fn data19(&self) -> DATA19_R {
        DATA19_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE9")
            .field("data18", &self.data18())
            .field("data19", &self.data19())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 18
    #[inline(always)]
    pub fn data18(&mut self) -> DATA18_W<'_, HUFFBASE9rs> {
        DATA18_W::new(self, 0)
    }
    ///Bits 16:24 - Data 19
    #[inline(always)]
    pub fn data19(&mut self) -> DATA19_W<'_, HUFFBASE9rs> {
        DATA19_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE9)*/
pub struct HUFFBASE9rs;
impl crate::RegisterSpec for HUFFBASE9rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase9::R`](R) reader structure
impl crate::Readable for HUFFBASE9rs {}
///`write(|w| ..)` method takes [`huffbase9::W`](W) writer structure
impl crate::Writable for HUFFBASE9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE9 to value 0
impl crate::Resettable for HUFFBASE9rs {}
